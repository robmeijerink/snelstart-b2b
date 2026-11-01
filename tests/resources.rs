mod common;

use common::*;
use snelstart_b2b::endpoints::*;
use snelstart_b2b::odata::Query;
use snelstart_b2b::resource::{Endpoint, Listable};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer};

async fn list_hits<E: Listable>(server: &MockServer, client: &snelstart_b2b::Client) {
    Mock::given(method("GET"))
        .and(path(format!("/v2/{}", E::PATH)))
        .and(header("authorization", "Bearer tok"))
        .respond_with(json(200, serde_json::json!([{}, {}])))
        .expect(1)
        .mount(server)
        .await;

    let rows = client.resource::<E>().list(&Query::new()).await.unwrap();
    assert_eq!(rows.len(), 2, "{}", E::PATH);
}

#[tokio::test]
async fn every_listable_collection_reads_an_array_from_its_own_path() {
    macro_rules! check {
        ($($endpoint:ty),+ $(,)?) => {$(
            let (server, client) = setup().await;
            mount_token(&server, "tok").await;
            list_hits::<$endpoint>(&server, &client).await;
        )+};
    }

    check!(
        Actieprijzen,
        Artikelen,
        ArtikelPrijsafspraken,
        Artikelomzetgroepen,
        Bankboekingen,
        Btwaangiftes,
        Btwtarieven,
        Dagboeken,
        Grootboeken,
        Grootboekmutaties,
        Inkoopfacturen,
        Kasboekingen,
        Kostenplaatsen,
        Landen,
        Offertes,
        Prijsafspraken,
        Relaties,
        Vatratedefinitions,
        Vatrates,
        Verkoopfacturen,
        Verkooporders,
        Verkoopordersjablonen,
    );
}

#[tokio::test]
async fn accessors_use_the_documented_paths() {
    let (_server, client) = setup().await;
    assert_eq!(<Relaties as Endpoint>::PATH, "relaties");
    assert_eq!(<Inkoopboekingen as Endpoint>::PATH, "inkoopboekingen");
    assert_eq!(<Documenten as Endpoint>::PATH, "documenten");
    let _ = client.kostenplaatsen();
    let _ = client.memoriaalboekingen();
}
