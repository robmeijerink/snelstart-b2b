//! Uploads bank statement files.
//!
//! Only sent when `SNELSTART_EXAMPLE_SEND` is set.
//!
//! `cargo run --example bankafschriften -- statement.sta`

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use snelstart_b2b::Client;
use snelstart_b2b::models::bankafschrift::BankafschriftBestand;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let Some(path) = std::env::args().nth(1) else {
        println!("pass the path of a bank statement file");
        return Ok(());
    };
    let file = BankafschriftBestand {
        name: Some(path.clone()),
        base64_encoded_content: Some(STANDARD.encode(std::fs::read(&path).expect("readable file"))),
    };

    if std::env::var_os("SNELSTART_EXAMPLE_SEND").is_none() {
        println!("would upload {path}");
        return Ok(());
    }

    for result in client.upload_bankafschriftbestanden(&[file]).await? {
        println!("{:?} success: {:?}", result.name, result.is_success);
        for error in result.errors {
            println!("  {:?}", error.description);
        }
    }
    Ok(())
}
