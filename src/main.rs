use anyhow::{Context, Result};
use args::{Cli, DeploymentCommand};
use atlas_local_sdk::Client;
use bollard::Docker;
use clap::Parser;

mod args;

#[tokio::main]
async fn main() -> Result<()> {
    let args: DeploymentCommand = Cli::parse().into();

    let docker = Docker::connect_with_defaults().context("connect to docker")?;
    let client = Client::new(docker);

    match args {
        DeploymentCommand::List => {
            println!("Local deployments:");
            let deployments = client.list_deployments().await?;
            for deployment in deployments {
                println!(
                    "ID: {}, creation source: {:?}",
                    deployment.container_id, deployment.creation_source
                );
            }
        }
        _ => todo!("implement other commands"),
    }

    Ok(())
}
