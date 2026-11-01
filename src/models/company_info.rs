use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{
    AangiftePeriodeSoort, ArtikelcodeSoort, Bic, BtwNummer, DateTime, Email, Iban,
    MarkeergedragInlezenBankafschriften, Rechtsvorm, Ref, RegelkortingRekenmethode,
    TekstregelsNaarBackorder, VerkoopOrderVoorraadNiveau, VoorraadBijwerkmoment, VoorraadSysteem,
};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CompanyInfo {
    #[serde(
        rename = "administratieIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub administratie_identifier: Option<Uuid>,
    #[serde(
        rename = "administratieNaam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub administratie_naam: Option<String>,
    #[serde(
        rename = "bedrijfsnaam",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bedrijfsnaam: Option<String>,
    #[serde(
        rename = "contactpersoon",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contactpersoon: Option<String>,
    #[serde(rename = "adres", default, skip_serializing_if = "Option::is_none")]
    pub adres: Option<String>,
    #[serde(rename = "postcode", default, skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    #[serde(rename = "plaats", default, skip_serializing_if = "Option::is_none")]
    pub plaats: Option<String>,
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
    #[serde(
        rename = "bankrekeningnummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bankrekeningnummer: Option<String>,
    #[serde(rename = "iban", default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<Iban>,
    #[serde(rename = "bic", default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<Bic>,
    #[serde(
        rename = "rechtsvorm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rechtsvorm: Option<Rechtsvorm>,
    #[serde(rename = "btwNummer", default, skip_serializing_if = "Option::is_none")]
    pub btw_nummer: Option<BtwNummer>,
    #[serde(
        rename = "btwIdentificatieNummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_identificatie_nummer: Option<String>,
    #[serde(rename = "kvKNummer", default, skip_serializing_if = "Option::is_none")]
    pub kv_k_nummer: Option<String>,
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(rename = "website", default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(
        rename = "vrijeTekst1",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vrije_tekst1: Option<String>,
    #[serde(
        rename = "vrijeTekst2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vrije_tekst2: Option<String>,
    #[serde(
        rename = "vrijeTekst3",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vrije_tekst3: Option<String>,
    #[serde(
        rename = "vrijeTekst4",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vrije_tekst4: Option<String>,
    #[serde(
        rename = "huidigBoekjaar",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub huidig_boekjaar: Option<i32>,
    #[serde(
        rename = "beginmaandFiscaleBoekjaar",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub beginmaand_fiscale_boekjaar: Option<i32>,
    #[serde(
        rename = "btwAangiftePeriodeSoort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_aangifte_periode_soort: Option<AangiftePeriodeSoort>,
    #[serde(
        rename = "icpAangiftePeriodeSoort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub icp_aangifte_periode_soort: Option<AangiftePeriodeSoort>,
    #[serde(
        rename = "btwNummerFiscaleEenheid",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_nummer_fiscale_eenheid: Option<String>,
    #[serde(
        rename = "tussentijdseSuppletiesBerekenen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tussentijdse_suppleties_berekenen: Option<bool>,
    #[serde(
        rename = "mapUBLBestanden",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub map_ubl_bestanden: Option<String>,
    #[serde(
        rename = "btwPercentageAangifteKredietbeperking",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub btw_percentage_aangifte_kredietbeperking: Option<Decimal>,
    #[serde(
        rename = "markeergedragInlezenBankafschriften",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub markeergedrag_inlezen_bankafschriften: Option<MarkeergedragInlezenBankafschriften>,
    #[serde(
        rename = "voorkeurenTijdensBoeken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub voorkeuren_tijdens_boeken: Option<bool>,
    #[serde(
        rename = "aantalVoorloopnullenGrootboekrekeningen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aantal_voorloopnullen_grootboekrekeningen: Option<i32>,
    #[serde(
        rename = "kleineOndernemersregeling",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kleine_ondernemersregeling: Option<KleineOndernemersRegeling>,
    #[serde(
        rename = "volgendFactuurnummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volgend_factuurnummer: Option<i32>,
    #[serde(
        rename = "volgendVerkoopordernummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volgend_verkoopordernummer: Option<i32>,
    #[serde(
        rename = "volgendContantbonnummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volgend_contantbonnummer: Option<i32>,
    #[serde(
        rename = "volgendInkoopordernummer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volgend_inkoopordernummer: Option<i32>,
    #[serde(
        rename = "voorraadcontroleOrderinvoer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub voorraadcontrole_orderinvoer: Option<bool>,
    #[serde(
        rename = "abonnementOvernemen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub abonnement_overnemen: Option<bool>,
    #[serde(
        rename = "kolomGeleverdAutomatischVullen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kolom_geleverd_automatisch_vullen: Option<bool>,
    #[serde(
        rename = "voorraadkolommenTonenInInkoop",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub voorraadkolommen_tonen_in_inkoop: Option<bool>,
    #[serde(
        rename = "backorderGebruiken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backorder_gebruiken: Option<bool>,
    #[serde(
        rename = "deelleveringOrdersDefaultAan",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deellevering_orders_default_aan: Option<bool>,
    #[serde(
        rename = "factuurAlsBijlageVerkoopboeking",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factuur_als_bijlage_verkoopboeking: Option<bool>,
    #[serde(
        rename = "tekstregelsOvernemenNaarBackorder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tekstregels_overnemen_naar_backorder: Option<TekstregelsNaarBackorder>,
    #[serde(
        rename = "regelkortingVerkooporder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub regelkorting_verkooporder: Option<RegelkortingRekenmethode>,
    #[serde(
        rename = "drempelbedragVerkooporderbeheer",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drempelbedrag_verkooporderbeheer: Option<Decimal>,
    #[serde(
        rename = "drempelbedragVerkooporderbeheerMaxDagenUitstel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drempelbedrag_verkooporderbeheer_max_dagen_uitstel: Option<i32>,
    #[serde(
        rename = "verkoopprijsArtikelbestandExclusiefBtw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verkoopprijs_artikelbestand_exclusief_btw: Option<bool>,
    #[serde(
        rename = "inkoopprijsArtikelbestandExclusiefBtw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inkoopprijs_artikelbestand_exclusief_btw: Option<bool>,
    #[serde(
        rename = "artikelcodeSoort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artikelcode_soort: Option<ArtikelcodeSoort>,
    #[serde(
        rename = "artikelcodeMaxLengte",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artikelcode_max_lengte: Option<i32>,
    #[serde(
        rename = "begindatumVoorraadtelling",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub begindatum_voorraadtelling: Option<DateTime>,
    #[serde(
        rename = "voorraadTonenInZoekvenster",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub voorraad_tonen_in_zoekvenster: Option<bool>,
    #[serde(
        rename = "aantalDecimalenArtikelprijzen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aantal_decimalen_artikelprijzen: Option<i32>,
    #[serde(
        rename = "aantalDecimalenArtikelaantallen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aantal_decimalen_artikelaantallen: Option<i32>,
    #[serde(
        rename = "verkooporderVoorraadVanafNiveau",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verkooporder_voorraad_vanaf_niveau: Option<VerkoopOrderVoorraadNiveau>,
    #[serde(
        rename = "voorraadSysteem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub voorraad_systeem: Option<VoorraadSysteem>,
    #[serde(
        rename = "momentVoorraadBijwerken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub moment_voorraad_bijwerken: Option<VoorraadBijwerkmoment>,
    #[serde(
        rename = "rekeningTeOntvangenInkoopfacturen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rekening_te_ontvangen_inkoopfacturen: Option<Ref<super::grootboeken::Grootboek>>,
    #[serde(
        rename = "dagboekVoorraadverschillen",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dagboek_voorraadverschillen: Option<Ref<super::grootboeken::Grootboek>>,
    #[serde(
        rename = "buitenlandseBtwGrootboek",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub buitenlandse_btw_grootboek: Option<Ref<super::grootboeken::Grootboek>>,
    #[serde(
        rename = "factureerBuitenlandsBtw",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub factureer_buitenlands_btw: Option<bool>,
    #[serde(
        rename = "factureerBuitenlandsBtwRanges",
        default,
        deserialize_with = "crate::types::null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub factureer_buitenlands_btw_ranges: Vec<FactureerBuitenlandseBtwDateRange>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FactureerBuitenlandseBtwDateRange {
    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct KleineOndernemersRegeling {
    #[serde(
        rename = "isKleineOndernemersRegelingActief",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_kleine_ondernemers_regeling_actief: Option<bool>,
    #[serde(
        rename = "maximaalBedragVolledigeVerrekening",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maximaal_bedrag_volledige_verrekening: Option<Decimal>,
    #[serde(
        rename = "maximaalBedragKleineOndernemersRegeling",
        with = "rust_decimal::serde::float_option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub maximaal_bedrag_kleine_ondernemers_regeling: Option<Decimal>,
}
