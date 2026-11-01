use futures_util::TryStreamExt;
use snelstart_b2b::Client;
use snelstart_b2b::odata::{Filter, Query};

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    let client = Client::from_env()?;

    let company = client.company_info().await?;
    println!("{}", company.administratie_naam.unwrap_or_default());

    let query = Query::new().filter(Filter::any_eq("Relatiesoort", "r", "Klant")?);
    let mut relations = Box::pin(client.relaties().all(query));
    while let Some(relation) = relations.try_next().await? {
        println!(
            "{:>6} {}",
            relation.relatiecode.unwrap_or_default(),
            relation.naam.map(|n| n.to_string()).unwrap_or_default()
        );
    }
    Ok(())
}
