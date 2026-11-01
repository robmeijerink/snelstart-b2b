use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::Ref;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct KolommenbalansRegel {
    #[serde(
        rename = "grootboekIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grootboek_identifier: Option<Ref<super::grootboeken::Grootboek>>,
    #[serde(
        rename = "grootboekOmschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grootboek_omschrijving: Option<String>,
    #[serde(
        rename = "grootboekNummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grootboek_nummer: Option<i32>,
    #[serde(
        rename = "verliesEnWinstDebet",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verlies_en_winst_debet: Option<Decimal>,
    #[serde(
        rename = "verliesEnWinstCredit",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verlies_en_winst_credit: Option<Decimal>,
    #[serde(
        rename = "balansDebet",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub balans_debet: Option<Decimal>,
    #[serde(
        rename = "balansCredit",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub balans_credit: Option<Decimal>,
    #[serde(
        rename = "rgsCode",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rgs_code: Vec<super::grootboeken::Rgs>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PeriodebalansRegel {
    #[serde(
        rename = "grootboekIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grootboek_identifier: Option<Ref<super::grootboeken::Grootboek>>,
    #[serde(
        rename = "grootboekOmschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grootboek_omschrijving: Option<String>,
    #[serde(
        rename = "grootboekNummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grootboek_nummer: Option<i32>,
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
        rename = "startSaldoPeriode",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_saldo_periode: Option<Decimal>,
    #[serde(
        rename = "eindSaldoPeriode",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub eind_saldo_periode: Option<Decimal>,
    #[serde(
        rename = "startSaldoBoekjaar",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_saldo_boekjaar: Option<Decimal>,
    #[serde(
        rename = "rgsCode",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rgs_code: Vec<super::grootboeken::Rgs>,
}
