//! Async client for the [SnelStart](https://www.snelstart.nl) B2B API v2.
//!
//! ```no_run
//! use futures_util::TryStreamExt;
//! use snelstart_b2b::odata::{Filter, Query};
//! use snelstart_b2b::{Client, ClientKey, SubscriptionKey};
//!
//! # async fn run() -> snelstart_b2b::Result<()> {
//! let client = Client::new(
//!     ClientKey::new("koppelsleutel")?,
//!     SubscriptionKey::new("subscription-key")?,
//! )?;
//!
//! let query = Query::new().filter(Filter::any_eq("Relatiesoort", "r", "Klant")?);
//! let mut relations = Box::pin(client.relaties().all(query));
//! while let Some(relation) = relations.try_next().await? {
//!     println!("{:?}", relation.naam);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Every collection only offers the calls SnelStart documents for it. Calling
//! `delete` on a collection that has none does not compile:
//!
//! ```compile_fail
//! # async fn run(client: snelstart_b2b::Client) -> snelstart_b2b::Result<()> {
//! # let id = snelstart_b2b::types::Id::parse("3f2504e0-4f89-41d3-9a0c-0305e82c3301").unwrap();
//! client.landen().delete(id).await
//! # }
//! ```
//!
//! Ids are typed by what they identify, so an article id is not accepted where
//! a relation id is expected:
//!
//! ```compile_fail
//! # async fn run(client: snelstart_b2b::Client) -> snelstart_b2b::Result<()> {
//! # use snelstart_b2b::models::artikelen::Artikel;
//! # let artikel: snelstart_b2b::types::Id<Artikel> =
//! #     snelstart_b2b::types::Id::parse("3f2504e0-4f89-41d3-9a0c-0305e82c3301").unwrap();
//! client.relaties().find(artikel).await?;
//! # Ok(())
//! # }
//! ```
//!
//! The same call compiles with the right id:
//!
//! ```no_run
//! # async fn run(client: snelstart_b2b::Client) -> snelstart_b2b::Result<()> {
//! # use snelstart_b2b::models::relaties::Relatie;
//! # let id: snelstart_b2b::types::Id<Relatie> =
//! #     snelstart_b2b::types::Id::parse("3f2504e0-4f89-41d3-9a0c-0305e82c3301").unwrap();
//! client.relaties().find(id).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! - `rustls-tls` (default): TLS through rustls.
//! - `native-tls`: TLS through the platform library.

#![forbid(unsafe_code)]

pub mod auth;
pub mod client;
pub mod endpoints;
pub mod error;
mod error_messages;
pub mod models;
pub mod odata;
mod operations;
pub mod resource;
pub mod types;

pub use auth::{ClientKey, SubscriptionKey};
pub use client::{Client, ClientBuilder};
pub use error::{Error, Result};
