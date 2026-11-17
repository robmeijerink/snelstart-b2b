//! Fetches the periodebalans and the kolommenbalans for a year.
//!
//! `cargo run --example reports`

use snelstart_b2b::Client;
use snelstart_b2b::types::DateTime;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let start: DateTime = "2024-01-01".parse()?;
    let end: DateTime = "2024-12-31".parse()?;

    let periodebalans = client.get_periodebalans(start, end).await?;
    println!("{} periodebalans rows", periodebalans.len());

    let kolommenbalans = client.get_kolommenbalans(start, end).await?;
    println!("{} kolommenbalans rows", kolommenbalans.len());
    Ok(())
}
