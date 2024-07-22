use tokio::main;
use reqwest::get;
use std::collections::HashMap;

#[main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = get("https://httpbin.org/ip").await?;
    let json: HashMap<String, String> = response.json().await?;
    println!("Enviado desde IP origen: {:#?}", json);
    Ok(())
}