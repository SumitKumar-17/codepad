//! Server backend for the Letterpad collaborative text editor

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use warp::{filters::BoxedFilter, Filter, Reply};
mod server;

fn frontend() -> BoxedFilter<(impl Reply,)> {
    /// Construct routes for static files from React
    warp::fs::dir("dist")
        .or(warp::get().and(warp::fs::file("dist/index.html")))
        .boxed()
}

fn backend() -> BoxedFilter<(impl Reply,)> {
    /// Construct backend routes, including WebSocket handlers
    server::routes()
}

fn server() -> BoxedFilter<(impl Reply,)> {
    /// A combined filter handling all server routes
    warp::path("api").and(backend()).or(frontend()).boxed()
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| String::from("3030"))
        .parse()
        .expect("Unable to parse PORT");

    warp::serve(server()).run(([0, 0, 0, 0], port)).await;
}
