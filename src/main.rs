use scraper::{Html , Selector};
#[tokio::main]
async fn main() {
    let html = reqwest::get("https://example.com").await.unwrap().text().await.unwrap();
    
    println!("{}" , html);
    println!("Hello, world!");
}
