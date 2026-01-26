# PassioGo Rust Client

> Please note, this project is still in a very early state.

A minimal Rust client for Passio GO APIs. The project currently supports fetching systems, routes, alerts, vehicles, and stops using `reqwest` and `tokio`. WebSocket support with `tungstenite` is planned.

## Features
- Fetch transportation systems
- Fetch routes
- Fetch alerts
- Fetch vehicles
- Fetch stops

## Status
Work in progress. API coverage is partial and may change.

## Requirements
- Rust 1.70+
- Linux/macOS/Windows
- Internet access

## Setup
```bash
cargo build
```

## Usage
Example from `src/main.rs`:
```rust
use passiogo_rs::PassioGoClient;

#[tokio::main]
async fn main() {
    let client = PassioGoClient::new();

    let systems = client.get_systems().await.unwrap();
    let uchicago = systems
        .iter()
        .find(|s| s.name.as_deref().unwrap_or("").contains("Chicago"))
        .expect("UChicago not found");

    let stops = client.get_stops(uchicago.id).await.unwrap();
    println!("{:#?}", stops);
}
