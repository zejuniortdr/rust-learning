use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt};

#[tokio::main]
async fn main() {
    let url = "ws://127.0.0.1:8080/ws";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to WebSocket server!");

    let (_, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => println!("New price: {}", text),
            _ => {}
        }
    }
}
