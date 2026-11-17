//! Prijsafspraken and actieprijzen.
//!
//! `cargo run --example prijzen`

use snelstart_b2b::Client;
use snelstart_b2b::odata::{Filter, Query};

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let promotions = client.actieprijzen().list(&Query::new()).await?;
    for promotion in &promotions {
        println!(
            "{:?} {:?} {:?}",
            promotion.omschrijving, promotion.startdatum, promotion.einddatum
        );
    }

    let all_agreements = client.artikel_prijsafspraken().list(&Query::new()).await?;
    println!("{} artikelprijsafspraken", all_agreements.len());

    let article = client.artikelen().list(&Query::new()).await?;
    if let Some(id) = article.first().and_then(|article| article.id) {
        let for_article = Query::new().filter(Filter::eq("Artikel/Id", id)?);
        for agreement in client.prijsafspraken().list(&for_article).await? {
            println!(
                "{:?} {:?}",
                agreement.relatie.map(|r| r.id),
                agreement.verkoopprijs
            );
        }
    }
    Ok(())
}
