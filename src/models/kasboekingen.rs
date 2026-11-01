use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::{BoundedText, BtwSoort, BtwTariefSoort, BtwType, DateTime, Id, Ref};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BtwBoekingregel {
    #[serde(
        rename = "debet",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub debet: Option<Decimal>,
    #[serde(
        rename = "credit",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credit: Option<Decimal>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<BtwType>,
    #[serde(rename = "tarief", default, skip_serializing_if = "Option::is_none")]
    pub tarief: Option<BtwTariefSoort>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GrootboekBoekingsRegel {
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<BoundedText<250>>,
    #[serde(rename = "grootboek", default, skip_serializing_if = "Option::is_none")]
    pub grootboek: Option<Ref<super::grootboeken::Grootboek>>,
    #[serde(
        rename = "kostenplaats",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kostenplaats: Option<Ref<super::kostenplaatsen::Kostenplaats>>,
    #[serde(
        rename = "debet",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub debet: Option<Decimal>,
    #[serde(
        rename = "credit",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credit: Option<Decimal>,
    #[serde(rename = "btwSoort", default, skip_serializing_if = "Option::is_none")]
    pub btw_soort: Option<BtwSoort>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InkoopBoekingVerantwoordingsRegel {
    #[serde(rename = "boekingId", default, skip_serializing_if = "Option::is_none")]
    pub boeking_id: Option<Ref<super::inkoopboekingen::Inkoopboeking>>,
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<String>,
    #[serde(
        rename = "debet",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub debet: Option<Decimal>,
    #[serde(
        rename = "credit",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credit: Option<Decimal>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Kasboeking {
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_on: Option<DateTime>,
    #[serde(rename = "datum", default, skip_serializing_if = "Option::is_none")]
    pub datum: Option<DateTime>,
    #[serde(rename = "markering", default, skip_serializing_if = "Option::is_none")]
    pub markering: Option<bool>,
    #[serde(rename = "boekstuk", default, skip_serializing_if = "Option::is_none")]
    pub boekstuk: Option<BoundedText<25>>,
    #[serde(
        rename = "gewijzigdDoorAccountant",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gewijzigd_door_accountant: Option<bool>,
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<BoundedText<250>>,
    #[serde(
        rename = "grootboekBoekingsRegels",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub grootboek_boekings_regels: Vec<GrootboekBoekingsRegel>,
    #[serde(
        rename = "inkoopboekingBoekingsRegels",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub inkoopboeking_boekings_regels: Vec<InkoopBoekingVerantwoordingsRegel>,
    #[serde(
        rename = "verkoopboekingBoekingsRegels",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub verkoopboeking_boekings_regels: Vec<VerkoopBoekingVerantwoordingsRegel>,
    #[serde(
        rename = "btwBoekingsregels",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub btw_boekingsregels: Vec<BtwBoekingregel>,
    #[serde(
        rename = "bedragUitgegeven",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bedrag_uitgegeven: Option<Decimal>,
    #[serde(
        rename = "bedragOntvangen",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bedrag_ontvangen: Option<Decimal>,
    #[serde(rename = "dagboek", default, skip_serializing_if = "Option::is_none")]
    pub dagboek: Option<Ref<super::dagboeken::Dagboek>>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Kasboeking>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VerkoopBoekingVerantwoordingsRegel {
    #[serde(rename = "boekingId", default, skip_serializing_if = "Option::is_none")]
    pub boeking_id: Option<Ref<super::verkoop_boekingen::VerkoopBoeking>>,
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<String>,
    #[serde(
        rename = "debet",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub debet: Option<Decimal>,
    #[serde(
        rename = "credit",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credit: Option<Decimal>,
}
