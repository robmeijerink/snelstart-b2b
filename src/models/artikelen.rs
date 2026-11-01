use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{ActieprijsStatus, BoundedText, BtwSoort, DateTime, Id, PrijsIngave, Ref};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Artikel {
    #[serde(
        rename = "artikelcode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artikelcode: Option<String>,
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<BoundedText<250>>,
    #[serde(
        rename = "artikelOmzetgroep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artikel_omzetgroep: Option<Ref<ArtikelOmzetGroep>>,
    #[serde(
        rename = "verkoopprijs",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verkoopprijs: Option<Decimal>,
    #[serde(
        rename = "inkoopprijs",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inkoopprijs: Option<Decimal>,
    #[serde(rename = "eenheid", default, skip_serializing_if = "Option::is_none")]
    pub eenheid: Option<String>,
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_on: Option<DateTime>,
    #[serde(rename = "relatie", default, skip_serializing_if = "Option::is_none")]
    pub relatie: Option<Ref<super::relaties::Relatie>>,
    #[serde(
        rename = "isNonActief",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_non_actief: Option<bool>,
    #[serde(
        rename = "voorraadControle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub voorraad_controle: Option<bool>,
    #[serde(
        rename = "technischeVoorraad",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing
    )]
    pub technische_voorraad: Option<Decimal>,
    #[serde(
        rename = "vrijeVoorraad",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing
    )]
    pub vrije_voorraad: Option<Decimal>,
    #[serde(
        rename = "extraVelden",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extra_velden: Vec<ExtraVeldArtikel>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Artikel>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ArtikelOmzetGroep {
    #[serde(rename = "nummer", default, skip_serializing_if = "Option::is_none")]
    pub nummer: Option<i32>,
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<BoundedText<50>>,
    #[serde(
        rename = "verkoopGrootboekNederlandIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verkoop_grootboek_nederland_identifier: Option<Ref<super::grootboeken::Grootboek>>,
    #[serde(
        rename = "verkoopNederlandBtwSoort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verkoop_nederland_btw_soort: Option<BtwSoort>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<ArtikelOmzetGroep>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ArtikelPrijsAfspraken {
    #[serde(
        rename = "artikelCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artikel_code: Option<String>,
    #[serde(
        rename = "artikelOmschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artikel_omschrijving: Option<String>,
    #[serde(
        rename = "basisprijs",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub basisprijs: Option<Decimal>,
    #[serde(
        rename = "klantAfspraken",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub klant_afspraken: Vec<KlantAfspraken>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ArtikelQuery {
    #[serde(rename = "isHoofdartikel", default, skip_serializing)]
    pub is_hoofdartikel: Option<bool>,
    #[serde(
        rename = "subartikelen",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subartikelen: Vec<Subartikel>,
    #[serde(
        rename = "artikelcode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artikelcode: Option<String>,
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<BoundedText<250>>,
    #[serde(
        rename = "artikelOmzetgroep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artikel_omzetgroep: Option<Ref<ArtikelOmzetGroep>>,
    #[serde(
        rename = "verkoopprijs",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verkoopprijs: Option<Decimal>,
    #[serde(
        rename = "inkoopprijs",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inkoopprijs: Option<Decimal>,
    #[serde(rename = "eenheid", default, skip_serializing_if = "Option::is_none")]
    pub eenheid: Option<String>,
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_on: Option<DateTime>,
    #[serde(rename = "relatie", default, skip_serializing_if = "Option::is_none")]
    pub relatie: Option<Ref<super::relaties::Relatie>>,
    #[serde(
        rename = "isNonActief",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_non_actief: Option<bool>,
    #[serde(
        rename = "voorraadControle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub voorraad_controle: Option<bool>,
    #[serde(
        rename = "technischeVoorraad",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing
    )]
    pub technische_voorraad: Option<Decimal>,
    #[serde(
        rename = "vrijeVoorraad",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing
    )]
    pub vrije_voorraad: Option<Decimal>,
    #[serde(
        rename = "extraVelden",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extra_velden: Vec<ExtraVeldArtikel>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Artikel>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExtraVeldArtikel {
    #[serde(rename = "naam", default, skip_serializing_if = "Option::is_none")]
    pub naam: Option<String>,
    #[serde(rename = "waarde", default, skip_serializing_if = "Option::is_none")]
    pub waarde: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct KlantAfspraken {
    #[serde(
        rename = "relatiePublicIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relatie_public_identifier: Option<Uuid>,
    #[serde(
        rename = "relatieCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relatie_code: Option<i32>,
    #[serde(
        rename = "relatieNaam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relatie_naam: Option<String>,
    #[serde(
        rename = "startdatum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub startdatum: Option<DateTime>,
    #[serde(rename = "einddatum", default, skip_serializing_if = "Option::is_none")]
    pub einddatum: Option<DateTime>,
    #[serde(
        rename = "prijsIngave",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prijs_ingave: Option<PrijsIngave>,
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ActieprijsStatus>,
    #[serde(
        rename = "prijsAfspraken",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prijs_afspraken: Vec<PrijsAfspraak>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PrijsAfspraak {
    #[serde(
        rename = "vanaf",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vanaf: Option<Decimal>,
    #[serde(
        rename = "korting",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub korting: Option<Decimal>,
    #[serde(
        rename = "verkoopprijs",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verkoopprijs: Option<Decimal>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Subartikel {
    #[serde(
        rename = "artikelcode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artikelcode: Option<String>,
    #[serde(
        rename = "aantal",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aantal: Option<Decimal>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Subartikel>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}

impl From<ArtikelQuery> for Artikel {
    fn from(read: ArtikelQuery) -> Self {
        Self {
            artikelcode: read.artikelcode,
            omschrijving: read.omschrijving,
            artikel_omzetgroep: read.artikel_omzetgroep,
            verkoopprijs: read.verkoopprijs,
            inkoopprijs: read.inkoopprijs,
            eenheid: read.eenheid,
            modified_on: read.modified_on,
            relatie: read.relatie,
            is_non_actief: read.is_non_actief,
            voorraad_controle: read.voorraad_controle,
            technische_voorraad: read.technische_voorraad,
            vrije_voorraad: read.vrije_voorraad,
            extra_velden: read.extra_velden,
            id: read.id,
            uri: read.uri,
        }
    }
}
