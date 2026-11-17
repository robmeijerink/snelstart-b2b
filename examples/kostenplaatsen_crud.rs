//! Creates, reads, updates and deletes a kostenplaats.
//!
//! `cargo run --example kostenplaatsen_crud`

use snelstart_b2b::Client;
use snelstart_b2b::models::kostenplaatsen::Kostenplaats;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let created = client
        .kostenplaatsen()
        .create(&Kostenplaats {
            omschrijving: Some("Example".to_owned()),
            ..Kostenplaats::default()
        })
        .await?;
    let id = created.id.expect("SnelStart returns an id");
    println!("created {id:?}");

    let mut cost_centre = client.kostenplaatsen().find(id).await?;
    cost_centre.omschrijving = Some("Example, renamed".to_owned());
    client.kostenplaatsen().update(id, &cost_centre).await?;

    client.kostenplaatsen().delete(id).await?;
    println!("deleted");
    Ok(())
}
