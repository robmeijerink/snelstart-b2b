//! Company info, user access and custom fields of a relatie.
//!
//! Updating is only sent when `SNELSTART_EXAMPLE_SEND` is set.
//!
//! `cargo run --example administratie`

use snelstart_b2b::Client;
use snelstart_b2b::models::custom_fields::RelatieUpdatedCustomFields;
use snelstart_b2b::odata::{Query, Top};

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;
    let send = std::env::var_os("SNELSTART_EXAMPLE_SEND").is_some();

    let mut info = client.company_info().await?;
    println!("{:?} {:?}", info.administratie_naam, info.huidig_boekjaar);

    if let Some(user) = std::env::args().nth(1) {
        let access = client
            .has_user_access(user.parse().expect("user id is a uuid"))
            .await?;
        println!("access: {:?}", access.has_access);
    }

    info.vrije_tekst1 = Some("Example".to_owned());
    if send {
        client.update_company_info(&info).await?;
        println!("company info updated");
    } else {
        println!("would set vrije_tekst1 to {:?}", info.vrije_tekst1);
    }

    let relations = client
        .relaties()
        .list(&Query::new().top(Top::new(1)?))
        .await?;
    if let Some(id) = relations.first().and_then(|relation| relation.id) {
        let fields = client.get_relatie_custom_fields(id).await?;
        println!("{} klant custom fields", fields.klant_custom_fields.len());

        let update = RelatieUpdatedCustomFields::default();
        if send {
            client.update_relatie_custom_fields(id, &update).await?;
            println!("custom fields updated");
        } else {
            println!("would send {}", serde_json::to_string(&update).unwrap());
        }
    }
    Ok(())
}
