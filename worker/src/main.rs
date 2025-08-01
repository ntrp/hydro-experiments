use dfir_rs::util::deserialize_from_bytes;
use dfir_rs::{
    dfir_syntax,
    util::deploy::{ConnectedDirect, ConnectedSource, init, launch_flow},
};

#[dfir_rs::main]
async fn main() {
    let ports = init::<()>().await;
    let server = ports
        .port("from_server")
        .connect::<ConnectedDirect>()
        .into_source();

    println!("Worker started, waiting for messages...");

    let df = dfir_syntax! {
        source_stream(server) ->
            map(|x| deserialize_from_bytes(x.unwrap())) ->
            for_each(|x: Result<String,_>| println!("Received -> {:?}", x.unwrap()));
    };

    launch_flow(df).await;
}
