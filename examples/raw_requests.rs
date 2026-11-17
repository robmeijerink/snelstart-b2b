//! Calls paths the typed API does not cover: `get`, `post`, `put` and `delete`.
//!
//! The last part creates and removes a kostenplaats.
//!
//! `cargo run --example raw_requests`

use serde_json::{Value, json};
use snelstart_b2b::Client;
use snelstart_b2b::odata::Query;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let countries: Value = client.get("landen", &Query::new()).await?;
    println!("{} countries", countries.as_array().map_or(0, Vec::len));

    let created: Value = client
        .post("kostenplaatsen", &json!({ "omschrijving": "Raw example" }))
        .await?;
    let id = created["id"].as_str().expect("SnelStart returns an id");
    let path = format!("kostenplaatsen/{id}");

    let updated: Value = client
        .put(
            &path,
            &json!({ "id": id, "omschrijving": "Raw example, renamed" }),
        )
        .await?;
    println!("{}", updated["omschrijving"]);

    client.delete(&path).await?;
    println!("deleted");
    Ok(())
}
