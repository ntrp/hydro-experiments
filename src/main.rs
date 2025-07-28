use hydro_deploy::{Deployment, LocalhostHost, RustCrate};
use hydro_lang::{FlowBuilder, Cluster};
use tokio::main;
use anyhow::Result;
use std::sync::Arc;

#[main]
async fn main() -> Result<()> {
    let mut deployment = Deployment::new();

    let builder = FlowBuilder::new();
    let cluster: Cluster<()> = builder.cluster();

    let localhost_hosts: Vec<Arc<LocalhostHost>> = (0..2).map(|id| deployment.add_host(|_| LocalhostHost::new(id))).collect();

    

    let mut my_program_services = Vec::new();
    for (i, host) in localhost_hosts.iter().enumerate() {
        my_program_services.push(deployment.add_service(RustCrate::new(
            "worker",
            host.clone(),
        ).args(&[i.to_string()])));
        master_input.link_to(my_program_services[i].clone().into());
    }

    

    // Initial deployment of the logical flow to the physical hosts
    let _nodes = builder
        .with_default_optimize()
        .with_cluster(&cluster, localhost_hosts.clone().into_iter())
        .deploy(&mut deployment);

    deployment.deploy().await?;
    deployment.start().await?;

    Ok(())
}
