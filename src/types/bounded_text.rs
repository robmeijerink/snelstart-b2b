use serde::{Deserialize, Serialize};

use super::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(into = "String")]
pub struct BoundedText<const MAX: usize>(String);

impl<const MAX: usize> BoundedText<MAX> {
    pub fn new(input: impl Into<String>) -> Result<Self, ValidationError> {
        let text = input.into();
        if text.chars().count() > MAX {
            return Err(ValidationError::new("text", "too long"));
        }
        Ok(Self(text))
    }

    pub fn is_valid(&self) -> bool {
        self.0.chars().count() <= MAX
    }
}

impl<'de, const MAX: usize> Deserialize<'de> for BoundedText<MAX> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d).map(Self)
    }
}

impl<const MAX: usize> std::fmt::Display for BoundedText<MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<const MAX: usize> AsRef<str> for BoundedText<MAX> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<const MAX: usize> std::str::FromStr for BoundedText<MAX> {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl<const MAX: usize> TryFrom<&str> for BoundedText<MAX> {
    type Error = ValidationError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl<const MAX: usize> From<BoundedText<MAX>> for String {
    fn from(v: BoundedText<MAX>) -> String {
        v.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enforces_the_maximum_only() {
        assert!(BoundedText::<3>::new("abc").is_ok());
        assert!(BoundedText::<3>::new("abcd").is_err());
        assert!(BoundedText::<3>::new("").is_ok());
    }

    #[test]
    fn keeps_the_text_untouched() {
        assert_eq!(BoundedText::<9>::new(" a ").unwrap().as_ref(), " a ");
    }

    #[test]
    fn counts_characters_not_bytes() {
        assert!(BoundedText::<3>::new("éèë").is_ok());
    }

    #[test]
    fn reading_keeps_what_the_api_sent() {
        let read: BoundedText<2> = serde_json::from_str("\"abc\"").unwrap();
        assert!(!read.is_valid());
        assert_eq!(serde_json::to_string(&read).unwrap(), "\"abc\"");
    }
}
