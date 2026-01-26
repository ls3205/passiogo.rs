use passiogo_rs::{PassioGoClient, StopData};

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

    let stops = match client.get_stops(lehigh.id).await {
        Ok(x) => x,
        Err(_e) => return,
    };

    let routes = match client.get_routes(lehigh.id).await {
        Ok(x) => x,
        Err(_e) => return,
    };

    let cc = routes
        .iter()
        .find(|r| r.name.as_deref().unwrap_or("").contains("Campus Conn"));

    if let Some(cc) = cc {
        let mut cc_stops: Vec<StopData> = stops
            .iter()
            .filter(|s| {
                s.routes_and_positions
                    .keys()
                    .any(|k| k == &cc.myid.clone().unwrap_or("".to_string()))
            })
            .cloned()
            .collect::<Vec<StopData>>();

        cc_stops.sort_by(|a, b| {
            let pos_a = a
                .routes_and_positions
                .get(&cc.myid.clone().unwrap_or("".to_string()))
                .and_then(|v| v.first())
                .unwrap_or(&f64::MAX);
            let pos_b = b
                .routes_and_positions
                .get(&cc.myid.clone().unwrap_or("".to_string()))
                .and_then(|v| v.first())
                .unwrap_or(&f64::MAX);

            pos_a
                .partial_cmp(pos_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !cc_stops.is_empty() {
            let stop = cc_stops.first().unwrap();

            let etas = match client
                .get_etas(
                    &stop.id,
                    &cc.myid.clone().unwrap_or("".to_string()),
                    stop.routes_and_positions
                        .get(&cc.myid.clone().unwrap_or("".to_string()))
                        .and_then(|v| v.first())
                        .unwrap_or(&0.0),
                    &lehigh.id,
                )
                .await
            {
                Ok(x) => x,
                Err(_e) => return,
            };

            println!("{:#?}", etas);
        }

        // println!(
        //     "{:#?}",
        //     cc_stops
        //         .iter()
        //         .map(|s| format!(
        //             "{}: {}",
        //             s.name.clone().unwrap_or("Unknown Stop".to_string()),
        //             s.routes_and_positions
        //                 .get(&cc.myid.clone().unwrap_or("".to_string()))
        //                 .unwrap_or(&vec![])
        //                 .first()
        //                 .unwrap_or(&f64::MAX)
        //         ))
        //         .collect::<Vec<String>>()
        // );
    }
}
