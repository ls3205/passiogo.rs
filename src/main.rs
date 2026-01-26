use passiogo_rs::PassioGoClient;

#[tokio::main]
async fn main() {
    let client = PassioGoClient::new();

    let systems = match client.get_systems().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to fetch systems: {}", e);
            return;
        }
    };

    let lehigh = systems
        .iter()
        .find(|s| s.name.as_deref().unwrap_or("").contains("Lehigh"));

    let lehigh = match lehigh {
        Some(s) => s,
        None => {
            eprint!("Lehigh not found");
            return;
        }
    };

    match client.get_stops(lehigh.id).await {
        Ok(x) => println!("{:#?}", x),
        Err(e) => eprint!("Failed to fetch alerts: {}", e),
    }
}
