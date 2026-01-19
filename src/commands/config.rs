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
