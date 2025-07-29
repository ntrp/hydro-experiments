use dfir_rs::{
    dfir_syntax,
    util::deploy::{init, launch_flow, ConnectedDirect, ConnectedSource, ConnectedSink},
};
use tokio::main;

#[main]
async fn main() {
    let mut ports = init::<()>().await;

    let mut master_input = ports
        .port("main")
        .connect::<ConnectedDirect>()
        .into_source();

    let df = dfir_syntax! {
        source_stream(master_input) ->
            for_each(|x| println!("echo {:?}", x));
    };

    launch_flow(df).await;
}

