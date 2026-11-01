mod common;

use std::time::Duration;

use common::*;
use snelstart_b2b::Error;
use snelstart_b2b::models::relaties::{Relatie, RelatieWrite};
use snelstart_b2b::odata::{Direction, Filter, Query, Top};
use snelstart_b2b::types::{Id, Relatiesoort};
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

const RELATIE_ID: &str = "3f2504e0-4f89-41d3-9a0c-0305e82c3301";

#[tokio::test]
async fn validation_errors_carry_the_snelstart_codes() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("POST"))
        .and(path("/v2/relaties"))
        .respond_with(
            ResponseTemplate::new(400).set_body_raw(fixture("error_rel_0007"), "application/json"),
        )
        .mount(&server)
        .await;

    let err = client
        .relaties()
        .create(&RelatieWrite::default())
        .await
        .unwrap_err();
    assert_eq!(err.status().map(|s| s.as_u16()), Some(400));
    assert!(err.has_code("REL-0007"));
    assert!(err.has_code("REL-0013"));
    assert_eq!(
        err.error_codes()[0].message(),
        Some("Naam moet gevuld zijn")
    );
    assert_eq!(
        err.to_string(),
        "SnelStart answered 400 Bad Request (REL-0007, REL-0013)"
    );
}

#[tokio::test]
async fn a_missing_resource_is_not_found() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("GET"))
        .and(path(format!("/v2/relaties/{RELATIE_ID}")))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let err = client
        .relaties()
        .find(Id::parse(RELATIE_ID).unwrap())
        .await
        .unwrap_err();
    assert!(matches!(err, Error::NotFound));
    assert_eq!(err.status().map(|s| s.as_u16()), Some(404));
}

#[tokio::test]
async fn rate_limiting_reports_the_wait_and_is_not_retried() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("GET"))
        .and(path("/v2/relaties"))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "7"))
        .expect(1)
        .mount(&server)
        .await;

    let err = client.relaties().list(&Query::new()).await.unwrap_err();
    assert!(
        matches!(err, Error::RateLimited { retry_after: Some(d) } if d == Duration::from_secs(7))
    );
}

#[tokio::test]
async fn server_errors_are_api_errors_and_not_retried() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("GET"))
        .and(path("/v2/relaties"))
        .respond_with(ResponseTemplate::new(503).set_body_string("down"))
        .expect(1)
        .mount(&server)
        .await;

    let err = client.relaties().list(&Query::new()).await.unwrap_err();
    assert!(
        matches!(&err, Error::Api { status, body, .. } if status.as_u16() == 503 && body == "down")
    );
}

#[tokio::test]
async fn keys_echoed_in_an_error_body_are_redacted() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("GET"))
        .and(path("/v2/relaties"))
        .respond_with(
            ResponseTemplate::new(400)
                .set_body_string(format!("bad {SUBSCRIPTION_KEY} / {CLIENT_KEY}")),
        )
        .mount(&server)
        .await;

    let err = client.relaties().list(&Query::new()).await.unwrap_err();
    let shown = format!("{err:?}");
    assert!(!shown.contains(SUBSCRIPTION_KEY) && !shown.contains(CLIENT_KEY));
}

#[tokio::test]
async fn odata_options_are_percent_encoded() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("GET"))
        .and(path("/v2/relaties"))
        .and(query_param(
            "$filter",
            "naam eq 'O''Brien' and nonactief eq false",
        ))
        .and(query_param("$orderby", "naam desc"))
        .and(query_param("$top", "25"))
        .and(query_param("$skip", "50"))
        .respond_with(json(200, serde_json::json!([])))
        .expect(1)
        .mount(&server)
        .await;

    let query = Query::new()
        .filter(
            Filter::eq("naam", "O'Brien")
                .unwrap()
                .and(Filter::eq("nonactief", false).unwrap()),
        )
        .order_by("naam", Direction::Desc)
        .unwrap()
        .top(Top::new(25).unwrap())
        .skip(50);
    assert!(client.relaties().list(&query).await.unwrap().is_empty());

    let requests = server.received_requests().await.unwrap();
    let raw = requests
        .iter()
        .find(|r| r.url.path() == "/v2/relaties")
        .unwrap()
        .url
        .query()
        .unwrap()
        .to_owned();
    assert!(
        raw.contains("%24filter=naam%20eq%20%27O%27%27Brien%27%20and%20nonactief%20eq%20false"),
        "{raw}"
    );
    assert!(!raw.contains('+'));
}

#[tokio::test]
async fn a_relatie_is_read_leniently() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("GET"))
        .and(path(format!("/v2/relaties/{RELATIE_ID}")))
        .respond_with(respond("relatie_read"))
        .mount(&server)
        .await;

    let relation: Relatie = client
        .relaties()
        .find(Id::parse(RELATIE_ID).unwrap())
        .await
        .unwrap();
    assert_eq!(relation.relatiesoort, [Relatiesoort::Klant]);
    assert_eq!(relation.naam.as_ref().unwrap().as_ref(), "Pietersen");
    assert_eq!(
        relation.modified_on.unwrap().to_string(),
        "2024-03-05T10:20:30.123"
    );
    assert_eq!(relation.id.unwrap().to_string(), RELATIE_ID);
    assert!(
        relation
            .verkoop_boekingen_uri
            .unwrap()
            .ends_with("/verkoopboekingen")
    );
}

#[tokio::test]
async fn creating_sends_the_write_model_without_empty_fields() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("POST"))
        .and(path("/v2/relaties"))
        .and(body_json(serde_json::json!({
            "naam": "Nieuw B.V.",
            "relatiesoort": ["Klant"]
        })))
        .respond_with(json(
            201,
            serde_json::json!({ "id": RELATIE_ID, "naam": "Nieuw B.V.", "relatiesoort": ["Klant"] }),
        ))
        .expect(1)
        .mount(&server)
        .await;

    let new_relation = RelatieWrite {
        naam: Some("Nieuw B.V.".parse().unwrap()),
        relatiesoort: vec![Relatiesoort::Klant],
        ..Default::default()
    };
    let created = client.relaties().create(&new_relation).await.unwrap();
    assert_eq!(created.id.unwrap().to_string(), RELATIE_ID);
}

#[tokio::test]
async fn updating_puts_to_the_item_path() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("PUT"))
        .and(path(format!("/v2/relaties/{RELATIE_ID}")))
        .and(body_json(serde_json::json!({ "memo": "bijgewerkt" })))
        .respond_with(json(
            200,
            serde_json::json!({ "id": RELATIE_ID, "memo": "bijgewerkt" }),
        ))
        .expect(1)
        .mount(&server)
        .await;

    let changes = RelatieWrite {
        memo: Some("bijgewerkt".into()),
        ..Default::default()
    };
    let updated = client
        .relaties()
        .update(Id::parse(RELATIE_ID).unwrap(), &changes)
        .await
        .unwrap();
    assert_eq!(updated.memo.as_deref(), Some("bijgewerkt"));
}

#[tokio::test]
async fn deleting_accepts_any_success_body() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("DELETE"))
        .and(path(format!("/v2/relaties/{RELATIE_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_string("\"verwijderd\""))
        .expect(1)
        .mount(&server)
        .await;

    client
        .relaties()
        .delete(Id::parse(RELATIE_ID).unwrap())
        .await
        .unwrap();
}

#[tokio::test]
async fn a_body_that_is_not_json_is_a_decode_error() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("GET"))
        .and(path("/v2/companyInfo"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<html>"))
        .mount(&server)
        .await;

    assert!(matches!(client.company_info().await, Err(Error::Decode(_))));
}

#[tokio::test]
async fn company_info_reads_enums_and_nested_objects() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("GET"))
        .and(path("/v2/companyInfo"))
        .respond_with(respond("company_info"))
        .mount(&server)
        .await;

    let info = client.company_info().await.unwrap();
    assert_eq!(
        info.btw_aangifte_periode_soort.unwrap().as_str(),
        "Kwartaal"
    );
    assert_eq!(info.huidig_boekjaar, Some(2024));
    let small_business = info.kleine_ondernemersregeling.unwrap();
    assert_eq!(
        small_business
            .maximaal_bedrag_kleine_ondernemers_regeling
            .unwrap()
            .to_string(),
        "2000"
    );
}

#[tokio::test]
async fn reports_send_the_period_as_query_parameters() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("GET"))
        .and(path("/v2/rapportages/periodebalans"))
        .and(query_param("start", "2024-01-01T00:00:00"))
        .and(query_param("end", "2024-03-31T00:00:00"))
        .respond_with(json(
            200,
            serde_json::json!([{ "grootboekNummer": 1000, "debet": 10.5 }]),
        ))
        .expect(1)
        .mount(&server)
        .await;

    let start = "2024-01-01".parse().unwrap();
    let end = "2024-03-31".parse().unwrap();
    let lines = client.get_periodebalans(start, end).await.unwrap();
    assert_eq!(lines[0].grootboek_nummer, Some(1000));
}
