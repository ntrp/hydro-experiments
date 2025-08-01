use dfir_rs::{
    dfir_syntax,
    util::deploy::{init, launch_flow, ConnectedDirect, ConnectedSource, ConnectedSink},
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
            map(|x| String::from_utf8(x.unwrap().to_vec()).unwrap()) ->
            for_each(|x| println!("echo {:?}", x));
    };

    launch_flow(df).await;
}
