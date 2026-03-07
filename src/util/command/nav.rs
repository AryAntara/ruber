use fantoccini::Client;
pub async fn go(url: String, page: &Client) {
    page.goto(&url).await;
}
