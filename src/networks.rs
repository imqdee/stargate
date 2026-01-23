pub struct Network {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub chain_id: u64,
    pub alchemy_subdomain: Option<&'static str>,
    pub explorer_url: Option<&'static str>,
}

impl Network {
    pub fn rpc_url(&self, api_key: &str) -> String {
        match self.alchemy_subdomain {
            Some(subdomain) => format!("https://{}.g.alchemy.com/v2/{}", subdomain, api_key),
            None => "http://127.0.0.1:8545".to_string(),
        }
    }

    pub fn matches(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        if self.name == query {
            return true;
        }
        if self.aliases.iter().any(|&alias| alias == query) {
            return true;
        }
        if let Ok(chain_id) = query.parse::<u64>() {
            return self.chain_id == chain_id;
        }
        false
    }
}

pub static NETWORKS: &[Network] = &[
    Network {
        name: "mainnet",
        aliases: &["eth", "ethereum"],
        chain_id: 1,
        alchemy_subdomain: Some("eth-mainnet"),
        explorer_url: Some("https://etherscan.io"),
    },
    Network {
        name: "polygon",
        aliases: &[],
        chain_id: 137,
        alchemy_subdomain: Some("polygon-mainnet"),
        explorer_url: Some("https://polygonscan.com"),
    },
    Network {
        name: "optimism",
        aliases: &["op"],
        chain_id: 10,
        alchemy_subdomain: Some("opt-mainnet"),
        explorer_url: Some("https://optimistic.etherscan.io"),
    },
    Network {
        name: "arbitrum",
        aliases: &["arb"],
        chain_id: 42161,
        alchemy_subdomain: Some("arb-mainnet"),
        explorer_url: Some("https://arbiscan.io"),
    },
    Network {
        name: "base",
        aliases: &[],
        chain_id: 8453,
        alchemy_subdomain: Some("base-mainnet"),
        explorer_url: Some("https://basescan.org"),
    },
    Network {
        name: "bnb",
        aliases: &["bsc"],
        chain_id: 56,
        alchemy_subdomain: Some("bnb-mainnet"),
        explorer_url: Some("https://bscscan.com"),
    },
    Network {
        name: "linea",
        aliases: &[],
        chain_id: 59144,
        alchemy_subdomain: Some("linea-mainnet"),
        explorer_url: Some("https://lineascan.build"),
    },
    Network {
        name: "ink",
        aliases: &[],
        chain_id: 57073,
        alchemy_subdomain: Some("ink-mainnet"),
        explorer_url: Some("https://explorer.inkonchain.com"),
    },
    Network {
        name: "arbnova",
        aliases: &["arbitrum-nova"],
        chain_id: 42170,
        alchemy_subdomain: Some("arbnova-mainnet"),
        explorer_url: Some("https://nova.arbiscan.io"),
    },
    Network {
        name: "zksync",
        aliases: &[],
        chain_id: 324,
        alchemy_subdomain: Some("zksync-mainnet"),
        explorer_url: Some("https://explorer.zksync.io"),
    },
    Network {
        name: "polygon-zkevm",
        aliases: &[],
        chain_id: 1101,
        alchemy_subdomain: Some("polygonzkevm-mainnet"),
        explorer_url: Some("https://zkevm.polygonscan.com"),
    },
    Network {
        name: "avalanche",
        aliases: &["avax"],
        chain_id: 43114,
        alchemy_subdomain: Some("avax-mainnet"),
        explorer_url: Some("https://snowtrace.io"),
    },
    Network {
        name: "gnosis",
        aliases: &["xdai"],
        chain_id: 100,
        alchemy_subdomain: Some("gnosis-mainnet"),
        explorer_url: Some("https://gnosisscan.io"),
    },
    Network {
        name: "scroll",
        aliases: &[],
        chain_id: 534352,
        alchemy_subdomain: Some("scroll-mainnet"),
        explorer_url: Some("https://scrollscan.com"),
    },
    Network {
        name: "celo",
        aliases: &[],
        chain_id: 42220,
        alchemy_subdomain: Some("celo-mainnet"),
        explorer_url: Some("https://celoscan.io"),
    },
    Network {
        name: "mantle",
        aliases: &[],
        chain_id: 5000,
        alchemy_subdomain: Some("mantle-mainnet"),
        explorer_url: Some("https://mantlescan.xyz"),
    },
    Network {
        name: "blast",
        aliases: &[],
        chain_id: 81457,
        alchemy_subdomain: Some("blast-mainnet"),
        explorer_url: Some("https://blastscan.io"),
    },
    Network {
        name: "sonic",
        aliases: &[],
        chain_id: 146,
        alchemy_subdomain: Some("sonic-mainnet"),
        explorer_url: Some("https://sonicscan.org"),
    },
    Network {
        name: "unichain",
        aliases: &[],
        chain_id: 130,
        alchemy_subdomain: Some("unichain-mainnet"),
        explorer_url: Some("https://unichain.blockscout.com/"),
    },
    Network {
        name: "flow",
        aliases: &[],
        chain_id: 747,
        alchemy_subdomain: Some("flow-mainnet"),
        explorer_url: Some("https://evm.flowscan.io"),
    },
    Network {
        name: "worldchain",
        aliases: &["world"],
        chain_id: 480,
        alchemy_subdomain: Some("worldchain-mainnet"),
        explorer_url: Some("https://worldscan.org"),
    },
    Network {
        name: "apechain",
        aliases: &[],
        chain_id: 33139,
        alchemy_subdomain: Some("apechain-mainnet"),
        explorer_url: Some("https://apescan.io"),
    },
    Network {
        name: "abstract",
        aliases: &[],
        chain_id: 2741,
        alchemy_subdomain: Some("abstract-mainnet"),
        explorer_url: Some("https://abscan.org"),
    },
    Network {
        name: "hyperevm",
        aliases: &["hyperliquid"],
        chain_id: 999,
        alchemy_subdomain: Some("hyperevm-mainnet"),
        explorer_url: Some("https://hyperscan.com"),
    },
    Network {
        name: "mode",
        aliases: &[],
        chain_id: 34443,
        alchemy_subdomain: Some("mode-mainnet"),
        explorer_url: Some("https://explorer.mode.network"),
    },
    Network {
        name: "anvil",
        aliases: &["local"],
        chain_id: 31337,
        alchemy_subdomain: None,
        explorer_url: None,
    },
];

pub fn find_network(query: &str) -> Option<&'static Network> {
    NETWORKS.iter().find(|n| n.matches(query))
}
