extern crate futures;
extern crate thruster;

use futures::future;
use thruster::{App, BasicContext as Ctx, MiddlewareChain, MiddlewareReturnValue};

fn hello(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {
    context.body = "Hello World".to_string();
    Box::new(future::ok(context))
}

fn main() {
    let mut app = App::<Ctx>::new();
    app.get("/", vec![hello]);
    App::start(app, "0.0.0.0", 8200);
    println!("Server running on port 8200");
}
