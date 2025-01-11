use serde::Deserialize;
use serde_json::Value;
use subxt::{OnlineClient, PolkadotConfig};
use subxt::rpc::RpcParams;
use tokio::runtime::Runtime;

#[derive(Debug, Deserialize)]
struct BlockHeader {
    #[serde(rename = "parentHash")]
    parent_hash: String,

    #[serde(rename = "number")]
    number: String,

    #[serde(rename = "stateRoot")]
    state_root: String,

    #[serde(rename = "extrinsicsRoot")]
    extrinsics_root: String,

    #[serde(rename = "digest")]
    digest: Digest,
}

#[derive(Debug, Deserialize)]
struct Digest {
    #[serde(rename = "logs")]
    logs: Vec<String>,
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {

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
        
                    if let Ok(parsed_header) = serde_json::from_value::<BlockHeader>(new_head.clone()) {
                        println!("Parsed Block Header: {:?}", parsed_header);
                    } else {
                        println!("Failed to parse block header.");
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving new block header: {:?}", e);
                    break;
                }
            }
        }
    });
}
