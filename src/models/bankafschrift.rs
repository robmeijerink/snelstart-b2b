use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BankafschriftBestand {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "base64EncodedContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub base64_encoded_content: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BankafschriftBestandResponse {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "errors",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<BankafschriftBestandResponseError>,
    #[serde(rename = "isSuccess", default, skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BankafschriftBestandResponseError {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    #[serde(
        rename = "details",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<String>,
}
