use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, RETRY_AFTER};
use reqwest::{Method, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;

use crate::auth::{ClientKey, Issued, SubscriptionKey, TokenCache, TokenResponse};
use crate::error::{Error, Result, find_codes};
use crate::models::company_info::CompanyInfo;
use crate::odata::Query;
use crate::resource::{Endpoint, Resource};

pub const DEFAULT_BASE_URL: &str = "https://b2bapi.snelstart.nl/v2/";
pub const DEFAULT_ECHO_URL: &str = "https://b2bapi.snelstart.nl/echo";
pub const DEFAULT_TOKEN_URL: &str = "https://auth.snelstart.nl/b2b/token";

const SUBSCRIPTION_HEADER: &str = "Ocp-Apim-Subscription-Key";

#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

struct Inner {
    http: reqwest::Client,
    base_url: Url,
    token_url: Url,
    echo_url: Url,
    client_key: ClientKey,
    subscription_key: SubscriptionKey,
    tokens: TokenCache,
}

pub struct ClientBuilder {
    client_key: ClientKey,
    subscription_key: SubscriptionKey,
    base_url: String,
    token_url: String,
    echo_url: String,
    timeout: Duration,
    connect_timeout: Duration,
    http: Option<reqwest::Client>,
}

impl ClientBuilder {
    pub fn from_env() -> Result<ClientBuilder> {
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    pub fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Result<ClientBuilder> {
        let value = |name: &str| {
            lookup(name)
                .map(|v| v.trim().to_owned())
                .filter(|v| !v.is_empty())
        };
        let seconds = |name: &str| {
            value(name)
                .and_then(|v| v.parse::<f64>().ok())
                .filter(|s| s.is_finite() && *s > 0.0)
                .map(Duration::from_secs_f64)
        };

        let client_key = ClientKey::new(
            value("SNELSTART_CLIENT_KEY").ok_or(Error::Config("SNELSTART_CLIENT_KEY"))?,
        )?;
        let subscription_key = SubscriptionKey::new(
            value("SNELSTART_SUBSCRIPTION_KEY")
                .ok_or(Error::Config("SNELSTART_SUBSCRIPTION_KEY"))?,
        )?;

        let mut builder = Client::builder(client_key, subscription_key);
        if let Some(url) = value("SNELSTART_BASE_URL") {
            builder = builder.base_url(url);
        }
        if let Some(url) = value("SNELSTART_TOKEN_URL") {
            builder = builder.token_url(url);
        }
        if let Some(url) = value("SNELSTART_ECHO_URL") {
            builder = builder.echo_url(url);
        }
        if let Some(timeout) = seconds("SNELSTART_TIMEOUT") {
            builder = builder.timeout(timeout);
        }
        if let Some(timeout) = seconds("SNELSTART_CONNECT_TIMEOUT") {
            builder = builder.connect_timeout(timeout);
        }
        Ok(builder)
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn token_url(mut self, url: impl Into<String>) -> Self {
        self.token_url = url.into();
        self
    }

    pub fn echo_url(mut self, url: impl Into<String>) -> Self {
        self.echo_url = url.into();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http = Some(client);
        self
    }

    pub fn build(self) -> Result<Client> {
        let mut base_url = parse_url(&self.base_url, "base url")?;
        if !base_url.path().ends_with('/') {
            base_url.set_path(&format!("{}/", base_url.path()));
        }
        let http = match self.http {
            Some(client) => client,
            None => reqwest::Client::builder()
                .timeout(self.timeout)
                .connect_timeout(self.connect_timeout)
                .redirect(reqwest::redirect::Policy::none())
                .build()?,
        };
        Ok(Client {
            inner: Arc::new(Inner {
                http,
                base_url,
                token_url: parse_url(&self.token_url, "token url")?,
                echo_url: parse_url(&self.echo_url, "echo url")?,
                client_key: self.client_key,
                subscription_key: self.subscription_key,
                tokens: TokenCache::default(),
            }),
        })
    }
}

fn parse_url(raw: &str, what: &'static str) -> Result<Url> {
    Url::parse(raw).map_err(|_| Error::Config(what))
}

impl Client {
    pub fn builder(client_key: ClientKey, subscription_key: SubscriptionKey) -> ClientBuilder {
        ClientBuilder {
            client_key,
            subscription_key,
            base_url: DEFAULT_BASE_URL.to_owned(),
            token_url: DEFAULT_TOKEN_URL.to_owned(),
            echo_url: DEFAULT_ECHO_URL.to_owned(),
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            http: None,
        }
    }

    pub fn new(client_key: ClientKey, subscription_key: SubscriptionKey) -> Result<Client> {
        Self::builder(client_key, subscription_key).build()
    }

    pub fn from_env() -> Result<Client> {
        ClientBuilder::from_env()?.build()
    }

    pub fn with_client_key(&self, client_key: ClientKey) -> Client {
        Client {
            inner: Arc::new(Inner {
                http: self.inner.http.clone(),
                base_url: self.inner.base_url.clone(),
                token_url: self.inner.token_url.clone(),
                echo_url: self.inner.echo_url.clone(),
                client_key,
                subscription_key: self.inner.subscription_key.clone(),
                tokens: TokenCache::default(),
            }),
        }
    }

    pub fn resource<E: Endpoint>(&self) -> Resource<'_, E> {
        Resource::new(self)
    }

    pub async fn company_info(&self) -> Result<CompanyInfo> {
        self.get("companyInfo", &Query::new()).await
    }

    pub async fn echo(&self) -> Result<serde_json::Value> {
        let mut url = self.inner.echo_url.clone();
        url.path_segments_mut()
            .map_err(|_| Error::Config("echo url"))?
            .pop_if_empty()
            .push("resource");
        url.set_query(Some("param1=sample"));
        let request = self
            .inner
            .http
            .get(url)
            .header(ACCEPT, "application/json")
            .header(SUBSCRIPTION_HEADER, self.inner.subscription_key.expose());
        decode(&self.finish(request.send().await).await?)
    }

    pub async fn forget_token(&self) {
        self.inner.tokens.forget().await;
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str, query: &Query) -> Result<T> {
        self.request(Method::GET, path, query.pairs(), None).await
    }

    pub async fn post<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        self.request(
            Method::POST,
            path,
            Vec::new(),
            Some(serde_json::to_vec(body)?),
        )
        .await
    }

    pub async fn put<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        self.request(
            Method::PUT,
            path,
            Vec::new(),
            Some(serde_json::to_vec(body)?),
        )
        .await
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        self.execute(Method::DELETE, self.url(path, Vec::new())?, None)
            .await
            .map(drop)
    }

    pub(crate) async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        pairs: Vec<(&str, String)>,
        body: Option<Vec<u8>>,
    ) -> Result<T> {
        decode(&self.execute(method, self.url(path, pairs)?, body).await?)
    }

    fn url(&self, path: &str, pairs: Vec<(&str, String)>) -> Result<Url> {
        let mut url = self
            .inner
            .base_url
            .join(path.trim_start_matches('/'))
            .map_err(|_| Error::Config("request path"))?;
        if !pairs.is_empty() {
            let query: Vec<String> = pairs
                .iter()
                .map(|(key, value)| format!("{}={}", encode(key), encode(value)))
                .collect();
            url.set_query(Some(&query.join("&")));
        }
        Ok(url)
    }

    async fn execute(&self, method: Method, url: Url, body: Option<Vec<u8>>) -> Result<String> {
        self.attempt(&method, &url, body.as_deref()).await
    }

    async fn attempt(&self, method: &Method, url: &Url, body: Option<&[u8]>) -> Result<String> {
        let token = self.token().await?;
        let result = self.send(method, url, &token.value, body).await;

        if matches!(result, Err(Error::Unauthorized)) && !token.fresh {
            self.inner.tokens.forget().await;
            let token = self.token().await?;
            return self.send(method, url, &token.value, body).await;
        }
        result
    }

    async fn send(
        &self,
        method: &Method,
        url: &Url,
        token: &str,
        body: Option<&[u8]>,
    ) -> Result<String> {
        let mut request = self
            .inner
            .http
            .request(method.clone(), url.clone())
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .header(SUBSCRIPTION_HEADER, self.inner.subscription_key.expose());
        if let Some(body) = body {
            request = request
                .header(CONTENT_TYPE, "application/json")
                .body(body.to_vec());
        }
        self.finish(request.send().await).await
    }

    async fn token(&self) -> Result<Issued> {
        self.inner.tokens.get_or_fetch(|| self.fetch_token()).await
    }

    async fn fetch_token(&self) -> Result<TokenResponse> {
        let response = self
            .inner
            .http
            .post(self.inner.token_url.clone())
            .header(ACCEPT, "application/json")
            .form(&[
                ("grant_type", "clientkey"),
                ("clientkey", self.inner.client_key.expose()),
            ])
            .send()
            .await;
        let body = self.finish(response).await?;
        serde_json::from_str(&body).map_err(Error::from)
    }

    async fn finish(&self, response: reqwest::Result<reqwest::Response>) -> Result<String> {
        let response = response?;
        let status = response.status();
        let retry_after = response
            .headers()
            .get(RETRY_AFTER)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.trim().parse::<u64>().ok())
            .map(Duration::from_secs);
        let body = response.text().await?;

        match status {
            s if s.is_success() => Ok(body),
            StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
            StatusCode::NOT_FOUND => Err(Error::NotFound),
            StatusCode::TOO_MANY_REQUESTS => Err(Error::RateLimited { retry_after }),
            status => Err(Error::Api {
                status,
                errors: find_codes(&body),
                body: self.redact(body),
            }),
        }
    }

    fn redact(&self, mut text: String) -> String {
        for secret in [
            self.inner.client_key.expose(),
            self.inner.subscription_key.expose(),
        ] {
            text = text.replace(secret, "[redacted]");
        }
        text
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("base_url", &self.inner.base_url.as_str())
            .finish_non_exhaustive()
    }
}

fn encode(component: &str) -> String {
    let mut out = String::with_capacity(component.len());
    for byte in component.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(char::from(byte))
            }
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}

fn decode<T: DeserializeOwned>(body: &str) -> Result<T> {
    let body = if body.trim().is_empty() { "null" } else { body };
    serde_json::from_str(body).map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys() -> (ClientKey, SubscriptionKey) {
        (
            ClientKey::new("ck").unwrap(),
            SubscriptionKey::new("sk").unwrap(),
        )
    }

    fn base(url: &str) -> Result<Client> {
        let (c, s) = keys();
        Client::builder(c, s).base_url(url).build()
    }

    #[test]
    fn base_url_always_ends_with_slash() {
        let client = base("https://example.test/v2").unwrap();
        assert_eq!(client.inner.base_url.as_str(), "https://example.test/v2/");
        assert_eq!(
            client.inner.base_url.join("relaties").unwrap().as_str(),
            "https://example.test/v2/relaties"
        );
    }

    #[test]
    fn invalid_urls_are_config_errors() {
        let (c, s) = keys();
        assert!(matches!(
            Client::builder(c, s).token_url("not a url").build(),
            Err(Error::Config("token url"))
        ));
    }

    #[test]
    fn query_pairs_are_appended_to_the_url() {
        let client = base("https://example.test/v2/").unwrap();
        let url = client
            .url(
                "relaties",
                vec![("$top", "10".into()), ("$filter", "naam eq 'a b'".into())],
            )
            .unwrap();
        assert_eq!(
            url.query(),
            Some("%24top=10&%24filter=naam%20eq%20%27a%20b%27")
        );
    }

    fn lookup<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        move |name| {
            pairs
                .iter()
                .find(|(k, _)| *k == name)
                .map(|(_, v)| (*v).to_owned())
        }
    }

    #[test]
    fn environment_needs_both_keys() {
        let only_client = [("SNELSTART_CLIENT_KEY", "ck")];
        assert!(matches!(
            ClientBuilder::from_lookup(lookup(&only_client)),
            Err(Error::Config("SNELSTART_SUBSCRIPTION_KEY"))
        ));
        let blank = [
            ("SNELSTART_CLIENT_KEY", "  "),
            ("SNELSTART_SUBSCRIPTION_KEY", "sk"),
        ];
        assert!(matches!(
            ClientBuilder::from_lookup(lookup(&blank)),
            Err(Error::Config("SNELSTART_CLIENT_KEY"))
        ));
    }

    #[test]
    fn environment_overrides_urls_and_ignores_unusable_timeouts() {
        let env = [
            ("SNELSTART_CLIENT_KEY", "ck"),
            ("SNELSTART_SUBSCRIPTION_KEY", "sk"),
            ("SNELSTART_BASE_URL", "https://example.test/v2"),
            ("SNELSTART_TIMEOUT", "abc"),
            ("SNELSTART_CONNECT_TIMEOUT", "-3"),
        ];
        let builder = ClientBuilder::from_lookup(lookup(&env)).unwrap();
        assert_eq!(builder.base_url, "https://example.test/v2");
        assert_eq!(builder.timeout, Duration::from_secs(30));
        assert_eq!(builder.connect_timeout, Duration::from_secs(10));
        assert!(builder.build().is_ok());
    }

    #[test]
    fn encoding_keeps_only_unreserved_characters() {
        assert_eq!(encode("aZ09-._~"), "aZ09-._~");
        assert_eq!(encode("a b/é"), "a%20b%2F%C3%A9");
    }

    #[test]
    fn empty_body_decodes_as_unit() {
        assert!(decode::<()>("").is_ok());
        assert!(decode::<Option<u8>>("  ").unwrap().is_none());
    }

    #[test]
    fn debug_hides_keys() {
        let (c, s) = keys();
        let out = format!("{:?}", Client::new(c, s).unwrap());
        assert!(!out.contains("client_key"));
        assert!(!out.contains("subscription_key"));
    }
}
