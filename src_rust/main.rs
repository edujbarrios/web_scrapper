mod scraper;

use std::io;
use tokio;

#[tokio::main]
async fn main() {
    println!("Web Scraper Menu");
    println!("1. Enter URL to scrape");
    println!("2. Exit");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => {
            println!("Enter the URL:");
            let mut url = String::new();
            io::stdin().read_line(&mut url).expect("Failed to read URL");
            let url = url.trim();

            match scraper::scrape_content(url).await {
                Ok(content) => {
                    let filename = "output.json";
                    scraper::export_to_json(&content, filename).expect("Failed to export to JSON");
                    println!("Content exported successfully to '{}'", filename);
                }
                Err(e) => println!("Error scraping content: {:?}", e),
            }
        }
        "2" => println!("Exiting program."),
        _ => println!("Invalid choice."),
    }
}
