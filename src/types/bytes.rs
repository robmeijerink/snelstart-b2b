use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(data: Vec<u8>) -> Self {
        Self(data)
    }
}

impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bytes({} bytes)", self.0.len())
    }
}

impl Serialize for Bytes {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&STANDARD.encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        STANDARD
            .decode(raw.as_bytes())
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_roundtrip() {
        let bytes = Bytes::new(b"hello".to_vec());
        let json = serde_json::to_string(&bytes).unwrap();
        assert_eq!(json, "\"aGVsbG8=\"");
        assert_eq!(serde_json::from_str::<Bytes>(&json).unwrap(), bytes);
    }

    #[test]
    fn invalid_base64_is_an_error() {
        assert!(serde_json::from_str::<Bytes>("\"***\"").is_err());
    }

    #[test]
    fn debug_does_not_dump_content() {
        assert_eq!(format!("{:?}", Bytes::new(vec![1, 2, 3])), "Bytes(3 bytes)");
    }
}
