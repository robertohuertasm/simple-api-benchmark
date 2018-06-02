extern crate actix_web;
use actix_web::{server, App, HttpRequest, Responder};

fn greet(_req: HttpRequest) -> impl Responder {
    "Hello World"
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(greet)))
        .bind("0.0.0.0:8888")
        .expect("Can not bind to port 8888")
        .run();
}
