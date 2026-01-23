use std::env;

pub fn run(target: Option<&str>, print: bool) {
    let network = env::var("STARGATE_NETWORK").ok();
    let Some(explorer) = env::var("BLOCK_EXPLORER").ok() else {
        eprintln!("No block explorer available for {}.", network.as_deref().unwrap_or("unknown network"));
        std::process::exit(1);
    };

    let url = match target {
        Some(t) if t.starts_with("0x") && t.len() == 66 => {
            // Transaction hash (0x + 64 hex chars)
            format!("{}/tx/{}", explorer, t)
        }
        Some(t) if t.starts_with("0x") && t.len() == 42 => {
            // Address (0x + 40 hex chars)
            format!("{}/address/{}", explorer, t)
        }
        Some(t) => {
            // Could be block number or other identifier
            format!("{}/search?q={}", explorer, t)
        }
        None => explorer,
    };

    if print {
        println!("{}", url);
    } else if let Err(e) = open::that(&url) {
        eprintln!("Failed to open browser: {}", e);
        eprintln!("URL: {}", url);
        std::process::exit(1);
    }
}
