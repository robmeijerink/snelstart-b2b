use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{Bytes, Id};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Document {
    #[serde(
        rename = "parentIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_identifier: Option<Uuid>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Document>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentContent {
    #[serde(rename = "content", default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Bytes>,
    #[serde(
        rename = "parentIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_identifier: Option<Uuid>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<DocumentContent>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentIdentifier {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Document>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}
