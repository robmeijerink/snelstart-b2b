//! Builds clients with their own settings, and a second client for another administration.
//!
//! `cargo run --example custom_client`

use std::time::Duration;

use snelstart_b2b::client::{DEFAULT_BASE_URL, DEFAULT_ECHO_URL, DEFAULT_TOKEN_URL};
use snelstart_b2b::endpoints::Landen;
use snelstart_b2b::odata::Query;
use snelstart_b2b::{Client, ClientBuilder, ClientKey, SubscriptionKey};

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client_key = std::env::var("SNELSTART_CLIENT_KEY").unwrap();
    let subscription_key = std::env::var("SNELSTART_SUBSCRIPTION_KEY").unwrap();

    let client = Client::builder(
        ClientKey::new(client_key.clone())?,
        SubscriptionKey::new(subscription_key.clone())?,
    )
    .timeout(Duration::from_secs(60))
    .connect_timeout(Duration::from_secs(5))
    .build()?;
    println!("{:?}", client.company_info().await?.administratie_naam);

    let client = ClientBuilder::from_env()?
        .base_url(DEFAULT_BASE_URL)
        .token_url(DEFAULT_TOKEN_URL)
        .echo_url(DEFAULT_ECHO_URL)
        .http_client(reqwest::Client::new())
        .build()?;
    client.echo().await?;

    let settings = [
        ("SNELSTART_CLIENT_KEY", client_key),
        ("SNELSTART_SUBSCRIPTION_KEY", subscription_key),
        ("SNELSTART_TIMEOUT", "45".to_owned()),
    ];
    let client = ClientBuilder::from_lookup(|name| {
        settings
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| value.clone())
    })?
    .build()?;

    let countries = client.resource::<Landen>().list(&Query::new()).await?;
    println!("{} countries", countries.len());

    client.forget_token().await;
    println!("{:?}", client.company_info().await?.administratie_naam);

    if let Ok(other_key) = std::env::var("SNELSTART_OTHER_CLIENT_KEY") {
        let other = client.with_client_key(ClientKey::new(other_key)?);
        println!("{:?}", other.company_info().await?.administratie_naam);
    }
    Ok(())
}
