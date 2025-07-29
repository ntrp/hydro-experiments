use hydro_deploy::{Deployment, LocalhostHost, RustCrate};
use hydro_lang::{Cluster, FlowBuilder};
use tokio::main;
use std::sync::Arc;

#[main]
async fn main() {
    let mut deployment = Deployment::new();

    let builder = FlowBuilder::new();
    let cluster: Cluster<()> = builder.cluster();

    let localhost_hosts: Vec<Arc<LocalhostHost>> = (0..2).map(|id| deployment.add_host(|_| LocalhostHost::new(id))).collect();

    let service = deployment.add_service(RustCrate::new("worker", localhost_hosts[0].clone()));


    let _nodes = builder
        .with_default_optimize()
        .with_cluster(&cluster, localhost_hosts.clone().into_iter())
        .deploy(&mut deployment);

    let _ = deployment.deploy().await;
    let _ = deployment.start().await;
}
