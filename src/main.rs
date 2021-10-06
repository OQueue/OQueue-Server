#[macro_use]
extern crate diesel;

use actix_web::web;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::r2d2::ConnectionManager;

use crate::db::{DbPool, DbService};

mod auth;
mod configuration;
mod db;
mod domain;
mod handlers;

#[macro_use]
extern crate diesel_migrations;
use diesel_migrations::{embed_migrations, EmbedMigrations};
embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = configuration::env_database_url();
    let host_url = configuration::env_host().unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let jwt_config_data = Data::new(configuration::load_jwt_config());

    let db_pool = DbPool::new(ConnectionManager::new(database_url)).unwrap();
    // Apply migrations
    println!("Running migration...");
    embedded_migrations::run_with_output(&db_pool.get().unwrap(), &mut std::io::stdout());

    let db_service = DbService::new(db_pool.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            // data
            .app_data(jwt_config_data.clone())
            .data(db_pool.clone())
            .data(db_service.clone())
            // routes
            .configure(configure_routes)
            .route("/ping", web::get().to(handlers::ping))
    })
    .bind(&host_url)?
    .run()
    .await
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(handlers::sign_up))
            .route("/signin", web::post().to(handlers::sign_in)),
    )
    .service(
        web::scope("/api")
            .wrap(HttpAuthentication::bearer(crate::auth::bearer_validator))
            .route("/users/{user_id}", web::get().to(handlers::user))
            .route("/queues", web::post().to(handlers::queue_create))
            .route("/queues", web::get().to(handlers::queues))
            .route("/queues/{queue_id}", web::delete().to(handlers::queue_delete))
            .route("/queues/{queue_id}", web::get().to(handlers::queue_get_info))
            .route(
                "/queues/{queue_id}/members",
                web::get().to(handlers::queue_members),
            )
            .route(
                "/queues/{queue_id}/members/me", // Join ME
                web::post().to(handlers::queue_add_member_me),
            )
            .route(
                "/queues/{queue_id}/members/{member_id}",
                web::post().to(handlers::queue_add_member),
            )
            .route(
                "/queues/{queue_id}/members/me", // Remove ME
                web::delete().to(handlers::queue_remove_member_me),
            )
            .route(
                "/queues/{queue_id}/members/{member_id}",
                web::delete().to(handlers::queue_remove_member),
            ),
    );
}
