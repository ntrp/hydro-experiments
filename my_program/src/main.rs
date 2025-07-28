use tokio::main;
use dfir_rs::util::deploy::{init};
use hydro_deploy_integration::ServerPort;
use std::collections::HashMap;

#[main]
async fn main() {
    let _ports = init::<()>().await;

    // Send the "ready" message
    let server_defns = HashMap::<String, ServerPort>::new();
    println!("ready: {}", serde_json::to_string(&server_defns).unwrap());

    // Keep the program running for a bit to allow hydro_deploy to interact with it
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let args: Vec<String> = std::env::args().collect();
    let node_id = args.get(1).unwrap_or(&"unknown".to_string()).clone();

    println!("Hello from my_program on cluster node {}!", node_id);
}
