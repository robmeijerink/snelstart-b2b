//! Reads what belongs to one relatie: custom fields and its bookings.
//!
//! `cargo run --example relatie_details`

use snelstart_b2b::Client;
use snelstart_b2b::odata::Query;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let relations = client.relaties().list(&Query::new()).await?;
    let Some(id) = relations.first().and_then(|relation| relation.id) else {
        println!("no relaties in this administration");
        return Ok(());
    };

    let fields = client.get_relatie_custom_fields(id).await?;
    println!("{fields:?}");

    let sales = client.list_relatie_verkoopboekingen(id).await?;
    println!("{} verkoopboekingen", sales.len());

    let purchases = client.list_relatie_inkoopboekingen(id).await?;
    println!("{} inkoopboekingen", purchases.len());

    let mandates = client.list_relatie_incassomachtigingen(id).await?;
    println!("{} incassomachtigingen", mandates.len());
    Ok(())
}
