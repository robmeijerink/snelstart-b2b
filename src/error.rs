use std::time::Duration;

use reqwest::StatusCode;
use thiserror::Error;

use crate::error_messages::MESSAGES;
pub use crate::types::ValidationError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("request failed: {0}")]
    Transport(#[source] reqwest::Error),

    #[error("SnelStart answered {status}{}", format_codes(.errors))]
    Api {
        status: StatusCode,
        errors: Vec<ErrorCode>,
        body: String,
    },

    #[error("not found")]
    NotFound,

    #[error("the access token was rejected")]
    Unauthorized,

    #[error("rate limited")]
    RateLimited { retry_after: Option<Duration> },

    #[error("unexpected response body: {0}")]
    Decode(#[from] serde_json::Error),

    #[error(transparent)]
    Validation(#[from] ValidationError),

    #[error("paging on {path} does not advance")]
    PagingStalled { path: &'static str },

    #[error("invalid configuration: {0}")]
    Config(&'static str),
}

impl Error {
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Error::Api { status, .. } => Some(*status),
            Error::NotFound => Some(StatusCode::NOT_FOUND),
            Error::Unauthorized => Some(StatusCode::UNAUTHORIZED),
            Error::RateLimited { .. } => Some(StatusCode::TOO_MANY_REQUESTS),
            Error::Transport(err) => err.status(),
            _ => None,
        }
    }

    pub fn error_codes(&self) -> &[ErrorCode] {
        match self {
            Error::Api { errors, .. } => errors,
            _ => &[],
        }
    }

    pub fn has_code(&self, code: &str) -> bool {
        matches!(self, Error::Api { errors, .. } if errors.iter().any(|e| e.as_str() == code))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Transport(err.without_url())
    }
}

fn format_codes(codes: &[ErrorCode]) -> String {
    if codes.is_empty() {
        return String::new();
    }
    let list: Vec<&str> = codes.iter().map(ErrorCode::as_str).collect();
    format!(" ({})", list.join(", "))
}

const RESOURCES: &[(&str, &str)] = &[
    ("ALG", "Algemeen"),
    ("ART", "Artikelen"),
    ("BLG", "Bijlagen"),
    ("BOE", "Boekingen"),
    ("BTW", "Btw Aangifte"),
    ("KPL", "Kostenplaatsen"),
    ("REF", "Extra velden"),
    ("REL", "Relaties"),
    ("VOR", "Verkooporders"),
];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorCode(String);

impl ErrorCode {
    pub fn new(input: &str) -> Result<Self> {
        let err = |reason| ValidationError::new("error code", reason);
        let (prefix, number) = input.split_once('-').ok_or(err("missing dash"))?;
        if !RESOURCES.iter().any(|(p, _)| *p == prefix) {
            return Err(err("unknown resource").into());
        }
        if number.len() != 4 || !number.bytes().all(|b| b.is_ascii_digit()) {
            return Err(err("number must be four digits").into());
        }
        Ok(Self(input.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn message(&self) -> Option<&'static str> {
        MESSAGES
            .binary_search_by_key(&self.0.as_str(), |(code, _)| code)
            .ok()
            .map(|i| MESSAGES[i].1)
    }

    pub fn resource(&self) -> &'static str {
        let prefix = &self.0[..3];
        RESOURCES
            .iter()
            .find(|(p, _)| *p == prefix)
            .map_or("", |(_, name)| name)
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub(crate) fn find_codes(text: &str) -> Vec<ErrorCode> {
    let bytes = text.as_bytes();
    let mut found: Vec<ErrorCode> = Vec::new();
    let mut i = 0;
    while i + 8 <= bytes.len() {
        let boundary_before = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
        let boundary_after = i + 8 == bytes.len() || !bytes[i + 8].is_ascii_digit();
        if boundary_before && boundary_after {
            if let Ok(code) = text
                .get(i..i + 8)
                .map_or(Err(()), |s| ErrorCode::new(s).map_err(|_| ()))
            {
                if !found.contains(&code) {
                    found.push(code);
                }
                i += 8;
                continue;
            }
        }
        i += 1;
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_codes() {
        let code = ErrorCode::new("REL-0007").unwrap();
        assert_eq!(code.resource(), "Relaties");
        assert_eq!(code.to_string(), "REL-0007");
    }

    #[test]
    fn rejects_unknown_or_malformed_codes() {
        for bad in [
            "",
            "REL0007",
            "XXX-0001",
            "REL-007",
            "REL-00071",
            "rel-0007",
        ] {
            assert!(ErrorCode::new(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn messages_are_looked_up_by_code() {
        assert_eq!(
            ErrorCode::new("REL-0007").unwrap().message(),
            Some("Naam moet gevuld zijn")
        );
        assert_eq!(ErrorCode::new("REL-9999").unwrap().message(), None);
    }

    #[test]
    fn every_documented_message_is_reachable_and_sorted() {
        assert_eq!(MESSAGES.len(), 272);
        assert!(MESSAGES.windows(2).all(|w| w[0].0 < w[1].0));
        assert!(
            MESSAGES
                .iter()
                .all(|(code, _)| ErrorCode::new(code).is_ok())
        );
    }

    #[test]
    fn finds_codes_in_free_text() {
        let body = r#"[{"message":"Naam moet gevuld zijn","code":"REL-0007"},{"code":"REL-0007"},{"code":"ART-0002"}]"#;
        let codes: Vec<String> = find_codes(body).iter().map(ToString::to_string).collect();
        assert_eq!(codes, ["REL-0007", "ART-0002"]);
    }

    #[test]
    fn ignores_lookalikes() {
        assert!(find_codes("XREL-0007 and REL-00071 and ZZZ-0001").is_empty());
        assert!(find_codes("héllo wörld REL-").is_empty());
    }

    #[test]
    fn has_code_checks_api_errors_only() {
        let err = Error::Api {
            status: StatusCode::BAD_REQUEST,
            errors: vec![ErrorCode::new("ART-0005").unwrap()],
            body: String::new(),
        };
        assert!(err.has_code("ART-0005"));
        assert!(!err.has_code("ART-0006"));
        assert!(!Error::NotFound.has_code("ART-0005"));
        assert_eq!(
            err.to_string(),
            "SnelStart answered 400 Bad Request (ART-0005)"
        );
    }
}
