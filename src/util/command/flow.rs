use std::time::Duration;
use tokio::time::sleep;

pub async fn wait_for(amount: u64, unit: String) {
    let seconds = amount;
    sleep(Duration::from_secs(seconds)).await;
}

pub async fn hangup() {
    println!("Tekan Ctrl+C di terminal ini untuk melanjutkan/menutup program...");
    tokio::signal::ctrl_c().await.expect("Gagal mendengarkan sinyal Ctrl+C");
}
