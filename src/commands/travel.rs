use crate::config::Config;
use crate::networks::find_network;

pub fn run(network_name: &str, silent: bool) {
    let config = Config::load();

    let Some(network) = find_network(network_name) else {
        eprintln!("Unknown network: {}", network_name);
        eprintln!("Run 'stargate list' to see available networks.");
        std::process::exit(1);
    };

    // For non-anvil networks, require API key
    if network.alchemy_subdomain.is_some() && config.api_key.is_none() {
        eprintln!("No API key configured. Run 'stargate config set api-key <your-key>' first.");
        std::process::exit(1);
    }

    let api_key = config.api_key.as_deref().unwrap_or("");
    let rpc_url = network.rpc_url(api_key);

    // Output export statements for shell to eval
    println!("export ETH_RPC_URL=\"{}\"", rpc_url);
    println!("export STARGATE_NETWORK=\"{}\"", network.name);
    println!("export STARGATE_CHAIN_ID=\"{}\"", network.chain_id);

    if let Some(explorer) = network.explorer_url {
        println!("export BLOCK_EXPLORER=\"{}\"", explorer);
    } else {
        println!("unset BLOCK_EXPLORER");
    }

    // User-friendly message (stderr, not captured by eval)
    if !silent {
        eprintln!("Moved to {} (chain_id: {})", network.name, network.chain_id);
    }
}
