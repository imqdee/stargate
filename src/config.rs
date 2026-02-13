use crate::networks::find_network;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub api_key: Option<String>,
    pub default_network: Option<String>,
}

impl Config {
    pub fn path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".stargate").join("config.toml"))
    }

    pub fn load() -> Self {
        let Some(path) = Self::path() else {
            return Self::default();
        };

        if !path.exists() {
            return Self::default();
        }

        fs::read_to_string(&path)
            .ok()
            .and_then(|content| toml::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::path().ok_or("Could not determine home directory")?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        fs::write(&path, content).map_err(|e| format!("Failed to write config: {}", e))?;

        Ok(())
    }

    pub fn set_api_key(&mut self, key: String) -> Result<(), String> {
        self.api_key = Some(key);
        self.save()
    }

    pub fn get_default_network(&self) -> &str {
        self.default_network.as_deref().unwrap_or("anvil")
    }

    pub fn set_default_network(&mut self, network: String) -> Result<(), String> {
        let found_network = find_network(&network).ok_or_else(|| {
            format!(
                "Unknown network: '{}'. Run 'stargate list' to see available networks.",
                network
            )
        })?;

        // Store canonical name, not alias
        self.default_network = Some(found_network.name.to_string());
        self.save()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_no_api_key() {
        let config = Config::default();
        assert!(config.api_key.is_none());
    }

    #[test]
    fn config_serializes_to_toml() {
        let config = Config {
            api_key: Some("test-key-123".to_string()),
            default_network: None,
        };
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("api_key"));
        assert!(toml_str.contains("test-key-123"));
    }

    #[test]
    fn config_deserializes_from_toml() {
        let toml_str = r#"api_key = "my-secret-key""#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.api_key, Some("my-secret-key".to_string()));
    }

    #[test]
    fn config_deserializes_empty_toml() {
        let toml_str = "";
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.api_key.is_none());
    }

    #[test]
    fn config_deserializes_with_missing_api_key() {
        let toml_str = "# empty config\n";
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.api_key.is_none());
    }

    #[test]
    fn config_path_ends_with_expected_segments() {
        if let Some(path) = Config::path() {
            let path_str = path.to_string_lossy();
            assert!(path_str.ends_with(".stargate/config.toml"));
        }
        // If home dir is not available, path() returns None which is acceptable
    }

    #[test]
    fn config_roundtrip_serialization() {
        let original = Config {
            api_key: Some("roundtrip-test-key".to_string()),
            default_network: None,
        };

        let toml_str = toml::to_string(&original).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(original.api_key, deserialized.api_key);
    }

    #[test]
    fn default_config_has_no_default_network() {
        let config = Config::default();
        assert!(config.default_network.is_none());
    }

    #[test]
    fn get_default_network_returns_anvil_when_not_set() {
        let config = Config::default();
        assert_eq!(config.get_default_network(), "anvil");
    }

    #[test]
    fn get_default_network_returns_configured_value() {
        let config = Config {
            api_key: None,
            default_network: Some("polygon".to_string()),
        };
        assert_eq!(config.get_default_network(), "polygon");
    }

    #[test]
    fn set_default_network_accepts_valid_network_name() {
        let mut config = Config::default();
        let _result = config.set_default_network("mainnet".to_string());

        // Should succeed (or fail only due to save, not validation)
        // We check that the network was stored correctly
        assert_eq!(config.default_network, Some("mainnet".to_string()));
    }

    #[test]
    fn set_default_network_accepts_valid_alias() {
        let mut config = Config::default();
        let _result = config.set_default_network("arb".to_string());

        // Should normalize "arb" to "arbitrum"
        assert_eq!(config.default_network, Some("arbitrum".to_string()));
    }

    #[test]
    fn set_default_network_accepts_valid_chain_id() {
        let mut config = Config::default();
        let _result = config.set_default_network("1".to_string());

        // Should normalize "1" to "mainnet"
        assert_eq!(config.default_network, Some("mainnet".to_string()));
    }

    #[test]
    fn set_default_network_rejects_invalid_network() {
        let mut config = Config::default();
        let result = config.set_default_network("invalid-network-xyz".to_string());

        // Should fail with descriptive error
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown network"));
    }

    #[test]
    fn config_serializes_with_default_network() {
        let config = Config {
            api_key: Some("test-key".to_string()),
            default_network: Some("polygon".to_string()),
        };
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("default_network"));
        assert!(toml_str.contains("polygon"));
    }

    #[test]
    fn config_deserializes_with_default_network() {
        let toml_str = r#"
api_key = "my-key"
default_network = "arbitrum"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.default_network, Some("arbitrum".to_string()));
        assert_eq!(config.get_default_network(), "arbitrum");
    }

    #[test]
    fn config_handles_missing_default_network_field() {
        let toml_str = r#"api_key = "my-key""#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.default_network.is_none());
        assert_eq!(config.get_default_network(), "anvil");
    }

    #[test]
    fn set_default_network_preserves_api_key() {
        let mut config = Config {
            api_key: Some("existing-api-key".to_string()),
            default_network: None,
        };

        let _result = config.set_default_network("mainnet".to_string());

        assert_eq!(config.api_key, Some("existing-api-key".to_string()));
        assert_eq!(config.default_network, Some("mainnet".to_string()));
    }
}
