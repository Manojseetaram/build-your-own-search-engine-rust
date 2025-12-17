use scraper::{Html , Selector};
#[tokio::main]
async fn main() {
    let html = reqwest::get("https://example.com").await.unwrap().text().await.unwrap();
    let document = Html::parse_document(&html);
    let body_selector = Selector::parse("body").unwrap();
    for body in document.select(&body_selector){
        let text = body.text().collect::<Vec<_>>().join(" ");
        println!("Page Text :\n{}" , text);
    }
   
    println!("Hello, world!");
}
