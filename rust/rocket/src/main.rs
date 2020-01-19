#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{config::Environment, get, routes, Config};

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

fn main() {
    let config = Config::build(Environment::Production)
        .address("0.0.0.0")
        .port(8000)
        .finalize()
        .unwrap();

    rocket::custom(config).mount("/", routes![index]).launch();
}
