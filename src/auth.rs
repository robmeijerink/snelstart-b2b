use std::fmt;
use std::time::{Duration, Instant};

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::types::ValidationError;

const EXPIRY_MARGIN: Duration = Duration::from_secs(60);
const DEFAULT_LIFETIME: u64 = 3600;

macro_rules! secret {
    ($name:ident, $kind:literal) => {
        #[derive(Clone, PartialEq, Eq)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, ValidationError> {
                let value = value.as_ref().trim();
                if value.is_empty() {
                    Err(ValidationError::new($kind, "must not be empty"))
                } else {
                    Ok(Self(value.to_owned()))
                }
            }

            pub(crate) fn expose(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(concat!(stringify!($name), "([redacted])"))
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("[redacted]")
            }
        }
    };
}

secret!(ClientKey, "client key");
secret!(SubscriptionKey, "subscription key");

#[derive(Deserialize)]
pub(crate) struct TokenResponse {
    pub access_token: String,
    pub expires_in: Option<u64>,
}

#[derive(Clone)]
struct AccessToken {
    value: String,
    expires_at: Instant,
}

impl AccessToken {
    fn from_response(response: TokenResponse, now: Instant) -> Self {
        let lifetime = Duration::from_secs(response.expires_in.unwrap_or(DEFAULT_LIFETIME));
        Self {
            value: response.access_token,
            expires_at: now + lifetime.saturating_sub(EXPIRY_MARGIN),
        }
    }

    fn is_usable(&self, now: Instant) -> bool {
        now < self.expires_at
    }
}

pub(crate) struct Issued {
    pub value: String,
    pub fresh: bool,
}

#[derive(Default)]
pub(crate) struct TokenCache {
    slot: Mutex<Option<AccessToken>>,
}

impl TokenCache {
    pub(crate) async fn get_or_fetch<F, Fut>(&self, fetch: F) -> crate::Result<Issued>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = crate::Result<TokenResponse>>,
    {
        let mut slot = self.slot.lock().await;
        if let Some(token) = slot.as_ref().filter(|t| t.is_usable(Instant::now())) {
            return Ok(Issued {
                value: token.value.clone(),
                fresh: false,
            });
        }
        let token = AccessToken::from_response(fetch().await?, Instant::now());
        let value = token.value.clone();
        *slot = Some(token);
        Ok(Issued { value, fresh: true })
    }

    pub(crate) async fn forget(&self) {
        *self.slot.lock().await = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response(expires_in: Option<u64>) -> TokenResponse {
        TokenResponse {
            access_token: "tok".into(),
            expires_in,
        }
    }

    #[test]
    fn secrets_never_print() {
        let key = ClientKey::new("super-secret").unwrap();
        assert!(!format!("{key:?}").contains("super-secret"));
        assert!(!key.to_string().contains("super-secret"));
        assert_eq!(key.expose(), "super-secret");
    }

    #[test]
    fn secrets_must_not_be_blank() {
        assert!(ClientKey::new("  ").is_err());
        assert!(SubscriptionKey::new("").is_err());
        assert_eq!(SubscriptionKey::new(" k ").unwrap().expose(), "k");
    }

    #[test]
    fn token_expires_a_minute_early() {
        let now = Instant::now();
        let token = AccessToken::from_response(response(Some(3600)), now);
        assert!(token.is_usable(now + Duration::from_secs(3500)));
        assert!(!token.is_usable(now + Duration::from_secs(3541)));
    }

    #[test]
    fn short_lived_token_is_never_usable() {
        let now = Instant::now();
        assert!(!AccessToken::from_response(response(Some(30)), now).is_usable(now));
    }

    #[test]
    fn missing_lifetime_falls_back_to_an_hour() {
        let now = Instant::now();
        let token = AccessToken::from_response(response(None), now);
        assert!(token.is_usable(now + Duration::from_secs(3000)));
    }

    #[tokio::test]
    async fn cache_fetches_once_and_forgets_on_request() {
        let cache = TokenCache::default();
        let first = cache
            .get_or_fetch(|| async { Ok(response(Some(3600))) })
            .await
            .unwrap();
        assert!(first.fresh);
        let second = cache
            .get_or_fetch(|| async { panic!("must not fetch again") })
            .await
            .unwrap();
        assert!(!second.fresh);
        cache.forget().await;
        let third = cache
            .get_or_fetch(|| async { Ok(response(Some(3600))) })
            .await
            .unwrap();
        assert!(third.fresh);
    }
}
