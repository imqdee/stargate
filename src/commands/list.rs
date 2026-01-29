use crate::networks::NETWORKS;

pub fn run() {
    println!(
        "{:<20} {:<15} {:>10}   Explorer",
        "Network", "Aliases", "Chain ID"
    );
    println!("{}", "-".repeat(75));

    for network in NETWORKS.iter() {
        let aliases = if network.aliases.is_empty() {
            "-".to_string()
        } else {
            network.aliases.join(", ")
        };

        let explorer = network.explorer_url.unwrap_or("-");

        println!(
            "{:<20} {:<15} {:>10}   {}",
            network.name, aliases, network.chain_id, explorer
        );
    }
}
