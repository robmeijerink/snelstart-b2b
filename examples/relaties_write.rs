//! Creates a relatie, updates it and deletes it again.
//!
//! `cargo run --example relaties_write`

use snelstart_b2b::Client;
use snelstart_b2b::models::relaties::RelatieWrite;
use snelstart_b2b::types::{BoundedText, Email, Relatiesoort};

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let new_relation = RelatieWrite {
        naam: Some(BoundedText::new("Example Installations B.V.")?),
        relatiesoort: vec![Relatiesoort::Klant],
        email: Some(Email::new("info@example.nl")?),
        ..Default::default()
    };
    let created = client.relaties().create(&new_relation).await?;
    let id = created.id.expect("SnelStart returns an id");
    println!("created {id:?}");

    let relation = client.relaties().find(id).await?;
    let mut changes = RelatieWrite::from(relation);
    changes.memo = Some("Regular customer".into());
    client.relaties().update(id, &changes).await?;

    client.relaties().delete(id).await?;
    println!("deleted");
    Ok(())
}
