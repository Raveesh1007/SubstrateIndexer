use tungstenite::{connect, Message};
use url::Url;

fn main(){
    let url = "wss://rpc.polkadot.io";

    let (mut socket, _response) = connect(Url::parse(url).unwrap()).expect("Can't Connect");

    println!("Connected to the polkadot node:");

    let request = r#"
        {
            "jsonrpc": "2.0",
            "method": "chain_subscribeNewHeads",
            "params": [],
            "id": 1
        }
    "#;
    socket.write_message(Message::Text(request.into())).unwrap();

    loop{
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {:?}", msg);
    }
}
