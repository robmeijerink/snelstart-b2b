use serde::Serialize;

use super::{ValidationError, string_newtype};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(into = "String")]
pub struct Iban(String);

string_newtype!(Iban);

impl Iban {
    pub fn new(input: &str) -> Result<Self, ValidationError> {
        let err = |reason| ValidationError::new("iban", reason);
        let iban: String = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_ascii_uppercase())
            .collect();

        if !(15..=34).contains(&iban.len()) {
            return Err(err("length must be between 15 and 34"));
        }
        let (head, rest) = iban.split_at(4);
        let bytes = head.as_bytes();
        if !bytes[..2].iter().all(u8::is_ascii_uppercase)
            || !bytes[2..].iter().all(u8::is_ascii_digit)
            || !rest.bytes().all(|b| b.is_ascii_alphanumeric())
        {
            return Err(err("unexpected characters"));
        }

        let remainder = rest
            .chars()
            .chain(head.chars())
            .flat_map(|c| c.to_digit(36).map(|d| d.to_string()))
            .flat_map(|s| s.into_bytes())
            .fold(0u32, |acc, d| (acc * 10 + u32::from(d - b'0')) % 97);

        if remainder != 1 {
            return Err(err("checksum mismatch"));
        }
        Ok(Self(iban))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_numbers_and_normalizes() {
        assert_eq!(
            Iban::new("nl91 abna 0417 1643 00").unwrap().as_ref(),
            "NL91ABNA0417164300"
        );
        assert!(Iban::new("DE89370400440532013000").is_ok());
        assert!(Iban::new("GB82WEST12345698765432").is_ok());
    }

    #[test]
    fn rejects_bad_checksum() {
        assert!(Iban::new("NL91ABNA0417164301").is_err());
    }

    #[test]
    fn rejects_bad_shape() {
        assert!(Iban::new("").is_err());
        assert!(Iban::new("NL91").is_err());
        assert!(Iban::new("9199ABNA0417164300").is_err());
        assert!(Iban::new("NL91ABNA0417-64300").is_err());
    }

    #[test]
    fn reading_is_lenient_but_flags_invalid_values() {
        let ok: Iban = serde_json::from_str("\"NL91ABNA0417164300\"").unwrap();
        let odd: Iban = serde_json::from_str("\"NL00ABNA0417164300\"").unwrap();
        assert!(ok.is_valid());
        assert!(!odd.is_valid());
        assert_eq!(
            serde_json::to_string(&odd).unwrap(),
            "\"NL00ABNA0417164300\""
        );
    }
}
