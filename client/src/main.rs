use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let body = reqwest::get("http://localhost:3000").await?.text().await?;

    println!("{}", body);
    Ok(())
}
