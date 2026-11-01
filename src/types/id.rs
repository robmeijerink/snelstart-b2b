use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Id<T>(Uuid, PhantomData<fn() -> T>);

impl<T> Id<T> {
    pub const fn new(uuid: Uuid) -> Self {
        Self(uuid, PhantomData)
    }

    pub fn parse(input: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(input).map(Self::new)
    }

    pub const fn uuid(&self) -> Uuid {
        self.0
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({})", self.0)
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<Uuid> for Id<T> {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Uuid::deserialize(d).map(Self::new)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(bound = "")]
pub struct Ref<T> {
    pub id: Id<T>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl<T> Ref<T> {
    pub fn to(id: Id<T>) -> Self {
        Self { id, uri: None }
    }
}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            uri: self.uri.clone(),
        }
    }
}

impl<T> fmt::Debug for Ref<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ref")
            .field("id", &self.id)
            .field("uri", &self.uri)
            .finish()
    }
}

impl<T> PartialEq for Ref<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> From<Id<T>> for Ref<T> {
    fn from(id: Id<T>) -> Self {
        Self::to(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct A;

    const RAW: &str = "6b1f4e0e-5a58-4b0a-9c0d-0b8a3f6d2c11";

    #[test]
    fn serializes_as_plain_uuid() {
        let id: Id<A> = Id::parse(RAW).unwrap();
        assert_eq!(serde_json::to_string(&id).unwrap(), format!("\"{RAW}\""));
        assert_eq!(
            serde_json::from_str::<Id<A>>(&format!("\"{RAW}\"")).unwrap(),
            id
        );
    }

    #[test]
    fn ref_skips_missing_uri() {
        let r: Ref<A> = Ref::to(Id::parse(RAW).unwrap());
        assert_eq!(
            serde_json::to_string(&r).unwrap(),
            format!("{{\"id\":\"{RAW}\"}}")
        );
    }

    #[test]
    fn ref_reads_uri() {
        let r: Ref<A> =
            serde_json::from_str(&format!("{{\"id\":\"{RAW}\",\"uri\":\"https://x/y\"}}")).unwrap();
        assert_eq!(r.uri.as_deref(), Some("https://x/y"));
    }

    #[test]
    fn rejects_garbage() {
        assert!(Id::<A>::parse("nope").is_err());
    }
}
