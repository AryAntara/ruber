use crate::case::login::auth;

mod browser;
mod case;

#[tokio::main]
async fn main() {
    auth().await;
}
