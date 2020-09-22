use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn say_hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    return format!("Hello {name}", name = &name);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(say_hello))
            .route("/{name}", web::get().to(say_hello))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
