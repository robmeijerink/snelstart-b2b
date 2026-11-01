use serde::Serialize;

use super::{ValidationError, string_newtype};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(into = "String")]
pub struct Bic(String);

string_newtype!(Bic);

impl Bic {
    pub fn new(input: &str) -> Result<Self, ValidationError> {
        let err = |reason| ValidationError::new("bic", reason);
        let bic = input.trim().to_ascii_uppercase();

        if bic.len() != 8 && bic.len() != 11 {
            return Err(err("length must be 8 or 11"));
        }
        let b = bic.as_bytes();
        if !b[..6].iter().all(u8::is_ascii_uppercase) {
            return Err(err("bank and country code must be letters"));
        }
        if !b[6..].iter().all(u8::is_ascii_alphanumeric) {
            return Err(err("unexpected characters"));
        }
        Ok(Self(bic))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_both_lengths() {
        assert_eq!(Bic::new("abnanl2a").unwrap().as_ref(), "ABNANL2A");
        assert!(Bic::new("DEUTDEFF500").is_ok());
    }

    #[test]
    fn rejects_other_shapes() {
        for bad in ["", "ABNANL2", "ABNANL2AX", "1BNANL2A", "ABNA-L2A"] {
            assert!(Bic::new(bad).is_err(), "{bad}");
        }
    }
}
