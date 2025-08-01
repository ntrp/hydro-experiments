use dfir_rs::util::serialize_to_bytes;
use dfir_rs::{
    dfir_syntax,
    util::deploy::{ConnectedDirect, ConnectedSink, init, launch_flow},
};
use std::time::{Duration, Instant};
use dfir_rs::util::deploy::ConnectedDemux;

#[dfir_rs::main]
async fn main() {
    let ports = init::<()>().await;

    let worker = ports
        .port("to_worker")
        .connect::<ConnectedDemux<ConnectedDirect>>()
        .into_sink();

    let df = dfir_syntax! {
        source_interval(Duration::from_secs(1))
            -> map(|_| (1, serialize_to_bytes(format!("Hello, world! at {:?}", Instant::now()))))
            -> dest_sink(worker);
    };

    launch_flow(df).await;
}
