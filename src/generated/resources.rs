// Code generated from the AveeCare OpenAPI specification. DO NOT EDIT.

use futures_util::Stream;
use reqwest::Method;

use super::models::*;
use crate::client::{encode_path, AveeCare, RequestOptions};
use crate::error::Result;
use crate::pagination::{auto_page, Page};

impl AveeCare {
    /// Patients are the clients your agency provides care to.
    pub fn patients(&self) -> Patients<'_> {
        Patients { client: self }
    }
}

/// Patients are the clients your agency provides care to.
#[derive(Debug, Clone, Copy)]
pub struct Patients<'a> {
    client: &'a AveeCare,
}

impl Patients<'_> {
    /// List patients.
    ///
    /// Returns a cursor-paginated list of patients, newest first.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: PatientListParams) -> Result<Page<Patient>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: PatientListParams,
        options: RequestOptions,
    ) -> Result<Page<Patient>> {
        self.client
            .get_page("/v1/patients", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: PatientListParams,
    ) -> impl Stream<Item = Result<Patient>> + Send + 'static {
        let client = self.client.clone();
        auto_page(move |cursor| {
            let client = client.clone();
            let mut params = params.clone();
            if cursor.is_some() {
                params.cursor = cursor;
            }
            async move {
                client
                    .get_page(
                        "/v1/patients",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a patient.
    ///
    /// Creates a new patient. Send an Idempotency-Key header to make retries safe.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: PatientCreate) -> Result<Patient> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: PatientCreate,
        options: RequestOptions,
    ) -> Result<Patient> {
        self.client
            .request::<Patient, _>(
                Method::POST,
                "/v1/patients",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a patient.
    ///
    /// Retrieves the patient with the given ID.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Patient> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Patient> {
        let path = format!("/v1/patients/{}", encode_path(id));
        self.client
            .request::<Patient, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a patient.
    ///
    /// Updates the given fields of a patient. Fields you leave out are unchanged.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: PatientUpdate) -> Result<Patient> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: PatientUpdate,
        options: RequestOptions,
    ) -> Result<Patient> {
        let path = format!("/v1/patients/{}", encode_path(id));
        self.client
            .request::<Patient, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a patient.
    ///
    /// Permanently deletes a patient.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<PatientDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<PatientDeleted> {
        let path = format!("/v1/patients/{}", encode_path(id));
        self.client
            .request::<PatientDeleted, _>(Method::DELETE, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Visits are scheduled or completed caregiver shifts at a patient home.
    pub fn visits(&self) -> Visits<'_> {
        Visits { client: self }
    }
}

/// Visits are scheduled or completed caregiver shifts at a patient home.
#[derive(Debug, Clone, Copy)]
pub struct Visits<'a> {
    client: &'a AveeCare,
}

impl Visits<'_> {
    /// List visits.
    ///
    /// Returns visits whose start time falls in the given window. `start_from` and `start_to` are
    /// required.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: VisitListParams) -> Result<Page<Visit>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: VisitListParams,
        options: RequestOptions,
    ) -> Result<Page<Visit>> {
        self.client
            .get_page("/v1/visits", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: VisitListParams,
    ) -> impl Stream<Item = Result<Visit>> + Send + 'static {
        let client = self.client.clone();
        auto_page(move |cursor| {
            let client = client.clone();
            let mut params = params.clone();
            if cursor.is_some() {
                params.cursor = cursor;
            }
            async move {
                client
                    .get_page("/v1/visits", params.to_query(), &RequestOptions::default())
                    .await
            }
        })
    }

    /// Create a visit.
    ///
    /// Creates a new visit. Send an Idempotency-Key header to make retries safe.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: VisitCreate) -> Result<Visit> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: VisitCreate,
        options: RequestOptions,
    ) -> Result<Visit> {
        self.client
            .request::<Visit, _>(
                Method::POST,
                "/v1/visits",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a visit.
    ///
    /// Retrieves the visit with the given ID.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Visit> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Visit> {
        let path = format!("/v1/visits/{}", encode_path(id));
        self.client
            .request::<Visit, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a visit.
    ///
    /// Updates the given fields of a visit. Fields you leave out are unchanged.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: VisitUpdate) -> Result<Visit> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: VisitUpdate,
        options: RequestOptions,
    ) -> Result<Visit> {
        let path = format!("/v1/visits/{}", encode_path(id));
        self.client
            .request::<Visit, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a visit.
    ///
    /// Permanently deletes a visit.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<VisitDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<VisitDeleted> {
        let path = format!("/v1/visits/{}", encode_path(id));
        self.client
            .request::<VisitDeleted, _>(Method::DELETE, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Invoices billed to patients and payers. Read-only.
    pub fn invoices(&self) -> Invoices<'_> {
        Invoices { client: self }
    }
}

/// Invoices billed to patients and payers. Read-only.
#[derive(Debug, Clone, Copy)]
pub struct Invoices<'a> {
    client: &'a AveeCare,
}

impl Invoices<'_> {
    /// List invoices.
    ///
    /// Returns a cursor-paginated list of invoices, newest first.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: InvoiceListParams) -> Result<Page<Invoice>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: InvoiceListParams,
        options: RequestOptions,
    ) -> Result<Page<Invoice>> {
        self.client
            .get_page("/v1/invoices", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: InvoiceListParams,
    ) -> impl Stream<Item = Result<Invoice>> + Send + 'static {
        let client = self.client.clone();
        auto_page(move |cursor| {
            let client = client.clone();
            let mut params = params.clone();
            if cursor.is_some() {
                params.cursor = cursor;
            }
            async move {
                client
                    .get_page(
                        "/v1/invoices",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Retrieve a invoice.
    ///
    /// Retrieves the invoice with the given ID.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Invoice> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Invoice> {
        let path = format!("/v1/invoices/{}", encode_path(id));
        self.client
            .request::<Invoice, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Information about the API key making the request.
    pub fn me(&self) -> MeResource<'_> {
        MeResource { client: self }
    }
}

/// Information about the API key making the request.
#[derive(Debug, Clone, Copy)]
pub struct MeResource<'a> {
    client: &'a AveeCare,
}

impl MeResource<'_> {
    /// Retrieve the current API key.
    ///
    /// Returns the API key making the request, its scopes and its organization.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self) -> Result<Me> {
        self.retrieve_with_options(RequestOptions::default()).await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(&self, options: RequestOptions) -> Result<Me> {
        self.client
            .request::<Me, _>(Method::GET, "/v1/me", &[], None::<&()>, false, &options)
            .await
    }
}
