//! Checks the subscription key and the client key.
//!
//! `cargo run --example check_keys`

use snelstart_b2b::Client;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    client.echo().await?;
    println!("subscription key accepted");

    let company = client.company_info().await?;
    println!("client key accepted: {:?}", company.administratie_naam);
    Ok(())
}
