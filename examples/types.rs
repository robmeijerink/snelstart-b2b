//! The validated types, and how reading differs from writing.
//!
//! Needs no keys and sends nothing.
//!
//! `cargo run --example types`

use snelstart_b2b::models::relaties::Relatie;
use snelstart_b2b::types::{
    Bic, BoundedText, BtwNummer, DateTime, Email, Iban, Id, Landcode, Relatiesoort,
};

fn main() {
    let id: Id<Relatie> = Id::parse("3f2504e0-4f89-41d3-9a0c-0305e82c3301").unwrap();
    println!("{id}");

    println!("{:?}", Iban::new("NL91ABNA0417164300"));
    println!("{:?}", Iban::new("NL00ABNA0417164300"));
    println!("{:?}", Bic::new("ABNANL2A"));
    println!("{:?}", BtwNummer::new("NL123456789B01"));
    println!("{:?}", Landcode::new("NL"));
    println!("{:?}", Email::new("not an email address"));
    println!("{:?}", BoundedText::<10>::new("far too long for ten"));

    let date: DateTime = "2024-01-01".parse().unwrap();
    println!("{date}");

    // A value SnelStart already stores is read as it is, validity can be checked afterwards.
    let stored: Email = serde_json::from_str("\"not an email address\"").unwrap();
    println!("{} is_valid: {}", stored, stored.is_valid());

    // Unknown enum values are kept in `Other`.
    let kind: Relatiesoort = serde_json::from_str("\"Nieuw\"").unwrap();
    println!("{kind:?}");
}
