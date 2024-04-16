use fantoccini::{Client, ClientBuilder};
use serde_json::json;
use std::fs::File;
use std::io::Write;
use tokio;

pub async fn scrape_content(url: &str) -> Result<String, fantoccini::error::CmdError> {
    let mut client = ClientBuilder::native().connect("http://localhost:4444").await?;
    client.goto(url).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await; // wait for the page to load

    let body = client.find(fantoccini::Locator::Css("body")).await?;
    let content = body.text().await?;
    client.close().await?;

    Ok(content)
}

pub fn export_to_json(data: &str, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    let content = json!({ "content": data });
    writeln!(file, "{}", serde_json::to_string_pretty(&content).unwrap())
}
