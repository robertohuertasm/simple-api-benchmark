extern crate actix_web;
use actix_web::*;

fn main() {
    HttpServer::new(|| Application::new().resource("/", |r| r.f(|_| "Hello World")))
        .bind("0.0.0.0:8888")
        .unwrap()
        .run();
}
