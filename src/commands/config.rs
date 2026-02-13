use crate::config::Config;

pub fn set_api_key(key: Option<String>) {
    let key = match key {
        Some(k) => k,
        None => prompt_api_key(),
    };

    if key.is_empty() {
        eprintln!("API key cannot be empty.");
        std::process::exit(1);
    }

    let mut config = Config::load();

    if let Err(e) = config.set_api_key(key) {
        eprintln!("Failed to save config: {}", e);
        std::process::exit(1);
    }

    println!("API key saved successfully.");
}

fn prompt_api_key() -> String {
    eprint!("Enter your Alchemy API key: ");
    match rpassword::read_password() {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Failed to read API key: {}", e);
            std::process::exit(1);
        }
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
