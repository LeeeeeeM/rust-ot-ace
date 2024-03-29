use std::net::{Ipv4Addr, SocketAddrV4};

use rust_ws_server::server;
use warp::serve;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| String::from("3030"))
        .parse()
        .expect("Unable to parse port");

    let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);

    serve(server()).run(socket).await;
}
