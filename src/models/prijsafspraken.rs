use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::{DateTime, PrijsBepalingSoort, Ref};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Prijsafspraak {
    #[serde(rename = "relatie", default, skip_serializing_if = "Option::is_none")]
    pub relatie: Option<Ref<super::relaties::Relatie>>,
    #[serde(rename = "artikel", default, skip_serializing_if = "Option::is_none")]
    pub artikel: Option<Ref<super::artikelen::Artikel>>,
    #[serde(rename = "datum", default, skip_serializing_if = "Option::is_none")]
    pub datum: Option<DateTime>,
    #[serde(
        rename = "aantal",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aantal: Option<Decimal>,
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
    #[serde(
        rename = "datumVanaf",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub datum_vanaf: Option<DateTime>,
    #[serde(
        rename = "datumTotEnMet",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub datum_tot_en_met: Option<DateTime>,
    #[serde(
        rename = "prijsBepalingSoort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prijs_bepaling_soort: Option<PrijsBepalingSoort>,
}
