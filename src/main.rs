use std::collections::HashMap;

use crate::util::{
    command::{Commander, parse},
    fs,
};

mod util;

#[tokio::main]
async fn main() {
    let config = util::config::boot();
    let client = util::client::new().await;
    let commands = match parse(fs::read_rub_file(&config.index_file).as_str()) {
        Some(d) => d,
        None => panic!("Failed to parse commands"),
    };

    let mut commander = Commander::new(client.clone(), commands, HashMap::new());
    commander.exec().await;

    let _ = client.close().await;
}
