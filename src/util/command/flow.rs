use std::time::Duration;
use tokio::time::sleep;

pub async fn wait_for(amount: u64, unit: String) {
    let seconds = amount;
    print!("Waiting for {seconds} {unit}...");
    sleep(Duration::from_secs(seconds)).await;
}
