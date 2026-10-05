use axum::{routing::get, Router};
use tokio::net::TcpListener;
use dotenv::dotenv;
use std::env;

mod handlers;

use handlers::root;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("TMDB_API_KEY").expect("TMDB_API_KEY must be set in .env");
    let app = Router::new().route("/", get(root));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}