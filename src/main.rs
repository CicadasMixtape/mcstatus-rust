use crate::info::Info;

mod parser;
mod info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parser::parse();

    let edition = args.edition.as_str();
    let ip = args.ip;

    let url = format!("https://api.mcstatus.io/v2/status/{}/{}", edition, ip);

    let client = reqwest::Client::new();
    let response: Info = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    println!("{:#?}", response);

    Ok(())
}