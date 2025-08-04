use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: PluginSubCommands,
}

#[derive(Subcommand)]
pub enum PluginSubCommands {
    /// Plugin root subcommand
    #[command(hide = true)]
    Local {
        #[command(subcommand)]
        command: DeploymentCommand,
    },
    #[command(flatten)]
    Flat(DeploymentCommand),
}

#[derive(Debug, Subcommand)]
pub enum DeploymentCommand {
    /// Connect to a local deployment.
    Connect,

    /// Fetch detailed information about all your deployments and system processes.
    Diagnostics,
    /// Get deployment logs.
    Logs,

    /// Return all deployments.
    List,
    /// Create a local deployment.
    Setup,
    /// Start a deployment.
    Start,
    /// Pause a deployment.
    Pause,
    /// Delete a deployment.
    Delete,
}

#[derive(Debug, Subcommand)]
pub enum DiagnosticsCommand {}

impl Into<DeploymentCommand> for Cli {
    fn into(self) -> DeploymentCommand {
        match self.command {
            PluginSubCommands::Local { command } => command,
            PluginSubCommands::Flat(command) => command,
        }
    }
}
