use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures_util::stream::StreamExt; // For `next`
use futures_util::sink::SinkExt; // For `send`
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let url = "wss://rpc.polkadot.io";

        
        match connect_async(url).await {
            Ok((mut ws_stream, _)) => {
                println!("Connected to the Polkadot node!");

                // JSON-RPC request to subscribe to new block headers
                let request = r#"{
                    "jsonrpc": "2.0",
                    "method": "chain_subscribeNewHeads",
                    "params": [],
                    "id": 1
                }"#;

                // Send subscription request
                if let Err(e) = ws_stream.send(Message::Text(request.into())).await {
                    eprintln!("Failed to send subscription request: {:?}", e);
                    return;
                }

                println!("Subscribed to new heads.");

                // Listen for incoming messages
                while let Some(msg) = ws_stream.next().await {
                    match msg {
                        Ok(message) => {
                            if let Message::Text(text) = message {
                                println!("Received: {}", text);
                            } else {
                                println!("Received non-text message: {:?}", message);
                            }
                        }
                        Err(e) => eprintln!("Error reading message: {:?}", e),
                    }
                }
            }
            Err(e) => eprintln!("Failed to connect to Polkadot node: {:?}", e),
        }
    });
}
