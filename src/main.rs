use scraper::{Html , Selector};
#[tokio::main]
async fn main() {
    let html = reqwest::get("https://example.com").await.unwrap().text().await.unwrap();
    let document = Html::parse_document(&html);
    let selector = Selector::parse("title").unwrap();
    for element in document.select(&selector){
        let title = element.inner_html();
         println!("Title : {}" , title);
    }
   
    println!("Hello, world!");
}
