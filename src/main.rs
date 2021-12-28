use actix_web::{App, HttpServer};
use rust_web::routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().configure(routes::routes))
    .bind("localhost:8000")?
    .run()
    .await
}
