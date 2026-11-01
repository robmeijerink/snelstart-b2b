mod common;

use common::*;
use snelstart_b2b::{ClientKey, Error};
use wiremock::matchers::{body_string_contains, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn mount_company_info(server: &MockServer, token: &str, status: u16) {
    Mock::given(method("GET"))
        .and(path("/v2/companyInfo"))
        .and(header("authorization", format!("Bearer {token}").as_str()))
        .respond_with(if status == 200 {
            respond("company_info")
        } else {
            ResponseTemplate::new(status)
        })
        .mount(server)
        .await;
}

#[tokio::test]
async fn token_request_uses_the_clientkey_grant_as_a_form() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .and(header("content-type", "application/x-www-form-urlencoded"))
        .and(body_string_contains("grant_type=clientkey"))
        .and(body_string_contains(
            format!("clientkey={CLIENT_KEY}").as_str(),
        ))
        .respond_with(token_response("tok", 3600))
        .expect(1)
        .mount(&server)
        .await;
    mount_company_info(&server, "tok", 200).await;

    client.company_info().await.unwrap();
}

#[tokio::test]
async fn api_calls_carry_token_and_subscription_key() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    Mock::given(method("GET"))
        .and(path("/v2/companyInfo"))
        .and(header("authorization", "Bearer tok"))
        .and(header("ocp-apim-subscription-key", SUBSCRIPTION_KEY))
        .and(header("accept", "application/json"))
        .respond_with(respond("company_info"))
        .expect(1)
        .mount(&server)
        .await;

    client.company_info().await.unwrap();
}

#[tokio::test]
async fn the_subscription_key_is_not_sent_to_the_token_endpoint() {
    let (server, client) = setup().await;
    mount_token(&server, "tok").await;
    mount_company_info(&server, "tok", 200).await;
    client.company_info().await.unwrap();

    let requests = server.received_requests().await.unwrap();
    let token_request = requests.iter().find(|r| r.url.path() == "/token").unwrap();
    assert!(
        token_request
            .headers
            .get("ocp-apim-subscription-key")
            .is_none()
    );
}

#[tokio::test]
async fn a_token_is_fetched_once_and_reused() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(token_response("tok", 3600))
        .expect(1)
        .mount(&server)
        .await;
    mount_company_info(&server, "tok", 200).await;

    for _ in 0..3 {
        client.company_info().await.unwrap();
    }
}

#[tokio::test]
async fn a_token_about_to_expire_is_not_reused() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(token_response("tok", 30))
        .expect(2)
        .mount(&server)
        .await;
    mount_company_info(&server, "tok", 200).await;

    client.company_info().await.unwrap();
    client.company_info().await.unwrap();
}

#[tokio::test]
async fn forgetting_the_token_fetches_a_new_one() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(token_response("tok", 3600))
        .expect(2)
        .mount(&server)
        .await;
    mount_company_info(&server, "tok", 200).await;

    client.company_info().await.unwrap();
    client.forget_token().await;
    client.company_info().await.unwrap();
}

#[tokio::test]
async fn a_rejected_cached_token_is_replaced_and_the_call_repeated_once() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(token_response("old", 3600))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(token_response("new", 3600))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/companyInfo"))
        .and(header("authorization", "Bearer old"))
        .respond_with(respond("company_info"))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/companyInfo"))
        .and(header("authorization", "Bearer old"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&server)
        .await;
    mount_company_info(&server, "new", 200).await;

    client.company_info().await.unwrap();
    let info = client.company_info().await.unwrap();
    assert_eq!(
        info.administratie_naam.as_deref(),
        Some("Demo administratie")
    );
}

#[tokio::test]
async fn a_401_on_a_fresh_token_is_not_retried() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(token_response("tok", 3600))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/companyInfo"))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&server)
        .await;

    assert!(matches!(
        client.company_info().await,
        Err(Error::Unauthorized)
    ));
}

#[tokio::test]
async fn a_second_401_after_refreshing_is_final() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(token_response("tok", 3600))
        .expect(2)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/companyInfo"))
        .respond_with(respond("company_info"))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/companyInfo"))
        .respond_with(ResponseTemplate::new(401))
        .expect(2)
        .mount(&server)
        .await;

    client.company_info().await.unwrap();
    assert!(matches!(
        client.company_info().await,
        Err(Error::Unauthorized)
    ));
}

#[tokio::test]
async fn a_rejected_client_key_is_an_api_error_without_the_key() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(json(
            400,
            serde_json::json!({ "error": "invalid_grant", "detail": format!("unknown key {CLIENT_KEY}") }),
        ))
        .mount(&server)
        .await;

    let err = client.company_info().await.unwrap_err();
    assert_eq!(err.status().map(|s| s.as_u16()), Some(400));
    assert!(!format!("{err} {err:?}").contains(CLIENT_KEY));
    assert!(matches!(&err, Error::Api { body, .. } if body.contains("[redacted]")));
}

#[tokio::test]
async fn a_token_response_without_access_token_is_a_decode_error() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(json(200, serde_json::json!({ "expires_in": 3600 })))
        .mount(&server)
        .await;

    assert!(matches!(client.company_info().await, Err(Error::Decode(_))));
}

#[tokio::test]
async fn another_client_key_gets_its_own_token() {
    let (server, client) = setup().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .and(body_string_contains(
            format!("clientkey={CLIENT_KEY}").as_str(),
        ))
        .respond_with(token_response("first", 3600))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .and(body_string_contains("clientkey=other-administration"))
        .respond_with(token_response("second", 3600))
        .expect(1)
        .mount(&server)
        .await;
    mount_company_info(&server, "first", 200).await;
    mount_company_info(&server, "second", 200).await;

    let other = client.with_client_key(ClientKey::new("other-administration").unwrap());
    client.company_info().await.unwrap();
    other.company_info().await.unwrap();
    client.company_info().await.unwrap();
}

#[tokio::test]
async fn the_echo_call_needs_no_token() {
    let (server, client) = setup().await;
    Mock::given(method("GET"))
        .and(path("/echo/resource"))
        .and(wiremock::matchers::query_param("param1", "sample"))
        .and(header("ocp-apim-subscription-key", SUBSCRIPTION_KEY))
        .respond_with(json(200, serde_json::json!({ "ok": true })))
        .expect(1)
        .mount(&server)
        .await;

    let answer = client.echo().await.unwrap();
    assert_eq!(answer["ok"], true);

    let requests = server.received_requests().await.unwrap();
    assert!(requests.iter().all(|r| r.url.path() != "/token"));
    assert!(
        requests
            .iter()
            .all(|r| r.headers.get("authorization").is_none())
    );
}
