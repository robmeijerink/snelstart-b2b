use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AdministrationAccess {
    #[serde(rename = "hasAccess", default, skip_serializing_if = "Option::is_none")]
    pub has_access: Option<bool>,
}
