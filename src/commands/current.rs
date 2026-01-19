use std::env;

pub fn run() {
    let network = env::var("STARGATE_NETWORK").ok();
    let chain_id = env::var("STARGATE_CHAIN_ID").ok();

    match (network, chain_id) {
        (Some(net), Some(id)) => {
            println!("{} ({})", net, id);
        }
        _ => {
            println!("No network selected. Run 'travel <network>' first.");
        }
    }
}
