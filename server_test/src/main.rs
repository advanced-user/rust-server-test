use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/hello")]
pub async fn test_hello() -> HttpResponse {
    HttpResponse::Ok().json("Hello leha!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(test_hello))
        .bind("0.0.0.0:8888")?
        .run()
        .await
}
