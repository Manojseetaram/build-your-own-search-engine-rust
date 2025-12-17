use scraper::{Html , Selector};
#[tokio::main]
async fn main() {
    let html = reqwest::get("https://example.com").await.unwrap().text().await.unwrap();
    let document = Html::parse_document(&html);
    let link_selector = Selector::parse("a").unwrap();
    println!("Links found : ");
    for element in document.select(&link_selector){
    //    if let text = body.text().collect::<Vec<_>>().join(" ");
     if let Some(link) = element.value().attr("href"){
    println!("{}" , link);
     }
        // println!("Page Text :\n{}" , text);
    }
   
    println!("Hello, world!");
}
