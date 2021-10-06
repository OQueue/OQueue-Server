use std::ops::{Add, Sub};
use std::time::{Duration, UNIX_EPOCH};

use actix_web::error::*;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpRequest, Responder};
use chrono::{FixedOffset, NaiveDateTime, Utc};
use log::{debug, error, info};
use uuid::Uuid;

use crate::auth::{Auth, JwtConfig};
use crate::db::actions as db_actions;
use crate::db::actions::QueueEntryToAdd;
use crate::db::models::{QueueDao, QueueEntryDao, UserDao};
use crate::db::{DbConnection, DbPool, DbService};
use crate::domain::{MemberInfo, QueueInfo, UserInfo};
use crate::handlers::req::*;

pub mod req;

type Error = actix_web::Error;
type RespResult<T> = std::result::Result<T, Error>;

impl From<crate::db::Error> for Error {
    fn from(e: crate::db::Error) -> Self {
        // TODO: add error wrapper
        error!("{:?}", &e);
        ErrorInternalServerError("")
    }
}

const MAX_NAME_LENGTH: usize = 35;
const MIN_NAME_LENGTH: usize = 5;

fn check_user_name(name: &str) -> Result<(), String> {
    match name.chars().count() {
        MIN_NAME_LENGTH..=MAX_NAME_LENGTH => Ok(()),
        _ => {
            let msg = format!(
                "Имя слишком короткое. Оно должно быть не менее {min} символов и не более {max}.",
                min = MIN_NAME_LENGTH,
                max = MAX_NAME_LENGTH,
            );
            Err(msg)
        }
    }
}

fn normalize_email(email: &str) -> RespResult<String> {
    Ok(email.to_string().to_lowercase())
}

// --------
// handlers
// --------

pub async fn ping() -> impl Responder {
    format!("Pong!")
}

pub async fn sign_up(db: Data<DbService>, data: Json<SignUp>) -> RespResult<impl Responder> {
    let SignUp {
        email,
        name,
        password,
    } = data.0;

    check_user_name(&name).map_err(|e| ErrorBadRequest(e))?;
    let email = normalize_email(&email)?;

    // Проверяем наличие такого же пользователя
    let is_exist = db.has_user_with_email(&email)?;

    if is_exist {
        return Err(ErrorBadRequest(format!(
            "User with this email {} already registered.",
            &email
        )));
    }

    // Создаем и добавляем нового пользователя
    let user_uuid = Uuid::new_v4();
    let pwhash =
        bcrypt::hash(&password, bcrypt::DEFAULT_COST).map_err(|_| ErrorInternalServerError(""))?;

    let user = UserDao {
        id: user_uuid,
        name,
        email,
        pwhash,
    };

    db.add_user(&user)?;

    Ok("")
}

pub async fn sign_in(
    req: HttpRequest,
    db: Data<DbService>,
    data: Json<SignIn>,
) -> RespResult<Json<SignInResponse>> {
    let SignIn { login, password } = data.0;

    const ILLEGAL_LOGIN_INFO_MSG: &str = "Illegal login or password.";
    let user = db
        .user_by_email(&login)?
        .ok_or_else(|| ErrorBadRequest(ILLEGAL_LOGIN_INFO_MSG))?;

    let pass_is_correct = bcrypt::verify(password, &user.pwhash).map_err(|e| {
        error!("{:?}", e);
        ErrorInternalServerError("")
    })?;

    if pass_is_correct {
        let exp = std::time::SystemTime::now()
            .add(Duration::from_secs(60 * 60 * 24))
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let auth = Auth {
            id: user.id.clone(),
            exp,
        };
        let jwt_config = req.app_data::<Data<JwtConfig>>().unwrap().get_ref();
        let token = crate::auth::encode_token(&auth, jwt_config)
            .map_err(|_e| ErrorInternalServerError(""))?;
        Ok(Json(SignInResponse { token }))
    } else {
        Err(ErrorBadRequest(ILLEGAL_LOGIN_INFO_MSG))
    }
}

pub async fn me(auth: Auth, db: Data<DbService>) -> impl Responder {
    db.user_by_id(&auth.id)?
        .ok_or(ErrorBadRequest("User with this id is not found"))
        .map(|dao| {
            let UserDao { id, name, .. } = dao;
            UserInfo { id, name }
        })
        .map(|x| Json(x))
}

pub async fn user(user_id: Path<Uuid>, db: Data<DbService>) -> impl Responder {
    let user_id = user_id.as_ref();
    db.user_by_id(user_id)?
        .ok_or(ErrorBadRequest("User with this id is not found"))
        .map(|dao| {
            let UserDao { id, name, .. } = dao;
            UserInfo { id, name }
        })
        .map(|x| Json(x))
}

pub async fn queue_create(
    auth: Auth,
    db: Data<DbService>,
    data: Json<CreateQueue>,
) -> RespResult<&'static str> {
    let CreateQueue {
        name,
        description,
    } = data.0;

    let now = Utc::now().naive_utc();

    let queue_id = Uuid::new_v4();

    let queue = QueueDao {
        id: queue_id,
        name,
        description,
        organizer_id: auth.id.clone(),
        created_at: now,
        exists_before: Utc::now().add(chrono::Duration::days(365 * 2)).naive_utc(),
    };

    db.add_queue(&queue)?;

    let me_entry = QueueEntryToAdd {
        queue_id,
        user_id: auth.id,
        has_priority: false,
        joined_at: now,
    };

    db.add_entry(&me_entry)?;

    Ok("")
}

pub async fn queue_delete(
    auth: Auth,
    queue_id: Path<Uuid>,
    db: Data<DbService>,
) -> impl Responder {
    let queue_id = queue_id.into_inner();

    let queue = match db.queue_by_id(&queue_id)? {
        Some(q) => q,
        None => return Err(ErrorBadRequest("Queue is not exist")),
    };

    if queue.organizer_id != auth.id {
        return Err(ErrorBadRequest("You is not queue organiser."));
    }

    db.delete_queue(&queue_id)?;
    Ok("")
}

pub async fn queue_get_info(
    _auth: Auth,
    queue_id: Path<Uuid>,
    db: Data<DbService>,
) -> RespResult<Json<QueueInfo>> {
    let queue_id = queue_id.into_inner();

    let queue = db
        .queue_by_id(&queue_id)?
        .ok_or(ErrorBadRequest("Queue is not exist"))?;

    let QueueDao {
        id,
        name,
        description,
        organizer_id,
        created_at,
        exists_before,
    } = queue;

    Ok(Json(QueueInfo {
        id,
        name,
        description,
        organizer_id,
        created_at,
        exists_before,
    }))
}

pub async fn queues(auth: Auth, db: Data<DbService>) -> RespResult<Json<Vec<QueueInfo>>> {
    let queue_infos = db
        .available_queues(&auth.id)?
        .into_iter()
        .map(|dao| {
            let QueueDao {
                id,
                name,
                description,
                organizer_id,
                created_at,
                exists_before,
            } = dao;
            QueueInfo {
                id,
                name,
                description,
                organizer_id,
                created_at,
                exists_before,
            }
        })
        .collect::<Vec<_>>();
    Ok(Json(queue_infos))
}

pub async fn queue_members(
    _auth: Auth,
    queue_id: Path<Uuid>,
    db: Data<DbService>,
) -> RespResult<Json<Vec<MemberInfo>>> {
    let queue_id = queue_id.into_inner();

    let entries = db.entries_ordered(&queue_id)?;

    let entries = entries
        .into_iter()
        .map(|entry| {
            let QueueEntryDao {
                user_id,
                order,
                has_priority,
                is_held,
                joined_at,
                ..
            } = entry;

            MemberInfo {
                id: user_id,
                order,
                has_priority,
                is_held,
                joined_at,
            }
        })
        .collect::<Vec<_>>();

    Ok(Json(entries))
}

async fn queue_join_inner(db: Data<DbService>, queue_id: Uuid, user_id: Uuid) -> RespResult<&'static str> {
    let entry = QueueEntryToAdd {
        queue_id,
        user_id,
        has_priority: false,
        joined_at: Utc::now().naive_utc(),
    };

    db.add_entry(&entry)?;
    Ok("")
}

pub async fn queue_add_member(
    _me: Auth,
    db: Data<DbService>,
    in_path: Path<(Uuid, Uuid)>,
) -> RespResult<&'static str> {
    let (queue_id, user_id) = in_path.into_inner();
    queue_join_inner(db, queue_id, user_id).await
}

pub async fn queue_add_member_me(
    me: Auth,
    db: Data<DbService>,
    in_path: Path<Uuid>,
) -> RespResult<&'static str> {
    let queue_id = in_path.into_inner();
    queue_join_inner(db, queue_id, me.id).await
}


async fn queue_remove_member_inner(db: Data<DbService>, queue_id: Uuid, user_id: Uuid) -> RespResult<&'static str> {
    db.delete_entry(&queue_id, &user_id)?;
    Ok("")
}

pub async fn queue_remove_member_me(
    me: Auth,
    db: Data<DbService>,
    queue_id: Path<Uuid>,
) -> RespResult<&'static str> {
    let queue_id = queue_id.into_inner();
    queue_remove_member_inner(db, queue_id, me.id).await
}

pub async fn queue_remove_member(
    _me: Auth,
    db: Data<DbService>,
    in_path: Path<(Uuid, Uuid)>,
) -> RespResult<&'static str> {
    let (queue_id, user_id) = in_path.into_inner();
    queue_remove_member_inner(db, queue_id, user_id).await
}


