//! Lists a collection and reads one item by id.
//!
//! `cargo run --example reading`

use snelstart_b2b::Client;
use snelstart_b2b::odata::Query;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let countries = client.landen().list(&Query::new()).await?;
    println!("{} countries", countries.len());

    if let Some(id) = countries.first().and_then(|country| country.id) {
        let country = client.landen().find(id).await?;
        println!("{:?} {:?}", country.naam, country.landcode_iso);
    }

    let cost_centres = client.kostenplaatsen().list(&Query::new()).await?;
    println!("{} kostenplaatsen", cost_centres.len());

    let relations = client.relaties().list(&Query::new()).await?;
    if let Some(id) = relations.first().and_then(|relation| relation.id) {
        let relation = client.relaties().find(id).await?;
        println!("{:?} {:?}", relation.relatiecode, relation.naam);
    }
    Ok(())
}
