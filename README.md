# snelstart-b2b

Async Rust client for the [SnelStart](https://www.snelstart.nl) B2B API (v2).

> This is an unofficial, community-maintained library. It is not affiliated with, endorsed by or supported by SnelStart.

It covers every collection of the API (relaties, artikelen, verkooporders, boekingen, grootboeken, btw and more), the reports, attachments and the other endpoints that are not a plain collection.

```toml
[dependencies]
snelstart-b2b = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
futures-util = "0.3"
```

## Usage

```rust
use futures_util::TryStreamExt;
use snelstart_b2b::odata::{Filter, Query};
use snelstart_b2b::{Client, ClientKey, SubscriptionKey};

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    let client = Client::new(
        ClientKey::new(std::env::var("SNELSTART_CLIENT_KEY").unwrap())?,
        SubscriptionKey::new(std::env::var("SNELSTART_SUBSCRIPTION_KEY").unwrap())?,
    )?;

    let customers = Query::new().filter(Filter::any_eq("Relatiesoort", "r", "Klant")?);
    let mut relations = Box::pin(client.relaties().all(customers));
    while let Some(relation) = relations.try_next().await? {
        println!("{:?}", relation.naam);
    }
    Ok(())
}
```

You need two keys. The subscription key comes from your profile on the [B2B developer portal](https://b2bapi-developer.snelstart.nl). The client key (*koppelsleutel*) belongs to one administration; create it in SnelStart under *Maatwerk*. The client exchanges the client key for an access token, keeps it until a minute before it expires and fetches a new one when SnelStart rejects it.

`Client::from_env()` reads `SNELSTART_CLIENT_KEY` and `SNELSTART_SUBSCRIPTION_KEY`, and optionally `SNELSTART_BASE_URL`, `SNELSTART_TOKEN_URL`, `SNELSTART_ECHO_URL`, `SNELSTART_TIMEOUT` and `SNELSTART_CONNECT_TIMEOUT` (seconds). `client.with_client_key(key)` gives you the same client for another administration, with its own token.

### Checking your keys

```rust
client.echo().await?;                      // subscription key only, no token
let company = client.company_info().await?; // client key
println!("{:?}", company.administratie_naam);
```

If `echo` works and `company_info` does not, the client key is the problem.

### Reading

```rust
use snelstart_b2b::types::Id;

let id: Id<Relatie> = Id::parse("3f2504e0-4f89-41d3-9a0c-0305e82c3301").unwrap();
let relation = client.relaties().find(id).await?;
println!("{:?} {:?}", relation.relatiecode, relation.naam);

let countries = client.landen().list(&Query::new()).await?;
```

### Creating

```rust
use snelstart_b2b::models::relaties::RelatieWrite;
use snelstart_b2b::types::{Email, Relatiesoort, BoundedText};

let new_relation = RelatieWrite {
    naam: Some(BoundedText::new("Example Installations B.V.")?),
    relatiesoort: vec![Relatiesoort::Klant],
    email: Some(Email::new("info@example.nl")?),
    ..Default::default()
};
let created = client.relaties().create(&new_relation).await?;
```

`BoundedText::new` and `Email::new` reject a value SnelStart would refuse, before anything is sent.

### Updating

`find` returns the read model and `update` takes the write model. Convert the one into the other, change what you need and send it back; SnelStart documents a whole model for this call.

```rust
let relation = client.relaties().find(id).await?;
let mut changes = RelatieWrite::from(relation);
changes.memo = Some("Regular customer".into());
client.relaties().update(id, &changes).await?;
```

### Deleting

```rust
client.kostenplaatsen().delete(id).await?;
```

### Types instead of strings

Ids are typed by what they identify, so passing an artikel id where a relatie id is expected does not compile. Enums such as `Relatiesoort` or `ProcesStatus` replace magic strings, and text fields that SnelStart limits are `BoundedText<N>`. `Iban`, `Bic`, `Email`, `BtwNummer` and `Landcode` are validated when you construct them.

Reading is lenient on purpose. A value that SnelStart already stores is accepted as it is, even when it would not pass validation today; `is_valid()` tells you. Unknown enum values end up in an `Other` variant, unknown fields are ignored and `null` where a list is expected reads as an empty list, so a change in the API does not break a response.

A collection only has the calls SnelStart documents for it:

| Collection | Calls | Paged |
|---|---|---|
| `actieprijzen` | list | yes |
| `artikelen` | list, find, create, update, delete | yes |
| `artikelen/prijsafspraken` | list | yes |
| `artikelomzetgroepen` | list, find | no |
| `bankboekingen` | list, find, create, update, delete | yes |
| `btwaangiftes` | list, find | yes |
| `btwtarieven` | list | no |
| `dagboeken` | list | no |
| `documenten` | find, update, delete | no |
| `grootboeken` | list, find, create | yes |
| `grootboekmutaties` | list, find | yes |
| `inkoopboekingen` | find, create, update, delete | no |
| `inkoopfacturen` | list | yes |
| `kasboekingen` | list, find, create, update, delete | yes |
| `kostenplaatsen` | list, find, create, update, delete | no |
| `landen` | list, find | no |
| `memoriaalboekingen` | find, create, update, delete | no |
| `offertes` | list, find, create, update, delete | yes |
| `prijsafspraken` | list | yes |
| `relaties` | list, find, create, update, delete | yes |
| `vatratedefinitions` | list | yes |
| `vatrates` | list, find | yes |
| `verkoopboekingen` | find, create, update, delete | no |
| `verkoopfacturen` | list, find | yes |
| `verkooporders` | list, find, create, update, delete | yes |
| `verkoopordersjablonen` | list, find | no |

### Querying and paging

`Query` takes `$filter`, `$orderby`, `$top` and `$skip`. Field names and values are checked when you build the filter, strings are quoted and escaped, and `Top` is limited to the 500 rows SnelStart returns per call.

```rust
use snelstart_b2b::odata::{Direction, Filter, Op, Top};

let query = Query::new()
    .filter(
        Filter::compare("openstaandSaldo", Op::Gt, 0)?
            .and(Filter::contains("factuurnummer", "2024")?),
    )
    .order_by("factuurdatum", Direction::Desc)?
    .top(Top::new(50)?);

let first_fifty = client.verkoopfacturen().list(&query).await?;
```

Ids and dates are written as OData v4 literals, which is the standard the API documentation points to. `prijsafspraken` needs a filter on `Artikel/Id`: `Filter::eq("Artikel/Id", artikel_id)`.

`list` is one call. `all` returns a stream that fetches page after page while you iterate, and stops fetching when you stop:

```rust
let mut invoices = Box::pin(client.verkoopfacturen().all(Query::new()));
while let Some(invoice) = invoices.try_next().await? {
    println!("{:?}", invoice.factuurnummer);
}
```

A collection that ignores `$skip` and returns the same page twice ends the stream with `Error::PagingStalled` instead of looping.

### Other endpoints

Calls that are not a collection are methods on `Client`:

```rust
use snelstart_b2b::types::DateTime;

let start: DateTime = "2024-01-01".parse()?;
let end: DateTime = "2024-12-31".parse()?;
let balance = client.get_periodebalans(start, end).await?;

let bookings = client.list_relatie_verkoopboekingen(relation_id).await?;
```

Others are `company_info`, `get_relatie_custom_fields`, `get_kolommenbalans`, `set_verkooporder_proces_status`, `get_verkoopfactuur_ubl`, `create_inkoopboeking_from_ubl`, `create_inkoopboeking_from_attachment`, `upload_bankafschriftbestanden` and `create_document`. `client.get`, `post`, `put` and `delete` reach any other path under the base URL.

### Errors

Failed calls return `snelstart_b2b::Error`. A 400 from SnelStart carries its error codes, with the Dutch text SnelStart documents for each:

```rust
match client.relaties().create(&new_relation).await {
    Err(e) if e.has_code("REL-0007") => println!("name is missing"),
    Err(e) => {
        for code in e.error_codes() {
            println!("{code}: {}", code.message().unwrap_or("unknown"));
        }
    }
    Ok(created) => println!("{:?}", created.id),
}
```

`Error::Unauthorized`, `NotFound` and `RateLimited { retry_after }` are separate variants. Rate limited calls are not retried. The client key, the subscription key and the access token are redacted from error bodies and from `Debug` output.

### Timeouts

Requests time out after 30 seconds (10 seconds to connect); change it with `Client::builder(..).timeout(..)`. Redirects are not followed. If you pass your own `reqwest::Client` with `http_client`, its settings apply as is.

## Features

| Feature | Default | |
|---|---|---|
| `rustls-tls` | yes | TLS through rustls |
| `native-tls` | no | TLS through the platform library |

Use `default-features = false, features = ["native-tls"]` to switch TLS backends.

## Testing

```sh
cargo test
```

The unit and integration tests run against a mock server. To exercise the real API:

```sh
SNELSTART_CLIENT_KEY=... SNELSTART_SUBSCRIPTION_KEY=... cargo test --test live -- --ignored
```

The live tests create and delete one kostenplaats in the administration the client key belongs to.

## License

MIT

---

This library is developed by [Solvalutions](https://solvalutions.nl) and [Rob Meijerink](https://robmeijerink.nl).

*&copy;&nbsp;2026 &mdash; Built by Rob Meijerink.*
