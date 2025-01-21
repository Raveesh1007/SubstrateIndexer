use serde::Deserialize;
use serde_json::Value;
use subxt::{OnlineClient, PolkadotConfig};
use subxt::rpc::RpcParams;
use tokio::runtime::Runtime;
use tokio_postgres::{NoTls, Client};
use dotenv::dotenv;
use std::env;

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


async fn connect_to_postgres() -> Option<Client> {
    dotenv().ok();

 
    let connection_str = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("DATABASE_URL not set in .env: {:?}", e);
            return None;
        }
    };

    println!("Attempting to connect to PostgreSQL using: {}", connection_str);


    match tokio_postgres::connect(&connection_str, NoTls).await {
        Ok((client, connection)) => {
            // Spawn the background task for this connection
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("PostgreSQL connection error: {:?}", e);
                }
            });

            println!("Connected to PostgreSQL database!");
            Some(client)
        }
        Err(e) => {
            eprintln!("Failed to connect to PostgreSQL: {:?}", e);
            None
        }
    }
}

async fn save_block_header(client: &Client, header: &BlockHeader) {
    let query = "
        INSERT INTO block_headers (parent_hash, number, state_root, extrinsics_root, digest_logs)
        VALUES ($1, $2, $3, $4, $5);
    ";

    if let Err(e) = client
        .execute(
            query,
            &[
                &header.parent_hash,
                &header.number,
                &header.state_root,
                &header.extrinsics_root,
                &header.digest.logs,
            ],
        )
        .await
    {
        eprintln!("Failed to insert block header: {:?}", e);
    } else {
        println!("Block header successfully saved: {:?}", header);
    }
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {

        let postgres_client = connect_to_postgres().await;

        // Step 2: Connect to the Polkadot WebSocket endpoint
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
        )
        .await
        {
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
                    if let Ok(parsed_header) = serde_json::from_value::<BlockHeader>(new_head.clone())
                    {
                        println!("Parsed Block Header: {:?}", parsed_header);

                        if let Some(ref client) = postgres_client {
                            save_block_header(client, &parsed_header).await;
                        } else {
                            println!("Skipping database save as PostgreSQL client is not initialized.");
                        }
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
