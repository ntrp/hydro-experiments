use dfir_rs::{
    dfir_syntax,
    util::deploy::{init, launch_flow, ConnectedDirect, ConnectedSource, ConnectedSink},
};

#[dfir_rs::main]
async fn main() {
    let ports = init::<()>().await;

    // let master_input = ports
    //     .port("main")
    //     .connect::<ConnectedDirect>()
    //     .into_source();

    let df = dfir_syntax! {
    };

    launch_flow(df).await;
}

