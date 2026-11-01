mod bic;
mod bounded_text;
mod btw_nummer;
mod bytes;
mod datetime;
mod email;
mod enums;
mod iban;
mod id;
mod landcode;

pub use bic::Bic;
pub use bounded_text::BoundedText;
pub use btw_nummer::BtwNummer;
pub use bytes::Bytes;
pub use datetime::DateTime;
pub use email::Email;
pub use enums::*;
pub use iban::Iban;
pub use id::{Id, Ref};
pub use landcode::Landcode;

use serde::{Deserialize, Deserializer};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("invalid {kind}: {reason}")]
pub struct ValidationError {
    kind: &'static str,
    reason: &'static str,
}

impl ValidationError {
    pub(crate) fn new(kind: &'static str, reason: &'static str) -> Self {
        Self { kind, reason }
    }
}

macro_rules! string_newtype {
    ($name:ident) => {
        impl $name {
            pub fn is_valid(&self) -> bool {
                Self::new(&self.0).is_ok()
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                String::deserialize(d).map(Self)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::str::FromStr for $name {
            type Err = $crate::types::ValidationError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::new(s)
            }
        }

        impl TryFrom<String> for $name {
            type Error = $crate::types::ValidationError;

            fn try_from(s: String) -> Result<Self, Self::Error> {
                Self::new(&s)
            }
        }

        impl From<$name> for String {
            fn from(v: $name) -> String {
                v.0
            }
        }
    };
}

pub(crate) use string_newtype;

pub(crate) fn null_as_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Option::<T>::deserialize(deserializer).map(Option::unwrap_or_default)
}
