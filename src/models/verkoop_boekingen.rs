use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::{BoundedText, BtwSoort, DateTime, Id, Ref, VerkoopBoekingBtwSoort};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VerkoopBoeking {
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_on: Option<DateTime>,
    #[serde(rename = "boekstuk", default, skip_serializing_if = "Option::is_none")]
    pub boekstuk: Option<BoundedText<25>>,
    #[serde(
        rename = "gewijzigdDoorAccountant",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gewijzigd_door_accountant: Option<bool>,
    #[serde(rename = "markering", default, skip_serializing_if = "Option::is_none")]
    pub markering: Option<bool>,
    #[serde(
        rename = "factuurdatum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuurdatum: Option<DateTime>,
    #[serde(
        rename = "factuurnummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuurnummer: Option<BoundedText<25>>,
    #[serde(rename = "klant", default, skip_serializing_if = "Option::is_none")]
    pub klant: Option<Ref<super::relaties::Relatie>>,
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<BoundedText<250>>,
    #[serde(
        rename = "factuurbedrag",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuurbedrag: Option<Decimal>,
    #[serde(
        rename = "betalingstermijn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub betalingstermijn: Option<i32>,
    #[serde(
        rename = "eenmaligeIncassoMachtiging",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub eenmalige_incasso_machtiging: Option<VerkoopBoekingEenmaligeIncassoMachtiging>,
    #[serde(
        rename = "doorlopendeIncassoMachtiging",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub doorlopende_incasso_machtiging: Option<Ref<IncassoMachtiging>>,
    #[serde(
        rename = "boekingsregels",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub boekingsregels: Vec<VerkoopBoekingRegel>,
    #[serde(
        rename = "btw",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub btw: Vec<VerkoopBoekingBtwRegel>,
    #[serde(
        rename = "documents",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub documents: Vec<super::documenten::Document>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<VerkoopBoeking>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VerkoopBoekingBtwRegel {
    #[serde(rename = "btwSoort", default, skip_serializing_if = "Option::is_none")]
    pub btw_soort: Option<VerkoopBoekingBtwSoort>,
    #[serde(
        rename = "btwBedrag",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_bedrag: Option<Decimal>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VerkoopBoekingEenmaligeIncassoMachtiging {
    #[serde(rename = "kenmerk", default, skip_serializing_if = "Option::is_none")]
    pub kenmerk: Option<BoundedText<50>>,
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<BoundedText<250>>,
    #[serde(rename = "datum", default, skip_serializing_if = "Option::is_none")]
    pub datum: Option<DateTime>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VerkoopBoekingRegel {
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
        rename = "bedrag",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bedrag: Option<Decimal>,
    #[serde(rename = "btwSoort", default, skip_serializing_if = "Option::is_none")]
    pub btw_soort: Option<BtwSoort>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IncassoMachtiging;
