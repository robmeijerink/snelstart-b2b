use reqwest::Method;
use serde_json::Value;
use uuid::Uuid;

use crate::client::Client;
use crate::error::Result;
use crate::models::{
    artikelen::{Artikel, ArtikelQuery},
    authorization::AdministrationAccess,
    bankafschrift::{BankafschriftBestand, BankafschriftBestandResponse},
    bijlagen::{UblContent, VerkoopBoekingBijlageReference},
    btw_aangiftes::{BtwAangifte, UpdateBtwAangifteStatus},
    company_info::CompanyInfo,
    custom_fields::{RelatieCustomFields, RelatieUpdatedCustomFields},
    documenten::{DocumentContent, DocumentIdentifier},
    inkoopboekingen::{
        Attachment, CreateFromAttachment, CreateFromAttachmentStatus, Inkoopboeking,
    },
    rapportage::{KolommenbalansRegel, PeriodebalansRegel},
    relaties::{DoorlopendeIncassoMachtiging, Relatie},
    verkoop_boekingen::VerkoopBoeking,
    verkoopfacturen::Verkoopfactuur,
    verkooporders::{VerkoopOrder, VerkooporderUpdateProcesStatus},
};
use crate::odata::Query;
use crate::types::{DateTime, Id};

impl Client {
    pub async fn update_company_info(&self, info: &CompanyInfo) -> Result<CompanyInfo> {
        self.put("companyInfo", info).await
    }

    pub async fn has_user_access(&self, user: Uuid) -> Result<AdministrationAccess> {
        self.request(
            Method::GET,
            "authorization/HasUserAccessToAdministration",
            vec![("userIdentifier", user.to_string())],
            None,
        )
        .await
    }

    pub async fn get_artikel_with_price(
        &self,
        id: Id<Artikel>,
        relation: Option<Id<Relatie>>,
        quantity: Option<i32>,
    ) -> Result<ArtikelQuery> {
        let mut pairs = Vec::new();
        if let Some(relation) = relation {
            pairs.push(("relatieId", relation.uuid().to_string()));
        }
        if let Some(quantity) = quantity {
            pairs.push(("aantal", quantity.to_string()));
        }
        self.request(
            Method::GET,
            &format!("artikelen/{}", id.uuid()),
            pairs,
            None,
        )
        .await
    }

    pub async fn get_relatie_custom_fields(&self, id: Id<Relatie>) -> Result<RelatieCustomFields> {
        self.get(
            &format!("relaties/{}/customFields", id.uuid()),
            &Query::new(),
        )
        .await
    }

    pub async fn update_relatie_custom_fields(
        &self,
        id: Id<Relatie>,
        fields: &RelatieUpdatedCustomFields,
    ) -> Result<RelatieCustomFields> {
        self.put(&format!("relaties/{}/customFields", id.uuid()), fields)
            .await
    }

    pub async fn list_relatie_incassomachtigingen(
        &self,
        id: Id<Relatie>,
    ) -> Result<Vec<DoorlopendeIncassoMachtiging>> {
        self.get(
            &format!("relaties/{}/doorlopendeincassomachtigingen", id.uuid()),
            &Query::new(),
        )
        .await
    }

    pub async fn list_relatie_inkoopboekingen(
        &self,
        id: Id<Relatie>,
    ) -> Result<Vec<Inkoopboeking>> {
        self.get(
            &format!("relaties/{}/inkoopboekingen", id.uuid()),
            &Query::new(),
        )
        .await
    }

    pub async fn list_relatie_verkoopboekingen(
        &self,
        id: Id<Relatie>,
    ) -> Result<Vec<VerkoopBoeking>> {
        self.get(
            &format!("relaties/{}/verkoopboekingen", id.uuid()),
            &Query::new(),
        )
        .await
    }

    pub async fn get_kolommenbalans(
        &self,
        start: DateTime,
        end: DateTime,
    ) -> Result<Vec<KolommenbalansRegel>> {
        self.request(
            Method::GET,
            "rapportages/kolommenbalans",
            period(start, end),
            None,
        )
        .await
    }

    pub async fn get_periodebalans(
        &self,
        start: DateTime,
        end: DateTime,
    ) -> Result<Vec<PeriodebalansRegel>> {
        self.request(
            Method::GET,
            "rapportages/periodebalans",
            period(start, end),
            None,
        )
        .await
    }

    pub async fn set_verkooporder_proces_status(
        &self,
        id: Id<VerkoopOrder>,
        status: &VerkooporderUpdateProcesStatus,
    ) -> Result<Value> {
        self.put(&format!("verkooporders/{}/ProcesStatus", id.uuid()), status)
            .await
    }

    pub async fn get_verkoopfactuur_ubl(&self, id: Id<Verkoopfactuur>) -> Result<Value> {
        self.get(&format!("verkoopfacturen/{}/ubl", id.uuid()), &Query::new())
            .await
    }

    pub async fn create_inkoopboeking_from_ubl(&self, ubl: &UblContent) -> Result<Inkoopboeking> {
        self.post("inkoopboekingen/ubl", ubl).await
    }

    pub async fn create_inkoopboeking_from_attachment(
        &self,
        attachment: &Attachment,
    ) -> Result<CreateFromAttachment> {
        self.post("inkoopboekingen/CreateFromAttachment", attachment)
            .await
    }

    pub async fn get_inkoopboeking_from_attachment_status(
        &self,
        instance_id: &str,
    ) -> Result<CreateFromAttachmentStatus> {
        self.request(
            Method::GET,
            "inkoopboekingen/GetCreateFromAttachmentStatus",
            vec![("instanceId", instance_id.to_owned())],
            None,
        )
        .await
    }

    pub async fn update_btwaangifte_status(
        &self,
        id: Id<BtwAangifte>,
        status: &UpdateBtwAangifteStatus,
    ) -> Result<()> {
        self.put(
            &format!("btwaangiftes/{}/externAangeven", id.uuid()),
            status,
        )
        .await
    }

    pub async fn upload_bankafschriftbestanden(
        &self,
        files: &[BankafschriftBestand],
    ) -> Result<Vec<BankafschriftBestandResponse>> {
        self.post("bankafschriftbestanden", files).await
    }

    pub async fn list_documents(
        &self,
        document_type: &str,
        parent: Uuid,
    ) -> Result<Vec<VerkoopBoekingBijlageReference>> {
        self.get(
            &format!("documenten/{document_type}/{parent}"),
            &Query::new(),
        )
        .await
    }

    pub async fn create_document(
        &self,
        document_type: &str,
        document: &DocumentContent,
    ) -> Result<DocumentIdentifier> {
        self.post(&format!("documenten/{document_type}"), document)
            .await
    }
}

fn period(start: DateTime, end: DateTime) -> Vec<(&'static str, String)> {
    vec![("start", start.to_string()), ("end", end.to_string())]
}
