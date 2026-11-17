//! Reads the artikelomzetgroepen, then creates, updates and deletes an artikel.
//!
//! `cargo run --example artikelen_crud`

use snelstart_b2b::Client;
use snelstart_b2b::models::artikelen::Artikel;
use snelstart_b2b::odata::Query;
use snelstart_b2b::types::{BoundedText, Ref};

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let groups = client.artikelomzetgroepen().list(&Query::new()).await?;
    for group in &groups {
        println!("{:?} {:?}", group.nummer, group.omschrijving);
    }
    let Some(group_id) = groups.first().and_then(|group| group.id) else {
        println!("no artikelomzetgroepen in this administration");
        return Ok(());
    };
    let group = client.artikelomzetgroepen().find(group_id).await?;
    println!("{:?}", group.verkoop_nederland_btw_soort);

    let created = client
        .artikelen()
        .create(&Artikel {
            artikelcode: Some("EXAMPLE-1".to_owned()),
            omschrijving: Some(BoundedText::new("Example article")?),
            artikel_omzetgroep: Some(Ref::to(group_id)),
            verkoopprijs: Some(10.into()),
            ..Default::default()
        })
        .await?;
    let id = created.id.expect("SnelStart returns an id");
    println!("created {id:?}");

    let found = client.artikelen().find(id).await?;
    println!("{:?} {:?}", found.artikelcode, found.vrije_voorraad);

    let mut article = created;
    article.artikelcode = Some("EXAMPLE-2".to_owned());
    client.artikelen().update(id, &article).await?;

    client.artikelen().delete(id).await?;
    println!("deleted");
    Ok(())
}
