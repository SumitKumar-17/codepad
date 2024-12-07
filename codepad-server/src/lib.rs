//! Server backend for the Codepad collaborative text editor.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::sync::Arc;

use dashmap::DashMap;
use codepad::Codepad;
use warp::{filters::BoxedFilter, ws::Ws, Filter, Reply};

mod codepad;

/// A combined filter handling all server routes.
pub fn server() -> BoxedFilter<(impl Reply,)> {
    warp::path("api").and(backend()).or(frontend()).boxed()
}

/// Construct routes for static files from React.
fn frontend() -> BoxedFilter<(impl Reply,)> {
    warp::fs::dir("build")
        .or(warp::get().and(warp::fs::file("build/index.html")))
        .boxed()
}

/// Construct backend routes, including WebSocket handlers.
fn backend() -> BoxedFilter<(impl Reply,)> {
    let codepad_map: Arc<DashMap<String, Arc<Codepad>>> = Default::default();
    let codepad_map = warp::any().map(move || Arc::clone(&codepad_map));

    let socket = warp::path("socket")
        .and(warp::path::param())
        .and(warp::path::end())
        .and(warp::ws())
        .and(codepad_map.clone())
        .map(
            |id: String, ws: Ws, codepad_map: Arc<DashMap<String, Arc<Codepad>>>| {
                let codepad = codepad_map.entry(id).or_default();
                let codepad = Arc::clone(codepad.value());
                ws.on_upgrade(move |socket| async move { codepad.on_connection(socket).await })
            },
        );

    let text = warp::path("text")
        .and(warp::path::param())
        .and(warp::path::end())
        .and(codepad_map.clone())
        .map(
            |id: String, codepad_map: Arc<DashMap<String, Arc<Codepad>>>| {
                codepad_map
                    .get(&id)
                    .map(|codepad| codepad.text())
                    .unwrap_or_default()
            },
        );

    socket.or(text).boxed()
}
