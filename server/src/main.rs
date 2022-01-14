use std::net::SocketAddr;

use axum::routing::{get, post};
use axum::Router;
use axum::extract;
use axum::Json;
use serde::Deserialize;
use serde_json::{Value, json};
#[derive(Deserialize)]
struct Greeting {
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root)).route("/v1/greet", post(greet_person));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn greet_person(payload: extract::Json<Greeting>) -> String {
    let payload: Greeting = payload.0;
    let mut temp_string:String = "hello, ".to_string();
    temp_string.push_str(&payload.name.to_owned());
    temp_string
}
