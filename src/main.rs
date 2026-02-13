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
    #[command(visible_alias = "sw")]
    Switch {
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
        /// Print URL instead of opening in browser
        #[arg(short, long)]
        print: bool,
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
    /// Get a configuration value
    Get {
        #[command(subcommand)]
        setting: ConfigGetter,
    },
}

#[derive(Subcommand)]
enum ConfigSetting {
    /// Set the Alchemy API key (prompts securely if not provided)
    ApiKey { key: Option<String> },
    /// Set the default network (used when starting new shells)
    DefaultNetwork { network: String },
}

#[derive(Subcommand)]
enum ConfigGetter {
    /// Show the current default network
    DefaultNetwork,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { shell } => commands::init::run(&shell),
        Commands::Switch { network, silent } => commands::switch::run(&network, silent),
        Commands::Current => commands::current::run(),
        Commands::Root { silent } => commands::switch::run("anvil", silent),
        Commands::Explorer { target, print } => commands::explorer::run(target.as_deref(), print),
        Commands::List => commands::list::run(),
        Commands::Config { action } => match action {
            ConfigAction::Set { setting } => match setting {
                ConfigSetting::ApiKey { key } => commands::config::set_api_key(key),
                ConfigSetting::DefaultNetwork { network } => {
                    commands::config::set_default_network(network)
                }
            },
            ConfigAction::Get { setting } => match setting {
                ConfigGetter::DefaultNetwork => commands::config::get_default_network(),
            },
        },
    }
}
