use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::any().map(|| "Hello World");
    println!("Server started at port 8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
