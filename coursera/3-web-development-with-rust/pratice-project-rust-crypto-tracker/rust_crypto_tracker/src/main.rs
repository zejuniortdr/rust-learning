use warp::Filter;
use futures_util::{StreamExt, SinkExt};
use reqwest::Client;
use serde::Deserialize;
use tokio::sync::broadcast;
use std::time::Duration;
// Um vetor de ativos

#[derive(Debug, Deserialize)]
struct CryptoPrice {
    asset_id: String,
    price_usd: f64,
}


#[derive(Debug, Deserialize)]
struct CoinApiResponse(Vec<CryptoPrice>);

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel::<String>(100); // Broadcasting cryptocurrency prices to clients

    // Start monitoring prices asynchronously
    tokio::spawn(fetch_crypto_prices(tx.clone()));

    // WebSocket server
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx = tx.clone(); // Clone tx before passing it into the closure
            ws.on_upgrade(move |socket| handle_connection(socket, tx)) // Accept warp WebSocket
        });

    // Start the server
    warp::serve(ws_route)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

// Handle a new WebSocket connection (accept warp WebSocket)
async fn handle_connection(ws: warp::ws::WebSocket, tx: broadcast::Sender<String>) {
    let (mut tx_socket, _rx_socket) = ws.split();

    // Broadcast updates to the connected clients
    let mut rx = tx.subscribe();
    while let Ok(message) = rx.recv().await {
        // Convert message to &str and send it to the WebSocket client
        let _ = tx_socket.send(warp::ws::Message::text(message.as_str())).await;
    }
}

// Function to periodically fetch cryptocurrency prices
async fn fetch_crypto_prices(tx: broadcast::Sender<String>) {
    let client = Client::new();
    loop {
        let response: CoinApiResponse = client
            .get("https://rest.coinapi.io/v1/assets/BTC") // Example URL from CoinAPI
            .header("X-CoinAPI-Key", "21292411-403d-47b4-8950-aa0758a6642c")
            .send()
            .await
            .expect("Failed to fetch data")
            .json()
            .await
            .expect("Failed to parse response");

        // Format and send the prices to the WebSocket clients
        for price in response.0.iter() {
            let message = format!(
                "Cryptocurrency: {}, Price (USD): {}",
                price.asset_id, price.price_usd
            );
            let _ = tx.send(message);
        }

        // Sleep for a while before the next API call
        tokio::time::sleep(Duration::from_secs(30)).await; // Adjust the frequency as needed
    }
}
