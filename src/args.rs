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
    RsExample {
        #[command(subcommand)]
        command: PluginCommand,
    },
    #[command(flatten)]
    Flat(PluginCommand),
}

#[derive(Subcommand)]
pub enum PluginCommand {
    /// The Hello World command
    Hello,
    /// Prints environment variables
    Printenv,
    /// Reads name and prints it
    Stdinreader,
}

impl Into<PluginCommand> for Cli {
    fn into(self) -> PluginCommand {
        match self.command {
            PluginSubCommands::RsExample { command } => command,
            PluginSubCommands::Flat(command) => command,
        }
    }
}
