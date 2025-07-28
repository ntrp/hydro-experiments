use dfir_rs::{
    dfir_syntax,
    util::deploy::{init, launch_flow, ConnectedDirect, ConnectedSource, ConnectedSink},
};
use tokio::main;

#[main]
async fn main() {
    let mut ports = init::<()>().await;

    let args: Vec<String> = std::env::args().collect();
    let node_id = args.get(1).unwrap_or(&"unknown".to_string()).clone();

    println!("Hello from worker on cluster node {}!", node_id);

    let mut master_input = ports
        .port("master_input")
        .connect::<ConnectedDirect>()
        .await
        .into_source();

    let df = dfir_syntax! {
        source_stream(master_input) ->
            for_each(|x| println!("echo {:?}", x));
    };

    launch_flow(df).await;
}

