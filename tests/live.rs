//! Runs against the real API. Needs `SNELSTART_CLIENT_KEY` and `SNELSTART_SUBSCRIPTION_KEY`:
//! `SNELSTART_CLIENT_KEY=... SNELSTART_SUBSCRIPTION_KEY=... cargo test --test live -- --ignored`

use snelstart_b2b::Client;
use snelstart_b2b::models::kostenplaatsen::Kostenplaats;
use snelstart_b2b::odata::Query;

#[tokio::test]
#[ignore = "talks to the real SnelStart API"]
async fn credentials_and_company_info() {
    let client = Client::from_env().unwrap();
    client.echo().await.unwrap();
    let info = client.company_info().await.unwrap();
    assert!(info.administratie_identifier.is_some());
}

#[tokio::test]
#[ignore = "talks to the real SnelStart API"]
async fn kostenplaats_round_trip() {
    let client = Client::from_env().unwrap();
    let new_cost_centre = Kostenplaats {
        omschrijving: Some("snelstart-b2b live test".to_owned()),
        ..Kostenplaats::default()
    };

    let created = client
        .kostenplaatsen()
        .create(&new_cost_centre)
        .await
        .unwrap();
    let id = created.id.unwrap();
    let result = async {
        let found = client.kostenplaatsen().find(id).await?;
        assert_eq!(found.omschrijving, new_cost_centre.omschrijving);
        let all: Vec<_> = client.kostenplaatsen().list(&Query::new()).await?;
        assert!(all.iter().any(|k| k.id == Some(id)));
        snelstart_b2b::Result::Ok(())
    }
    .await;

    client.kostenplaatsen().delete(id).await.unwrap();
    result.unwrap();
}
