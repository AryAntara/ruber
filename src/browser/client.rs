use fantoccini::{Client, ClientBuilder};

pub async fn new() -> Client {
    return ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("Cannot connect to the client.");
    
}
