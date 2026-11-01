use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::{BtwSoort, DateTime};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BtwTarief {
    #[serde(rename = "btwSoort", default, skip_serializing_if = "Option::is_none")]
    pub btw_soort: Option<BtwSoort>,
    #[serde(
        rename = "btwPercentage",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_percentage: Option<Decimal>,
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
}
