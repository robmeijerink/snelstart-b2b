use serde::{Deserialize, Serialize};

use crate::types::{BtwSoort, DateTime, Grootboekfunctie, Id, Rekeningcode};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Grootboek {
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_on: Option<DateTime>,
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<String>,
    #[serde(
        rename = "kostenplaatsVerplicht",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kostenplaats_verplicht: Option<bool>,
    #[serde(
        rename = "rekeningCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rekening_code: Option<Rekeningcode>,
    #[serde(rename = "nonactief", default, skip_serializing_if = "Option::is_none")]
    pub nonactief: Option<bool>,
    #[serde(rename = "nummer", default, skip_serializing_if = "Option::is_none")]
    pub nummer: Option<i32>,
    #[serde(
        rename = "grootboekfunctie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grootboekfunctie: Option<Grootboekfunctie>,
    #[serde(rename = "grootboekRubriek", default, skip_serializing)]
    pub grootboek_rubriek: Option<String>,
    #[serde(
        rename = "rgsCode",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing
    )]
    pub rgs_code: Vec<Rgs>,
    #[serde(
        rename = "btwSoort",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing
    )]
    pub btw_soort: Vec<BtwSoort>,
    #[serde(
        rename = "vatRateCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vat_rate_code: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Grootboek>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Rgs {
    #[serde(rename = "versie", default, skip_serializing_if = "Option::is_none")]
    pub versie: Option<String>,
    #[serde(rename = "rgsCode", default, skip_serializing_if = "Option::is_none")]
    pub rgs_code: Option<String>,
}
