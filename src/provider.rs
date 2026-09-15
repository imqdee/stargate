use crate::networks::Network;
use serde::{Deserialize, Serialize};

/// An RPC provider. Determines how an endpoint URL is built for a network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    /// Alchemy: URL keyed by a per-network subdomain.
    #[default]
    Alchemy,
    /// routeme.sh: URL keyed by chain ID, so any chain is reachable.
    RouteMe,
}

impl Provider {
    /// Builds the RPC URL for `network`, or `None` when this provider cannot
    /// serve it (Alchemy without a known subdomain).
    pub fn rpc_url(&self, network: &Network, api_key: &str) -> Option<String> {
        match self {
            Provider::Alchemy => network
                .alchemy_subdomain
                .map(|subdomain| format!("https://{}.g.alchemy.com/v2/{}", subdomain, api_key)),
            Provider::RouteMe => Some(Self::routeme_url(network.chain_id, api_key)),
        }
    }

    /// Builds a routeme.sh URL from a raw chain ID. Used for unlisted chains
    /// that have no curated network entry.
    pub fn routeme_url(chain_id: u64, api_key: &str) -> String {
        format!("https://lb.routeme.sh/rpc/{}/{}", chain_id, api_key)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Provider::Alchemy => "alchemy",
            Provider::RouteMe => "routeme",
        }
    }

    pub fn parse(value: &str) -> Option<Provider> {
        match value.to_lowercase().as_str() {
            "alchemy" => Some(Provider::Alchemy),
            "routeme" => Some(Provider::RouteMe),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networks::find_network;

    #[test]
    fn alchemy_url_uses_subdomain() {
        let mainnet = find_network("mainnet").unwrap();
        assert_eq!(
            Provider::Alchemy.rpc_url(mainnet, "key123"),
            Some("https://eth-mainnet.g.alchemy.com/v2/key123".to_string())
        );
    }

    #[test]
    fn routeme_url_uses_chain_id() {
        let mainnet = find_network("mainnet").unwrap();
        assert_eq!(
            Provider::RouteMe.rpc_url(mainnet, "key123"),
            Some("https://lb.routeme.sh/rpc/1/key123".to_string())
        );

        let arbitrum = find_network("arbitrum").unwrap();
        assert_eq!(
            Provider::RouteMe.rpc_url(arbitrum, "key123"),
            Some("https://lb.routeme.sh/rpc/42161/key123".to_string())
        );
    }

    #[test]
    fn routeme_url_from_raw_chain_id() {
        assert_eq!(
            Provider::routeme_url(80094, "abc"),
            "https://lb.routeme.sh/rpc/80094/abc"
        );
    }

    #[test]
    fn parse_is_case_insensitive() {
        assert_eq!(Provider::parse("alchemy"), Some(Provider::Alchemy));
        assert_eq!(Provider::parse("ALCHEMY"), Some(Provider::Alchemy));
        assert_eq!(Provider::parse("routeme"), Some(Provider::RouteMe));
        assert_eq!(Provider::parse("RouteMe"), Some(Provider::RouteMe));
        assert_eq!(Provider::parse("routemesh"), None);
        assert_eq!(Provider::parse("unknown"), None);
    }

    #[test]
    fn default_provider_is_alchemy() {
        assert_eq!(Provider::default(), Provider::Alchemy);
    }

    #[test]
    fn as_str_roundtrips_through_parse() {
        for provider in [Provider::Alchemy, Provider::RouteMe] {
            assert_eq!(Provider::parse(provider.as_str()), Some(provider));
        }
    }

    #[test]
    fn serializes_to_lowercase() {
        // Round-trip through a wrapping struct (bare enums are not valid TOML docs).
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Wrap {
            provider: Provider,
        }
        let w = Wrap {
            provider: Provider::RouteMe,
        };
        let s = toml::to_string(&w).unwrap();
        assert!(s.contains("provider = \"routeme\""));
        assert_eq!(toml::from_str::<Wrap>(&s).unwrap(), w);
    }
}
