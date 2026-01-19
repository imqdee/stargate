mod commands;
mod config;
mod networks;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "stargate")]
#[command(about = "Blockchain network switcher for Foundry", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install shell integration to config file
    #[command(name = "init")]
    Init {
        /// Shell type (zsh, bash)
        shell: String,
    },
    /// Switch to a network
    #[command(visible_alias = "t")]
    Travel {
        /// Network name or alias (e.g., mainnet, eth, polygon, arb)
        network: String,
        /// Suppress output message
        #[arg(short, long)]
        silent: bool,
    },
    /// Print current network name and chain ID
    #[command(visible_alias = "c")]
    Current,
    /// Switch to anvil (local network)
    Root {
        /// Suppress output message
        #[arg(short, long)]
        silent: bool,
    },
    /// Open block explorer in browser
    #[command(visible_alias = "e")]
    Explorer {
        /// Address or transaction hash to look up
        target: Option<String>,
    },
    /// List all available networks
    List,
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Set a configuration value
    Set {
        #[command(subcommand)]
        setting: ConfigSetting,
    },
}

#[derive(Subcommand)]
enum ConfigSetting {
    /// Set the Alchemy API key (prompts securely if not provided)
    ApiKey {
        key: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { shell } => commands::init::run(&shell),
        Commands::Travel { network, silent } => commands::travel::run(&network, silent),
        Commands::Current => commands::current::run(),
        Commands::Root { silent } => commands::travel::run("anvil", silent),
        Commands::Explorer { target } => commands::explorer::run(target.as_deref()),
        Commands::List => commands::list::run(),
        Commands::Config { action } => match action {
            ConfigAction::Set { setting } => match setting {
                ConfigSetting::ApiKey { key } => commands::config::set_api_key(key),
            },
        },
    }
}
