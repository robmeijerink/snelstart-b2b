#![allow(dead_code)]

use snelstart_b2b::{Client, ClientKey, SubscriptionKey};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub const CLIENT_KEY: &str = "client-key-123";
pub const SUBSCRIPTION_KEY: &str = "subscription-key-456";

pub async fn setup() -> (MockServer, Client) {
    let server = MockServer::start().await;
    let client = client_for(&server);
    (server, client)
}

pub fn client_for(server: &MockServer) -> Client {
    Client::builder(
        ClientKey::new(CLIENT_KEY).unwrap(),
        SubscriptionKey::new(SUBSCRIPTION_KEY).unwrap(),
    )
    .base_url(format!("{}/v2/", server.uri()))
    .token_url(format!("{}/token", server.uri()))
    .echo_url(format!("{}/echo", server.uri()))
    .build()
    .unwrap()
}

pub fn token_response(token: &str, expires_in: u64) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "access_token": token,
        "token_type": "bearer",
        "expires_in": expires_in,
    }))
}

pub async fn mount_token(server: &MockServer, token: &str) {
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(token_response(token, 3600))
        .mount(server)
        .await;
}

pub fn fixture(name: &str) -> String {
    let path = format!("{}/tests/fixtures/{name}.json", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

pub fn fixture_json(name: &str) -> serde_json::Value {
    serde_json::from_str(&fixture(name)).unwrap()
}

pub fn respond(name: &str) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(fixture(name), "application/json")
}

pub fn json(status: u16, body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(status).set_body_json(body)
}
