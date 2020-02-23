use franz::client::connect;

#[tokio::main]
async fn main() {
    env_logger::init();

    let _client = connect("127.0.0.1:9092".parse().unwrap()).await.unwrap();
    println!("Stopping.");
}
