

pub mod network;
pub mod client;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::client::connect;

    #[tokio::test]
    async fn it_works() {
        let _client = connect("127.0.0.1:9092".parse().unwrap()).await.unwrap();
        tokio::time::delay_for(Duration::from_secs(10)).await;
        println!("Stopping.");

        assert!(false);
    }
}
