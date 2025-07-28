use hydro_deploy::{Deployment, LocalhostHost, RustCrate};
use hydro_lang::{FlowBuilder, Cluster};
use tokio::main;
use anyhow::Result;
use std::sync::Arc;
use futures::future::join_all;

#[main]
async fn main() -> Result<()> {
    let mut deployment = Deployment::new();

    let builder = FlowBuilder::new();
    let cluster: Cluster<()> = builder.cluster();

    let mut localhost_hosts: Vec<Arc<LocalhostHost>> = (0..2).map(|id| deployment.add_host(|_| LocalhostHost::new(id))).collect();

    let mut my_program_services = Vec::new();
    for (i, host) in localhost_hosts.iter().enumerate() {
        my_program_services.push(deployment.add_service(RustCrate::new(
            "my_program",
            host.clone(),
        ).args(&[i.to_string()])));
    }

    // Initial deployment of the logical flow to the physical hosts
    let _nodes = builder
        .with_default_optimize()
        .with_cluster(&cluster, localhost_hosts.clone().into_iter())
        .deploy(&mut deployment);

    deployment.deploy().await?;
    deployment.start().await?;

    println!("Initial deployment complete. Adding a new worker in 5 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let new_host_id = localhost_hosts.len();
    let new_host = deployment.add_host(|_| LocalhostHost::new(new_host_id));
    localhost_hosts.push(new_host.clone());

    my_program_services.push(deployment.add_service(RustCrate::new(
        "my_program",
        new_host.clone(),
    ).args(&[new_host_id.to_string()])));

    // Redeploy and restart to include the new worker
    deployment.deploy().await?;
    deployment.start().await?;

    println!("New worker added and deployed.");

    // Collect and print stdout from each my_program instance
    let mut stdout_receivers = Vec::new();
    for service in my_program_services {
        stdout_receivers.push(service.read().await.stdout());
    }

    let all_outputs = join_all(stdout_receivers.into_iter().map(|mut receiver| async move {
        let mut outputs = Vec::new();
        while let Some(line) = receiver.recv().await {
            outputs.push(line);
        }
        outputs
    })).await;

    for (i, outputs) in all_outputs.into_iter().enumerate() {
        println!("Output from my_program instance {}:
{}", i, outputs.join("\n"));
    }

    Ok(())
}