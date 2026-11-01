use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::{DateTime, Id, Ref};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GrootboekMutatie {
    #[serde(rename = "grootboek", default, skip_serializing_if = "Option::is_none")]
    pub grootboek: Option<Ref<super::grootboeken::Grootboek>>,
    #[serde(
        rename = "kostenplaats",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kostenplaats: Option<Ref<super::kostenplaatsen::Kostenplaats>>,
    #[serde(rename = "datum", default, skip_serializing_if = "Option::is_none")]
    pub datum: Option<DateTime>,
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_on: Option<DateTime>,
    #[serde(rename = "dagboek", default, skip_serializing_if = "Option::is_none")]
    pub dagboek: Option<Ref<super::dagboeken::Dagboek>>,
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
    #[serde(
        rename = "saldo",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub saldo: Option<Decimal>,
    #[serde(
        rename = "documents",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub documents: Vec<super::documenten::Document>,
    #[serde(rename = "boekstuk", default, skip_serializing_if = "Option::is_none")]
    pub boekstuk: Option<String>,
    #[serde(
        rename = "factuurNummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuur_nummer: Option<String>,
    #[serde(
        rename = "relatiePublicIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relatie_public_identifier: Option<Ref<super::relaties::Relatie>>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<GrootboekMutatie>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}
