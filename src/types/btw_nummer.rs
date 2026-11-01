use serde::Serialize;

use super::{ValidationError, string_newtype};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(into = "String")]
pub struct BtwNummer(String);

string_newtype!(BtwNummer);

impl BtwNummer {
    pub fn new(input: &str) -> Result<Self, ValidationError> {
        let vat = input.trim();
        match vat.chars().count() {
            0..=1 => Err(ValidationError::new("vat number", "at least 2 characters")),
            2..=50 => Ok(Self(vat.to_owned())),
            _ => Err(ValidationError::new("vat number", "at most 50 characters")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_the_documented_length_rule() {
        assert!(BtwNummer::new("NL").is_ok());
        assert!(BtwNummer::new("NL123456789B01").is_ok());
        assert!(BtwNummer::new(&"x".repeat(50)).is_ok());
        assert!(BtwNummer::new("N").is_err());
        assert!(BtwNummer::new("  ").is_err());
        assert!(BtwNummer::new(&"x".repeat(51)).is_err());
    }

    #[test]
    fn is_trimmed() {
        assert_eq!(BtwNummer::new(" NL1 ").unwrap().as_ref(), "NL1");
    }
}
