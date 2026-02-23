#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let body = reqwest::get("https://lichess.org/api/puzzle/next")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{body}");
}
