use crate::client::Client;
use crate::models::*;
use crate::resource::*;

pub struct Artikelen;

impl Endpoint for Artikelen {
    type Model = artikelen::ArtikelQuery;
    type Entity = artikelen::Artikel;
    const PATH: &'static str = "artikelen";
}

impl Listable for Artikelen {}

impl Paged for Artikelen {}

impl Findable for Artikelen {}

impl Creatable for Artikelen {
    type Input = artikelen::Artikel;
    type Output = artikelen::Artikel;
}

impl Updatable for Artikelen {
    type Input = artikelen::Artikel;
    type Output = artikelen::Artikel;
}

impl Deletable for Artikelen {}

pub struct Artikelomzetgroepen;

impl Endpoint for Artikelomzetgroepen {
    type Model = artikelen::ArtikelOmzetGroep;
    type Entity = artikelen::ArtikelOmzetGroep;
    const PATH: &'static str = "artikelomzetgroepen";
}

impl Listable for Artikelomzetgroepen {}

impl Findable for Artikelomzetgroepen {}

pub struct Bankboekingen;

impl Endpoint for Bankboekingen {
    type Model = bankboekingen::Bankboeking;
    type Entity = bankboekingen::Bankboeking;
    const PATH: &'static str = "bankboekingen";
}

impl Listable for Bankboekingen {}

impl Paged for Bankboekingen {}

impl Findable for Bankboekingen {}

impl Creatable for Bankboekingen {
    type Input = bankboekingen::Bankboeking;
    type Output = bankboekingen::Bankboeking;
}

impl Updatable for Bankboekingen {
    type Input = bankboekingen::Bankboeking;
    type Output = bankboekingen::Bankboeking;
}

impl Deletable for Bankboekingen {}

pub struct Dagboeken;

impl Endpoint for Dagboeken {
    type Model = dagboeken::Dagboek;
    type Entity = dagboeken::Dagboek;
    const PATH: &'static str = "dagboeken";
}

impl Listable for Dagboeken {}

pub struct Documenten;

impl Endpoint for Documenten {
    type Model = documenten::DocumentContent;
    type Entity = documenten::DocumentContent;
    const PATH: &'static str = "documenten";
}

impl Findable for Documenten {}

impl Updatable for Documenten {
    type Input = documenten::DocumentContent;
    type Output = documenten::DocumentIdentifier;
}

impl Deletable for Documenten {}

pub struct Grootboeken;

impl Endpoint for Grootboeken {
    type Model = grootboeken::Grootboek;
    type Entity = grootboeken::Grootboek;
    const PATH: &'static str = "grootboeken";
}

impl Listable for Grootboeken {}

impl Paged for Grootboeken {}

impl Findable for Grootboeken {}

impl Creatable for Grootboeken {
    type Input = grootboeken::Grootboek;
    type Output = grootboeken::Grootboek;
}

pub struct Grootboekmutaties;

impl Endpoint for Grootboekmutaties {
    type Model = grootboek_mutaties::GrootboekMutatie;
    type Entity = grootboek_mutaties::GrootboekMutatie;
    const PATH: &'static str = "grootboekmutaties";
}

impl Listable for Grootboekmutaties {}

impl Paged for Grootboekmutaties {}

impl Findable for Grootboekmutaties {}

pub struct Inkoopboekingen;

impl Endpoint for Inkoopboekingen {
    type Model = inkoopboekingen::Inkoopboeking;
    type Entity = inkoopboekingen::Inkoopboeking;
    const PATH: &'static str = "inkoopboekingen";
}

impl Findable for Inkoopboekingen {}

impl Creatable for Inkoopboekingen {
    type Input = inkoopboekingen::Inkoopboeking;
    type Output = inkoopboekingen::Inkoopboeking;
}

impl Updatable for Inkoopboekingen {
    type Input = inkoopboekingen::Inkoopboeking;
    type Output = inkoopboekingen::Inkoopboeking;
}

impl Deletable for Inkoopboekingen {}

pub struct Inkoopfacturen;

impl Endpoint for Inkoopfacturen {
    type Model = inkoopfacturen::Inkoopfactuur;
    type Entity = inkoopfacturen::Inkoopfactuur;
    const PATH: &'static str = "inkoopfacturen";
}

impl Listable for Inkoopfacturen {}

impl Paged for Inkoopfacturen {}

pub struct Kasboekingen;

impl Endpoint for Kasboekingen {
    type Model = kasboekingen::Kasboeking;
    type Entity = kasboekingen::Kasboeking;
    const PATH: &'static str = "kasboekingen";
}

impl Listable for Kasboekingen {}

impl Paged for Kasboekingen {}

impl Findable for Kasboekingen {}

impl Creatable for Kasboekingen {
    type Input = kasboekingen::Kasboeking;
    type Output = kasboekingen::Kasboeking;
}

impl Updatable for Kasboekingen {
    type Input = kasboekingen::Kasboeking;
    type Output = kasboekingen::Kasboeking;
}

impl Deletable for Kasboekingen {}

pub struct Kostenplaatsen;

impl Endpoint for Kostenplaatsen {
    type Model = kostenplaatsen::Kostenplaats;
    type Entity = kostenplaatsen::Kostenplaats;
    const PATH: &'static str = "kostenplaatsen";
}

impl Listable for Kostenplaatsen {}

impl Findable for Kostenplaatsen {}

impl Creatable for Kostenplaatsen {
    type Input = kostenplaatsen::Kostenplaats;
    type Output = kostenplaatsen::Kostenplaats;
}

impl Updatable for Kostenplaatsen {
    type Input = kostenplaatsen::Kostenplaats;
    type Output = kostenplaatsen::Kostenplaats;
}

impl Deletable for Kostenplaatsen {}

pub struct Landen;

impl Endpoint for Landen {
    type Model = landen::Land;
    type Entity = landen::Land;
    const PATH: &'static str = "landen";
}

impl Listable for Landen {}

impl Findable for Landen {}

pub struct Memoriaalboekingen;

impl Endpoint for Memoriaalboekingen {
    type Model = bankboekingen::Memoriaalboeking;
    type Entity = bankboekingen::Memoriaalboeking;
    const PATH: &'static str = "memoriaalboekingen";
}

impl Findable for Memoriaalboekingen {}

impl Creatable for Memoriaalboekingen {
    type Input = bankboekingen::Memoriaalboeking;
    type Output = bankboekingen::Memoriaalboeking;
}

impl Updatable for Memoriaalboekingen {
    type Input = bankboekingen::Memoriaalboeking;
    type Output = bankboekingen::Memoriaalboeking;
}

impl Deletable for Memoriaalboekingen {}

pub struct Prijsafspraken;

impl Endpoint for Prijsafspraken {
    type Model = prijsafspraken::Prijsafspraak;
    type Entity = prijsafspraken::Prijsafspraak;
    const PATH: &'static str = "prijsafspraken";
}

impl Listable for Prijsafspraken {}

impl Paged for Prijsafspraken {}

pub struct Relaties;

impl Endpoint for Relaties {
    type Model = relaties::Relatie;
    type Entity = relaties::Relatie;
    const PATH: &'static str = "relaties";
}

impl Listable for Relaties {}

impl Paged for Relaties {}

impl Findable for Relaties {}

impl Creatable for Relaties {
    type Input = relaties::RelatieWrite;
    type Output = relaties::RelatieWrite;
}

impl Updatable for Relaties {
    type Input = relaties::RelatieWrite;
    type Output = relaties::RelatieWrite;
}

impl Deletable for Relaties {}

pub struct Verkoopboekingen;

impl Endpoint for Verkoopboekingen {
    type Model = verkoop_boekingen::VerkoopBoeking;
    type Entity = verkoop_boekingen::VerkoopBoeking;
    const PATH: &'static str = "verkoopboekingen";
}

impl Findable for Verkoopboekingen {}

impl Creatable for Verkoopboekingen {
    type Input = verkoop_boekingen::VerkoopBoeking;
    type Output = verkoop_boekingen::VerkoopBoeking;
}

impl Updatable for Verkoopboekingen {
    type Input = verkoop_boekingen::VerkoopBoeking;
    type Output = verkoop_boekingen::VerkoopBoeking;
}

impl Deletable for Verkoopboekingen {}

pub struct Verkoopfacturen;

impl Endpoint for Verkoopfacturen {
    type Model = verkoopfacturen::Verkoopfactuur;
    type Entity = verkoopfacturen::Verkoopfactuur;
    const PATH: &'static str = "verkoopfacturen";
}

impl Listable for Verkoopfacturen {}

impl Paged for Verkoopfacturen {}

impl Findable for Verkoopfacturen {}

pub struct Verkooporders;

impl Endpoint for Verkooporders {
    type Model = verkooporders::VerkoopOrder;
    type Entity = verkooporders::VerkoopOrder;
    const PATH: &'static str = "verkooporders";
}

impl Listable for Verkooporders {}

impl Paged for Verkooporders {}

impl Findable for Verkooporders {}

impl Creatable for Verkooporders {
    type Input = verkooporders::VerkoopOrder;
    type Output = verkooporders::VerkoopOrder;
}

impl Updatable for Verkooporders {
    type Input = verkooporders::VerkoopOrder;
    type Output = verkooporders::VerkoopOrder;
}

impl Deletable for Verkooporders {}

pub struct Verkoopordersjablonen;

impl Endpoint for Verkoopordersjablonen {
    type Model = verkoopordersjablonen::VerkoopOrderSjabloon;
    type Entity = verkoopordersjablonen::VerkoopOrderSjabloon;
    const PATH: &'static str = "verkoopordersjablonen";
}

impl Listable for Verkoopordersjablonen {}

impl Findable for Verkoopordersjablonen {}

pub struct Btwaangiftes;

impl Endpoint for Btwaangiftes {
    type Model = btw_aangiftes::BtwAangifte;
    type Entity = btw_aangiftes::BtwAangifte;
    const PATH: &'static str = "btwaangiftes";
}

impl Listable for Btwaangiftes {}

impl Paged for Btwaangiftes {}

impl Findable for Btwaangiftes {}

pub struct Offertes;

impl Endpoint for Offertes {
    type Model = verkooporders::Offerte;
    type Entity = verkooporders::Offerte;
    const PATH: &'static str = "offertes";
}

impl Listable for Offertes {}

impl Paged for Offertes {}

impl Findable for Offertes {}

impl Creatable for Offertes {
    type Input = verkooporders::Offerte;
    type Output = verkooporders::Offerte;
}

impl Updatable for Offertes {
    type Input = verkooporders::Offerte;
    type Output = verkooporders::Offerte;
}

impl Deletable for Offertes {}

pub struct Btwtarieven;

impl Endpoint for Btwtarieven {
    type Model = btw_tarieven::BtwTarief;
    type Entity = btw_tarieven::BtwTarief;
    const PATH: &'static str = "btwtarieven";
}

impl Listable for Btwtarieven {}

pub struct Actieprijzen;

impl Endpoint for Actieprijzen {
    type Model = actieprijzen::Actieprijzen;
    type Entity = actieprijzen::Actieprijzen;
    const PATH: &'static str = "actieprijzen";
}

impl Listable for Actieprijzen {}

impl Paged for Actieprijzen {}

pub struct Vatratedefinitions;

impl Endpoint for Vatratedefinitions {
    type Model = vat_rates::VatRateDefinition;
    type Entity = vat_rates::VatRateDefinition;
    const PATH: &'static str = "vatratedefinitions";
}

impl Listable for Vatratedefinitions {}

impl Paged for Vatratedefinitions {}

pub struct Vatrates;

impl Endpoint for Vatrates {
    type Model = vat_rates::VatRates;
    type Entity = vat_rates::VatRates;
    const PATH: &'static str = "vatrates";
}

impl Listable for Vatrates {}

impl Paged for Vatrates {}

impl Findable for Vatrates {}

pub struct ArtikelPrijsafspraken;

impl Endpoint for ArtikelPrijsafspraken {
    type Model = artikelen::ArtikelPrijsAfspraken;
    type Entity = artikelen::ArtikelPrijsAfspraken;
    const PATH: &'static str = "artikelen/prijsafspraken";
}

impl Listable for ArtikelPrijsafspraken {}

impl Paged for ArtikelPrijsafspraken {}

impl Client {
    pub fn artikelen(&self) -> Resource<'_, Artikelen> {
        self.resource()
    }

    pub fn artikelomzetgroepen(&self) -> Resource<'_, Artikelomzetgroepen> {
        self.resource()
    }

    pub fn bankboekingen(&self) -> Resource<'_, Bankboekingen> {
        self.resource()
    }

    pub fn dagboeken(&self) -> Resource<'_, Dagboeken> {
        self.resource()
    }

    pub fn documenten(&self) -> Resource<'_, Documenten> {
        self.resource()
    }

    pub fn grootboeken(&self) -> Resource<'_, Grootboeken> {
        self.resource()
    }

    pub fn grootboekmutaties(&self) -> Resource<'_, Grootboekmutaties> {
        self.resource()
    }

    pub fn inkoopboekingen(&self) -> Resource<'_, Inkoopboekingen> {
        self.resource()
    }

    pub fn inkoopfacturen(&self) -> Resource<'_, Inkoopfacturen> {
        self.resource()
    }

    pub fn kasboekingen(&self) -> Resource<'_, Kasboekingen> {
        self.resource()
    }

    pub fn kostenplaatsen(&self) -> Resource<'_, Kostenplaatsen> {
        self.resource()
    }

    pub fn landen(&self) -> Resource<'_, Landen> {
        self.resource()
    }

    pub fn memoriaalboekingen(&self) -> Resource<'_, Memoriaalboekingen> {
        self.resource()
    }

    pub fn prijsafspraken(&self) -> Resource<'_, Prijsafspraken> {
        self.resource()
    }

    pub fn relaties(&self) -> Resource<'_, Relaties> {
        self.resource()
    }

    pub fn verkoopboekingen(&self) -> Resource<'_, Verkoopboekingen> {
        self.resource()
    }

    pub fn verkoopfacturen(&self) -> Resource<'_, Verkoopfacturen> {
        self.resource()
    }

    pub fn verkooporders(&self) -> Resource<'_, Verkooporders> {
        self.resource()
    }

    pub fn verkoopordersjablonen(&self) -> Resource<'_, Verkoopordersjablonen> {
        self.resource()
    }

    pub fn btwaangiftes(&self) -> Resource<'_, Btwaangiftes> {
        self.resource()
    }

    pub fn offertes(&self) -> Resource<'_, Offertes> {
        self.resource()
    }

    pub fn btwtarieven(&self) -> Resource<'_, Btwtarieven> {
        self.resource()
    }

    pub fn actieprijzen(&self) -> Resource<'_, Actieprijzen> {
        self.resource()
    }

    pub fn vatratedefinitions(&self) -> Resource<'_, Vatratedefinitions> {
        self.resource()
    }

    pub fn vatrates(&self) -> Resource<'_, Vatrates> {
        self.resource()
    }

    pub fn artikel_prijsafspraken(&self) -> Resource<'_, ArtikelPrijsafspraken> {
        self.resource()
    }
}
