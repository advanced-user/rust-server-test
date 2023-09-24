use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/hello")]
pub async fn test_hello() -> HttpResponse {
    HttpResponse::Ok().json("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(test_hello))
        .bind("localhost:8080")?
        .run()
        .await
}
