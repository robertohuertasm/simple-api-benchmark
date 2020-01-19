use thruster::server::Server;
use thruster::thruster_proc::{async_middleware, middleware_fn};
use thruster::ThrusterServer;
use thruster::{App, BasicContext as Ctx, Request};
use thruster::{MiddlewareNext, MiddlewareReturnValue};

#[middleware_fn]
async fn hello(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> Ctx {
    let val = "Hello World";
    context.body(val);
    context
}

fn main() {
    let mut app = App::<Request, Ctx>::new_basic();
    app.get("/", async_middleware!(Ctx, [hello]));
    let server = Server::new(app);
    println!("Server running on port 4321");
    server.start("0.0.0.0", 4321);
}
