//! Lists artikelen and asks for the price of one, optionally for a relatie and quantity.
//!
//! `cargo run --example artikelen`

use snelstart_b2b::Client;
use snelstart_b2b::odata::{Query, Top};

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let articles = client
        .artikelen()
        .list(&Query::new().top(Top::new(10)?))
        .await?;
    for article in &articles {
        println!("{:?} {:?}", article.artikelcode, article.omschrijving);
    }

    if let Some(id) = articles.first().and_then(|article| article.id) {
        let priced = client.get_artikel_with_price(id, None, Some(5)).await?;
        println!("{priced:?}");
    }
    Ok(())
}
