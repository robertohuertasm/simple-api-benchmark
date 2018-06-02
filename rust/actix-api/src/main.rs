extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder};

fn greet(_req: HttpRequest) -> impl Responder {
    "Hello World"
}

fn main() {
    println!("Server running in port 8000");
    server::new(|| App::new().resource("/", |r| r.f(greet)))
        .bind("0.0.0.0:8000")
        .expect("Can not bind to port 8000")
        .run();
}
