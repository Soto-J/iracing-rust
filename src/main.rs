use data::IracingClient;
mod data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = IracingClient::new().await?;

    println!("");

    client.get_iracing_data().await?;

    Ok(())
}

// Blocking
// fn main() {
//     let body = blocking_get().unwrap();
// }
