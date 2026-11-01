use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::DateTime;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VatRate {
    #[serde(rename = "rateCode", default, skip_serializing_if = "Option::is_none")]
    pub rate_code: Option<String>,
    #[serde(
        rename = "rate",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rate: Option<Decimal>,
    #[serde(rename = "validFrom", default, skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VatRateDefinition {
    #[serde(rename = "rateCode", default, skip_serializing_if = "Option::is_none")]
    pub rate_code: Option<String>,
    #[serde(rename = "shortName", default, skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VatRates {
    #[serde(
        rename = "countryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub country_code: Option<String>,
    #[serde(
        rename = "vatRates",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vat_rates: Vec<VatRate>,
}
