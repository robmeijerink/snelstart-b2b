//! Inkoopfacturen and inkoopboekingen, including booking from a UBL file or an attachment.
//!
//! Booking is only sent when `SNELSTART_EXAMPLE_SEND` is set. Pass a UBL file and a PDF:
//!
//! `cargo run --example inkoop -- invoice.xml invoice.pdf`

use snelstart_b2b::Client;
use snelstart_b2b::models::bijlagen::UblContent;
use snelstart_b2b::models::inkoopboekingen::Attachment;
use snelstart_b2b::odata::{Query, Top};
use snelstart_b2b::types::Bytes;

#[tokio::main]
async fn main() -> snelstart_b2b::Result<()> {
    dotenvy::dotenv().ok();
    let client = Client::from_env()?;

    let invoices = client
        .inkoopfacturen()
        .list(&Query::new().top(Top::new(10)?))
        .await?;
    println!("{} inkoopfacturen", invoices.len());

    if let Some(id) = invoices
        .first()
        .and_then(|invoice| invoice.inkoop_boeking.as_ref())
        .map(|booking| booking.id)
    {
        let booking = client.inkoopboekingen().find(id).await?;
        println!("{:?} {:?}", booking.factuurnummer, booking.factuurbedrag);
    }

    let mut args = std::env::args().skip(1);
    let (Some(ubl_path), Some(pdf_path)) = (args.next(), args.next()) else {
        return Ok(());
    };
    let ubl = std::fs::read(&ubl_path).expect("readable ubl file");
    let pdf = std::fs::read(&pdf_path).expect("readable pdf file");

    let from_ubl = UblContent {
        file_name: Some(ubl_path),
        content: Some(Bytes::new(ubl)),
        pdf_content: Some(Bytes::new(pdf.clone())),
        force_persist: Some(false),
    };
    let from_attachment = Attachment {
        file_name: Some(pdf_path),
        content: Some(Bytes::new(pdf)),
    };

    if std::env::var_os("SNELSTART_EXAMPLE_SEND").is_none() {
        println!(
            "would book {:?} and {:?}",
            from_ubl.file_name, from_attachment.file_name
        );
        return Ok(());
    }

    let booking = client.create_inkoopboeking_from_ubl(&from_ubl).await?;
    println!("booked {:?}", booking.id);

    let started = client
        .create_inkoopboeking_from_attachment(&from_attachment)
        .await?;
    if let Some(instance) = started.instance_id {
        let status = client
            .get_inkoopboeking_from_attachment_status(&instance)
            .await?;
        println!("{:?} {:?}", status.status, status.inkoopboeking_id);
    }
    Ok(())
}
