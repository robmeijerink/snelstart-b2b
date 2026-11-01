use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{
    BoundedText, BtwBoekingSoort, BtwSoort, Bytes, DateTime, Id, InstanceStatus, Ref,
};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "content", default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Bytes>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BtwBoeking {
    #[serde(rename = "btwSoort", default, skip_serializing_if = "Option::is_none")]
    pub btw_soort: Option<BtwBoekingSoort>,
    #[serde(
        rename = "btwBedrag",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_bedrag: Option<Decimal>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFromAttachment {
    #[serde(
        rename = "instanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFromAttachmentStatus {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceStatus>,
    #[serde(
        rename = "inkoopboekingId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inkoopboeking_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Inkoopboeking {
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_on: Option<DateTime>,
    #[serde(rename = "boekstuk", default, skip_serializing_if = "Option::is_none")]
    pub boekstuk: Option<BoundedText<25>>,
    #[serde(
        rename = "gewijzigdDoorAccountant",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gewijzigd_door_accountant: Option<bool>,
    #[serde(rename = "markering", default, skip_serializing_if = "Option::is_none")]
    pub markering: Option<bool>,
    #[serde(
        rename = "factuurdatum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuurdatum: Option<DateTime>,
    #[serde(
        rename = "factuurnummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuurnummer: Option<BoundedText<25>>,
    #[serde(
        rename = "leverancier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub leverancier: Option<Ref<super::relaties::Relatie>>,
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<BoundedText<250>>,
    #[serde(
        rename = "factuurbedrag",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuurbedrag: Option<Decimal>,
    #[serde(
        rename = "boekingsregels",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub boekingsregels: Vec<InkoopboekingRegel>,
    #[serde(
        rename = "btw",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub btw: Vec<BtwBoeking>,
    #[serde(
        rename = "documents",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub documents: Vec<super::documenten::Document>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Inkoopboeking>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InkoopboekingRegel {
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<BoundedText<250>>,
    #[serde(rename = "grootboek", default, skip_serializing_if = "Option::is_none")]
    pub grootboek: Option<Ref<super::grootboeken::Grootboek>>,
    #[serde(
        rename = "kostenplaats",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kostenplaats: Option<Ref<super::kostenplaatsen::Kostenplaats>>,
    #[serde(
        rename = "bedrag",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bedrag: Option<Decimal>,
    #[serde(rename = "btwSoort", default, skip_serializing_if = "Option::is_none")]
    pub btw_soort: Option<BtwSoort>,
}
