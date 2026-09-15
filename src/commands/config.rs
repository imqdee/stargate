use crate::config::Config;
use crate::provider::Provider;

pub fn set_api_key(key: Option<String>, provider_arg: Option<String>) {
    let provider = match provider_arg {
        Some(name) => match Provider::parse(&name) {
            Some(p) => p,
            None => {
                eprintln!(
                    "Unknown provider: '{}'. Valid providers: alchemy, routeme.",
                    name
                );
                std::process::exit(1);
            }
        },
        None => Config::load().provider(),
    };

    let key = match key {
        Some(k) => k,
        None => prompt_api_key(provider),
    };

    if let Err(e) = validate_key(&key) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    // Reload after the (possibly slow, interactive) prompt so a concurrent edit
    // from another shell is merged in rather than clobbered.
    let mut config = Config::load();

    if let Err(e) = config.set_key(provider, key) {
        eprintln!("Failed to save config: {}", e);
        std::process::exit(1);
    }

    println!("{} API key saved successfully.", provider.as_str());
}

/// Rejects keys that are empty or carry characters outside the set both
/// providers use. This keeps the key safe to embed in a URL and in the shell
/// `export` statements the `sg` wrapper evaluates.
fn validate_key(key: &str) -> Result<(), String> {
    if key.is_empty() {
        return Err("API key cannot be empty.".to_string());
    }
    if !key
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    {
        return Err(
            "API key has invalid characters. Allowed: letters, digits, '-', '_', '.'".to_string(),
        );
    }
    Ok(())
}

fn prompt_api_key(provider: Provider) -> String {
    eprint!("Enter your {} API key: ", provider.as_str());
    match rpassword::read_password() {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Failed to read API key: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn set_provider(provider: String) {
    let Some(parsed) = Provider::parse(&provider) else {
        eprintln!(
            "Unknown provider: '{}'. Valid providers: alchemy, routeme.",
            provider
        );
        std::process::exit(1);
    };

    let mut config = Config::load();

    match config.set_provider(parsed) {
        Ok(()) => {
            println!("Provider set to '{}' successfully.", parsed.as_str());
            println!("This will be used when you start a new shell.");
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

pub fn get_provider() {
    let config = Config::load();
    if config.provider.is_some() {
        println!("Provider: {}", config.provider().as_str());
    } else {
        println!("Provider: {} (default)", config.provider().as_str());
    }
}

pub fn set_default_network(network: String) {
    let mut config = Config::load();

    match config.set_default_network(network.clone()) {
        Ok(()) => {
            // Show canonical name that was stored (may differ from input if alias was used)
            let stored = config.default_network.as_ref().unwrap();
            println!("Default network set to '{}' successfully.", stored);
            println!("This will be used when you start a new shell.");
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

pub fn get_default_network() {
    let config = Config::load();
    let default = config.get_default_network();

    if config.default_network.is_some() {
        println!("Default network: {}", default);
    } else {
        println!("Default network: {} (system default)", default);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_key_accepts_real_key_shapes() {
        // Alchemy-style and routeme UUID-style keys.
        assert!(validate_key("AbCdEf0123456789_key").is_ok());
        assert!(validate_key("51a7cb4a-fc94-4ba4-ab93-8a3bf301464b").is_ok());
    }

    #[test]
    fn validate_key_rejects_empty() {
        assert!(validate_key("").is_err());
    }

    #[test]
    fn validate_key_rejects_shell_metacharacters() {
        for bad in [
            "abc\"; echo pwned",
            "key with space",
            "a/b",
            "a$b",
            "a`b",
            "a\nb",
        ] {
            assert!(validate_key(bad).is_err(), "should reject {:?}", bad);
        }
    }
}
