//! Walks through every page of a collection with `all`.
//!
//! `cargo run --example paging`

use futures_util::TryStreamExt;
use snelstart_b2b::Client;
use snelstart_b2b::odata::Query;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let mut invoices = Box::pin(client.verkoopfacturen().all(Query::new()));
    let mut count = 0;
    while let Some(invoice) = invoices.try_next().await? {
        println!("{:?}", invoice.factuurnummer);
        count += 1;
    }
    println!("{count} invoices");
    Ok(())
}
