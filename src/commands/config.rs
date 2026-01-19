use crate::config::Config;

pub fn set_api_key(key: &str) {
    let mut config = Config::load();

    if let Err(e) = config.set_api_key(key.to_string()) {
        eprintln!("Failed to save config: {}", e);
        std::process::exit(1);
    }

    println!("API key saved successfully.");
}
