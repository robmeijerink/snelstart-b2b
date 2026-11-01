use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::{ActieprijsStatus, DateTime, PrijsIngave};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Actieprijzen {
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<String>,
    #[serde(
        rename = "startdatum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub startdatum: Option<DateTime>,
    #[serde(rename = "einddatum", default, skip_serializing_if = "Option::is_none")]
    pub einddatum: Option<DateTime>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ActieprijsStatus>,
    #[serde(
        rename = "artikelPrijzen",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artikel_prijzen: Vec<ArtikelPrijs>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ArtikelPrijs {
    #[serde(
        rename = "artikelcode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artikelcode: Option<String>,
    #[serde(
        rename = "artikelOmschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artikel_omschrijving: Option<String>,
    #[serde(
        rename = "prijsIngave",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prijs_ingave: Option<PrijsIngave>,
    #[serde(
        rename = "prijzen",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prijzen: Vec<Prijs>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Prijs {
    #[serde(
        rename = "vanaf",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vanaf: Option<Decimal>,
    #[serde(
        rename = "korting",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub korting: Option<Decimal>,
    #[serde(
        rename = "verkoopprijs",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verkoopprijs: Option<Decimal>,
    #[serde(
        rename = "basisprijs",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub basisprijs: Option<Decimal>,
}
