use axum::{routing::get, Router};
use tokio::net::TcpListener;
use dotenv::dotenv;
use std::env;

mod handlers;
mod models;
mod state;
use handlers::root;
use state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("TMDB_API_KEY").expect("TMDB_API_KEY must be set in .env");
    let state = AppState {
        tmdb_api_key: api_key,
        client: reqwest::Client::new(),
    };
    let app = Router::new().route("/", get(root)).with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}