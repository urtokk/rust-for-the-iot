use std::net::SocketAddr;

use axum::{
    Router,
    routing::get,
};
use tokio;

#[tokio::main]
async fn main() {
    // axum boilerplate
    let app = Router::new()
        .route("/test", get(print_test));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn print_test() -> &'static str {
    "test"
}
