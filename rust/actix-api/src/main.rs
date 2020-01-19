use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Server running in port 8080");
    HttpServer::new(|| App::new().service(web::resource("/").to(|| async { "Hello World" })))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
