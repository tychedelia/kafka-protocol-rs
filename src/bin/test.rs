use std::time::Duration;

use franz::client::connect;

#[tokio::main]
async fn main() {
    let _client = connect("127.0.0.1:9092".parse().unwrap()).await.unwrap();
    tokio::time::delay_for(Duration::from_secs(10)).await;
    println!("Stopping.");
}
