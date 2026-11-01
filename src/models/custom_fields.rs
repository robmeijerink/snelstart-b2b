use serde::{Deserialize, Serialize};

use crate::types::CustomFieldType;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomField {
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(
        rename = "definition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub definition: Option<CustomFieldDefinition>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldDefinition {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CustomFieldType>,
    #[serde(
        rename = "properties",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub properties: Vec<CustomFieldDefinitionProperty>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldDefinitionProperty {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RelatieCustomFields {
    #[serde(
        rename = "klantCustomFields",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub klant_custom_fields: Vec<CustomField>,
    #[serde(
        rename = "leverancierCustomFields",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub leverancier_custom_fields: Vec<CustomField>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RelatieUpdatedCustomFields {
    #[serde(
        rename = "klantCustomFields",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub klant_custom_fields: Vec<UpdatedCustomField>,
    #[serde(
        rename = "leverancierCustomFields",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub leverancier_custom_fields: Vec<UpdatedCustomField>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdatedCustomField {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
