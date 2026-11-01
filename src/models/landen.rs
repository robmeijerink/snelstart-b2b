use serde::{Deserialize, Serialize};

use crate::types::{Id, Landcode};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Land {
    #[serde(rename = "naam", default, skip_serializing_if = "Option::is_none")]
    pub naam: Option<String>,
    #[serde(
        rename = "landcodeISO",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub landcode_iso: Option<Landcode>,
    #[serde(rename = "landcode", default, skip_serializing_if = "Option::is_none")]
    pub landcode: Option<String>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Land>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}
