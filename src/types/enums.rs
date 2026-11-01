use serde::{Deserialize, Serialize};

macro_rules! open_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $($variant,)+
            Other(String),
        }

        impl $name {
            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => stringify!($variant),)+
                    Self::Other(other) => other,
                }
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                match value {
                    $(stringify!($variant) => Self::$variant,)+
                    other => Self::Other(other.to_owned()),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                String::deserialize(d).map(|s| Self::from(s.as_str()))
            }
        }
    };
}

open_enum!(AangiftePeriodeSoort {
    Maand,
    Kwartaal,
    Jaar,
});

open_enum!(Aanmaningsoort {
    Nee,
    Onderneming,
    Consument,
});

open_enum!(ActieprijsStatus {
    Verlopen,
    Gepland,
    Actief,
});

open_enum!(ArtikelcodeSoort {
    Numeriek,
    Alfanumeriek,
});

open_enum!(BtwAangiftePeriode {
    Onbekend,
    Maand,
    Kwartaal,
    Jaar,
});

open_enum!(BtwAangifteStatus {
    NietVerzonden,
    Verzonden,
    Geaccepteerd,
    Fout,
    GeaccepteerdEnHerberekend,
    GeaccepteerdExtern,
});

open_enum!(BtwBoekingSoort {
    Geen,
    VerkopenLaag,
    VerkopenHoog,
    VerkopenOverig,
    InkopenLaag,
    InkopenHoog,
    InkopenOverig,
    InkopenVerlegd,
});

open_enum!(BtwSoort {
    Geen,
    Laag,
    Hoog,
    Overig,
});

open_enum!(BtwTariefSoort { Hoog, Laag, Overig });

open_enum!(BtwType {
    TeVorderenBtwType,
    AfTeDragenBtwType,
});

open_enum!(CustomFieldType {
    Text,
    Integer,
    Money,
    Float,
    DateTime,
    Boolean,
    Enum,
    ByteArray,
});

open_enum!(DagboekSoort {
    Geen,
    Kas,
    Bank,
    Verkoop,
    Inkoop,
    Memoriaal,
    Balans,
});

open_enum!(Grootboekfunctie {
    Diversen,
    DagboekKas,
    DagboekBank,
    DagboekGiro,
    DagboekVerkoop,
    DagboekInkoop,
    DagboekMemoriaal,
    DagboekBalans,
    VerkopenOmzetOnbelastVerlegd,
    VerkopenOmzetLaag,
    VerkopenOmzetHoog,
    VerkopenOmzetOverig,
    VerkopenBtwVrij,
    BtwAfteDragenLaag,
    BtwAfTeDragenHoog,
    BtwAfTeDragenOverig,
    BtwAfTeDragenVerlegdVerkopen,
    BtwTeVorderenVerlegd,
    BtwAfteDragenLaagGlobalisatie,
    BtwAfTeDragenHoogGlobalisatie,
    BtwAfTeDragenOverigGlobalisatie,
    InkopenKostenAlleBtwTarieven,
    InkopenKostenLaag,
    InkopenKostenHoog,
    InkopenKostenOverig,
    InkopenVraagPosten,
    BtwTeVorderenLaag,
    BtwTeVorderenHoog,
    BtwTeVorderenOverig,
    BtwAfTeDragenVerlegdInkopen,
    BtwTeVorderenVerlegdInkopen,
    InkopenImportBinnenEUlaag,
    InkopenImportBinnenEUhoog,
    InkopenImportBinnenEUoverig,
    InkopenImportBuitenEUlaag,
    InkopenImportBuitenEUhoog,
    InkopenImportBuitenEUoverig,
    VerkopenExportBinnenEU,
    VerkopenExportBuitenEU,
    OninbareVorderingen,
    HerrekeningCorrecties,
    InstallatieTeleverkoopBinnenEUgeen,
    BpmVerkopen,
    BpmInkopen,
    BpmVoorraad,
    DienstverleningBinnenEU,
    VerkoopGlobalisatieLaag,
    VerkoopGlobalisatieHoog,
    VerkoopGlobalisatieOverig,
    InkoopGlobalisatieLaag,
    InkoopGlobalisatieHoog,
    InkoopGlobalisatieOverig,
    BtwGlobalisatie,
    KredietbeperkingVerkopen,
    KredietbeperkingInkopen,
    Betalingskorting,
    TussenrekeningBetalingen,
    OnbekendeBetalingen,
    Kruisposten,
    ContanteBetaling,
    ElectronischeBetaling,
    Winstboeking,
});

open_enum!(Incassosoort { Geen, Core, B2B });

open_enum!(IngaveSoortVeld {
    GeheelGetal,
    Bedrag,
    Datum,
    Tekst,
    Klant,
    Leverancier,
    Artikel,
    Artikelgroep,
    Land,
    Factuur,
    Grootboekrekening,
});

open_enum!(InstanceStatus {
    Running,
    Completed,
    ContinuedAsNew,
    Failed,
    Canceled,
    Terminated,
    Pending,
    NotFound,
    Unknown,
});

open_enum!(MarkeergedragInlezenBankafschriften {
    AlleenBelangrijkste,
    Altijd,
});

open_enum!(PrijsBepalingSoort {
    NormaleVerkoopprijs,
    ActieprijzenPerArtikel,
    ActieprijzenPerArtikelkortingsgroep,
    AfspraakPerArtikelklant,
    AfspraakPerArtikelPerKlantkortingsgroep,
    AfspraakPerKlantPerArtikelkortingsgroep,
    AfspraakPerArtikelkortingsgroepPerKlantkortingsgroep,
});

open_enum!(PrijsIngave {
    Bedrag,
    StaffelBedrag,
    Korting,
    StaffelKorting,
});

open_enum!(ProcesStatus {
    Order,
    Offerte,
    Bevestiging,
    Werkbon,
    Pakbon,
    Afhaalbon,
    Contantbon,
    Factuur,
});

open_enum!(Rechtsvorm {
    NotSet,
    Eenmanszaak,
    Vof,
    Bv,
    Nv,
    Overige,
});

open_enum!(RegelkortingRekenmethode {
    BerekenenOverRegelBedrag,
    BerekenenOverStukprijs,
});

open_enum!(Rekeningcode {
    Balans,
    WinstEnVerlies,
});

open_enum!(Relatiesoort {
    Klant,
    Leverancier,
    Eigen,
});

open_enum!(TekstregelsNaarBackorder {
    Geen,
    BovenArtikel,
    OnderArtikel,
    Alle,
});

open_enum!(VerkoopBoekingBtwSoort {
    Geen,
    VerkopenLaag,
    VerkopenHoog,
    VerkopenOverig,
    VerkopenVerlegd,
    InkopenLaag,
    InkopenHoog,
    InkopenOverig,
});

open_enum!(VerkoopOrderVoorraadNiveau {
    Alle,
    Offerte,
    BackOrder,
    Bevestiging,
    Werkbon,
    PakbonEnAfhaalbon,
    FactuurEnContantbon,
});

open_enum!(VerkooporderBtwIngave {
    Inclusief,
    Exclusief,
});

open_enum!(VerkooporderStatus {
    InBehandeling,
    Uitgevoerd,
    Service,
});

open_enum!(VoorraadBijwerkmoment {
    BijBoekenOntvangst,
    BijBoekenInkoopfactuur,
});

open_enum!(VoorraadSysteem { Fifo, Lifo });

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_values_roundtrip() {
        let json = serde_json::to_string(&Relatiesoort::Klant).unwrap();
        assert_eq!(json, "\"Klant\"");
        assert_eq!(
            serde_json::from_str::<Relatiesoort>(&json).unwrap(),
            Relatiesoort::Klant
        );
    }

    #[test]
    fn unknown_values_survive() {
        let v: BtwSoort = serde_json::from_str("\"Nieuw\"").unwrap();
        assert_eq!(v, BtwSoort::Other("Nieuw".into()));
        assert_eq!(serde_json::to_string(&v).unwrap(), "\"Nieuw\"");
    }

    #[test]
    fn matching_is_case_sensitive() {
        assert!(matches!(
            Relatiesoort::from("klant"),
            Relatiesoort::Other(_)
        ));
    }

    #[test]
    fn real_value_named_onbekend_is_not_confused_with_the_fallback() {
        assert_eq!(
            BtwAangiftePeriode::from("Onbekend"),
            BtwAangiftePeriode::Onbekend
        );
        assert_eq!(BtwAangiftePeriode::Onbekend.as_str(), "Onbekend");
    }

    #[test]
    fn display_uses_the_wire_value() {
        assert_eq!(Incassosoort::B2B.to_string(), "B2B");
    }
}
