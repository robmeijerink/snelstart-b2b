use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{Bytes, Id};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UblContent {
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "content", default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Bytes>,
    #[serde(
        rename = "pdfContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pdf_content: Option<Bytes>,
    #[serde(
        rename = "forcePersist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_persist: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VerkoopBoekingBijlageReference {
    #[serde(
        rename = "verkoopBoekingId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verkoop_boeking_id: Option<Uuid>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<VerkoopBoekingBijlageReference>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}
