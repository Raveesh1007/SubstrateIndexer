use subxt::OnlineClient;
use subxt::PolkadotConfig;
use tokio::runtime::Runtime;

// Import StreamExt so we can call `.next()` on streams.
use futures_util::stream::StreamExt;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // 1. Connect to the Polkadot node
        let api = match OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io").await {
            Ok(client) => {
                println!("Connected to the Polkadot node!");
                client
            }
            Err(e) => {
                eprintln!("Failed to connect: {:?}", e);
                return;
            }
        };

    
        let subscription = match api
            .rpc()
            .subscribe_raw("chain_subscribeNewHeads", None, "chain_unsubscribeNewHeads")
            .await
        {
            Ok(sub) => sub,
            Err(e) => {
                eprintln!("Failed to subscribe to new heads: {:?}", e);
                return;
            }
        };

        println!("Subscribed to new block headers.");

        let mut messages = subscription.messages();

        while let Some(response) = messages.next().await {
            match response {
                Ok(subscription_response) => {
                    
                    println!("New block message: {:?}", subscription_response);
                }
                Err(e) => {
                    eprintln!("Error receiving block: {:?}", e);
                    break;
                }
            }
        }

        println!("Subscription ended.");
    });
}
