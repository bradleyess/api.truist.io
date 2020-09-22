use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check(_req: HttpRequest) -> HttpResponse {
    return HttpResponse::Ok().finish();
}

async fn say_hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    return format!("Hello {name}", name = &name);
}

pub fn start_server() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(say_hello))
            .route("/{name}", web::get().to(say_hello))
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();

    return Ok(server);
}
