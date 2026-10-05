// Code generated from the AveeCare OpenAPI specification. DO NOT EDIT.

use futures_util::Stream;
use reqwest::Method;

use super::models::*;
use crate::client::{encode_path, AveeCare, RequestOptions};
use crate::error::Result;
use crate::pagination::{auto_page, Page};

impl AveeCare {
    /// Patients are the clients your agency provides care to, including prospective clients you
    /// have not yet admitted.
    pub fn patients(&self) -> Patients<'_> {
        Patients { client: self }
    }
}

/// Patients are the clients your agency provides care to, including prospective clients you have
/// not yet admitted.
#[derive(Debug, Clone, Copy)]
pub struct Patients<'a> {
    client: &'a AveeCare,
}

impl Patients<'_> {
    /// List patients.
    ///
    /// Returns a page of patients. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
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
    /// Creates a patient and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
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
    /// Returns one patient by ID.
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
    /// Changes the fields you send and returns the updated patient. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
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
    /// Deletes a patient. The patient's visits, notes and other related records are not deleted.
    /// Available to keys belonging to office staff. Requires a key with `write` scope.
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
    /// Contacts are family members and other people connected to a patient, such as an emergency
    /// contact.
    pub fn contacts(&self) -> Contacts<'_> {
        Contacts { client: self }
    }
}

/// Contacts are family members and other people connected to a patient, such as an emergency
/// contact.
#[derive(Debug, Clone, Copy)]
pub struct Contacts<'a> {
    client: &'a AveeCare,
}

impl Contacts<'_> {
    /// List contacts.
    ///
    /// Returns a page of contacts. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: ContactListParams) -> Result<Page<Contact>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: ContactListParams,
        options: RequestOptions,
    ) -> Result<Page<Contact>> {
        self.client
            .get_page("/v1/contacts", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: ContactListParams,
    ) -> impl Stream<Item = Result<Contact>> + Send + 'static {
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
                        "/v1/contacts",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a contact.
    ///
    /// Creates a contact and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: ContactCreate) -> Result<Contact> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: ContactCreate,
        options: RequestOptions,
    ) -> Result<Contact> {
        self.client
            .request::<Contact, _>(
                Method::POST,
                "/v1/contacts",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a contact.
    ///
    /// Returns one contact by ID.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Contact> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Contact> {
        let path = format!("/v1/contacts/{}", encode_path(id));
        self.client
            .request::<Contact, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a contact.
    ///
    /// Changes the fields you send and returns the updated contact. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: ContactUpdate) -> Result<Contact> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: ContactUpdate,
        options: RequestOptions,
    ) -> Result<Contact> {
        let path = format!("/v1/contacts/{}", encode_path(id));
        self.client
            .request::<Contact, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a contact.
    ///
    /// Deletes a contact. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<ContactDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ContactDeleted> {
        let path = format!("/v1/contacts/{}", encode_path(id));
        self.client
            .request::<ContactDeleted, _>(Method::DELETE, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Allergies recorded on a patient's profile.
    pub fn allergies(&self) -> Allergies<'_> {
        Allergies { client: self }
    }
}

/// Allergies recorded on a patient's profile.
#[derive(Debug, Clone, Copy)]
pub struct Allergies<'a> {
    client: &'a AveeCare,
}

impl Allergies<'_> {
    /// List allergies.
    ///
    /// Returns a page of allergies. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: AllergyListParams) -> Result<Page<Allergy>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: AllergyListParams,
        options: RequestOptions,
    ) -> Result<Page<Allergy>> {
        self.client
            .get_page("/v1/allergies", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: AllergyListParams,
    ) -> impl Stream<Item = Result<Allergy>> + Send + 'static {
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
                        "/v1/allergies",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create an allergy.
    ///
    /// Creates an allergy and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: AllergyCreate) -> Result<Allergy> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: AllergyCreate,
        options: RequestOptions,
    ) -> Result<Allergy> {
        self.client
            .request::<Allergy, _>(
                Method::POST,
                "/v1/allergies",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve an allergy.
    ///
    /// Returns one allergy by ID.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Allergy> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Allergy> {
        let path = format!("/v1/allergies/{}", encode_path(id));
        self.client
            .request::<Allergy, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update an allergy.
    ///
    /// Changes the fields you send and returns the updated allergy. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: AllergyUpdate) -> Result<Allergy> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: AllergyUpdate,
        options: RequestOptions,
    ) -> Result<Allergy> {
        let path = format!("/v1/allergies/{}", encode_path(id));
        self.client
            .request::<Allergy, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete an allergy.
    ///
    /// Deletes an allergy. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<AllergyDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<AllergyDeleted> {
        let path = format!("/v1/allergies/{}", encode_path(id));
        self.client
            .request::<AllergyDeleted, _>(Method::DELETE, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Medications on a patient's medication list, with the schedule caregivers follow when they
    /// give or assist with them.
    pub fn medications(&self) -> Medications<'_> {
        Medications { client: self }
    }
}

/// Medications on a patient's medication list, with the schedule caregivers follow when they give
/// or assist with them.
#[derive(Debug, Clone, Copy)]
pub struct Medications<'a> {
    client: &'a AveeCare,
}

impl Medications<'_> {
    /// List medications.
    ///
    /// Returns a page of medications. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: MedicationListParams) -> Result<Page<Medication>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: MedicationListParams,
        options: RequestOptions,
    ) -> Result<Page<Medication>> {
        self.client
            .get_page("/v1/medications", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: MedicationListParams,
    ) -> impl Stream<Item = Result<Medication>> + Send + 'static {
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
                        "/v1/medications",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a medication.
    ///
    /// Creates a medication and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: MedicationCreate) -> Result<Medication> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: MedicationCreate,
        options: RequestOptions,
    ) -> Result<Medication> {
        self.client
            .request::<Medication, _>(
                Method::POST,
                "/v1/medications",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a medication.
    ///
    /// Returns one medication by ID.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Medication> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Medication> {
        let path = format!("/v1/medications/{}", encode_path(id));
        self.client
            .request::<Medication, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a medication.
    ///
    /// Changes the fields you send and returns the updated medication. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: MedicationUpdate) -> Result<Medication> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: MedicationUpdate,
        options: RequestOptions,
    ) -> Result<Medication> {
        let path = format!("/v1/medications/{}", encode_path(id));
        self.client
            .request::<Medication, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a medication.
    ///
    /// Deletes a medication. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<MedicationDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<MedicationDeleted> {
        let path = format!("/v1/medications/{}", encode_path(id));
        self.client
            .request::<MedicationDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// Goals of care set for a patient, such as walking to the mailbox unaided.
    pub fn care_goals(&self) -> CareGoals<'_> {
        CareGoals { client: self }
    }
}

/// Goals of care set for a patient, such as walking to the mailbox unaided.
#[derive(Debug, Clone, Copy)]
pub struct CareGoals<'a> {
    client: &'a AveeCare,
}

impl CareGoals<'_> {
    /// List care goals.
    ///
    /// Returns a page of care goals. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: CareGoalListParams) -> Result<Page<CareGoal>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: CareGoalListParams,
        options: RequestOptions,
    ) -> Result<Page<CareGoal>> {
        self.client
            .get_page("/v1/care-goals", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: CareGoalListParams,
    ) -> impl Stream<Item = Result<CareGoal>> + Send + 'static {
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
                        "/v1/care-goals",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a care goal.
    ///
    /// Creates a care goal and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: CareGoalCreate) -> Result<CareGoal> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: CareGoalCreate,
        options: RequestOptions,
    ) -> Result<CareGoal> {
        self.client
            .request::<CareGoal, _>(
                Method::POST,
                "/v1/care-goals",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a care goal.
    ///
    /// Returns one care goal by ID.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<CareGoal> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CareGoal> {
        let path = format!("/v1/care-goals/{}", encode_path(id));
        self.client
            .request::<CareGoal, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a care goal.
    ///
    /// Changes the fields you send and returns the updated care goal. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: CareGoalUpdate) -> Result<CareGoal> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: CareGoalUpdate,
        options: RequestOptions,
    ) -> Result<CareGoal> {
        let path = format!("/v1/care-goals/{}", encode_path(id));
        self.client
            .request::<CareGoal, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a care goal.
    ///
    /// Deletes a care goal. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<CareGoalDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CareGoalDeleted> {
        let path = format!("/v1/care-goals/{}", encode_path(id));
        self.client
            .request::<CareGoalDeleted, _>(Method::DELETE, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Notes your staff and caregivers keep on a patient's profile.
    pub fn patient_notes(&self) -> PatientNotes<'_> {
        PatientNotes { client: self }
    }
}

/// Notes your staff and caregivers keep on a patient's profile.
#[derive(Debug, Clone, Copy)]
pub struct PatientNotes<'a> {
    client: &'a AveeCare,
}

impl PatientNotes<'_> {
    /// List patient notes.
    ///
    /// Returns a page of patient notes. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: PatientNoteListParams) -> Result<Page<PatientNote>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: PatientNoteListParams,
        options: RequestOptions,
    ) -> Result<Page<PatientNote>> {
        self.client
            .get_page("/v1/patient-notes", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: PatientNoteListParams,
    ) -> impl Stream<Item = Result<PatientNote>> + Send + 'static {
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
                        "/v1/patient-notes",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a patient note.
    ///
    /// Creates a patient note and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: PatientNoteCreate) -> Result<PatientNote> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: PatientNoteCreate,
        options: RequestOptions,
    ) -> Result<PatientNote> {
        self.client
            .request::<PatientNote, _>(
                Method::POST,
                "/v1/patient-notes",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a patient note.
    ///
    /// Returns one patient note by ID. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<PatientNote> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<PatientNote> {
        let path = format!("/v1/patient-notes/{}", encode_path(id));
        self.client
            .request::<PatientNote, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a patient note.
    ///
    /// Changes the fields you send and returns the updated patient note. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: PatientNoteUpdate) -> Result<PatientNote> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: PatientNoteUpdate,
        options: RequestOptions,
    ) -> Result<PatientNote> {
        let path = format!("/v1/patient-notes/{}", encode_path(id));
        self.client
            .request::<PatientNote, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a patient note.
    ///
    /// Deletes a patient note. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<PatientNoteDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<PatientNoteDeleted> {
        let path = format!("/v1/patient-notes/{}", encode_path(id));
        self.client
            .request::<PatientNoteDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// The medication administration record (MAR): each dose a caregiver gave, or the reason it was
    /// not given. Read-only.
    pub fn medication_administrations(&self) -> MedicationAdministrations<'_> {
        MedicationAdministrations { client: self }
    }
}

/// The medication administration record (MAR): each dose a caregiver gave, or the reason it was not
/// given. Read-only.
#[derive(Debug, Clone, Copy)]
pub struct MedicationAdministrations<'a> {
    client: &'a AveeCare,
}

impl MedicationAdministrations<'_> {
    /// List medication administrations.
    ///
    /// Returns a page of medication administrations. Filters combine with AND. Use `next_cursor` to
    /// fetch the next page.
    ///
    /// Requires the `read` scope.
    pub async fn list(
        &self,
        params: MedicationAdministrationListParams,
    ) -> Result<Page<MedicationAdministration>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: MedicationAdministrationListParams,
        options: RequestOptions,
    ) -> Result<Page<MedicationAdministration>> {
        self.client
            .get_page(
                "/v1/medication-administrations",
                params.to_query(),
                &options,
            )
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: MedicationAdministrationListParams,
    ) -> impl Stream<Item = Result<MedicationAdministration>> + Send + 'static {
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
                        "/v1/medication-administrations",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Retrieve a medication administration.
    ///
    /// Returns one medication administration by ID.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<MedicationAdministration> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<MedicationAdministration> {
        let path = format!("/v1/medication-administrations/{}", encode_path(id));
        self.client
            .request::<MedicationAdministration, _>(
                Method::GET,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// OASIS assessments recorded for Medicare home health patients. This lists each assessment and
    /// its status; the item-level answers are not included. Read-only.
    pub fn oasis_assessments(&self) -> OasisAssessments<'_> {
        OasisAssessments { client: self }
    }
}

/// OASIS assessments recorded for Medicare home health patients. This lists each assessment and its
/// status; the item-level answers are not included. Read-only.
#[derive(Debug, Clone, Copy)]
pub struct OasisAssessments<'a> {
    client: &'a AveeCare,
}

impl OasisAssessments<'_> {
    /// List OASIS assessments.
    ///
    /// Returns a page of OASIS assessments. Filters combine with AND. Use `next_cursor` to fetch
    /// the next page. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: OasisAssessmentListParams) -> Result<Page<OasisAssessment>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: OasisAssessmentListParams,
        options: RequestOptions,
    ) -> Result<Page<OasisAssessment>> {
        self.client
            .get_page("/v1/oasis-assessments", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: OasisAssessmentListParams,
    ) -> impl Stream<Item = Result<OasisAssessment>> + Send + 'static {
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
                        "/v1/oasis-assessments",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Retrieve an OASIS assessment.
    ///
    /// Returns one OASIS assessment by ID. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<OasisAssessment> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<OasisAssessment> {
        let path = format!("/v1/oasis-assessments/{}", encode_path(id));
        self.client
            .request::<OasisAssessment, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Caregivers are the aides, nurses and therapists your agency schedules to visit patients.
    pub fn caregivers(&self) -> Caregivers<'_> {
        Caregivers { client: self }
    }
}

/// Caregivers are the aides, nurses and therapists your agency schedules to visit patients.
#[derive(Debug, Clone, Copy)]
pub struct Caregivers<'a> {
    client: &'a AveeCare,
}

impl Caregivers<'_> {
    /// List caregivers.
    ///
    /// Returns a page of caregivers. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: CaregiverListParams) -> Result<Page<Caregiver>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: CaregiverListParams,
        options: RequestOptions,
    ) -> Result<Page<Caregiver>> {
        self.client
            .get_page("/v1/caregivers", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: CaregiverListParams,
    ) -> impl Stream<Item = Result<Caregiver>> + Send + 'static {
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
                        "/v1/caregivers",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a caregiver.
    ///
    /// Creates a caregiver and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: CaregiverCreate) -> Result<Caregiver> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: CaregiverCreate,
        options: RequestOptions,
    ) -> Result<Caregiver> {
        self.client
            .request::<Caregiver, _>(
                Method::POST,
                "/v1/caregivers",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a caregiver.
    ///
    /// Returns one caregiver by ID. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Caregiver> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Caregiver> {
        let path = format!("/v1/caregivers/{}", encode_path(id));
        self.client
            .request::<Caregiver, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a caregiver.
    ///
    /// Changes the fields you send and returns the updated caregiver. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: CaregiverUpdate) -> Result<Caregiver> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: CaregiverUpdate,
        options: RequestOptions,
    ) -> Result<Caregiver> {
        let path = format!("/v1/caregivers/{}", encode_path(id));
        self.client
            .request::<Caregiver, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a caregiver.
    ///
    /// Deletes a caregiver. The caregiver's past visits and other related records are not deleted.
    /// Available to keys belonging to office staff. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<CaregiverDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CaregiverDeleted> {
        let path = format!("/v1/caregivers/{}", encode_path(id));
        self.client
            .request::<CaregiverDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// Notes your staff keep on a caregiver's profile.
    pub fn caregiver_notes(&self) -> CaregiverNotes<'_> {
        CaregiverNotes { client: self }
    }
}

/// Notes your staff keep on a caregiver's profile.
#[derive(Debug, Clone, Copy)]
pub struct CaregiverNotes<'a> {
    client: &'a AveeCare,
}

impl CaregiverNotes<'_> {
    /// List caregiver notes.
    ///
    /// Returns a page of caregiver notes. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: CaregiverNoteListParams) -> Result<Page<CaregiverNote>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: CaregiverNoteListParams,
        options: RequestOptions,
    ) -> Result<Page<CaregiverNote>> {
        self.client
            .get_page("/v1/caregiver-notes", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: CaregiverNoteListParams,
    ) -> impl Stream<Item = Result<CaregiverNote>> + Send + 'static {
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
                        "/v1/caregiver-notes",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a caregiver note.
    ///
    /// Creates a caregiver note and returns it. Send an `Idempotency-Key` header to make the
    /// request safe to retry. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: CaregiverNoteCreate) -> Result<CaregiverNote> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: CaregiverNoteCreate,
        options: RequestOptions,
    ) -> Result<CaregiverNote> {
        self.client
            .request::<CaregiverNote, _>(
                Method::POST,
                "/v1/caregiver-notes",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a caregiver note.
    ///
    /// Returns one caregiver note by ID. Available to keys belonging to office staff and
    /// caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<CaregiverNote> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CaregiverNote> {
        let path = format!("/v1/caregiver-notes/{}", encode_path(id));
        self.client
            .request::<CaregiverNote, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a caregiver note.
    ///
    /// Changes the fields you send and returns the updated caregiver note. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: CaregiverNoteUpdate) -> Result<CaregiverNote> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: CaregiverNoteUpdate,
        options: RequestOptions,
    ) -> Result<CaregiverNote> {
        let path = format!("/v1/caregiver-notes/{}", encode_path(id));
        self.client
            .request::<CaregiverNote, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a caregiver note.
    ///
    /// Deletes a caregiver note. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<CaregiverNoteDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CaregiverNoteDeleted> {
        let path = format!("/v1/caregiver-notes/{}", encode_path(id));
        self.client
            .request::<CaregiverNoteDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// Certifications, licenses and background checks on a caregiver's profile, with their expiry
    /// dates.
    pub fn certifications(&self) -> Certifications<'_> {
        Certifications { client: self }
    }
}

/// Certifications, licenses and background checks on a caregiver's profile, with their expiry
/// dates.
#[derive(Debug, Clone, Copy)]
pub struct Certifications<'a> {
    client: &'a AveeCare,
}

impl Certifications<'_> {
    /// List certifications.
    ///
    /// Returns a page of certifications. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: CertificationListParams) -> Result<Page<Certification>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: CertificationListParams,
        options: RequestOptions,
    ) -> Result<Page<Certification>> {
        self.client
            .get_page("/v1/certifications", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: CertificationListParams,
    ) -> impl Stream<Item = Result<Certification>> + Send + 'static {
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
                        "/v1/certifications",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a certification.
    ///
    /// Creates a certification and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: CertificationCreate) -> Result<Certification> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: CertificationCreate,
        options: RequestOptions,
    ) -> Result<Certification> {
        self.client
            .request::<Certification, _>(
                Method::POST,
                "/v1/certifications",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a certification.
    ///
    /// Returns one certification by ID. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Certification> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Certification> {
        let path = format!("/v1/certifications/{}", encode_path(id));
        self.client
            .request::<Certification, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a certification.
    ///
    /// Changes the fields you send and returns the updated certification. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: CertificationUpdate) -> Result<Certification> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: CertificationUpdate,
        options: RequestOptions,
    ) -> Result<Certification> {
        let path = format!("/v1/certifications/{}", encode_path(id));
        self.client
            .request::<Certification, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a certification.
    ///
    /// Deletes a certification. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<CertificationDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CertificationDeleted> {
        let path = format!("/v1/certifications/{}", encode_path(id));
        self.client
            .request::<CertificationDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// When a caregiver can or cannot work. A window is either a weekly recurring time on one
    /// weekday, or a one-off date range such as time off.
    pub fn availability(&self) -> AvailabilityResource<'_> {
        AvailabilityResource { client: self }
    }
}

/// When a caregiver can or cannot work. A window is either a weekly recurring time on one weekday,
/// or a one-off date range such as time off.
#[derive(Debug, Clone, Copy)]
pub struct AvailabilityResource<'a> {
    client: &'a AveeCare,
}

impl AvailabilityResource<'_> {
    /// List availability windows.
    ///
    /// Returns a page of availability windows. Filters combine with AND. Use `next_cursor` to fetch
    /// the next page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: AvailabilityListParams) -> Result<Page<Availability>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: AvailabilityListParams,
        options: RequestOptions,
    ) -> Result<Page<Availability>> {
        self.client
            .get_page("/v1/availability", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: AvailabilityListParams,
    ) -> impl Stream<Item = Result<Availability>> + Send + 'static {
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
                        "/v1/availability",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create an availability window.
    ///
    /// Creates an availability window and returns it. Send an `Idempotency-Key` header to make the
    /// request safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: AvailabilityCreate) -> Result<Availability> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: AvailabilityCreate,
        options: RequestOptions,
    ) -> Result<Availability> {
        self.client
            .request::<Availability, _>(
                Method::POST,
                "/v1/availability",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve an availability window.
    ///
    /// Returns one availability window by ID. Available to keys belonging to office staff and
    /// caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Availability> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Availability> {
        let path = format!("/v1/availability/{}", encode_path(id));
        self.client
            .request::<Availability, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update an availability window.
    ///
    /// Changes the fields you send and returns the updated availability window. Send `null` to
    /// clear an optional field. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: AvailabilityUpdate) -> Result<Availability> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: AvailabilityUpdate,
        options: RequestOptions,
    ) -> Result<Availability> {
        let path = format!("/v1/availability/{}", encode_path(id));
        self.client
            .request::<Availability, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete an availability window.
    ///
    /// Deletes an availability window. Available to keys belonging to office staff and caregivers.
    /// Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<AvailabilityDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<AvailabilityDeleted> {
        let path = format!("/v1/availability/{}", encode_path(id));
        self.client
            .request::<AvailabilityDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// Skills and specialties on a caregiver's profile, such as dementia care or Hoyer lift
    /// transfers.
    pub fn specializations(&self) -> Specializations<'_> {
        Specializations { client: self }
    }
}

/// Skills and specialties on a caregiver's profile, such as dementia care or Hoyer lift transfers.
#[derive(Debug, Clone, Copy)]
pub struct Specializations<'a> {
    client: &'a AveeCare,
}

impl Specializations<'_> {
    /// List specializations.
    ///
    /// Returns a page of specializations. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: SpecializationListParams) -> Result<Page<Specialization>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: SpecializationListParams,
        options: RequestOptions,
    ) -> Result<Page<Specialization>> {
        self.client
            .get_page("/v1/specializations", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: SpecializationListParams,
    ) -> impl Stream<Item = Result<Specialization>> + Send + 'static {
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
                        "/v1/specializations",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a specialization.
    ///
    /// Creates a specialization and returns it. Send an `Idempotency-Key` header to make the
    /// request safe to retry. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: SpecializationCreate) -> Result<Specialization> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: SpecializationCreate,
        options: RequestOptions,
    ) -> Result<Specialization> {
        self.client
            .request::<Specialization, _>(
                Method::POST,
                "/v1/specializations",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a specialization.
    ///
    /// Returns one specialization by ID. Available to keys belonging to office staff and
    /// caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Specialization> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Specialization> {
        let path = format!("/v1/specializations/{}", encode_path(id));
        self.client
            .request::<Specialization, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a specialization.
    ///
    /// Changes the fields you send and returns the updated specialization. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: SpecializationUpdate) -> Result<Specialization> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: SpecializationUpdate,
        options: RequestOptions,
    ) -> Result<Specialization> {
        let path = format!("/v1/specializations/{}", encode_path(id));
        self.client
            .request::<Specialization, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a specialization.
    ///
    /// Deletes a specialization. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<SpecializationDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<SpecializationDeleted> {
        let path = format!("/v1/specializations/{}", encode_path(id));
        self.client
            .request::<SpecializationDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// People who applied to work for your agency, tracked through your hiring stages until they
    /// are hired as caregivers.
    pub fn applicants(&self) -> Applicants<'_> {
        Applicants { client: self }
    }
}

/// People who applied to work for your agency, tracked through your hiring stages until they are
/// hired as caregivers.
#[derive(Debug, Clone, Copy)]
pub struct Applicants<'a> {
    client: &'a AveeCare,
}

impl Applicants<'_> {
    /// List applicants.
    ///
    /// Returns a page of applicants. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: ApplicantListParams) -> Result<Page<Applicant>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: ApplicantListParams,
        options: RequestOptions,
    ) -> Result<Page<Applicant>> {
        self.client
            .get_page("/v1/applicants", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: ApplicantListParams,
    ) -> impl Stream<Item = Result<Applicant>> + Send + 'static {
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
                        "/v1/applicants",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create an applicant.
    ///
    /// Creates an applicant and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: ApplicantCreate) -> Result<Applicant> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: ApplicantCreate,
        options: RequestOptions,
    ) -> Result<Applicant> {
        self.client
            .request::<Applicant, _>(
                Method::POST,
                "/v1/applicants",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve an applicant.
    ///
    /// Returns one applicant by ID. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Applicant> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Applicant> {
        let path = format!("/v1/applicants/{}", encode_path(id));
        self.client
            .request::<Applicant, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update an applicant.
    ///
    /// Changes the fields you send and returns the updated applicant. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: ApplicantUpdate) -> Result<Applicant> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: ApplicantUpdate,
        options: RequestOptions,
    ) -> Result<Applicant> {
        let path = format!("/v1/applicants/{}", encode_path(id));
        self.client
            .request::<Applicant, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete an applicant.
    ///
    /// Deletes an applicant. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<ApplicantDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ApplicantDeleted> {
        let path = format!("/v1/applicants/{}", encode_path(id));
        self.client
            .request::<ApplicantDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// Visits are scheduled or completed caregiver shifts at a patient's home. A visit's patient,
    /// caregiver, times and status are set when it is created; reschedule or reassign a visit in
    /// AveeCare.
    pub fn visits(&self) -> Visits<'_> {
        Visits { client: self }
    }
}

/// Visits are scheduled or completed caregiver shifts at a patient's home. A visit's patient,
/// caregiver, times and status are set when it is created; reschedule or reassign a visit in
/// AveeCare.
#[derive(Debug, Clone, Copy)]
pub struct Visits<'a> {
    client: &'a AveeCare,
}

impl Visits<'_> {
    /// List visits.
    ///
    /// Returns a page of visits, ordered by scheduled start, earliest first. Filters combine with
    /// AND. Use `next_cursor` to fetch the next page.
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
    /// Creates a visit and returns it. Send an `Idempotency-Key` header to make the request safe to
    /// retry. Available to keys belonging to office staff. Requires a key with `write` scope.
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
    /// Returns one visit by ID.
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
    /// Changes the fields you send and returns the updated visit. Send `null` to clear an optional
    /// field. Available to keys belonging to office staff and caregivers. With a caregiver's or
    /// patient's key, a saved record that key cannot read back is answered with only `object` and
    /// `id`. Requires a key with `write` scope.
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
    /// Deletes a visit. The visit moves to the schedule's recycle bin in AveeCare, where staff can
    /// restore it. Available to keys belonging to office staff. Requires a key with `write` scope.
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
    /// Notes written about a visit by the caregiver or office staff.
    pub fn visit_notes(&self) -> VisitNotes<'_> {
        VisitNotes { client: self }
    }
}

/// Notes written about a visit by the caregiver or office staff.
#[derive(Debug, Clone, Copy)]
pub struct VisitNotes<'a> {
    client: &'a AveeCare,
}

impl VisitNotes<'_> {
    /// List visit notes.
    ///
    /// Returns a page of visit notes. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: VisitNoteListParams) -> Result<Page<VisitNote>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: VisitNoteListParams,
        options: RequestOptions,
    ) -> Result<Page<VisitNote>> {
        self.client
            .get_page("/v1/visit-notes", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: VisitNoteListParams,
    ) -> impl Stream<Item = Result<VisitNote>> + Send + 'static {
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
                        "/v1/visit-notes",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a visit note.
    ///
    /// Creates a visit note and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: VisitNoteCreate) -> Result<VisitNote> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: VisitNoteCreate,
        options: RequestOptions,
    ) -> Result<VisitNote> {
        self.client
            .request::<VisitNote, _>(
                Method::POST,
                "/v1/visit-notes",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a visit note.
    ///
    /// Returns one visit note by ID. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<VisitNote> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<VisitNote> {
        let path = format!("/v1/visit-notes/{}", encode_path(id));
        self.client
            .request::<VisitNote, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a visit note.
    ///
    /// Changes the fields you send and returns the updated visit note. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: VisitNoteUpdate) -> Result<VisitNote> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: VisitNoteUpdate,
        options: RequestOptions,
    ) -> Result<VisitNote> {
        let path = format!("/v1/visit-notes/{}", encode_path(id));
        self.client
            .request::<VisitNote, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a visit note.
    ///
    /// Deletes a visit note. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<VisitNoteDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<VisitNoteDeleted> {
        let path = format!("/v1/visit-notes/{}", encode_path(id));
        self.client
            .request::<VisitNoteDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// Requests from caregivers to pick up visits with a patient, which office staff approve or
    /// deny.
    pub fn visit_requests(&self) -> VisitRequests<'_> {
        VisitRequests { client: self }
    }
}

/// Requests from caregivers to pick up visits with a patient, which office staff approve or deny.
#[derive(Debug, Clone, Copy)]
pub struct VisitRequests<'a> {
    client: &'a AveeCare,
}

impl VisitRequests<'_> {
    /// List visit requests.
    ///
    /// Returns a page of visit requests. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: VisitRequestListParams) -> Result<Page<VisitRequest>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: VisitRequestListParams,
        options: RequestOptions,
    ) -> Result<Page<VisitRequest>> {
        self.client
            .get_page("/v1/visit-requests", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: VisitRequestListParams,
    ) -> impl Stream<Item = Result<VisitRequest>> + Send + 'static {
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
                        "/v1/visit-requests",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a visit request.
    ///
    /// Creates a visit request and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: VisitRequestCreate) -> Result<VisitRequest> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: VisitRequestCreate,
        options: RequestOptions,
    ) -> Result<VisitRequest> {
        self.client
            .request::<VisitRequest, _>(
                Method::POST,
                "/v1/visit-requests",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a visit request.
    ///
    /// Returns one visit request by ID. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<VisitRequest> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<VisitRequest> {
        let path = format!("/v1/visit-requests/{}", encode_path(id));
        self.client
            .request::<VisitRequest, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a visit request.
    ///
    /// Changes the fields you send and returns the updated visit request. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: VisitRequestUpdate) -> Result<VisitRequest> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: VisitRequestUpdate,
        options: RequestOptions,
    ) -> Result<VisitRequest> {
        let path = format!("/v1/visit-requests/{}", encode_path(id));
        self.client
            .request::<VisitRequest, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a visit request.
    ///
    /// Deletes a visit request. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<VisitRequestDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<VisitRequestDeleted> {
        let path = format!("/v1/visit-requests/{}", encode_path(id));
        self.client
            .request::<VisitRequestDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// Open shifts are unassigned visits offered to caregivers so one of them can claim it.
    pub fn open_shifts(&self) -> OpenShifts<'_> {
        OpenShifts { client: self }
    }
}

/// Open shifts are unassigned visits offered to caregivers so one of them can claim it.
#[derive(Debug, Clone, Copy)]
pub struct OpenShifts<'a> {
    client: &'a AveeCare,
}

impl OpenShifts<'_> {
    /// List open shifts.
    ///
    /// Returns a page of open shifts. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: OpenShiftListParams) -> Result<Page<OpenShift>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: OpenShiftListParams,
        options: RequestOptions,
    ) -> Result<Page<OpenShift>> {
        self.client
            .get_page("/v1/open-shifts", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: OpenShiftListParams,
    ) -> impl Stream<Item = Result<OpenShift>> + Send + 'static {
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
                        "/v1/open-shifts",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create an open shift.
    ///
    /// Creates an open shift and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: OpenShiftCreate) -> Result<OpenShift> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: OpenShiftCreate,
        options: RequestOptions,
    ) -> Result<OpenShift> {
        self.client
            .request::<OpenShift, _>(
                Method::POST,
                "/v1/open-shifts",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve an open shift.
    ///
    /// Returns one open shift by ID. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<OpenShift> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<OpenShift> {
        let path = format!("/v1/open-shifts/{}", encode_path(id));
        self.client
            .request::<OpenShift, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update an open shift.
    ///
    /// Changes the fields you send and returns the updated open shift. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: OpenShiftUpdate) -> Result<OpenShift> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: OpenShiftUpdate,
        options: RequestOptions,
    ) -> Result<OpenShift> {
        let path = format!("/v1/open-shifts/{}", encode_path(id));
        self.client
            .request::<OpenShift, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete an open shift.
    ///
    /// Deletes an open shift. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<OpenShiftDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<OpenShiftDeleted> {
        let path = format!("/v1/open-shifts/{}", encode_path(id));
        self.client
            .request::<OpenShiftDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// Requests from one caregiver to hand a visit to another, which the other caregiver and then
    /// the office approve.
    pub fn shift_swaps(&self) -> ShiftSwaps<'_> {
        ShiftSwaps { client: self }
    }
}

/// Requests from one caregiver to hand a visit to another, which the other caregiver and then the
/// office approve.
#[derive(Debug, Clone, Copy)]
pub struct ShiftSwaps<'a> {
    client: &'a AveeCare,
}

impl ShiftSwaps<'_> {
    /// List shift swaps.
    ///
    /// Returns a page of shift swaps. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: ShiftSwapListParams) -> Result<Page<ShiftSwap>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: ShiftSwapListParams,
        options: RequestOptions,
    ) -> Result<Page<ShiftSwap>> {
        self.client
            .get_page("/v1/shift-swaps", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: ShiftSwapListParams,
    ) -> impl Stream<Item = Result<ShiftSwap>> + Send + 'static {
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
                        "/v1/shift-swaps",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a shift swap.
    ///
    /// Creates a shift swap and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: ShiftSwapCreate) -> Result<ShiftSwap> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: ShiftSwapCreate,
        options: RequestOptions,
    ) -> Result<ShiftSwap> {
        self.client
            .request::<ShiftSwap, _>(
                Method::POST,
                "/v1/shift-swaps",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a shift swap.
    ///
    /// Returns one shift swap by ID. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<ShiftSwap> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ShiftSwap> {
        let path = format!("/v1/shift-swaps/{}", encode_path(id));
        self.client
            .request::<ShiftSwap, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a shift swap.
    ///
    /// Changes the fields you send and returns the updated shift swap. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: ShiftSwapUpdate) -> Result<ShiftSwap> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: ShiftSwapUpdate,
        options: RequestOptions,
    ) -> Result<ShiftSwap> {
        let path = format!("/v1/shift-swaps/{}", encode_path(id));
        self.client
            .request::<ShiftSwap, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a shift swap.
    ///
    /// Deletes a shift swap. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<ShiftSwapDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ShiftSwapDeleted> {
        let path = format!("/v1/shift-swaps/{}", encode_path(id));
        self.client
            .request::<ShiftSwapDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// Care plans describe the services a patient receives, how often, and when the plan is next
    /// reviewed. The plan's goals and task lists are managed in AveeCare and are not included here.
    pub fn care_plans(&self) -> CarePlans<'_> {
        CarePlans { client: self }
    }
}

/// Care plans describe the services a patient receives, how often, and when the plan is next
/// reviewed. The plan's goals and task lists are managed in AveeCare and are not included here.
#[derive(Debug, Clone, Copy)]
pub struct CarePlans<'a> {
    client: &'a AveeCare,
}

impl CarePlans<'_> {
    /// List care plans.
    ///
    /// Returns a page of care plans. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: CarePlanListParams) -> Result<Page<CarePlan>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: CarePlanListParams,
        options: RequestOptions,
    ) -> Result<Page<CarePlan>> {
        self.client
            .get_page("/v1/care-plans", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: CarePlanListParams,
    ) -> impl Stream<Item = Result<CarePlan>> + Send + 'static {
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
                        "/v1/care-plans",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a care plan.
    ///
    /// Creates a care plan and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: CarePlanCreate) -> Result<CarePlan> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: CarePlanCreate,
        options: RequestOptions,
    ) -> Result<CarePlan> {
        self.client
            .request::<CarePlan, _>(
                Method::POST,
                "/v1/care-plans",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a care plan.
    ///
    /// Returns one care plan by ID. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<CarePlan> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CarePlan> {
        let path = format!("/v1/care-plans/{}", encode_path(id));
        self.client
            .request::<CarePlan, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a care plan.
    ///
    /// Changes the fields you send and returns the updated care plan. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: CarePlanUpdate) -> Result<CarePlan> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: CarePlanUpdate,
        options: RequestOptions,
    ) -> Result<CarePlan> {
        let path = format!("/v1/care-plans/{}", encode_path(id));
        self.client
            .request::<CarePlan, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a care plan.
    ///
    /// Deletes a care plan. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<CarePlanDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CarePlanDeleted> {
        let path = format!("/v1/care-plans/{}", encode_path(id));
        self.client
            .request::<CarePlanDeleted, _>(Method::DELETE, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Progress notes are the clinical summaries caregivers write after a visit.
    pub fn progress_notes(&self) -> ProgressNotes<'_> {
        ProgressNotes { client: self }
    }
}

/// Progress notes are the clinical summaries caregivers write after a visit.
#[derive(Debug, Clone, Copy)]
pub struct ProgressNotes<'a> {
    client: &'a AveeCare,
}

impl ProgressNotes<'_> {
    /// List progress notes.
    ///
    /// Returns a page of progress notes. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: ProgressNoteListParams) -> Result<Page<ProgressNote>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: ProgressNoteListParams,
        options: RequestOptions,
    ) -> Result<Page<ProgressNote>> {
        self.client
            .get_page("/v1/progress-notes", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: ProgressNoteListParams,
    ) -> impl Stream<Item = Result<ProgressNote>> + Send + 'static {
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
                        "/v1/progress-notes",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a progress note.
    ///
    /// Creates a progress note and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: ProgressNoteCreate) -> Result<ProgressNote> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: ProgressNoteCreate,
        options: RequestOptions,
    ) -> Result<ProgressNote> {
        self.client
            .request::<ProgressNote, _>(
                Method::POST,
                "/v1/progress-notes",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a progress note.
    ///
    /// Returns one progress note by ID. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<ProgressNote> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ProgressNote> {
        let path = format!("/v1/progress-notes/{}", encode_path(id));
        self.client
            .request::<ProgressNote, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a progress note.
    ///
    /// Changes the fields you send and returns the updated progress note. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: ProgressNoteUpdate) -> Result<ProgressNote> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: ProgressNoteUpdate,
        options: RequestOptions,
    ) -> Result<ProgressNote> {
        let path = format!("/v1/progress-notes/{}", encode_path(id));
        self.client
            .request::<ProgressNote, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a progress note.
    ///
    /// Deletes a progress note. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<ProgressNoteDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ProgressNoteDeleted> {
        let path = format!("/v1/progress-notes/{}", encode_path(id));
        self.client
            .request::<ProgressNoteDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// Payer authorizations: how many units of a service a payer has approved for a patient, and
    /// over what dates.
    pub fn authorizations(&self) -> Authorizations<'_> {
        Authorizations { client: self }
    }
}

/// Payer authorizations: how many units of a service a payer has approved for a patient, and over
/// what dates.
#[derive(Debug, Clone, Copy)]
pub struct Authorizations<'a> {
    client: &'a AveeCare,
}

impl Authorizations<'_> {
    /// List authorizations.
    ///
    /// Returns a page of authorizations. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: AuthorizationListParams) -> Result<Page<Authorization>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: AuthorizationListParams,
        options: RequestOptions,
    ) -> Result<Page<Authorization>> {
        self.client
            .get_page("/v1/authorizations", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: AuthorizationListParams,
    ) -> impl Stream<Item = Result<Authorization>> + Send + 'static {
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
                        "/v1/authorizations",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create an authorization.
    ///
    /// Creates an authorization and returns it. Send an `Idempotency-Key` header to make the
    /// request safe to retry. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: AuthorizationCreate) -> Result<Authorization> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: AuthorizationCreate,
        options: RequestOptions,
    ) -> Result<Authorization> {
        self.client
            .request::<Authorization, _>(
                Method::POST,
                "/v1/authorizations",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve an authorization.
    ///
    /// Returns one authorization by ID. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Authorization> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Authorization> {
        let path = format!("/v1/authorizations/{}", encode_path(id));
        self.client
            .request::<Authorization, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update an authorization.
    ///
    /// Changes the fields you send and returns the updated authorization. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: AuthorizationUpdate) -> Result<Authorization> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: AuthorizationUpdate,
        options: RequestOptions,
    ) -> Result<Authorization> {
        let path = format!("/v1/authorizations/{}", encode_path(id));
        self.client
            .request::<Authorization, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete an authorization.
    ///
    /// Deletes an authorization. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<AuthorizationDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<AuthorizationDeleted> {
        let path = format!("/v1/authorizations/{}", encode_path(id));
        self.client
            .request::<AuthorizationDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// Incident reports: falls, injuries, medication errors and other events that need follow-up,
    /// often for regulatory reporting.
    pub fn incidents(&self) -> Incidents<'_> {
        Incidents { client: self }
    }
}

/// Incident reports: falls, injuries, medication errors and other events that need follow-up, often
/// for regulatory reporting.
#[derive(Debug, Clone, Copy)]
pub struct Incidents<'a> {
    client: &'a AveeCare,
}

impl Incidents<'_> {
    /// List incidents.
    ///
    /// Returns a page of incidents. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: IncidentListParams) -> Result<Page<Incident>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: IncidentListParams,
        options: RequestOptions,
    ) -> Result<Page<Incident>> {
        self.client
            .get_page("/v1/incidents", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: IncidentListParams,
    ) -> impl Stream<Item = Result<Incident>> + Send + 'static {
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
                        "/v1/incidents",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create an incident.
    ///
    /// Creates an incident and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: IncidentCreate) -> Result<Incident> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: IncidentCreate,
        options: RequestOptions,
    ) -> Result<Incident> {
        self.client
            .request::<Incident, _>(
                Method::POST,
                "/v1/incidents",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve an incident.
    ///
    /// Returns one incident by ID. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Incident> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Incident> {
        let path = format!("/v1/incidents/{}", encode_path(id));
        self.client
            .request::<Incident, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update an incident.
    ///
    /// Changes the fields you send and returns the updated incident. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: IncidentUpdate) -> Result<Incident> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: IncidentUpdate,
        options: RequestOptions,
    ) -> Result<Incident> {
        let path = format!("/v1/incidents/{}", encode_path(id));
        self.client
            .request::<Incident, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete an incident.
    ///
    /// Deletes an incident. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<IncidentDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<IncidentDeleted> {
        let path = format!("/v1/incidents/{}", encode_path(id));
        self.client
            .request::<IncidentDeleted, _>(Method::DELETE, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Events your agency documents about a patient, caregiver or visit, such as a complaint, a
    /// compliment or an attendance issue.
    pub fn events(&self) -> Events<'_> {
        Events { client: self }
    }
}

/// Events your agency documents about a patient, caregiver or visit, such as a complaint, a
/// compliment or an attendance issue.
#[derive(Debug, Clone, Copy)]
pub struct Events<'a> {
    client: &'a AveeCare,
}

impl Events<'_> {
    /// List events.
    ///
    /// Returns a page of events. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: EventListParams) -> Result<Page<Event>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: EventListParams,
        options: RequestOptions,
    ) -> Result<Page<Event>> {
        self.client
            .get_page("/v1/events", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: EventListParams,
    ) -> impl Stream<Item = Result<Event>> + Send + 'static {
        let client = self.client.clone();
        auto_page(move |cursor| {
            let client = client.clone();
            let mut params = params.clone();
            if cursor.is_some() {
                params.cursor = cursor;
            }
            async move {
                client
                    .get_page("/v1/events", params.to_query(), &RequestOptions::default())
                    .await
            }
        })
    }

    /// Create an event.
    ///
    /// Creates an event and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: EventCreate) -> Result<Event> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: EventCreate,
        options: RequestOptions,
    ) -> Result<Event> {
        self.client
            .request::<Event, _>(
                Method::POST,
                "/v1/events",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve an event.
    ///
    /// Returns one event by ID. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Event> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Event> {
        let path = format!("/v1/events/{}", encode_path(id));
        self.client
            .request::<Event, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update an event.
    ///
    /// Changes the fields you send and returns the updated event. Send `null` to clear an optional
    /// field. Available to keys belonging to office staff. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: EventUpdate) -> Result<Event> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: EventUpdate,
        options: RequestOptions,
    ) -> Result<Event> {
        let path = format!("/v1/events/{}", encode_path(id));
        self.client
            .request::<Event, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete an event.
    ///
    /// Deletes an event. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<EventDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<EventDeleted> {
        let path = format!("/v1/events/{}", encode_path(id));
        self.client
            .request::<EventDeleted, _>(Method::DELETE, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Hospitals, physicians, case managers and other organizations that refer clients to your
    /// agency.
    pub fn referral_sources(&self) -> ReferralSources<'_> {
        ReferralSources { client: self }
    }
}

/// Hospitals, physicians, case managers and other organizations that refer clients to your agency.
#[derive(Debug, Clone, Copy)]
pub struct ReferralSources<'a> {
    client: &'a AveeCare,
}

impl ReferralSources<'_> {
    /// List referral sources.
    ///
    /// Returns a page of referral sources. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: ReferralSourceListParams) -> Result<Page<ReferralSource>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: ReferralSourceListParams,
        options: RequestOptions,
    ) -> Result<Page<ReferralSource>> {
        self.client
            .get_page("/v1/referral-sources", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: ReferralSourceListParams,
    ) -> impl Stream<Item = Result<ReferralSource>> + Send + 'static {
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
                        "/v1/referral-sources",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a referral source.
    ///
    /// Creates a referral source and returns it. Send an `Idempotency-Key` header to make the
    /// request safe to retry. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: ReferralSourceCreate) -> Result<ReferralSource> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: ReferralSourceCreate,
        options: RequestOptions,
    ) -> Result<ReferralSource> {
        self.client
            .request::<ReferralSource, _>(
                Method::POST,
                "/v1/referral-sources",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a referral source.
    ///
    /// Returns one referral source by ID. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<ReferralSource> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ReferralSource> {
        let path = format!("/v1/referral-sources/{}", encode_path(id));
        self.client
            .request::<ReferralSource, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a referral source.
    ///
    /// Changes the fields you send and returns the updated referral source. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: ReferralSourceUpdate) -> Result<ReferralSource> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: ReferralSourceUpdate,
        options: RequestOptions,
    ) -> Result<ReferralSource> {
        let path = format!("/v1/referral-sources/{}", encode_path(id));
        self.client
            .request::<ReferralSource, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a referral source.
    ///
    /// Deletes a referral source. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<ReferralSourceDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ReferralSourceDeleted> {
        let path = format!("/v1/referral-sources/{}", encode_path(id));
        self.client
            .request::<ReferralSourceDeleted, _>(
                Method::DELETE,
                &path,
                &[],
                None::<&()>,
                false,
                &options,
            )
            .await
    }
}

impl AveeCare {
    /// Individual referrals of a prospective client, tracked from first contact to admission.
    pub fn referrals(&self) -> Referrals<'_> {
        Referrals { client: self }
    }
}

/// Individual referrals of a prospective client, tracked from first contact to admission.
#[derive(Debug, Clone, Copy)]
pub struct Referrals<'a> {
    client: &'a AveeCare,
}

impl Referrals<'_> {
    /// List referrals.
    ///
    /// Returns a page of referrals. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: ReferralListParams) -> Result<Page<Referral>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: ReferralListParams,
        options: RequestOptions,
    ) -> Result<Page<Referral>> {
        self.client
            .get_page("/v1/referrals", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: ReferralListParams,
    ) -> impl Stream<Item = Result<Referral>> + Send + 'static {
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
                        "/v1/referrals",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a referral.
    ///
    /// Creates a referral and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: ReferralCreate) -> Result<Referral> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: ReferralCreate,
        options: RequestOptions,
    ) -> Result<Referral> {
        self.client
            .request::<Referral, _>(
                Method::POST,
                "/v1/referrals",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a referral.
    ///
    /// Returns one referral by ID. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Referral> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Referral> {
        let path = format!("/v1/referrals/{}", encode_path(id));
        self.client
            .request::<Referral, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update a referral.
    ///
    /// Changes the fields you send and returns the updated referral. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: ReferralUpdate) -> Result<Referral> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: ReferralUpdate,
        options: RequestOptions,
    ) -> Result<Referral> {
        let path = format!("/v1/referrals/{}", encode_path(id));
        self.client
            .request::<Referral, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete a referral.
    ///
    /// Deletes a referral. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<ReferralDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ReferralDeleted> {
        let path = format!("/v1/referrals/{}", encode_path(id));
        self.client
            .request::<ReferralDeleted, _>(Method::DELETE, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Invoices billed to patients and their families for private-pay care. Read-only.
    pub fn invoices(&self) -> Invoices<'_> {
        Invoices { client: self }
    }
}

/// Invoices billed to patients and their families for private-pay care. Read-only.
#[derive(Debug, Clone, Copy)]
pub struct Invoices<'a> {
    client: &'a AveeCare,
}

impl Invoices<'_> {
    /// List invoices.
    ///
    /// Returns a page of invoices. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff.
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

    /// Retrieve an invoice.
    ///
    /// Returns one invoice by ID. Available to keys belonging to office staff.
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
    /// The individual charges on an invoice. Read-only.
    pub fn invoice_line_items(&self) -> InvoiceLineItems<'_> {
        InvoiceLineItems { client: self }
    }
}

/// The individual charges on an invoice. Read-only.
#[derive(Debug, Clone, Copy)]
pub struct InvoiceLineItems<'a> {
    client: &'a AveeCare,
}

impl InvoiceLineItems<'_> {
    /// List invoice line items.
    ///
    /// Returns a page of invoice line items. Filters combine with AND. Use `next_cursor` to fetch
    /// the next page. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: InvoiceLineItemListParams) -> Result<Page<InvoiceLineItem>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: InvoiceLineItemListParams,
        options: RequestOptions,
    ) -> Result<Page<InvoiceLineItem>> {
        self.client
            .get_page("/v1/invoice-line-items", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: InvoiceLineItemListParams,
    ) -> impl Stream<Item = Result<InvoiceLineItem>> + Send + 'static {
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
                        "/v1/invoice-line-items",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Retrieve an invoice line item.
    ///
    /// Returns one invoice line item by ID. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<InvoiceLineItem> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<InvoiceLineItem> {
        let path = format!("/v1/invoice-line-items/{}", encode_path(id));
        self.client
            .request::<InvoiceLineItem, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Insurance claims billed to Medicaid, Medicare and other payers, with their adjudication
    /// status. Read-only.
    pub fn claims(&self) -> Claims<'_> {
        Claims { client: self }
    }
}

/// Insurance claims billed to Medicaid, Medicare and other payers, with their adjudication status.
/// Read-only.
#[derive(Debug, Clone, Copy)]
pub struct Claims<'a> {
    client: &'a AveeCare,
}

impl Claims<'_> {
    /// List claims.
    ///
    /// Returns a page of claims. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: ClaimListParams) -> Result<Page<Claim>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: ClaimListParams,
        options: RequestOptions,
    ) -> Result<Page<Claim>> {
        self.client
            .get_page("/v1/claims", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: ClaimListParams,
    ) -> impl Stream<Item = Result<Claim>> + Send + 'static {
        let client = self.client.clone();
        auto_page(move |cursor| {
            let client = client.clone();
            let mut params = params.clone();
            if cursor.is_some() {
                params.cursor = cursor;
            }
            async move {
                client
                    .get_page("/v1/claims", params.to_query(), &RequestOptions::default())
                    .await
            }
        })
    }

    /// Retrieve a claim.
    ///
    /// Returns one claim by ID. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Claim> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Claim> {
        let path = format!("/v1/claims/{}", encode_path(id));
        self.client
            .request::<Claim, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// One caregiver's hours and pay for one pay period. Read-only.
    pub fn payroll_entries(&self) -> PayrollEntries<'_> {
        PayrollEntries { client: self }
    }
}

/// One caregiver's hours and pay for one pay period. Read-only.
#[derive(Debug, Clone, Copy)]
pub struct PayrollEntries<'a> {
    client: &'a AveeCare,
}

impl PayrollEntries<'_> {
    /// List payroll entries.
    ///
    /// Returns a page of payroll entries. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: PayrollEntryListParams) -> Result<Page<PayrollEntry>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: PayrollEntryListParams,
        options: RequestOptions,
    ) -> Result<Page<PayrollEntry>> {
        self.client
            .get_page("/v1/payroll-entries", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: PayrollEntryListParams,
    ) -> impl Stream<Item = Result<PayrollEntry>> + Send + 'static {
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
                        "/v1/payroll-entries",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Retrieve a payroll entry.
    ///
    /// Returns one payroll entry by ID. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<PayrollEntry> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<PayrollEntry> {
        let path = format!("/v1/payroll-entries/{}", encode_path(id));
        self.client
            .request::<PayrollEntry, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// In-app message threads between people at your agency: one-to-one chats and named group
    /// chats. Read-only; start conversations in AveeCare.
    pub fn conversations(&self) -> Conversations<'_> {
        Conversations { client: self }
    }
}

/// In-app message threads between people at your agency: one-to-one chats and named group chats.
/// Read-only; start conversations in AveeCare.
#[derive(Debug, Clone, Copy)]
pub struct Conversations<'a> {
    client: &'a AveeCare,
}

impl Conversations<'_> {
    /// List conversations.
    ///
    /// Returns a page of conversations. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: ConversationListParams) -> Result<Page<Conversation>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: ConversationListParams,
        options: RequestOptions,
    ) -> Result<Page<Conversation>> {
        self.client
            .get_page("/v1/conversations", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: ConversationListParams,
    ) -> impl Stream<Item = Result<Conversation>> + Send + 'static {
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
                        "/v1/conversations",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Retrieve a conversation.
    ///
    /// Returns one conversation by ID.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Conversation> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Conversation> {
        let path = format!("/v1/conversations/{}", encode_path(id));
        self.client
            .request::<Conversation, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Messages in a conversation. Messages you send through the API are sent as the API key's
    /// user.
    pub fn messages(&self) -> Messages<'_> {
        Messages { client: self }
    }
}

/// Messages in a conversation. Messages you send through the API are sent as the API key's user.
#[derive(Debug, Clone, Copy)]
pub struct Messages<'a> {
    client: &'a AveeCare,
}

impl Messages<'_> {
    /// List messages.
    ///
    /// Returns a page of messages. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: MessageListParams) -> Result<Page<Message>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: MessageListParams,
        options: RequestOptions,
    ) -> Result<Page<Message>> {
        self.client
            .get_page("/v1/messages", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: MessageListParams,
    ) -> impl Stream<Item = Result<Message>> + Send + 'static {
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
                        "/v1/messages",
                        params.to_query(),
                        &RequestOptions::default(),
                    )
                    .await
            }
        })
    }

    /// Create a message.
    ///
    /// Creates a message and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. With a caregiver's or patient's key, a saved record that key cannot read back is
    /// answered with only `object` and `id`. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: MessageCreate) -> Result<Message> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: MessageCreate,
        options: RequestOptions,
    ) -> Result<Message> {
        self.client
            .request::<Message, _>(
                Method::POST,
                "/v1/messages",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve a message.
    ///
    /// Returns one message by ID.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Message> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Message> {
        let path = format!("/v1/messages/{}", encode_path(id));
        self.client
            .request::<Message, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Alerts on your agency's alert board, such as a credential about to expire or a missed
    /// clock-in, and announcements staff post for caregivers.
    pub fn alerts(&self) -> Alerts<'_> {
        Alerts { client: self }
    }
}

/// Alerts on your agency's alert board, such as a credential about to expire or a missed clock-in,
/// and announcements staff post for caregivers.
#[derive(Debug, Clone, Copy)]
pub struct Alerts<'a> {
    client: &'a AveeCare,
}

impl Alerts<'_> {
    /// List alerts.
    ///
    /// Returns a page of alerts. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: AlertListParams) -> Result<Page<Alert>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: AlertListParams,
        options: RequestOptions,
    ) -> Result<Page<Alert>> {
        self.client
            .get_page("/v1/alerts", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: AlertListParams,
    ) -> impl Stream<Item = Result<Alert>> + Send + 'static {
        let client = self.client.clone();
        auto_page(move |cursor| {
            let client = client.clone();
            let mut params = params.clone();
            if cursor.is_some() {
                params.cursor = cursor;
            }
            async move {
                client
                    .get_page("/v1/alerts", params.to_query(), &RequestOptions::default())
                    .await
            }
        })
    }

    /// Create an alert.
    ///
    /// Creates an alert and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: AlertCreate) -> Result<Alert> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: AlertCreate,
        options: RequestOptions,
    ) -> Result<Alert> {
        self.client
            .request::<Alert, _>(
                Method::POST,
                "/v1/alerts",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve an alert.
    ///
    /// Returns one alert by ID. Available to keys belonging to office staff and caregivers.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Alert> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Alert> {
        let path = format!("/v1/alerts/{}", encode_path(id));
        self.client
            .request::<Alert, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update an alert.
    ///
    /// Changes the fields you send and returns the updated alert. Send `null` to clear an optional
    /// field. Available to keys belonging to office staff. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: AlertUpdate) -> Result<Alert> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: AlertUpdate,
        options: RequestOptions,
    ) -> Result<Alert> {
        let path = format!("/v1/alerts/{}", encode_path(id));
        self.client
            .request::<Alert, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete an alert.
    ///
    /// Deletes an alert. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<AlertDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<AlertDeleted> {
        let path = format!("/v1/alerts/{}", encode_path(id));
        self.client
            .request::<AlertDeleted, _>(Method::DELETE, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// Your agency's offices (branches). Patients, caregivers and applicants can each belong to one
    /// office. There is always exactly one default office.
    pub fn offices(&self) -> Offices<'_> {
        Offices { client: self }
    }
}

/// Your agency's offices (branches). Patients, caregivers and applicants can each belong to one
/// office. There is always exactly one default office.
#[derive(Debug, Clone, Copy)]
pub struct Offices<'a> {
    client: &'a AveeCare,
}

impl Offices<'_> {
    /// List offices.
    ///
    /// Returns a page of offices. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn list(&self, params: OfficeListParams) -> Result<Page<Office>> {
        self.list_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::list`] with per-request options.
    pub async fn list_with_options(
        &self,
        params: OfficeListParams,
        options: RequestOptions,
    ) -> Result<Page<Office>> {
        self.client
            .get_page("/v1/offices", params.to_query(), &options)
            .await
    }

    /// Every item across every page of [`Self::list`], fetched lazily.
    pub fn list_auto_paging(
        &self,
        params: OfficeListParams,
    ) -> impl Stream<Item = Result<Office>> + Send + 'static {
        let client = self.client.clone();
        auto_page(move |cursor| {
            let client = client.clone();
            let mut params = params.clone();
            if cursor.is_some() {
                params.cursor = cursor;
            }
            async move {
                client
                    .get_page("/v1/offices", params.to_query(), &RequestOptions::default())
                    .await
            }
        })
    }

    /// Create an office.
    ///
    /// Creates an office and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn create(&self, params: OfficeCreate) -> Result<Office> {
        self.create_with_options(params, RequestOptions::default())
            .await
    }

    /// [`Self::create`] with per-request options.
    pub async fn create_with_options(
        &self,
        params: OfficeCreate,
        options: RequestOptions,
    ) -> Result<Office> {
        self.client
            .request::<Office, _>(
                Method::POST,
                "/v1/offices",
                &[],
                Some(&params),
                true,
                &options,
            )
            .await
    }

    /// Retrieve an office.
    ///
    /// Returns one office by ID. Available to keys belonging to office staff.
    ///
    /// Requires the `read` scope.
    pub async fn retrieve(&self, id: &str) -> Result<Office> {
        self.retrieve_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::retrieve`] with per-request options.
    pub async fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Office> {
        let path = format!("/v1/offices/{}", encode_path(id));
        self.client
            .request::<Office, _>(Method::GET, &path, &[], None::<&()>, false, &options)
            .await
    }

    /// Update an office.
    ///
    /// Changes the fields you send and returns the updated office. Send `null` to clear an optional
    /// field. Available to keys belonging to office staff. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn update(&self, id: &str, params: OfficeUpdate) -> Result<Office> {
        self.update_with_options(id, params, RequestOptions::default())
            .await
    }

    /// [`Self::update`] with per-request options.
    pub async fn update_with_options(
        &self,
        id: &str,
        params: OfficeUpdate,
        options: RequestOptions,
    ) -> Result<Office> {
        let path = format!("/v1/offices/{}", encode_path(id));
        self.client
            .request::<Office, _>(Method::PATCH, &path, &[], Some(&params), false, &options)
            .await
    }

    /// Delete an office.
    ///
    /// Deletes an office. The default office, and an agency's only office, cannot be deleted.
    /// Available to keys belonging to office staff. Requires a key with `write` scope.
    ///
    /// Requires the `write` scope.
    pub async fn delete(&self, id: &str) -> Result<OfficeDeleted> {
        self.delete_with_options(id, RequestOptions::default())
            .await
    }

    /// [`Self::delete`] with per-request options.
    pub async fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<OfficeDeleted> {
        let path = format!("/v1/offices/{}", encode_path(id));
        self.client
            .request::<OfficeDeleted, _>(Method::DELETE, &path, &[], None::<&()>, false, &options)
            .await
    }
}

impl AveeCare {
    /// The API key making the request: its scopes, its user's role and its agency.
    pub fn me(&self) -> MeResource<'_> {
        MeResource { client: self }
    }
}

/// The API key making the request: its scopes, its user's role and its agency.
#[derive(Debug, Clone, Copy)]
pub struct MeResource<'a> {
    client: &'a AveeCare,
}

impl MeResource<'_> {
    /// Retrieve the current API key.
    ///
    /// Returns the API key making the request, its scopes, the role of the user it belongs to, and
    /// its agency. Useful to check a key works.
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
