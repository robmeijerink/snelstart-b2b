use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::{BtwAangiftePeriode, BtwAangifteStatus, BtwNummer, DateTime, Id};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Boekjaar {
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BtwAangifte {
    #[serde(rename = "boekjaar", default, skip_serializing_if = "Option::is_none")]
    pub boekjaar: Option<Boekjaar>,
    #[serde(
        rename = "betalenVoor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub betalen_voor: Option<DateTime>,
    #[serde(
        rename = "aangiftePeriodeBeginDatum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aangifte_periode_begin_datum: Option<DateTime>,
    #[serde(
        rename = "btwAangiftePeriode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_aangifte_periode: Option<BtwAangiftePeriode>,
    #[serde(
        rename = "datumTijdBerekening",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub datum_tijd_berekening: Option<DateTime>,
    #[serde(
        rename = "datumTijdVerzending",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub datum_tijd_verzending: Option<DateTime>,
    #[serde(
        rename = "isSuppletie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_suppletie: Option<bool>,
    #[serde(
        rename = "isAangifteGeschat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_aangifte_geschat: Option<bool>,
    #[serde(
        rename = "btwPercentageHoog",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_percentage_hoog: Option<Decimal>,
    #[serde(
        rename = "btwPercentageLaag",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_percentage_laag: Option<Decimal>,
    #[serde(
        rename = "btwPercentageOverig",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_percentage_overig: Option<Decimal>,
    #[serde(
        rename = "betalingskenmerk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub betalingskenmerk: Option<String>,
    #[serde(
        rename = "foutBericht",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fout_bericht: Option<String>,
    #[serde(
        rename = "btwAangifteStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_aangifte_status: Option<BtwAangifteStatus>,
    #[serde(rename = "btwNummer", default, skip_serializing_if = "Option::is_none")]
    pub btw_nummer: Option<BtwNummer>,
    #[serde(rename = "rubriek1A", default, skip_serializing_if = "Option::is_none")]
    pub rubriek1_a: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek1B", default, skip_serializing_if = "Option::is_none")]
    pub rubriek1_b: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek1C", default, skip_serializing_if = "Option::is_none")]
    pub rubriek1_c: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek1D", default, skip_serializing_if = "Option::is_none")]
    pub rubriek1_d: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek1E", default, skip_serializing_if = "Option::is_none")]
    pub rubriek1_e: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek2A", default, skip_serializing_if = "Option::is_none")]
    pub rubriek2_a: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek3A", default, skip_serializing_if = "Option::is_none")]
    pub rubriek3_a: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek3B", default, skip_serializing_if = "Option::is_none")]
    pub rubriek3_b: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek3C", default, skip_serializing_if = "Option::is_none")]
    pub rubriek3_c: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek4A", default, skip_serializing_if = "Option::is_none")]
    pub rubriek4_a: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek4B", default, skip_serializing_if = "Option::is_none")]
    pub rubriek4_b: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek5A", default, skip_serializing_if = "Option::is_none")]
    pub rubriek5_a: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek5B", default, skip_serializing_if = "Option::is_none")]
    pub rubriek5_b: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek5C", default, skip_serializing_if = "Option::is_none")]
    pub rubriek5_c: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek5D", default, skip_serializing_if = "Option::is_none")]
    pub rubriek5_d: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek5E", default, skip_serializing_if = "Option::is_none")]
    pub rubriek5_e: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek5F", default, skip_serializing_if = "Option::is_none")]
    pub rubriek5_f: Option<BtwAangifteRubriek>,
    #[serde(rename = "rubriek5G", default, skip_serializing_if = "Option::is_none")]
    pub rubriek5_g: Option<BtwAangifteRubriek>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<BtwAangifte>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BtwAangifteRubriek {
    #[serde(rename = "btw", default, skip_serializing_if = "Option::is_none")]
    pub btw: Option<i32>,
    #[serde(rename = "omzet", default, skip_serializing_if = "Option::is_none")]
    pub omzet: Option<i32>,
    #[serde(
        rename = "btwSchatting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_schatting: Option<i32>,
    #[serde(
        rename = "omzetSchatting",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omzet_schatting: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateBtwAangifteStatus {
    #[serde(
        rename = "isExternAangegeven",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_extern_aangegeven: Option<bool>,
}
