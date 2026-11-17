//! Reads, uploads, replaces and deletes documents.
//!
//! Uploading, replacing and deleting only happen when `SNELSTART_EXAMPLE_SEND` is set. The
//! document type is `Inkoopboekingen`, `Verkoopboekingen` or `Relaties`, and the parent id is
//! the id of such a booking or relatie.
//!
//! `cargo run --example documenten -- <document type> <parent id> <file>`

use snelstart_b2b::Client;
use snelstart_b2b::models::documenten::DocumentContent;
use snelstart_b2b::types::{Bytes, Id};
use uuid::Uuid;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let args: Vec<String> = std::env::args().skip(1).collect();
    let [document_type, parent, path] = args.as_slice() else {
        println!("usage: documenten <document type> <parent id> <file>");
        return Ok(());
    };
    let parent: Uuid = parent.parse().expect("parent id is a uuid");

    for reference in client.list_documents(document_type, parent).await? {
        println!("{:?} {:?}", reference.id, reference.file_name);
    }

    let document = DocumentContent {
        content: Some(Bytes::new(std::fs::read(path).expect("readable file"))),
        parent_identifier: Some(parent),
        file_name: Some(path.clone()),
        ..Default::default()
    };
    if std::env::var_os("SNELSTART_EXAMPLE_SEND").is_none() {
        println!("would upload {path}");
        return Ok(());
    }

    let identifier = client.create_document(document_type, &document).await?;
    let id: Id<DocumentContent> = Id::from(identifier.id.expect("SnelStart returns an id").uuid());
    println!("uploaded {id:?}");

    let mut stored = client.documenten().find(id).await?;
    stored.file_name = Some("renamed".to_owned());
    client.documenten().update(id, &stored).await?;

    client.documenten().delete(id).await?;
    println!("deleted");
    Ok(())
}
