use std::env;

pub fn run() {
    match env::var("ETH_RPC_URL").ok() {
        Some(rpc_url) => {
            println!("{}", rpc_url);
        }
        None => {
            println!("No network selected. Run 'switch <network>' first.");
        }
    }
}
