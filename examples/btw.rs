//! Btw-aangiftes, btwtarieven and the vat rates per country.
//!
//! Marking a declaration as filed elsewhere is only sent when `SNELSTART_EXAMPLE_SEND` is set.
//!
//! `cargo run --example btw`

use snelstart_b2b::Client;
use snelstart_b2b::models::btw_aangiftes::UpdateBtwAangifteStatus;
use snelstart_b2b::odata::Query;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    for rate in client.btwtarieven().list(&Query::new()).await? {
        println!(
            "{:?} {:?} from {:?}",
            rate.btw_soort, rate.btw_percentage, rate.datum_vanaf
        );
    }

    let definitions = client.vatratedefinitions().list(&Query::new()).await?;
    println!("{} vatratedefinitions", definitions.len());

    let vat_rates = client.vatrates().list(&Query::new()).await?;
    for rates in &vat_rates {
        println!("{:?}: {} rates", rates.country_code, rates.vat_rates.len());
    }

    let declarations = client.btwaangiftes().list(&Query::new()).await?;
    println!("{} btwaangiftes", declarations.len());
    if let Some(id) = declarations.first().and_then(|declaration| declaration.id) {
        let declaration = client.btwaangiftes().find(id).await?;
        println!(
            "{:?} {:?}",
            declaration.btw_aangifte_status, declaration.betalen_voor
        );

        let status = UpdateBtwAangifteStatus {
            is_extern_aangegeven: Some(true),
        };
        if std::env::var_os("SNELSTART_EXAMPLE_SEND").is_some() {
            client.update_btwaangifte_status(id, &status).await?;
            println!("marked as filed");
        } else {
            println!("would set {}", serde_json::to_string(&status).unwrap());
        }
    }
    Ok(())
}
