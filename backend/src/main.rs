use std::{net::SocketAddr, env};

use axum::{
    Router,
    routing::get,
};
use tokio;

mod http;
mod actions;

#[derive(Clone)]
struct DBConnection {
    conn: sqlx::Pool<sqlx::Postgres>,
}

#[tokio::main]
async fn main() {
    let db_address = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = sqlx::postgres::PgPool::connect(&db_address).await.unwrap();
    let db_connection = DBConnection {
        conn: db_pool,
    };
    // axum boilerplate
    let app = Router::new()
        .route("/test", get(print_test))
        .with_state(db_connection);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn print_test() -> &'static str {
    "test"
}
