use serde::{Deserialize, Serialize};

use crate::types::{Id, IngaveSoortVeld};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IngaveGegevenSjabloon {
    #[serde(
        rename = "veldNummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub veld_nummer: Option<i32>,
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<String>,
    #[serde(
        rename = "ingaveMasker",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ingave_masker: Option<String>,
    #[serde(rename = "soortVeld", default, skip_serializing_if = "Option::is_none")]
    pub soort_veld: Option<IngaveSoortVeld>,
    #[serde(
        rename = "maximumTekstLengte",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_tekst_lengte: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VerkoopOrderSjabloon {
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<String>,
    #[serde(rename = "nonactief", default, skip_serializing_if = "Option::is_none")]
    pub nonactief: Option<bool>,
    #[serde(
        rename = "prijsIngaveExclusiefBtw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prijs_ingave_exclusief_btw: Option<bool>,
    #[serde(
        rename = "nieuweOrdersBlokkeren",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub nieuwe_orders_blokkeren: Option<bool>,
    #[serde(
        rename = "extraHoofdVelden",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extra_hoofd_velden: Vec<IngaveGegevenSjabloon>,
    #[serde(
        rename = "extraRegelVelden",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extra_regel_velden: Vec<IngaveGegevenSjabloon>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<VerkoopOrderSjabloon>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}
