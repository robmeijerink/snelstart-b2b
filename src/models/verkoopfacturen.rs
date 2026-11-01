use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::{BoundedText, DateTime, Id, Ref};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Verkoopfactuur {
    #[serde(
        rename = "verkoopBoeking",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verkoop_boeking: Option<Ref<super::verkoop_boekingen::VerkoopBoeking>>,
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_on: Option<DateTime>,
    #[serde(
        rename = "openstaandSaldo",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub openstaand_saldo: Option<Decimal>,
    #[serde(
        rename = "factuurnummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuurnummer: Option<BoundedText<25>>,
    #[serde(
        rename = "vervalDatum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verval_datum: Option<DateTime>,
    #[serde(rename = "relatie", default, skip_serializing_if = "Option::is_none")]
    pub relatie: Option<Ref<super::relaties::Relatie>>,
    #[serde(
        rename = "factuurDatum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuur_datum: Option<DateTime>,
    #[serde(
        rename = "factuurBedrag",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuur_bedrag: Option<Decimal>,
    #[serde(
        rename = "verkoopOrders",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub verkoop_orders: Vec<Ref<super::verkooporders::VerkoopOrder>>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Verkoopfactuur>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}
