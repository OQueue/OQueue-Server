use actix_web::web;

use crate::handlers;
use actix_web_httpauth::middleware::HttpAuthentication;

// this function could be located in a different module
fn _routers_config(_cfg: &mut web::ServiceConfig) {
    // cfg.service(
    //     web::resource("/test")
    //         .route(web::get().to(|| HttpResponse::Ok().body("test")))
    //         .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    // );
}

pub fn configure_sign(cfg: &mut web::ServiceConfig) {
    cfg.route("/signup", web::post().to(handlers::sign_up))
        .route("/signin", web::post().to(handlers::sign_in));
}

pub fn configure_authed_section(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/authed")
            .wrap(HttpAuthentication::bearer(crate::auth::bearer_validator))
            .route("/get_user_info", web::post().to(handlers::user_by_id))
            .route("/create_queue", web::post().to(handlers::create_queue))
            .route("/delete_queue", web::post().to(handlers::delete_queue))
            .route("/queues", web::post().to(handlers::my_queues))
            .route("/get_queue_info", web::post().to(handlers::queue_by_id))
            .route("/get_members", web::post().to(handlers::get_members))
            .route("/join", web::post().to(handlers::join_to_queue))
            .route("/leave", web::post().to(handlers::leave_from_queue)),
    );
}
