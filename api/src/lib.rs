use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
mod routes;

pub fn start_server() -> Result<Server, std::io::Error> {
    let server: Server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::say_hello))
            .route("/{name}", web::get().to(routes::say_hello))
            .route("/checks/health", web::get().to(routes::health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();

    return Ok(server);
}
