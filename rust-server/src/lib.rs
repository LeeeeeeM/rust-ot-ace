use business::RustDoc;
use std::sync::Arc;
use warp::{any, filters::BoxedFilter, path, ws, ws::Ws, Filter, Reply};

mod business;

pub fn server() -> BoxedFilter<(impl Reply,)> {
    // path("api").and(backend()).or(frontend()).boxed()
    routes().boxed()
}

// // serve static resource
// fn frontend() -> BoxedFilter<(impl Reply,)> {
//     fs::dir("build")
//         .or(warp::get().and(fs::file("build/index.html")))
//         .boxed()
// }

// backend service api
fn routes() -> BoxedFilter<(impl Reply,)> {
    let rust_doc = Arc::new(RustDoc::new());
    let rust_doc = any().map(move || Arc::clone(&rust_doc));

    // ws api
    let socket_prefix = path("ws");
    let socket = socket_prefix
        .and(path("socket"))
        // .and(path::param()) // restful params
        .and(path::end())
        .and(ws())
        .and(rust_doc.clone())
        .map(|ws: Ws, rust_doc: Arc<RustDoc>| {
            ws.on_upgrade(|socket| async move { rust_doc.on_connection(socket).await })
        });

    // api
    let api_prefix = path("api");
    let text = api_prefix
        .and(path("text"))
        // .and(path::param())
        .and(path::end())
        .and(rust_doc.clone())
        .map(|rust_doc: Arc<RustDoc>| rust_doc.text());

    socket.or(text).boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_single_message() {
        let routes = routes();
        let mut client = warp::test::ws()
            .path("/ws/socket")
            .handshake(routes)
            .await
            .expect("handshake");
        client.send_text("hello world").await;
        let connected = client.recv().await.expect("recv");
        let connected = connected.to_str().expect("init recv");
        assert_eq!(connected, "{\"Identity\":0}");
    }
}
