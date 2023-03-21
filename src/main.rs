use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let public_api_key = fs::read_to_string("api_key.txt").expect("Should have been able to read the file");
    let private_api_key = fs::read_to_string("private_api_key.txt").expect("Should have been able to read the file");
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .text()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}