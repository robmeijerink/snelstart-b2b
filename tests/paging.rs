mod common;

use common::*;
use futures::TryStreamExt;
use snelstart_b2b::Error;
use snelstart_b2b::odata::{Filter, Query};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn rows(from: usize, count: usize) -> serde_json::Value {
    (from..from + count)
        .map(|n| serde_json::json!({ "relatiecode": n, "naam": format!("relatie {n}") }))
        .collect()
}

async fn mount_page(server: &MockServer, skip: &str, body: serde_json::Value) {
    Mock::given(method("GET"))
        .and(path("/v2/relaties"))
        .and(query_param("$skip", skip))
        .and(query_param("$top", "500"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(server)
        .await;
}

#[tokio::test]
async fn all_follows_the_pages_until_a_short_one() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    mount_page(&server, "0", rows(0, 500)).await;
    mount_page(&server, "500", rows(500, 500)).await;
    mount_page(&server, "1000", rows(1000, 20)).await;

    let all: Vec<_> = client
        .relaties()
        .all(Query::new())
        .try_collect()
        .await
        .unwrap();
    assert_eq!(all.len(), 1020);
    assert_eq!(all[1019].relatiecode, Some(1019));
}

#[tokio::test]
async fn an_exactly_full_last_page_costs_one_more_call() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    mount_page(&server, "0", rows(0, 500)).await;
    mount_page(&server, "500", serde_json::json!([])).await;

    let all: Vec<_> = client
        .relaties()
        .all(Query::new())
        .try_collect()
        .await
        .unwrap();
    assert_eq!(all.len(), 500);
}

#[tokio::test]
async fn an_empty_collection_is_one_call() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    mount_page(&server, "0", serde_json::json!([])).await;

    let all: Vec<_> = client
        .relaties()
        .all(Query::new())
        .try_collect()
        .await
        .unwrap();
    assert!(all.is_empty());
}

#[tokio::test]
async fn the_filter_and_the_start_offset_are_kept() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("GET"))
        .and(path("/v2/relaties"))
        .and(query_param("$filter", "nonactief eq false"))
        .and(query_param("$skip", "40"))
        .and(query_param("$top", "500"))
        .respond_with(ResponseTemplate::new(200).set_body_json(rows(40, 3)))
        .expect(1)
        .mount(&server)
        .await;

    let query = Query::new()
        .filter(Filter::eq("nonactief", false).unwrap())
        .skip(40);
    let all: Vec<_> = client.relaties().all(query).try_collect().await.unwrap();
    assert_eq!(all.len(), 3);
}

#[tokio::test]
async fn a_collection_that_ignores_skip_is_reported_instead_of_looping() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("GET"))
        .and(path("/v2/relaties"))
        .respond_with(ResponseTemplate::new(200).set_body_json(rows(0, 500)))
        .expect(2)
        .mount(&server)
        .await;

    let err = client
        .relaties()
        .all(Query::new())
        .try_collect::<Vec<_>>()
        .await
        .unwrap_err();
    assert!(matches!(err, Error::PagingStalled { path: "relaties" }));
}

#[tokio::test]
async fn nothing_is_fetched_before_the_stream_is_polled() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;

    let _stream = client.relaties().all(Query::new());
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn dropping_the_stream_stops_fetching() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    mount_page(&server, "0", rows(0, 500)).await;

    let mut stream = Box::pin(client.relaties().all(Query::new()));
    assert!(stream.try_next().await.unwrap().is_some());
    drop(stream);
}

#[tokio::test]
async fn an_error_on_a_later_page_ends_the_stream() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    mount_page(&server, "0", rows(0, 500)).await;
    Mock::given(method("GET"))
        .and(path("/v2/relaties"))
        .and(query_param("$skip", "500"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let mut stream = Box::pin(client.relaties().all(Query::new()));
    for _ in 0..500 {
        assert!(stream.try_next().await.unwrap().is_some());
    }
    assert!(matches!(stream.try_next().await, Err(Error::Api { .. })));
}
