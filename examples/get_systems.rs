#[tokio::main]
async fn main() {
    let client = passiogo_rs::PassioGoClient::default();
    let systems = client.get_systems().await.unwrap_or_default();

    println!("{:#?}", systems);
}
