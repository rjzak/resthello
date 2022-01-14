use std::net::SocketAddr;

use axum::routing::{get, post};
use axum::Router;
use axum::extract;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(Deserialize)]
struct Greeting {
    name: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "server")]
struct ServerOpt {
    // The IP address to bind to
    #[structopt(name="listen string", short, long, default_value="127.0.0.1:3000")]
    listen_addr: String
}

#[tokio::main]
async fn main() {
    let opt = ServerOpt::from_args();

    let app = Router::new().route("/", get(root)).route("/v1/greet", post(greet_person));

    let server: SocketAddr = opt.listen_addr
        .parse()
        .expect("Unable to parse socket address");
    let addr = SocketAddr::from(server);
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
