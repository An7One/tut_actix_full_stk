use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../db_access.rs"]
mod db_access;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let url_db = env::var("DATABASE_URL").expect("DATABASE_URL is missings");
    let pool_db = PgPoolOptions::new().connect(&url_db).await.unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK".to_owned(),
        visit_count: Mutex::new(0),
        db: pool_db,
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routers)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
