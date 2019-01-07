use warp::Filter;

fn main() {
    let routes = warp::any().map(|| "Hello World");
    warp::serve(routes).run(([0, 0, 0, 0], 8000));
}
