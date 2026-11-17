//! Handles a rejected call by its SnelStart error code.
//!
//! The call below is refused because the relatie has no name, so nothing is created.
//!
//! `cargo run --example errors`

use snelstart_b2b::models::relaties::RelatieWrite;
use snelstart_b2b::{Client, Error};

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    match client.relaties().create(&RelatieWrite::default()).await {
        Err(e) if e.has_code("REL-0007") => println!("name is missing"),
        Err(Error::Unauthorized) => println!("check the client key"),
        Err(Error::RateLimited { retry_after }) => println!("retry after {retry_after:?}"),
        Err(e) => {
            for code in e.error_codes() {
                println!("{code}: {}", code.message().unwrap_or("unknown"));
            }
        }
        Ok(created) => println!("created {:?}", created.id),
    }
    Ok(())
}
