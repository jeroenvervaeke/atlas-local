use anyhow::Result;
use args::{Cli, DeploymentCommand};
use clap::Parser;

mod args;

fn main() -> Result<()> {
    let args: DeploymentCommand = Cli::parse().into();

    dbg!(args);

    Ok(())
}
