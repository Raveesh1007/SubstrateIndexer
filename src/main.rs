use subxt::{OnlineClient, PolkadotConfig};
use subxt::rpc::RpcParams;
use serde_json::Value; 
use tokio::runtime::Runtime;


fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Connect to the Polkadot WebSocket endpoint
        let api = match OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await {
            Ok(client) => {
                println!("Connected to the Polkadot node!");
                client
            }
            Err(e) => {
                eprintln!("Failed to connect to Polkadot node: {:?}", e);
                return;
            }
        };

        let mut subscription = match api.rpc().subscribe::<Value>(
            "chain_subscribeNewHeads",
            RpcParams::new(),
            "chain_unsubscribeNewHeads",
        ).await {
            Ok(sub) => sub,
            Err(e) => {
                eprintln!("Failed to subscribe to new block headers: {:?}", e);
                return;
            }
        };

        println!("Subscribed to new block headers. Listening for new blocks...");

        while let Some(result) = subscription.next().await {
            match result {
                Ok(new_head) => {
                    println!("New block header: {:?}", new_head);
                }
                Err(e) => {
                    eprintln!("Error receiving new block header: {:?}", e);
                    break;
                }
            }
        }
    });
}
