use std::borrow::Borrow;
use std::error::Error;

use structopt::StructOpt;
use serde::{Serialize, Deserialize};

#[derive(StructOpt, Debug, Serialize, Deserialize)]
#[structopt(name = "client")]
struct ClientOptions {
    // The IP address to bind to
    #[structopt(name="name", short, long)]
    name: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = ClientOptions::from_args();
    let mut greeting_post:String = "{\"name\":\"".to_string();
    greeting_post.push_str(&opt.name);
    greeting_post.push_str("\"}");

    //let body = reqwest::get("http://localhost:3000").await?.text().await?;
    //let body = reqwest::post("http://localhost:3000/v1/greet").body(greetingPost).await?.text().await?;

    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:3000/v1/greet")
        .header("Content-Type", "application/json")
        .json(&opt)
        .send()
        .await?;

    let body = resp.bytes().await?;


    println!("{}", String::from_utf8_lossy(body.borrow()).to_string());
    Ok(())
}
