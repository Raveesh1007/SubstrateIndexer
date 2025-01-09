use subxt::{OnlineClient, PolkadotConfig};
use tokio::runtime::Runtime;

fn main() {
    // Create a Tokio runtime for async execution
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Connect to the Polkadot WebSocket endpoint
        match OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await {
            Ok(client) => {
                println!("Connected to the Polkadot node!");

                
                match client.rpc().block_hash(None).await {
                    Ok(Some(hash)) => println!("Latest block hash: {:?}", hash),
                    Ok(None) => println!("No block hash found."),
                    Err(e) => eprintln!("Error fetching block hash: {:?}", e),
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to Polkadot node: {:?}", e);
            }
        }
    });
}
