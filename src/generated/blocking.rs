// Code generated from the AveeCare OpenAPI specification. DO NOT EDIT.

use super::models::*;
use crate::blocking::{AveeCare, BlockingIter};
use crate::client::RequestOptions;
use crate::error::Result;
use crate::pagination::Page;

impl AveeCare {
    /// Patients are the clients your agency provides care to.
    pub fn patients(&self) -> Patients<'_> {
        Patients { client: self }
    }
}

/// Patients are the clients your agency provides care to. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Patients<'a> {
    client: &'a AveeCare,
}

impl Patients<'_> {
    /// List patients.
    ///
    /// Returns a cursor-paginated list of patients, newest first.
    pub fn list(&self, params: PatientListParams) -> Result<Page<Patient>> {
        self.client
            .block_on(self.client.inner.patients().list(params))
    }

    pub fn list_with_options(
        &self,
        params: PatientListParams,
        options: RequestOptions,
    ) -> Result<Page<Patient>> {
        self.client.block_on(
            self.client
                .inner
                .patients()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: PatientListParams) -> BlockingIter<Patient> {
        self.client
            .iter(self.client.inner.patients().list_auto_paging(params))
    }

    /// Create a patient.
    ///
    /// Creates a new patient. Send an Idempotency-Key header to make retries safe.
    pub fn create(&self, params: PatientCreate) -> Result<Patient> {
        self.client
            .block_on(self.client.inner.patients().create(params))
    }

    pub fn create_with_options(
        &self,
        params: PatientCreate,
        options: RequestOptions,
    ) -> Result<Patient> {
        self.client.block_on(
            self.client
                .inner
                .patients()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a patient.
    ///
    /// Retrieves the patient with the given ID.
    pub fn retrieve(&self, id: &str) -> Result<Patient> {
        self.client
            .block_on(self.client.inner.patients().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Patient> {
        self.client.block_on(
            self.client
                .inner
                .patients()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a patient.
    ///
    /// Updates the given fields of a patient. Fields you leave out are unchanged.
    pub fn update(&self, id: &str, params: PatientUpdate) -> Result<Patient> {
        self.client
            .block_on(self.client.inner.patients().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: PatientUpdate,
        options: RequestOptions,
    ) -> Result<Patient> {
        self.client.block_on(
            self.client
                .inner
                .patients()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a patient.
    ///
    /// Permanently deletes a patient.
    pub fn delete(&self, id: &str) -> Result<PatientDeleted> {
        self.client
            .block_on(self.client.inner.patients().delete(id))
    }

    pub fn delete_with_options(&self, id: &str, options: RequestOptions) -> Result<PatientDeleted> {
        self.client.block_on(
            self.client
                .inner
                .patients()
                .delete_with_options(id, options),
        )
    }
}

impl AveeCare {
    /// Visits are scheduled or completed caregiver shifts at a patient home.
    pub fn visits(&self) -> Visits<'_> {
        Visits { client: self }
    }
}

/// Visits are scheduled or completed caregiver shifts at a patient home. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Visits<'a> {
    client: &'a AveeCare,
}

impl Visits<'_> {
    /// List visits.
    ///
    /// Returns visits whose start time falls in the given window. `start_from` and `start_to` are
    /// required.
    pub fn list(&self, params: VisitListParams) -> Result<Page<Visit>> {
        self.client
            .block_on(self.client.inner.visits().list(params))
    }

    pub fn list_with_options(
        &self,
        params: VisitListParams,
        options: RequestOptions,
    ) -> Result<Page<Visit>> {
        self.client.block_on(
            self.client
                .inner
                .visits()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: VisitListParams) -> BlockingIter<Visit> {
        self.client
            .iter(self.client.inner.visits().list_auto_paging(params))
    }

    /// Create a visit.
    ///
    /// Creates a new visit. Send an Idempotency-Key header to make retries safe.
    pub fn create(&self, params: VisitCreate) -> Result<Visit> {
        self.client
            .block_on(self.client.inner.visits().create(params))
    }

    pub fn create_with_options(
        &self,
        params: VisitCreate,
        options: RequestOptions,
    ) -> Result<Visit> {
        self.client.block_on(
            self.client
                .inner
                .visits()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a visit.
    ///
    /// Retrieves the visit with the given ID.
    pub fn retrieve(&self, id: &str) -> Result<Visit> {
        self.client
            .block_on(self.client.inner.visits().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Visit> {
        self.client.block_on(
            self.client
                .inner
                .visits()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a visit.
    ///
    /// Updates the given fields of a visit. Fields you leave out are unchanged.
    pub fn update(&self, id: &str, params: VisitUpdate) -> Result<Visit> {
        self.client
            .block_on(self.client.inner.visits().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: VisitUpdate,
        options: RequestOptions,
    ) -> Result<Visit> {
        self.client.block_on(
            self.client
                .inner
                .visits()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a visit.
    ///
    /// Permanently deletes a visit.
    pub fn delete(&self, id: &str) -> Result<VisitDeleted> {
        self.client.block_on(self.client.inner.visits().delete(id))
    }

    pub fn delete_with_options(&self, id: &str, options: RequestOptions) -> Result<VisitDeleted> {
        self.client
            .block_on(self.client.inner.visits().delete_with_options(id, options))
    }
}

impl AveeCare {
    /// Invoices billed to patients and payers. Read-only.
    pub fn invoices(&self) -> Invoices<'_> {
        Invoices { client: self }
    }
}

/// Invoices billed to patients and payers. Read-only. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Invoices<'a> {
    client: &'a AveeCare,
}

impl Invoices<'_> {
    /// List invoices.
    ///
    /// Returns a cursor-paginated list of invoices, newest first.
    pub fn list(&self, params: InvoiceListParams) -> Result<Page<Invoice>> {
        self.client
            .block_on(self.client.inner.invoices().list(params))
    }

    pub fn list_with_options(
        &self,
        params: InvoiceListParams,
        options: RequestOptions,
    ) -> Result<Page<Invoice>> {
        self.client.block_on(
            self.client
                .inner
                .invoices()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: InvoiceListParams) -> BlockingIter<Invoice> {
        self.client
            .iter(self.client.inner.invoices().list_auto_paging(params))
    }

    /// Retrieve a invoice.
    ///
    /// Retrieves the invoice with the given ID.
    pub fn retrieve(&self, id: &str) -> Result<Invoice> {
        self.client
            .block_on(self.client.inner.invoices().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Invoice> {
        self.client.block_on(
            self.client
                .inner
                .invoices()
                .retrieve_with_options(id, options),
        )
    }
}

impl AveeCare {
    /// Information about the API key making the request.
    pub fn me(&self) -> MeResource<'_> {
        MeResource { client: self }
    }
}

/// Information about the API key making the request. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct MeResource<'a> {
    client: &'a AveeCare,
}

impl MeResource<'_> {
    /// Retrieve the current API key.
    ///
    /// Returns the API key making the request, its scopes and its organization.
    pub fn retrieve(&self) -> Result<Me> {
        self.client.block_on(self.client.inner.me().retrieve())
    }

    pub fn retrieve_with_options(&self, options: RequestOptions) -> Result<Me> {
        self.client
            .block_on(self.client.inner.me().retrieve_with_options(options))
    }
}
