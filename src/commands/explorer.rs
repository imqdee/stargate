use std::env;

/// Builds the explorer URL based on the target type.
/// Returns the full URL to open.
pub fn build_explorer_url(base_url: &str, target: Option<&str>) -> String {
    match target {
        Some(t) if t.starts_with("0x") && t.len() == 66 => {
            // Transaction hash (0x + 64 hex chars)
            format!("{}/tx/{}", base_url, t)
        }
        Some(t) if t.starts_with("0x") && t.len() == 42 => {
            // Address (0x + 40 hex chars)
            format!("{}/address/{}", base_url, t)
        }
        Some(t) => {
            // Could be block number or other identifier
            format!("{}/search?q={}", base_url, t)
        }
        None => base_url.to_string(),
    }
}

pub fn run(target: Option<&str>, print: bool) {
    let network = env::var("STARGATE_NETWORK").ok();
    let Some(explorer) = env::var("BLOCK_EXPLORER").ok() else {
        eprintln!("No block explorer available for {}.", network.as_deref().unwrap_or("unknown network"));
        std::process::exit(1);
    };

    let url = build_explorer_url(&explorer, target);

    if print {
        println!("{}", url);
    } else if let Err(e) = open::that(&url) {
        eprintln!("Failed to open browser: {}", e);
        eprintln!("URL: {}", url);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_URL: &str = "https://etherscan.io";

    // ==================== Transaction hash tests ====================

    #[test]
    fn builds_tx_url_for_transaction_hash() {
        let tx_hash = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        assert_eq!(tx_hash.len(), 66); // 0x + 64 chars

        let url = build_explorer_url(BASE_URL, Some(tx_hash));
        assert_eq!(url, format!("{}/tx/{}", BASE_URL, tx_hash));
    }

    #[test]
    fn tx_hash_must_be_exact_length() {
        // 65 chars (too short for tx hash)
        let short = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcde";
        assert_eq!(short.len(), 65);

        let url = build_explorer_url(BASE_URL, Some(short));
        // Should fall through to search since it's not exactly 66 chars
        assert!(url.contains("/search?q="));
    }

    // ==================== Address tests ====================

    #[test]
    fn builds_address_url_for_address() {
        let address = "0x1234567890abcdef1234567890abcdef12345678";
        assert_eq!(address.len(), 42); // 0x + 40 chars

        let url = build_explorer_url(BASE_URL, Some(address));
        assert_eq!(url, format!("{}/address/{}", BASE_URL, address));
    }

    #[test]
    fn address_must_be_exact_length() {
        // 41 chars (too short for address)
        let short = "0x1234567890abcdef1234567890abcdef1234567";
        assert_eq!(short.len(), 41);

        let url = build_explorer_url(BASE_URL, Some(short));
        assert!(url.contains("/search?q="));
    }

    // ==================== Search/fallback tests ====================

    #[test]
    fn builds_search_url_for_block_number() {
        let block = "12345678";
        let url = build_explorer_url(BASE_URL, Some(block));
        assert_eq!(url, format!("{}/search?q={}", BASE_URL, block));
    }

    #[test]
    fn builds_search_url_for_arbitrary_string() {
        let query = "some-token-name";
        let url = build_explorer_url(BASE_URL, Some(query));
        assert_eq!(url, format!("{}/search?q={}", BASE_URL, query));
    }

    #[test]
    fn builds_search_url_for_short_hex() {
        // Too short to be address or tx hash
        let short_hex = "0x1234";
        let url = build_explorer_url(BASE_URL, Some(short_hex));
        assert_eq!(url, format!("{}/search?q={}", BASE_URL, short_hex));
    }

    // ==================== No target tests ====================

    #[test]
    fn returns_base_url_when_no_target() {
        let url = build_explorer_url(BASE_URL, None);
        assert_eq!(url, BASE_URL);
    }

    #[test]
    fn preserves_trailing_slash_in_base_url() {
        let base_with_slash = "https://etherscan.io/";
        let url = build_explorer_url(base_with_slash, None);
        assert_eq!(url, base_with_slash);
    }

    // ==================== Different explorer URLs ====================

    #[test]
    fn works_with_different_explorers() {
        let address = "0x1234567890abcdef1234567890abcdef12345678";

        let url = build_explorer_url("https://polygonscan.com", Some(address));
        assert_eq!(url, format!("https://polygonscan.com/address/{}", address));

        let url = build_explorer_url("https://arbiscan.io", Some(address));
        assert_eq!(url, format!("https://arbiscan.io/address/{}", address));
    }
}
