use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::types::{
    Aanmaningsoort, Bic, BoundedText, BtwNummer, DateTime, Email, Iban, Id, Incassosoort, Ref,
    Relatiesoort,
};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Adres {
    #[serde(
        rename = "contactpersoon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contactpersoon: Option<String>,
    #[serde(rename = "straat", default, skip_serializing_if = "Option::is_none")]
    pub straat: Option<String>,
    #[serde(rename = "postcode", default, skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    #[serde(rename = "plaats", default, skip_serializing_if = "Option::is_none")]
    pub plaats: Option<String>,
    #[serde(rename = "land", default, skip_serializing_if = "Option::is_none")]
    pub land: Option<Ref<super::landen::Land>>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DoorlopendeIncassoMachtiging {
    #[serde(rename = "kenmerk", default, skip_serializing_if = "Option::is_none")]
    pub kenmerk: Option<String>,
    #[serde(
        rename = "afsluitDatum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub afsluit_datum: Option<DateTime>,
    #[serde(
        rename = "omschrijving",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub omschrijving: Option<String>,
    #[serde(rename = "klant", default, skip_serializing_if = "Option::is_none")]
    pub klant: Option<Ref<Relatie>>,
    #[serde(
        rename = "intrekkingsDatum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub intrekkings_datum: Option<DateTime>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<DoorlopendeIncassoMachtiging>>,
    #[serde(rename = "uri", default, skip_serializing)]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EmailVersturen {
    #[serde(
        rename = "shouldSend",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub should_send: Option<bool>,
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(rename = "ccEmail", default, skip_serializing_if = "Option::is_none")]
    pub cc_email: Option<Email>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExtraVeldRelatie {
    #[serde(rename = "naam", default, skip_serializing_if = "Option::is_none")]
    pub naam: Option<String>,
    #[serde(rename = "waarde", default, skip_serializing_if = "Option::is_none")]
    pub waarde: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Relatie {
    #[serde(
        rename = "relatiesoort",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub relatiesoort: Vec<Relatiesoort>,
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_on: Option<DateTime>,
    #[serde(
        rename = "relatiecode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relatiecode: Option<i32>,
    #[serde(rename = "naam", default, skip_serializing_if = "Option::is_none")]
    pub naam: Option<BoundedText<50>>,
    #[serde(
        rename = "vestigingsAdres",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vestigings_adres: Option<Adres>,
    #[serde(
        rename = "correspondentieAdres",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub correspondentie_adres: Option<Adres>,
    #[serde(rename = "telefoon", default, skip_serializing_if = "Option::is_none")]
    pub telefoon: Option<String>,
    #[serde(
        rename = "mobieleTelefoon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mobiele_telefoon: Option<String>,
    #[serde(rename = "fax", default, skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(rename = "btwNummer", default, skip_serializing_if = "Option::is_none")]
    pub btw_nummer: Option<BtwNummer>,
    #[serde(
        rename = "factuurkorting",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuurkorting: Option<Decimal>,
    #[serde(
        rename = "krediettermijn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krediettermijn: Option<i32>,
    #[serde(rename = "bankieren", default, skip_serializing_if = "Option::is_none")]
    pub bankieren: Option<bool>,
    #[serde(rename = "nonactief", default, skip_serializing_if = "Option::is_none")]
    pub nonactief: Option<bool>,
    #[serde(
        rename = "kredietLimiet",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krediet_limiet: Option<Decimal>,
    #[serde(rename = "memo", default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(rename = "kvkNummer", default, skip_serializing_if = "Option::is_none")]
    pub kvk_nummer: Option<BoundedText<12>>,
    #[serde(rename = "oin", default, skip_serializing_if = "Option::is_none")]
    pub oin: Option<BoundedText<20>>,
    #[serde(
        rename = "websiteUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub website_url: Option<BoundedText<100>>,
    #[serde(
        rename = "aanmaningsoort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aanmaningsoort: Option<Aanmaningsoort>,
    #[serde(
        rename = "offerteEmailVersturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offerte_email_versturen: Option<EmailVersturen>,
    #[serde(
        rename = "bevestigingsEmailVersturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bevestigings_email_versturen: Option<EmailVersturen>,
    #[serde(
        rename = "factuurEmailVersturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuur_email_versturen: Option<EmailVersturen>,
    #[serde(
        rename = "aanmaningEmailVersturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aanmaning_email_versturen: Option<EmailVersturen>,
    #[serde(
        rename = "offerteAanvraagEmailVersturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offerte_aanvraag_email_versturen: Option<EmailVersturen>,
    #[serde(
        rename = "bestellingEmailVersturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bestelling_email_versturen: Option<EmailVersturen>,
    #[serde(
        rename = "ublBestandAlsBijlage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ubl_bestand_als_bijlage: Option<bool>,
    #[serde(rename = "iban", default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<Iban>,
    #[serde(rename = "bic", default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<Bic>,
    #[serde(
        rename = "incassoSoort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub incasso_soort: Option<Incassosoort>,
    #[serde(
        rename = "factuurRelatie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuur_relatie: Option<Ref<Relatie>>,
    #[serde(rename = "inkoopBoekingenUri", default, skip_serializing)]
    pub inkoop_boekingen_uri: Option<String>,
    #[serde(rename = "verkoopBoekingenUri", default, skip_serializing)]
    pub verkoop_boekingen_uri: Option<String>,
    #[serde(
        rename = "documents",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub documents: Vec<super::documenten::Document>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Relatie>>,
    #[serde(rename = "uri", default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(
        rename = "extraVeldenKlant",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extra_velden_klant: Vec<ExtraVeldRelatie>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RelatieWrite {
    #[serde(
        rename = "relatiesoort",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub relatiesoort: Vec<Relatiesoort>,
    #[serde(
        rename = "modifiedOn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub modified_on: Option<DateTime>,
    #[serde(
        rename = "relatiecode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub relatiecode: Option<i32>,
    #[serde(rename = "naam", default, skip_serializing_if = "Option::is_none")]
    pub naam: Option<BoundedText<50>>,
    #[serde(
        rename = "vestigingsAdres",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vestigings_adres: Option<Adres>,
    #[serde(
        rename = "correspondentieAdres",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub correspondentie_adres: Option<Adres>,
    #[serde(rename = "telefoon", default, skip_serializing_if = "Option::is_none")]
    pub telefoon: Option<String>,
    #[serde(
        rename = "mobieleTelefoon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mobiele_telefoon: Option<String>,
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(rename = "btwNummer", default, skip_serializing_if = "Option::is_none")]
    pub btw_nummer: Option<BtwNummer>,
    #[serde(
        rename = "factuurkorting",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuurkorting: Option<Decimal>,
    #[serde(
        rename = "krediettermijn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krediettermijn: Option<i32>,
    #[serde(rename = "bankieren", default, skip_serializing_if = "Option::is_none")]
    pub bankieren: Option<bool>,
    #[serde(rename = "nonactief", default, skip_serializing_if = "Option::is_none")]
    pub nonactief: Option<bool>,
    #[serde(
        rename = "kredietLimiet",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub krediet_limiet: Option<Decimal>,
    #[serde(rename = "memo", default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(rename = "kvkNummer", default, skip_serializing_if = "Option::is_none")]
    pub kvk_nummer: Option<BoundedText<12>>,
    #[serde(rename = "oin", default, skip_serializing_if = "Option::is_none")]
    pub oin: Option<BoundedText<20>>,
    #[serde(
        rename = "websiteUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub website_url: Option<BoundedText<100>>,
    #[serde(
        rename = "aanmaningsoort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aanmaningsoort: Option<Aanmaningsoort>,
    #[serde(
        rename = "offerteEmailVersturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offerte_email_versturen: Option<EmailVersturen>,
    #[serde(
        rename = "bevestigingsEmailVersturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bevestigings_email_versturen: Option<EmailVersturen>,
    #[serde(
        rename = "factuurEmailVersturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuur_email_versturen: Option<EmailVersturen>,
    #[serde(
        rename = "aanmaningEmailVersturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aanmaning_email_versturen: Option<EmailVersturen>,
    #[serde(
        rename = "offerteAanvraagEmailVersturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offerte_aanvraag_email_versturen: Option<EmailVersturen>,
    #[serde(
        rename = "bestellingEmailVersturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bestelling_email_versturen: Option<EmailVersturen>,
    #[serde(
        rename = "ublBestandAlsBijlage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ubl_bestand_als_bijlage: Option<bool>,
    #[serde(rename = "iban", default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<Iban>,
    #[serde(rename = "bic", default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<Bic>,
    #[serde(
        rename = "incassoSoort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub incasso_soort: Option<Incassosoort>,
    #[serde(
        rename = "factuurRelatie",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuur_relatie: Option<Ref<Relatie>>,
    #[serde(rename = "inkoopBoekingenUri", default, skip_serializing)]
    pub inkoop_boekingen_uri: Option<String>,
    #[serde(rename = "verkoopBoekingenUri", default, skip_serializing)]
    pub verkoop_boekingen_uri: Option<String>,
    #[serde(
        rename = "documents",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub documents: Vec<super::documenten::Document>,
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<Relatie>>,
    #[serde(rename = "uri", default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(
        rename = "extraVeldenKlant",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub extra_velden_klant: Vec<ExtraVeldRelatie>,
}

impl From<Relatie> for RelatieWrite {
    fn from(read: Relatie) -> Self {
        Self {
            relatiesoort: read.relatiesoort,
            modified_on: read.modified_on,
            relatiecode: read.relatiecode,
            naam: read.naam,
            vestigings_adres: read.vestigings_adres,
            correspondentie_adres: read.correspondentie_adres,
            telefoon: read.telefoon,
            mobiele_telefoon: read.mobiele_telefoon,
            email: read.email,
            btw_nummer: read.btw_nummer,
            factuurkorting: read.factuurkorting,
            krediettermijn: read.krediettermijn,
            bankieren: read.bankieren,
            nonactief: read.nonactief,
            krediet_limiet: read.krediet_limiet,
            memo: read.memo,
            kvk_nummer: read.kvk_nummer,
            oin: read.oin,
            website_url: read.website_url,
            aanmaningsoort: read.aanmaningsoort,
            offerte_email_versturen: read.offerte_email_versturen,
            bevestigings_email_versturen: read.bevestigings_email_versturen,
            factuur_email_versturen: read.factuur_email_versturen,
            aanmaning_email_versturen: read.aanmaning_email_versturen,
            offerte_aanvraag_email_versturen: read.offerte_aanvraag_email_versturen,
            bestelling_email_versturen: read.bestelling_email_versturen,
            ubl_bestand_als_bijlage: read.ubl_bestand_als_bijlage,
            iban: read.iban,
            bic: read.bic,
            incasso_soort: read.incasso_soort,
            factuur_relatie: read.factuur_relatie,
            inkoop_boekingen_uri: read.inkoop_boekingen_uri,
            verkoop_boekingen_uri: read.verkoop_boekingen_uri,
            documents: read.documents,
            id: read.id,
            uri: read.uri,
            extra_velden_klant: read.extra_velden_klant,
        }
    }
}
