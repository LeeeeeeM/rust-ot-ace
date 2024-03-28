use business::RustDoc;
use std::sync::Arc;
use warp::{filters::BoxedFilter, fs, path, ws, ws::Ws, Filter, Reply, any};

mod business;

pub fn server() -> BoxedFilter<(impl Reply,)> {
    path("api").and(backend()).or(frontend()).boxed()
}

// serve static resource
fn frontend() -> BoxedFilter<(impl Reply,)> {
    fs::dir("build")
        .or(warp::get().and(fs::file("build/index.html")))
        .boxed()
}

// backend service api
fn backend() -> BoxedFilter<(impl Reply,)> {
    let rust_doc = Arc::new(RustDoc::new());
    let rust_doc = any().map(move || Arc::clone(&rust_doc));

    // ws api
    let socket = path("socket")
        // .and(path::param()) // restful params
        .and(path::end())
        .and(ws())
        .and(rust_doc.clone())
        .map(
            |ws: Ws, rust_doc: Arc<RustDoc>| {
                ws.on_upgrade(|socket| async move {
                    rust_doc.on_connection(socket).await
                })
            },
        );

    // api
    let text = path("text")
        // .and(path::param())
        .and(path::end())
        .and(rust_doc.clone())
        .map(|rust_doc: Arc<RustDoc>| rust_doc.text());

    socket.or(text).boxed()
}
