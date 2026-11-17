//! Verkooporders, offertes, sjablonen, verkoopfacturen and verkoopboekingen.
//!
//! Changing the proces status of an order is only sent when `SNELSTART_EXAMPLE_SEND` is set.
//!
//! `cargo run --example verkoop`

use futures_util::TryStreamExt;
use snelstart_b2b::Client;
use snelstart_b2b::models::verkooporders::VerkooporderUpdateProcesStatus;
use snelstart_b2b::odata::{Direction, Query, Top};
use snelstart_b2b::types::ProcesStatus;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let templates = client.verkoopordersjablonen().list(&Query::new()).await?;
    println!("{} verkoopordersjablonen", templates.len());
    if let Some(id) = templates.first().and_then(|template| template.id) {
        let template = client.verkoopordersjablonen().find(id).await?;
        println!("{:?} {:?}", template.omschrijving, template.nonactief);
    }

    let recent = Query::new()
        .order_by("datum", Direction::Desc)?
        .top(Top::new(10)?);
    let orders = client.verkooporders().list(&recent).await?;
    for order in &orders {
        println!(
            "{:?} {:?} {:?}",
            order.nummer, order.proces_status, order.totaal_inclusief_btw
        );
    }

    let mut offers = Box::pin(client.offertes().all(Query::new()));
    let mut first_offer = None;
    let mut offer_count = 0;
    while let Some(offer) = offers.try_next().await? {
        first_offer = first_offer.or(offer.id);
        offer_count += 1;
    }
    println!("{offer_count} offertes");
    if let Some(id) = first_offer {
        let offer = client.offertes().find(id).await?;
        println!("{:?} {:?}", offer.nummer, offer.totaal_inclusief_btw);
    }

    if let Some(order_id) = orders.first().and_then(|order| order.id) {
        let order = client.verkooporders().find(order_id).await?;
        println!("{:?} {:?}", order.omschrijving, order.verkoop_order_status);

        let status = VerkooporderUpdateProcesStatus {
            proces_status: Some(ProcesStatus::Bevestiging),
            ..Default::default()
        };
        if std::env::var_os("SNELSTART_EXAMPLE_SEND").is_some() {
            client
                .set_verkooporder_proces_status(order_id, &status)
                .await?;
            println!("proces status changed");
        } else {
            println!("would set {}", serde_json::to_string(&status).unwrap());
        }
    }

    let invoices = client.verkoopfacturen().list(&recent).await?;
    if let Some(invoice_id) = invoices.first().and_then(|invoice| invoice.id) {
        let invoice = client.verkoopfacturen().find(invoice_id).await?;
        println!("{:?} {:?}", invoice.factuurnummer, invoice.openstaand_saldo);

        let ubl = client.get_verkoopfactuur_ubl(invoice_id).await?;
        println!("ubl: {} bytes of json", ubl.to_string().len());

        if let Some(booking) = invoice.verkoop_boeking {
            let booking = client.verkoopboekingen().find(booking.id).await?;
            println!("{:?} {:?}", booking.boekstuk, booking.factuurbedrag);
        }
    }
    Ok(())
}
