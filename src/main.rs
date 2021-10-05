#[macro_use]
extern crate diesel;

use actix_web::web;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use diesel::r2d2::ConnectionManager;

use crate::db::{DbPool, DbService};

mod auth;
mod configuration;
mod db;
mod domain;
mod handlers;
mod routes_configure;

#[macro_use]
extern crate diesel_migrations;
use diesel_migrations::{EmbedMigrations, embed_migrations};
embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = configuration::env_database_url();

    let db_pool = DbPool::new(ConnectionManager::new(database_url)).unwrap();
    // Apply migrations
    println!("Running migration...");
    embedded_migrations::run_with_output(&db_pool.get().unwrap(), &mut std::io::stdout());

    let db_service = DbService::new(db_pool.clone());

    let jwt_config_data = Data::new(configuration::load_jwt_config());

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            // data
            .app_data(jwt_config_data.clone())
            .data(db_pool.clone())
            .data(db_service.clone())
            // routes
            .configure(routes_configure::configure_authed_section)
            .configure(routes_configure::configure_sign)
            .route("/ping", web::get().to(handlers::ping))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
