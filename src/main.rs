use payments::start_server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    return start_server()?.await;
}
