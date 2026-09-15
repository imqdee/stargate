use crate::config::Config;
use crate::networks::{Network, find_network};
use crate::provider::Provider;

/// Represents the shell exports to be generated when switching networks.
#[derive(Debug, PartialEq)]
pub struct NetworkExports {
    pub rpc_url: String,
    pub network_name: String,
    pub chain_id: u64,
    pub explorer_url: Option<String>,
}

impl NetworkExports {
    /// Creates exports for a known network via `provider`. Returns `None` when
    /// the provider cannot serve the network (Alchemy without a subdomain).
    pub fn from_network(network: &Network, provider: Provider, api_key: &str) -> Option<Self> {
        Some(Self {
            rpc_url: network.rpc_url(provider, api_key)?,
            network_name: network.name.to_string(),
            chain_id: network.chain_id,
            explorer_url: network.explorer_url.map(|s| s.to_string()),
        })
    }

    /// Creates exports for an unlisted chain ID, routed through routeme.sh (the
    /// only provider that needs nothing but the chain ID). No curated name or
    /// explorer is available, so the chain ID stands in as the network name.
    pub fn from_raw_chain(chain_id: u64, api_key: &str) -> Self {
        Self {
            rpc_url: Provider::routeme_url(chain_id, api_key),
            network_name: chain_id.to_string(),
            chain_id,
            explorer_url: None,
        }
    }

    /// The stderr confirmation line. An unlisted chain has no curated name, so
    /// its name equals its chain ID and the message reflects the routeme route.
    pub fn moved_message(&self) -> String {
        if self.network_name == self.chain_id.to_string() {
            format!("Moved to chain {} (via routeme)", self.chain_id)
        } else {
            format!("Moved to {} ({})", self.network_name, self.chain_id)
        }
    }

    /// Formats the exports as shell export statements.
    pub fn to_shell_exports(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("export ETH_RPC_URL=\"{}\"\n", self.rpc_url));
        output.push_str(&format!(
            "export STARGATE_NETWORK=\"{}\"\n",
            self.network_name
        ));
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

    match resolve(network_name, &config) {
        Ok(exports) => {
            println!("{}", exports.to_shell_exports());
            if !silent {
                eprintln!("{}", exports.moved_message());
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

/// Resolves a switch target against config into the exports to print, or a
/// user-facing error. Pure, so the whole routing matrix is unit-testable.
pub fn resolve(target: &str, config: &Config) -> Result<NetworkExports, String> {
    let provider = config.provider();

    if let Some(network) = find_network(target) {
        resolve_known(network, config, provider)
    } else if let Ok(chain_id) = target.parse::<u64>() {
        resolve_raw_chain(chain_id, config)
    } else {
        Err(format!(
            "Unknown network: {}\nRun 'stargate list' to see available networks.",
            target
        ))
    }
}

/// Resolves a curated network using the configured provider.
fn resolve_known(
    network: &Network,
    config: &Config,
    provider: Provider,
) -> Result<NetworkExports, String> {
    // Local networks need no provider or key.
    let api_key = if network.is_local() {
        ""
    } else {
        config.key_for(provider).ok_or_else(|| {
            format!(
                "No {0} API key configured. Run 'stargate config set api-key --provider {0}' first.",
                provider.as_str()
            )
        })?
    };

    NetworkExports::from_network(network, provider, api_key).ok_or_else(|| {
        format!(
            "Network '{}' is not available on {}. Switch provider with 'stargate config set provider routeme'.",
            network.name,
            provider.as_str()
        )
    })
}

/// Routes an unlisted chain ID through routeme.sh, the only provider that needs
/// nothing but the chain ID.
fn resolve_raw_chain(chain_id: u64, config: &Config) -> Result<NetworkExports, String> {
    if chain_id == 0 {
        return Err("Chain ID 0 is not valid.".to_string());
    }

    let api_key = config.key_for(Provider::RouteMe).ok_or_else(|| {
        format!(
            "Chain {} is not a known network. Routing unlisted chains needs a routeme key: 'stargate config set api-key --provider routeme'.",
            chain_id
        )
    })?;

    Ok(NetworkExports::from_raw_chain(chain_id, api_key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networks::find_network;

    // ==================== NetworkExports::from_network() tests ====================

    #[test]
    fn creates_exports_for_mainnet() {
        let mainnet = find_network("mainnet").unwrap();
        let exports = NetworkExports::from_network(mainnet, Provider::Alchemy, "test-key").unwrap();

        assert_eq!(exports.network_name, "mainnet");
        assert_eq!(exports.chain_id, 1);
        assert_eq!(
            exports.rpc_url,
            "https://eth-mainnet.g.alchemy.com/v2/test-key"
        );
        assert_eq!(
            exports.explorer_url,
            Some("https://etherscan.io".to_string())
        );
    }

    #[test]
    fn creates_routeme_exports_for_mainnet() {
        let mainnet = find_network("mainnet").unwrap();
        let exports = NetworkExports::from_network(mainnet, Provider::RouteMe, "rk").unwrap();

        assert_eq!(exports.network_name, "mainnet");
        assert_eq!(exports.chain_id, 1);
        assert_eq!(exports.rpc_url, "https://lb.routeme.sh/rpc/1/rk");
    }

    #[test]
    fn creates_exports_for_anvil() {
        let anvil = find_network("anvil").unwrap();
        let exports = NetworkExports::from_network(anvil, Provider::Alchemy, "").unwrap();

        assert_eq!(exports.network_name, "anvil");
        assert_eq!(exports.chain_id, 31337);
        assert_eq!(exports.rpc_url, "http://127.0.0.1:8545");
        assert!(exports.explorer_url.is_none());
    }

    #[test]
    fn creates_exports_for_polygon() {
        let polygon = find_network("polygon").unwrap();
        let exports =
            NetworkExports::from_network(polygon, Provider::Alchemy, "my-api-key").unwrap();

        assert_eq!(exports.network_name, "polygon");
        assert_eq!(exports.chain_id, 137);
        assert!(exports.rpc_url.contains("polygon-mainnet"));
        assert!(exports.rpc_url.contains("my-api-key"));
    }

    #[test]
    fn creates_exports_for_raw_chain() {
        let exports = NetworkExports::from_raw_chain(80094, "rk");

        assert_eq!(exports.network_name, "80094");
        assert_eq!(exports.chain_id, 80094);
        assert_eq!(exports.rpc_url, "https://lb.routeme.sh/rpc/80094/rk");
        assert!(exports.explorer_url.is_none());

        let shell = exports.to_shell_exports();
        assert!(shell.contains("export ETH_RPC_URL=\"https://lb.routeme.sh/rpc/80094/rk\""));
        assert!(shell.contains("export STARGATE_CHAIN_ID=\"80094\""));
        assert!(shell.contains("unset BLOCK_EXPLORER"));
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
        let exports = NetworkExports::from_network(mainnet, Provider::Alchemy, "key123").unwrap();
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
        let exports = NetworkExports::from_network(mainnet, Provider::Alchemy, "key").unwrap();
        let shell = exports.to_shell_exports();

        assert!(shell.contains("ETH_RPC_URL"), "Missing ETH_RPC_URL");
        assert!(
            shell.contains("STARGATE_NETWORK"),
            "Missing STARGATE_NETWORK"
        );
        assert!(
            shell.contains("STARGATE_CHAIN_ID"),
            "Missing STARGATE_CHAIN_ID"
        );
        assert!(shell.contains("BLOCK_EXPLORER"), "Missing BLOCK_EXPLORER");
    }

    // ==================== resolve() routing matrix ====================

    fn config_with(
        provider: Option<Provider>,
        alchemy: Option<&str>,
        routeme: Option<&str>,
    ) -> Config {
        Config {
            provider,
            keys: crate::config::ProviderKeys {
                alchemy: alchemy.map(str::to_string),
                routeme: routeme.map(str::to_string),
            },
            ..Default::default()
        }
    }

    #[test]
    fn resolve_known_network_with_alchemy() {
        let config = config_with(Some(Provider::Alchemy), Some("ak"), None);
        let exports = resolve("mainnet", &config).unwrap();
        assert_eq!(exports.rpc_url, "https://eth-mainnet.g.alchemy.com/v2/ak");
    }

    #[test]
    fn resolve_known_network_with_routeme() {
        let config = config_with(Some(Provider::RouteMe), None, Some("rk"));
        let exports = resolve("mainnet", &config).unwrap();
        assert_eq!(exports.rpc_url, "https://lb.routeme.sh/rpc/1/rk");
    }

    #[test]
    fn resolve_anvil_needs_no_key() {
        let config = config_with(Some(Provider::RouteMe), None, None);
        let exports = resolve("anvil", &config).unwrap();
        assert_eq!(exports.rpc_url, "http://127.0.0.1:8545");
    }

    #[test]
    fn resolve_known_network_without_key_errors() {
        let config = config_with(Some(Provider::Alchemy), None, None);
        let err = resolve("mainnet", &config).unwrap_err();
        assert!(err.contains("No alchemy API key"));
    }

    #[test]
    fn resolve_unlisted_chain_via_routeme() {
        let config = config_with(Some(Provider::Alchemy), Some("ak"), Some("rk"));
        // Provider is alchemy, but an unlisted chain always routes via routeme.
        let exports = resolve("80094", &config).unwrap();
        assert_eq!(exports.rpc_url, "https://lb.routeme.sh/rpc/80094/rk");
        assert_eq!(
            exports.moved_message(),
            "Moved to chain 80094 (via routeme)"
        );
    }

    #[test]
    fn resolve_unlisted_chain_without_routeme_key_errors() {
        let config = config_with(Some(Provider::Alchemy), Some("ak"), None);
        let err = resolve("80094", &config).unwrap_err();
        assert!(err.contains("routeme key"));
    }

    #[test]
    fn resolve_chain_zero_errors() {
        let config = config_with(Some(Provider::RouteMe), None, Some("rk"));
        assert!(resolve("0", &config).unwrap_err().contains("Chain ID 0"));
    }

    #[test]
    fn resolve_unknown_name_errors() {
        let config = config_with(Some(Provider::RouteMe), None, Some("rk"));
        let err = resolve("not-a-network", &config).unwrap_err();
        assert!(err.contains("Unknown network"));
    }

    #[test]
    fn moved_message_for_known_network() {
        let mainnet = find_network("mainnet").unwrap();
        let exports = NetworkExports::from_network(mainnet, Provider::Alchemy, "ak").unwrap();
        assert_eq!(exports.moved_message(), "Moved to mainnet (1)");
    }
}
