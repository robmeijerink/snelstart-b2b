mod common;

use common::fixture_json;
use pretty_assertions::assert_eq;
use snelstart_b2b::models::company_info::CompanyInfo;
use snelstart_b2b::models::relaties::{Relatie, RelatieWrite};
use snelstart_b2b::types::{Iban, Incassosoort, Relatiesoort};

#[test]
fn a_full_relatie_survives_a_round_trip_unchanged() {
    let json = fixture_json("relatie");
    let relation: RelatieWrite = serde_json::from_value(json.clone()).unwrap();
    assert_eq!(serde_json::to_value(&relation).unwrap(), json);
}

#[test]
fn typed_fields_are_filled_from_the_json() {
    let relation: RelatieWrite = serde_json::from_value(fixture_json("relatie")).unwrap();
    assert_eq!(
        relation.relatiesoort,
        [Relatiesoort::Klant, Relatiesoort::Leverancier]
    );
    assert_eq!(relation.incasso_soort, Some(Incassosoort::Core));
    assert_eq!(
        relation.iban,
        Some(Iban::new("NL91ABNA0417164300").unwrap())
    );
    assert!(relation.btw_nummer.unwrap().is_valid());
    let factuur_relatie = relation.factuur_relatie.unwrap();
    assert_eq!(
        factuur_relatie.id.to_string(),
        "0f8fad5b-d9cb-469f-a165-70867728950e"
    );
}

#[test]
fn read_only_fields_are_never_sent_back() {
    let read: snelstart_b2b::models::relaties::Relatie =
        serde_json::from_value(fixture_json("relatie_read")).unwrap();
    let sent = serde_json::to_value(&read).unwrap();
    assert!(sent.get("inkoopBoekingenUri").is_none());
    assert!(sent.get("verkoopBoekingenUri").is_none());
    assert!(sent.get("fax").is_some());
}

#[test]
fn unknown_enum_values_and_fields_do_not_break_reading() {
    let info: CompanyInfo = serde_json::from_value(serde_json::json!({
        "rechtsvorm": "Stichting",
        "nieuwVeld": true
    }))
    .unwrap();
    assert_eq!(info.rechtsvorm.unwrap().as_str(), "Stichting");
}

#[test]
fn an_existing_but_invalid_value_is_read_and_flagged() {
    let relation: RelatieWrite = serde_json::from_value(serde_json::json!({
        "iban": "NL00ABNA0417164300",
        "email": "geen-adres"
    }))
    .unwrap();
    assert!(!relation.iban.unwrap().is_valid());
    assert!(!relation.email.unwrap().is_valid());
}

#[test]
fn null_lists_are_read_as_empty() {
    let relation: RelatieWrite = serde_json::from_value(serde_json::json!({
        "relatiesoort": null,
        "extraVeldenKlant": null,
        "documents": null
    }))
    .unwrap();
    assert!(relation.relatiesoort.is_empty());
    assert!(relation.extra_velden_klant.is_empty());
}

#[test]
fn a_read_relatie_converts_to_the_write_model_for_updating() {
    let read: Relatie = serde_json::from_value(fixture_json("relatie_read")).unwrap();
    let mut write = RelatieWrite::from(read);
    write.memo = Some("bijgewerkt".into());

    let sent = serde_json::to_value(&write).unwrap();
    assert_eq!(sent["naam"], "Pietersen");
    assert_eq!(sent["memo"], "bijgewerkt");
    assert_eq!(sent["id"], "3f2504e0-4f89-41d3-9a0c-0305e82c3301");
    assert!(sent.get("fax").is_none());
    assert!(sent.get("inkoopBoekingenUri").is_none());
}
