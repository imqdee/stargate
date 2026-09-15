use crate::networks::find_network;
use crate::provider::Provider;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Per-provider API keys, so an Alchemy key and a routeme key coexist and the
/// user can switch providers without re-entering credentials.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProviderKeys {
    pub alchemy: Option<String>,
    pub routeme: Option<String>,
}

impl ProviderKeys {
    fn is_empty(&self) -> bool {
        self.alchemy.is_none() && self.routeme.is_none()
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// Legacy single-key field from pre-provider versions. Read on load and
    /// folded into `keys.alchemy`, then never written back (`skip_serializing`).
    #[serde(default, skip_serializing)]
    pub api_key: Option<String>,
    pub default_network: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider>,
    #[serde(default, skip_serializing_if = "ProviderKeys::is_empty")]
    pub keys: ProviderKeys,
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

        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read config at {}: {}", path.display(), e);
                std::process::exit(1);
            }
        };

        // A malformed config must never be treated as empty: doing so would let
        // the next write overwrite the file and destroy the keys still in it.
        match Self::from_toml(&content) {
            Ok(mut config) => {
                config.migrate_legacy();
                config
            }
            Err(e) => {
                eprintln!(
                    "Failed to parse config at {}: {}\nFix or remove the file. Stargate will not overwrite a malformed config.",
                    path.display(),
                    e
                );
                std::process::exit(1);
            }
        }
    }

    /// Parses a config from TOML. Pure and side-effect free, so tests can cover
    /// valid and malformed documents without touching the filesystem.
    pub fn from_toml(content: &str) -> Result<Self, String> {
        toml::from_str(content).map_err(|e| e.to_string())
    }

    /// Folds a legacy top-level `api_key` into `keys.alchemy` so the rest of the
    /// code only reads per-provider keys. Idempotent: an existing `keys.alchemy`
    /// wins, and the legacy value is dropped so it is not written back.
    fn migrate_legacy(&mut self) {
        if let Some(key) = self.api_key.take()
            && self.keys.alchemy.is_none()
        {
            self.keys.alchemy = Some(key);
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::path().ok_or("Could not determine home directory")?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
            restrict_dir(parent);
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        // Write to a sibling temp file then rename, so a crash mid-write cannot
        // truncate an existing config. The temp file is created private so the
        // credentials are never briefly world-readable.
        let tmp = path.with_extension("toml.tmp");
        write_private(&tmp, &content)?;
        fs::rename(&tmp, &path).map_err(|e| {
            let _ = fs::remove_file(&tmp);
            format!("Failed to persist config: {}", e)
        })?;

        Ok(())
    }

    /// The active provider, defaulting to Alchemy for configs that predate the
    /// provider field.
    pub fn provider(&self) -> Provider {
        self.provider.unwrap_or_default()
    }

    pub fn set_provider(&mut self, provider: Provider) -> Result<(), String> {
        self.provider = Some(provider);
        self.save()
    }

    /// The API key for `provider`, or `None` when unset.
    pub fn key_for(&self, provider: Provider) -> Option<&str> {
        match provider {
            Provider::Alchemy => self.keys.alchemy.as_deref(),
            Provider::RouteMe => self.keys.routeme.as_deref(),
        }
    }

    pub fn set_key(&mut self, provider: Provider, key: String) -> Result<(), String> {
        match provider {
            Provider::Alchemy => self.keys.alchemy = Some(key),
            Provider::RouteMe => self.keys.routeme = Some(key),
        }
        self.save()
    }

    pub fn get_default_network(&self) -> &str {
        self.default_network.as_deref().unwrap_or("anvil")
    }

    pub fn set_default_network(&mut self, network: String) -> Result<(), String> {
        self.assign_default_network(network)?;
        self.save()
    }

    /// Validates and normalizes the network without persisting. Kept separate
    /// from `set_default_network` so tests exercise the logic without writing to
    /// the real config file.
    fn assign_default_network(&mut self, network: String) -> Result<(), String> {
        if let Some(found_network) = find_network(&network) {
            // Store canonical name, not alias.
            self.default_network = Some(found_network.name.to_string());
            return Ok(());
        }

        // An unlisted chain ID is a valid switch target (routed via routeme), so
        // it is a valid default too. Store the normalized numeric form.
        if let Ok(chain_id) = network.parse::<u64>() {
            if chain_id == 0 {
                return Err("Chain ID 0 is not valid.".to_string());
            }
            self.default_network = Some(chain_id.to_string());
            return Ok(());
        }

        Err(format!(
            "Unknown network: '{}'. Run 'stargate list' to see available networks.",
            network
        ))
    }
}

/// Writes `content` to `path` with owner-only permissions on unix.
#[cfg(unix)]
fn write_private(path: &Path, content: &str) -> Result<(), String> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)
        .map_err(|e| format!("Failed to write config: {}", e))?;
    file.write_all(content.as_bytes())
        .map_err(|e| format!("Failed to write config: {}", e))
}

#[cfg(not(unix))]
fn write_private(path: &Path, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("Failed to write config: {}", e))
}

/// Restricts the config directory to owner-only on unix. Best effort: a failure
/// here should not block a save.
#[cfg(unix)]
fn restrict_dir(dir: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = fs::set_permissions(dir, fs::Permissions::from_mode(0o700));
}

#[cfg(not(unix))]
fn restrict_dir(_dir: &Path) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_no_keys() {
        let config = Config::default();
        assert!(config.key_for(Provider::Alchemy).is_none());
        assert!(config.key_for(Provider::RouteMe).is_none());
    }

    #[test]
    fn config_serializes_keys_as_table() {
        let mut config = Config::default();
        config.keys.alchemy = Some("alchemy-key-123".to_string());
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("[keys]"));
        assert!(toml_str.contains("alchemy-key-123"));
    }

    #[test]
    fn empty_config_omits_keys_table() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(!toml_str.contains("[keys]"));
    }

    #[test]
    fn config_deserializes_per_provider_keys() {
        let toml_str = r#"
provider = "routeme"

[keys]
alchemy = "a-key"
routeme = "r-key"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.provider(), Provider::RouteMe);
        assert_eq!(config.key_for(Provider::Alchemy), Some("a-key"));
        assert_eq!(config.key_for(Provider::RouteMe), Some("r-key"));
    }

    #[test]
    fn config_deserializes_empty_toml() {
        let toml_str = "";
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.key_for(Provider::Alchemy).is_none());
        assert_eq!(config.provider(), Provider::Alchemy);
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
        let mut original = Config::default();
        original.keys.routeme = Some("roundtrip-test-key".to_string());
        original.provider = Some(Provider::RouteMe);

        let toml_str = toml::to_string(&original).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(
            deserialized.key_for(Provider::RouteMe),
            Some("roundtrip-test-key")
        );
        assert_eq!(deserialized.provider(), Provider::RouteMe);
    }

    #[test]
    fn legacy_api_key_migrates_into_alchemy_slot() {
        let mut config: Config = toml::from_str(r#"api_key = "legacy-key""#).unwrap();
        config.migrate_legacy();
        assert!(config.api_key.is_none());
        assert_eq!(config.key_for(Provider::Alchemy), Some("legacy-key"));
    }

    #[test]
    fn migrate_prefers_existing_alchemy_key() {
        let toml_str = r#"
api_key = "legacy-key"

[keys]
alchemy = "explicit-key"
"#;
        let mut config: Config = toml::from_str(toml_str).unwrap();
        config.migrate_legacy();
        assert!(config.api_key.is_none());
        assert_eq!(config.key_for(Provider::Alchemy), Some("explicit-key"));
    }

    #[test]
    fn migrated_config_drops_legacy_api_key_on_serialize() {
        let mut config: Config = toml::from_str(r#"api_key = "legacy-key""#).unwrap();
        config.migrate_legacy();
        let toml_str = toml::to_string(&config).unwrap();
        assert!(!toml_str.contains("api_key"));
        assert!(toml_str.contains("legacy-key"));
    }

    #[test]
    fn provider_defaults_to_alchemy() {
        assert_eq!(Config::default().provider(), Provider::Alchemy);
    }

    #[test]
    fn key_for_reads_the_matching_provider() {
        let mut config = Config::default();
        config.keys.alchemy = Some("a".to_string());
        config.keys.routeme = Some("r".to_string());
        assert_eq!(config.key_for(Provider::Alchemy), Some("a"));
        assert_eq!(config.key_for(Provider::RouteMe), Some("r"));
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
            default_network: Some("polygon".to_string()),
            ..Default::default()
        };
        assert_eq!(config.get_default_network(), "polygon");
    }

    #[test]
    fn set_default_network_accepts_valid_network_name() {
        let mut config = Config::default();
        let _result = config.assign_default_network("mainnet".to_string());

        // Should succeed (or fail only due to save, not validation)
        // We check that the network was stored correctly
        assert_eq!(config.default_network, Some("mainnet".to_string()));
    }

    #[test]
    fn set_default_network_accepts_valid_alias() {
        let mut config = Config::default();
        let _result = config.assign_default_network("arb".to_string());

        // Should normalize "arb" to "arbitrum"
        assert_eq!(config.default_network, Some("arbitrum".to_string()));
    }

    #[test]
    fn set_default_network_accepts_valid_chain_id() {
        let mut config = Config::default();
        let _result = config.assign_default_network("1".to_string());

        // Should normalize "1" to "mainnet"
        assert_eq!(config.default_network, Some("mainnet".to_string()));
    }

    #[test]
    fn set_default_network_rejects_invalid_network() {
        let mut config = Config::default();
        let result = config.assign_default_network("invalid-network-xyz".to_string());

        // Should fail with descriptive error
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown network"));
    }

    #[test]
    fn set_default_network_accepts_unlisted_chain_id() {
        let mut config = Config::default();
        config.assign_default_network("80094".to_string()).unwrap();
        assert_eq!(config.default_network, Some("80094".to_string()));
    }

    #[test]
    fn set_default_network_rejects_chain_zero() {
        let mut config = Config::default();
        assert!(config.assign_default_network("0".to_string()).is_err());
    }

    #[test]
    fn from_toml_rejects_unknown_provider() {
        // A hand-edited or newer-binary value must not silently become default.
        assert!(Config::from_toml(r#"provider = "infura""#).is_err());
    }

    #[test]
    fn from_toml_rejects_malformed_document() {
        assert!(Config::from_toml("this is not = valid = toml").is_err());
    }

    #[test]
    fn from_toml_accepts_valid_document() {
        let config = Config::from_toml(r#"provider = "routeme""#).unwrap();
        assert_eq!(config.provider(), Provider::RouteMe);
    }

    #[test]
    fn config_serializes_with_default_network() {
        let config = Config {
            default_network: Some("polygon".to_string()),
            ..Default::default()
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
    fn set_default_network_preserves_keys() {
        let mut config = Config::default();
        config.keys.alchemy = Some("existing-api-key".to_string());

        let _result = config.assign_default_network("mainnet".to_string());

        assert_eq!(config.key_for(Provider::Alchemy), Some("existing-api-key"));
        assert_eq!(config.default_network, Some("mainnet".to_string()));
    }
}
