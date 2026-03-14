use std::env;

use fantoccini::{Client, ClientBuilder};
use serde_json::json;

pub async fn new() -> Client {
    let mut caps = serde_json::Map::new();
    let current_dir = env::home_dir().expect("Gagal mendapatkan direktori saat ini");
    let chrome_path =
        current_dir.join("Dev/Projects/Personal/ruber/browsers/chrome-linux64/chrome");
    let chrome_path_str = chrome_path.to_str().expect("Path tidak valid");
    caps.insert(
        "goog:chromeOptions".to_string(),
        json!({
            "binary": chrome_path_str
        }),
    );

    ClientBuilder::native()
        .capabilities(caps)
        .connect("http://localhost:44877")
        .await
        .expect("Cannot connect to the client.")
}
