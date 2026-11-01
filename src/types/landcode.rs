use serde::Serialize;

use super::{ValidationError, string_newtype};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(into = "String")]
pub struct Landcode(String);

string_newtype!(Landcode);

impl Landcode {
    pub fn new(input: &str) -> Result<Self, ValidationError> {
        let code = input.trim().to_ascii_uppercase();
        if code.len() == 2 && code.bytes().all(|b| b.is_ascii_uppercase()) {
            Ok(Self(code))
        } else {
            Err(ValidationError::new("country code", "must be two letters"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_letters_uppercased() {
        assert_eq!(Landcode::new("nl").unwrap().as_ref(), "NL");
        for bad in ["", "N", "NLD", "N1"] {
            assert!(Landcode::new(bad).is_err(), "{bad}");
        }
    }
}
