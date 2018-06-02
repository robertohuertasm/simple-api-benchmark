extern crate futures;
extern crate thruster;

use thruster::{App, BasicContext as Ctx, MiddlewareChain, MiddlewareReturnValue, Request};
use futures::future;

fn generate_context(request: Request) -> Ctx {
    Ctx {
        body: "".to_string(),
        params: request.params().clone(),
        query_params: request.query_params().clone()
    }
}

fn hello(mut context: Ctx, _chain: &MiddlewareChain<Ctx>) -> MiddlewareReturnValue<Ctx> {
    context.body = "Hello World".to_string();
    Box::new(future::ok(context))
}

fn main() {
    let mut app = App::<Ctx>::create(generate_context);
    app.get("/", vec![hello]);
    App::start(app, "0.0.0.0", 8200);
    println!("Server running on port 8200");
}
