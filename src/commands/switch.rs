use crate::config::Config;
use crate::networks::{find_network, Network};

/// Represents the shell exports to be generated when switching networks.
#[derive(Debug, PartialEq)]
pub struct NetworkExports {
    pub rpc_url: String,
    pub network_name: String,
    pub chain_id: u64,
    pub explorer_url: Option<String>,
}

impl NetworkExports {
    /// Creates exports for a given network and API key.
    pub fn from_network(network: &Network, api_key: &str) -> Self {
        Self {
            rpc_url: network.rpc_url(api_key),
            network_name: network.name.to_string(),
            chain_id: network.chain_id,
            explorer_url: network.explorer_url.map(|s| s.to_string()),
        }
    }

    /// Formats the exports as shell export statements.
    pub fn to_shell_exports(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("export ETH_RPC_URL=\"{}\"\n", self.rpc_url));
        output.push_str(&format!("export STARGATE_NETWORK=\"{}\"\n", self.network_name));
        output.push_str(&format!("export STARGATE_CHAIN_ID=\"{}\"\n", self.chain_id));

        if let Some(ref explorer) = self.explorer_url {
            output.push_str(&format!("export BLOCK_EXPLORER=\"{}\"", explorer));
        } else {
            output.push_str("unset BLOCK_EXPLORER");
        }

        output
    }
}

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
    let exports = NetworkExports::from_network(network, api_key);

    // Output export statements for shell to eval
    println!("{}", exports.to_shell_exports());

    // User-friendly message (stderr, not captured by eval)
    if !silent {
        eprintln!("Moved to {} ({})", network.name, network.chain_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networks::find_network;

    // ==================== NetworkExports::from_network() tests ====================

    #[test]
    fn creates_exports_for_mainnet() {
        let mainnet = find_network("mainnet").unwrap();
        let exports = NetworkExports::from_network(mainnet, "test-key");

        assert_eq!(exports.network_name, "mainnet");
        assert_eq!(exports.chain_id, 1);
        assert_eq!(exports.rpc_url, "https://eth-mainnet.g.alchemy.com/v2/test-key");
        assert_eq!(exports.explorer_url, Some("https://etherscan.io".to_string()));
    }

    #[test]
    fn creates_exports_for_anvil() {
        let anvil = find_network("anvil").unwrap();
        let exports = NetworkExports::from_network(anvil, "");

        assert_eq!(exports.network_name, "anvil");
        assert_eq!(exports.chain_id, 31337);
        assert_eq!(exports.rpc_url, "http://127.0.0.1:8545");
        assert!(exports.explorer_url.is_none());
    }

    #[test]
    fn creates_exports_for_polygon() {
        let polygon = find_network("polygon").unwrap();
        let exports = NetworkExports::from_network(polygon, "my-api-key");

        assert_eq!(exports.network_name, "polygon");
        assert_eq!(exports.chain_id, 137);
        assert!(exports.rpc_url.contains("polygon-mainnet"));
        assert!(exports.rpc_url.contains("my-api-key"));
    }

    // ==================== NetworkExports::to_shell_exports() tests ====================

    #[test]
    fn generates_correct_shell_exports_with_explorer() {
        let exports = NetworkExports {
            rpc_url: "https://example.com/rpc".to_string(),
            network_name: "testnet".to_string(),
            chain_id: 123,
            explorer_url: Some("https://explorer.example.com".to_string()),
        };

        let shell = exports.to_shell_exports();

        assert!(shell.contains("export ETH_RPC_URL=\"https://example.com/rpc\""));
        assert!(shell.contains("export STARGATE_NETWORK=\"testnet\""));
        assert!(shell.contains("export STARGATE_CHAIN_ID=\"123\""));
        assert!(shell.contains("export BLOCK_EXPLORER=\"https://explorer.example.com\""));
    }

    #[test]
    fn generates_unset_for_missing_explorer() {
        let exports = NetworkExports {
            rpc_url: "http://127.0.0.1:8545".to_string(),
            network_name: "anvil".to_string(),
            chain_id: 31337,
            explorer_url: None,
        };

        let shell = exports.to_shell_exports();

        assert!(shell.contains("unset BLOCK_EXPLORER"));
        assert!(!shell.contains("export BLOCK_EXPLORER"));
    }

    #[test]
    fn shell_exports_are_valid_shell_syntax() {
        let mainnet = find_network("mainnet").unwrap();
        let exports = NetworkExports::from_network(mainnet, "key123");
        let shell = exports.to_shell_exports();

        // Each line should start with "export" or "unset"
        for line in shell.lines() {
            assert!(
                line.starts_with("export ") || line.starts_with("unset "),
                "Invalid shell line: {}",
                line
            );
        }
    }

    #[test]
    fn exports_contain_all_required_variables() {
        let mainnet = find_network("mainnet").unwrap();
        let exports = NetworkExports::from_network(mainnet, "key");
        let shell = exports.to_shell_exports();

        assert!(shell.contains("ETH_RPC_URL"), "Missing ETH_RPC_URL");
        assert!(shell.contains("STARGATE_NETWORK"), "Missing STARGATE_NETWORK");
        assert!(shell.contains("STARGATE_CHAIN_ID"), "Missing STARGATE_CHAIN_ID");
        assert!(shell.contains("BLOCK_EXPLORER"), "Missing BLOCK_EXPLORER");
    }
}
