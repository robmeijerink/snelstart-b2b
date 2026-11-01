use serde::Serialize;

use super::{ValidationError, string_newtype};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(into = "String")]
pub struct Email(String);

string_newtype!(Email);

impl Email {
    pub fn new(input: &str) -> Result<Self, ValidationError> {
        let err = |reason| ValidationError::new("email", reason);
        let email = input.trim();

        if email.len() > 250 {
            return Err(err("too long"));
        }
        let (local, domain) = email.rsplit_once('@').ok_or(err("missing @"))?;
        if local.is_empty() || local.len() > 64 || local.contains(char::is_whitespace) {
            return Err(err("invalid local part"));
        }
        let labels_ok = domain.split('.').all(|l| {
            !l.is_empty()
                && !l.starts_with('-')
                && !l.ends_with('-')
                && l.chars().all(|c| c.is_alphanumeric() || c == '-')
        });
        if !domain.contains('.') || !labels_ok {
            return Err(err("invalid domain"));
        }
        Ok(Self(email.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_regular_addresses() {
        assert!(Email::new("info@example.nl").is_ok());
        assert!(Email::new("a.b+c@sub.example.co.uk").is_ok());
        assert_eq!(Email::new("  x@y.nl ").unwrap().as_ref(), "x@y.nl");
    }

    #[test]
    fn rejects_malformed_addresses() {
        for bad in [
            "",
            "plain",
            "@example.nl",
            "a@b",
            "a b@example.nl",
            "a@-x.nl",
            "a@x..nl",
        ] {
            assert!(Email::new(bad).is_err(), "{bad}");
        }
    }
}
