use std::sync::Arc;
use hydro_deploy::{Deployment, RustCrate};
use hydro_deploy::rust_crate::ports::RustCrateSink;
use hydro_lang::deploy::{DeployClusterSpec, TrybuildHost};
use hydro_lang::{Cluster, FlowBuilder};

#[tokio::main]
async fn main() {
    let mut deployment = Deployment::new();

    let builder = FlowBuilder::new();
    let cluster: Cluster<()> = builder.cluster();

    let server = RustCrate::new("server", deployment.Localhost().clone());
    let worker = RustCrate::new("worker", deployment.Localhost().clone());

    // let workers: Vec<_> = (0..2)
    //     .map(|_| { RustCrate::new("worker", deployment.Localhost().clone()) })
    //     .collect();

    let server_service = deployment.add_service(server);
    let worker_service = deployment.add_service(worker);
    let worker_lock = worker_service.try_read().expect("read lock worker");
    let port = worker_lock
        .get_port("echo".to_string(), &worker_service);
    server_service
        .try_write()
        .expect("write lock server")
        .add_connection(&worker_service, "echo".to_string(), &port)
        .expect("add connection");

    let _nodes = builder
        .with_default_optimize()
        // .with_process(&server, TrybuildHost::new(server))
        // .with_cluster(&cluster, DeployClusterSpec::new(workers))
        .with_cluster(&cluster, vec![deployment.Localhost()])
        .deploy(&mut deployment);

    deployment.run_ctrl_c().await.unwrap();
}
