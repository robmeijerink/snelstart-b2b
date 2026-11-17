//! Dagboeken, grootboeken, grootboekmutaties and the bank, kas and memoriaal boekingen.
//!
//! A memoriaalboeking cannot be listed, so it is read when you pass its id:
//!
//! `cargo run --example boekingen -- <memoriaalboeking id>`

use futures_util::TryStreamExt;
use snelstart_b2b::Client;
use snelstart_b2b::odata::{Direction, Query, Top};
use snelstart_b2b::types::Id;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;
    let recent = Query::new()
        .order_by("datum", Direction::Desc)?
        .top(Top::new(10)?);

    for journal in client.dagboeken().list(&Query::new()).await? {
        println!(
            "{:?} {:?} {:?}",
            journal.nummer, journal.omschrijving, journal.soort
        );
    }

    let mut ledgers = Box::pin(client.grootboeken().all(Query::new()));
    let mut first_ledger = None;
    while let Some(ledger) = ledgers.try_next().await? {
        println!("{:?} {:?}", ledger.nummer, ledger.omschrijving);
        first_ledger = first_ledger.or(ledger.id);
    }
    if let Some(id) = first_ledger {
        let ledger = client.grootboeken().find(id).await?;
        println!("{:?} {:?}", ledger.grootboekfunctie, ledger.btw_soort);
    }

    let mutations = client.grootboekmutaties().list(&recent).await?;
    for mutation in &mutations {
        println!(
            "{:?} {:?} {:?}",
            mutation.boekstuk, mutation.debet, mutation.credit
        );
    }
    if let Some(id) = mutations.first().and_then(|mutation| mutation.id) {
        println!("{:?}", client.grootboekmutaties().find(id).await?.saldo);
    }

    let bank = client.bankboekingen().list(&recent).await?;
    println!("{} bankboekingen", bank.len());
    if let Some(id) = bank.first().and_then(|booking| booking.id) {
        println!("{:?}", client.bankboekingen().find(id).await?.omschrijving);
    }

    let cash = client.kasboekingen().list(&recent).await?;
    println!("{} kasboekingen", cash.len());
    if let Some(id) = cash.first().and_then(|booking| booking.id) {
        println!("{:?}", client.kasboekingen().find(id).await?.omschrijving);
    }

    if let Some(arg) = std::env::args().nth(1) {
        let id = Id::parse(&arg).expect("memoriaalboeking id is a uuid");
        let journal_entry = client.memoriaalboekingen().find(id).await?;
        println!(
            "{:?} {:?}",
            journal_entry.boekstuk, journal_entry.omschrijving
        );
    }
    Ok(())
}
