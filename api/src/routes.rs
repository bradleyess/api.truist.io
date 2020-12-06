use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn health_check(_req: HttpRequest) -> impl Responder {
    return HttpResponse::Ok();
}

pub async fn say_hello(req: HttpRequest) -> impl Responder {
    let name: &str = req.match_info().get("name").unwrap_or("world");
    return format!("Hello {name}", name = &name);
}
