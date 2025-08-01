use hydro_deploy::rust_crate::ports::RustCrateSource;
use hydro_deploy::{Deployment, RustCrate};
use hydro_lang::{Cluster, FlowBuilder};

#[tokio::main]
async fn main() {
    let mut deployment = Deployment::new();

    let builder = FlowBuilder::new();
    let cluster: Cluster<()> = builder.cluster();

    let server = RustCrate::new("server", deployment.Localhost().clone());
    let server_service = deployment.add_service(server);
    let server_port = server_service
        .try_write()
        .unwrap()
        .get_port("to_worker".to_string(), &server_service);

    (0..2)
        .map(|_| {
            let worker = RustCrate::new("worker", deployment.Localhost().clone());
            deployment.add_service(worker)
        })
        .for_each(|worker_service| {
            let worker_port = worker_service
                .try_write()
                .unwrap()
                .get_port("from_server".to_string(), &worker_service);
            server_port.send_to(&worker_port);
        });

    let _nodes = builder
        .with_default_optimize()
        .with_cluster(&cluster, vec![deployment.Localhost()])
        .deploy(&mut deployment);

    deployment.run_ctrl_c().await.unwrap();
}
