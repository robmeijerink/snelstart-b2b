//! Filters, sorts and limits a list with `Query`.
//!
//! `cargo run --example querying`

use snelstart_b2b::Client;
use snelstart_b2b::odata::{Direction, Filter, Op, Query, Top};

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let customers = Query::new().filter(Filter::any_eq("Relatiesoort", "r", "Klant")?);
    for relation in client.relaties().list(&customers).await? {
        println!("{:?}", relation.naam);
    }

    let open_invoices = Query::new()
        .filter(
            Filter::compare("openstaandSaldo", Op::Gt, 0)?
                .and(Filter::contains("factuurnummer", "2024")?),
        )
        .order_by("factuurdatum", Direction::Desc)?
        .top(Top::new(50)?);
    for invoice in client.verkoopfacturen().list(&open_invoices).await? {
        println!("{:?} {:?}", invoice.factuurnummer, invoice.openstaand_saldo);
    }
    Ok(())
}
