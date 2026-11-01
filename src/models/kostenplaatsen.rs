use serde::{Deserialize, Serialize};

use crate::types::Id;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Kostenplaats {
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<String>,
    #[serde(rename = "nonactief", default, skip_serializing_if = "Option::is_none")]
    pub nonactief: Option<bool>,
    #[serde(rename = "nummer", default, skip_serializing_if = "Option::is_none")]
    pub nummer: Option<i32>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Kostenplaats>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}
