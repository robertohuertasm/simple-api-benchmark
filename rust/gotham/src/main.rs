use gotham::state::State;

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, "Hello World")
}

fn main() {
    let addr = "0.0.0.0:8000";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}
