use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// to setup the route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("health", web::get().to(health_check_handler));
}

// to setup the handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web Service is running!")
}

// to instantiate the HTTP server
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // to construct the app, and setup the route
    let app = move || App::new().configure(general_routes);
    // to run the HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
