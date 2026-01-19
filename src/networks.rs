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
