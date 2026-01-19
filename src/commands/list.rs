use crate::networks::NETWORKS;

pub fn run() {
    println!("{:<12} {:<15} {:>10}   {}", "Network", "Aliases", "Chain ID", "Explorer");
    println!("{}", "-".repeat(70));

    for network in NETWORKS.iter() {
        let aliases = if network.aliases.is_empty() {
            "-".to_string()
        } else {
            network.aliases.join(", ")
        };

        let explorer = network.explorer_url.unwrap_or("-");

        println!(
            "{:<12} {:<15} {:>10}   {}",
            network.name, aliases, network.chain_id, explorer
        );
    }
}
