#[tokio::main]
async fn main() {
    let response = reqwest::get("").await.unwrap().text().await.unwrap();
    println!("{}" , response);
    println!("Hello, world!");
}
