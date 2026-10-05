// Code generated from the AveeCare OpenAPI specification. DO NOT EDIT.

use super::models::*;
use crate::blocking::{AveeCare, BlockingIter};
use crate::client::RequestOptions;
use crate::error::Result;
use crate::pagination::Page;

impl AveeCare {
    /// Patients are the clients your agency provides care to, including prospective clients you
    /// have not yet admitted.
    pub fn patients(&self) -> Patients<'_> {
        Patients { client: self }
    }
}

/// Patients are the clients your agency provides care to, including prospective clients you have
/// not yet admitted. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Patients<'a> {
    client: &'a AveeCare,
}

impl Patients<'_> {
    /// List patients.
    ///
    /// Returns a page of patients. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
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
    /// Creates a patient and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
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
    /// Returns one patient by ID.
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
    /// Changes the fields you send and returns the updated patient. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
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
    /// Deletes a patient. The patient's visits, notes and other related records are not deleted.
    /// Available to keys belonging to office staff. Requires a key with `write` scope.
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
    /// Contacts are family members and other people connected to a patient, such as an emergency
    /// contact.
    pub fn contacts(&self) -> Contacts<'_> {
        Contacts { client: self }
    }
}

/// Contacts are family members and other people connected to a patient, such as an emergency
/// contact. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Contacts<'a> {
    client: &'a AveeCare,
}

impl Contacts<'_> {
    /// List contacts.
    ///
    /// Returns a page of contacts. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page.
    pub fn list(&self, params: ContactListParams) -> Result<Page<Contact>> {
        self.client
            .block_on(self.client.inner.contacts().list(params))
    }

    pub fn list_with_options(
        &self,
        params: ContactListParams,
        options: RequestOptions,
    ) -> Result<Page<Contact>> {
        self.client.block_on(
            self.client
                .inner
                .contacts()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: ContactListParams) -> BlockingIter<Contact> {
        self.client
            .iter(self.client.inner.contacts().list_auto_paging(params))
    }

    /// Create a contact.
    ///
    /// Creates a contact and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
    pub fn create(&self, params: ContactCreate) -> Result<Contact> {
        self.client
            .block_on(self.client.inner.contacts().create(params))
    }

    pub fn create_with_options(
        &self,
        params: ContactCreate,
        options: RequestOptions,
    ) -> Result<Contact> {
        self.client.block_on(
            self.client
                .inner
                .contacts()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a contact.
    ///
    /// Returns one contact by ID.
    pub fn retrieve(&self, id: &str) -> Result<Contact> {
        self.client
            .block_on(self.client.inner.contacts().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Contact> {
        self.client.block_on(
            self.client
                .inner
                .contacts()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a contact.
    ///
    /// Changes the fields you send and returns the updated contact. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: ContactUpdate) -> Result<Contact> {
        self.client
            .block_on(self.client.inner.contacts().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: ContactUpdate,
        options: RequestOptions,
    ) -> Result<Contact> {
        self.client.block_on(
            self.client
                .inner
                .contacts()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a contact.
    ///
    /// Deletes a contact. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn delete(&self, id: &str) -> Result<ContactDeleted> {
        self.client
            .block_on(self.client.inner.contacts().delete(id))
    }

    pub fn delete_with_options(&self, id: &str, options: RequestOptions) -> Result<ContactDeleted> {
        self.client.block_on(
            self.client
                .inner
                .contacts()
                .delete_with_options(id, options),
        )
    }
}

impl AveeCare {
    /// Allergies recorded on a patient's profile.
    pub fn allergies(&self) -> Allergies<'_> {
        Allergies { client: self }
    }
}

/// Allergies recorded on a patient's profile. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Allergies<'a> {
    client: &'a AveeCare,
}

impl Allergies<'_> {
    /// List allergies.
    ///
    /// Returns a page of allergies. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page.
    pub fn list(&self, params: AllergyListParams) -> Result<Page<Allergy>> {
        self.client
            .block_on(self.client.inner.allergies().list(params))
    }

    pub fn list_with_options(
        &self,
        params: AllergyListParams,
        options: RequestOptions,
    ) -> Result<Page<Allergy>> {
        self.client.block_on(
            self.client
                .inner
                .allergies()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: AllergyListParams) -> BlockingIter<Allergy> {
        self.client
            .iter(self.client.inner.allergies().list_auto_paging(params))
    }

    /// Create an allergy.
    ///
    /// Creates an allergy and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
    pub fn create(&self, params: AllergyCreate) -> Result<Allergy> {
        self.client
            .block_on(self.client.inner.allergies().create(params))
    }

    pub fn create_with_options(
        &self,
        params: AllergyCreate,
        options: RequestOptions,
    ) -> Result<Allergy> {
        self.client.block_on(
            self.client
                .inner
                .allergies()
                .create_with_options(params, options),
        )
    }

    /// Retrieve an allergy.
    ///
    /// Returns one allergy by ID.
    pub fn retrieve(&self, id: &str) -> Result<Allergy> {
        self.client
            .block_on(self.client.inner.allergies().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Allergy> {
        self.client.block_on(
            self.client
                .inner
                .allergies()
                .retrieve_with_options(id, options),
        )
    }

    /// Update an allergy.
    ///
    /// Changes the fields you send and returns the updated allergy. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: AllergyUpdate) -> Result<Allergy> {
        self.client
            .block_on(self.client.inner.allergies().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: AllergyUpdate,
        options: RequestOptions,
    ) -> Result<Allergy> {
        self.client.block_on(
            self.client
                .inner
                .allergies()
                .update_with_options(id, params, options),
        )
    }

    /// Delete an allergy.
    ///
    /// Deletes an allergy. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn delete(&self, id: &str) -> Result<AllergyDeleted> {
        self.client
            .block_on(self.client.inner.allergies().delete(id))
    }

    pub fn delete_with_options(&self, id: &str, options: RequestOptions) -> Result<AllergyDeleted> {
        self.client.block_on(
            self.client
                .inner
                .allergies()
                .delete_with_options(id, options),
        )
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
/// or assist with them. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Medications<'a> {
    client: &'a AveeCare,
}

impl Medications<'_> {
    /// List medications.
    ///
    /// Returns a page of medications. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page.
    pub fn list(&self, params: MedicationListParams) -> Result<Page<Medication>> {
        self.client
            .block_on(self.client.inner.medications().list(params))
    }

    pub fn list_with_options(
        &self,
        params: MedicationListParams,
        options: RequestOptions,
    ) -> Result<Page<Medication>> {
        self.client.block_on(
            self.client
                .inner
                .medications()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: MedicationListParams) -> BlockingIter<Medication> {
        self.client
            .iter(self.client.inner.medications().list_auto_paging(params))
    }

    /// Create a medication.
    ///
    /// Creates a medication and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn create(&self, params: MedicationCreate) -> Result<Medication> {
        self.client
            .block_on(self.client.inner.medications().create(params))
    }

    pub fn create_with_options(
        &self,
        params: MedicationCreate,
        options: RequestOptions,
    ) -> Result<Medication> {
        self.client.block_on(
            self.client
                .inner
                .medications()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a medication.
    ///
    /// Returns one medication by ID.
    pub fn retrieve(&self, id: &str) -> Result<Medication> {
        self.client
            .block_on(self.client.inner.medications().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Medication> {
        self.client.block_on(
            self.client
                .inner
                .medications()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a medication.
    ///
    /// Changes the fields you send and returns the updated medication. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: MedicationUpdate) -> Result<Medication> {
        self.client
            .block_on(self.client.inner.medications().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: MedicationUpdate,
        options: RequestOptions,
    ) -> Result<Medication> {
        self.client.block_on(
            self.client
                .inner
                .medications()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a medication.
    ///
    /// Deletes a medication. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<MedicationDeleted> {
        self.client
            .block_on(self.client.inner.medications().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<MedicationDeleted> {
        self.client.block_on(
            self.client
                .inner
                .medications()
                .delete_with_options(id, options),
        )
    }
}

impl AveeCare {
    /// Goals of care set for a patient, such as walking to the mailbox unaided.
    pub fn care_goals(&self) -> CareGoals<'_> {
        CareGoals { client: self }
    }
}

/// Goals of care set for a patient, such as walking to the mailbox unaided. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct CareGoals<'a> {
    client: &'a AveeCare,
}

impl CareGoals<'_> {
    /// List care goals.
    ///
    /// Returns a page of care goals. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page.
    pub fn list(&self, params: CareGoalListParams) -> Result<Page<CareGoal>> {
        self.client
            .block_on(self.client.inner.care_goals().list(params))
    }

    pub fn list_with_options(
        &self,
        params: CareGoalListParams,
        options: RequestOptions,
    ) -> Result<Page<CareGoal>> {
        self.client.block_on(
            self.client
                .inner
                .care_goals()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: CareGoalListParams) -> BlockingIter<CareGoal> {
        self.client
            .iter(self.client.inner.care_goals().list_auto_paging(params))
    }

    /// Create a care goal.
    ///
    /// Creates a care goal and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn create(&self, params: CareGoalCreate) -> Result<CareGoal> {
        self.client
            .block_on(self.client.inner.care_goals().create(params))
    }

    pub fn create_with_options(
        &self,
        params: CareGoalCreate,
        options: RequestOptions,
    ) -> Result<CareGoal> {
        self.client.block_on(
            self.client
                .inner
                .care_goals()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a care goal.
    ///
    /// Returns one care goal by ID.
    pub fn retrieve(&self, id: &str) -> Result<CareGoal> {
        self.client
            .block_on(self.client.inner.care_goals().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<CareGoal> {
        self.client.block_on(
            self.client
                .inner
                .care_goals()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a care goal.
    ///
    /// Changes the fields you send and returns the updated care goal. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: CareGoalUpdate) -> Result<CareGoal> {
        self.client
            .block_on(self.client.inner.care_goals().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: CareGoalUpdate,
        options: RequestOptions,
    ) -> Result<CareGoal> {
        self.client.block_on(
            self.client
                .inner
                .care_goals()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a care goal.
    ///
    /// Deletes a care goal. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<CareGoalDeleted> {
        self.client
            .block_on(self.client.inner.care_goals().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CareGoalDeleted> {
        self.client.block_on(
            self.client
                .inner
                .care_goals()
                .delete_with_options(id, options),
        )
    }
}

impl AveeCare {
    /// Notes your staff and caregivers keep on a patient's profile.
    pub fn patient_notes(&self) -> PatientNotes<'_> {
        PatientNotes { client: self }
    }
}

/// Notes your staff and caregivers keep on a patient's profile. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct PatientNotes<'a> {
    client: &'a AveeCare,
}

impl PatientNotes<'_> {
    /// List patient notes.
    ///
    /// Returns a page of patient notes. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: PatientNoteListParams) -> Result<Page<PatientNote>> {
        self.client
            .block_on(self.client.inner.patient_notes().list(params))
    }

    pub fn list_with_options(
        &self,
        params: PatientNoteListParams,
        options: RequestOptions,
    ) -> Result<Page<PatientNote>> {
        self.client.block_on(
            self.client
                .inner
                .patient_notes()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: PatientNoteListParams) -> BlockingIter<PatientNote> {
        self.client
            .iter(self.client.inner.patient_notes().list_auto_paging(params))
    }

    /// Create a patient note.
    ///
    /// Creates a patient note and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    pub fn create(&self, params: PatientNoteCreate) -> Result<PatientNote> {
        self.client
            .block_on(self.client.inner.patient_notes().create(params))
    }

    pub fn create_with_options(
        &self,
        params: PatientNoteCreate,
        options: RequestOptions,
    ) -> Result<PatientNote> {
        self.client.block_on(
            self.client
                .inner
                .patient_notes()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a patient note.
    ///
    /// Returns one patient note by ID. Available to keys belonging to office staff and caregivers.
    pub fn retrieve(&self, id: &str) -> Result<PatientNote> {
        self.client
            .block_on(self.client.inner.patient_notes().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<PatientNote> {
        self.client.block_on(
            self.client
                .inner
                .patient_notes()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a patient note.
    ///
    /// Changes the fields you send and returns the updated patient note. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: PatientNoteUpdate) -> Result<PatientNote> {
        self.client
            .block_on(self.client.inner.patient_notes().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: PatientNoteUpdate,
        options: RequestOptions,
    ) -> Result<PatientNote> {
        self.client.block_on(
            self.client
                .inner
                .patient_notes()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a patient note.
    ///
    /// Deletes a patient note. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<PatientNoteDeleted> {
        self.client
            .block_on(self.client.inner.patient_notes().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<PatientNoteDeleted> {
        self.client.block_on(
            self.client
                .inner
                .patient_notes()
                .delete_with_options(id, options),
        )
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
/// given. Read-only. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct MedicationAdministrations<'a> {
    client: &'a AveeCare,
}

impl MedicationAdministrations<'_> {
    /// List medication administrations.
    ///
    /// Returns a page of medication administrations. Filters combine with AND. Use `next_cursor` to
    /// fetch the next page.
    pub fn list(
        &self,
        params: MedicationAdministrationListParams,
    ) -> Result<Page<MedicationAdministration>> {
        self.client
            .block_on(self.client.inner.medication_administrations().list(params))
    }

    pub fn list_with_options(
        &self,
        params: MedicationAdministrationListParams,
        options: RequestOptions,
    ) -> Result<Page<MedicationAdministration>> {
        self.client.block_on(
            self.client
                .inner
                .medication_administrations()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(
        &self,
        params: MedicationAdministrationListParams,
    ) -> BlockingIter<MedicationAdministration> {
        self.client.iter(
            self.client
                .inner
                .medication_administrations()
                .list_auto_paging(params),
        )
    }

    /// Retrieve a medication administration.
    ///
    /// Returns one medication administration by ID.
    pub fn retrieve(&self, id: &str) -> Result<MedicationAdministration> {
        self.client
            .block_on(self.client.inner.medication_administrations().retrieve(id))
    }

    pub fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<MedicationAdministration> {
        self.client.block_on(
            self.client
                .inner
                .medication_administrations()
                .retrieve_with_options(id, options),
        )
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
/// status; the item-level answers are not included. Read-only. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct OasisAssessments<'a> {
    client: &'a AveeCare,
}

impl OasisAssessments<'_> {
    /// List OASIS assessments.
    ///
    /// Returns a page of OASIS assessments. Filters combine with AND. Use `next_cursor` to fetch
    /// the next page. Available to keys belonging to office staff.
    pub fn list(&self, params: OasisAssessmentListParams) -> Result<Page<OasisAssessment>> {
        self.client
            .block_on(self.client.inner.oasis_assessments().list(params))
    }

    pub fn list_with_options(
        &self,
        params: OasisAssessmentListParams,
        options: RequestOptions,
    ) -> Result<Page<OasisAssessment>> {
        self.client.block_on(
            self.client
                .inner
                .oasis_assessments()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(
        &self,
        params: OasisAssessmentListParams,
    ) -> BlockingIter<OasisAssessment> {
        self.client.iter(
            self.client
                .inner
                .oasis_assessments()
                .list_auto_paging(params),
        )
    }

    /// Retrieve an OASIS assessment.
    ///
    /// Returns one OASIS assessment by ID. Available to keys belonging to office staff.
    pub fn retrieve(&self, id: &str) -> Result<OasisAssessment> {
        self.client
            .block_on(self.client.inner.oasis_assessments().retrieve(id))
    }

    pub fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<OasisAssessment> {
        self.client.block_on(
            self.client
                .inner
                .oasis_assessments()
                .retrieve_with_options(id, options),
        )
    }
}

impl AveeCare {
    /// Caregivers are the aides, nurses and therapists your agency schedules to visit patients.
    pub fn caregivers(&self) -> Caregivers<'_> {
        Caregivers { client: self }
    }
}

/// Caregivers are the aides, nurses and therapists your agency schedules to visit patients.
/// (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Caregivers<'a> {
    client: &'a AveeCare,
}

impl Caregivers<'_> {
    /// List caregivers.
    ///
    /// Returns a page of caregivers. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: CaregiverListParams) -> Result<Page<Caregiver>> {
        self.client
            .block_on(self.client.inner.caregivers().list(params))
    }

    pub fn list_with_options(
        &self,
        params: CaregiverListParams,
        options: RequestOptions,
    ) -> Result<Page<Caregiver>> {
        self.client.block_on(
            self.client
                .inner
                .caregivers()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: CaregiverListParams) -> BlockingIter<Caregiver> {
        self.client
            .iter(self.client.inner.caregivers().list_auto_paging(params))
    }

    /// Create a caregiver.
    ///
    /// Creates a caregiver and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn create(&self, params: CaregiverCreate) -> Result<Caregiver> {
        self.client
            .block_on(self.client.inner.caregivers().create(params))
    }

    pub fn create_with_options(
        &self,
        params: CaregiverCreate,
        options: RequestOptions,
    ) -> Result<Caregiver> {
        self.client.block_on(
            self.client
                .inner
                .caregivers()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a caregiver.
    ///
    /// Returns one caregiver by ID. Available to keys belonging to office staff and caregivers.
    pub fn retrieve(&self, id: &str) -> Result<Caregiver> {
        self.client
            .block_on(self.client.inner.caregivers().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Caregiver> {
        self.client.block_on(
            self.client
                .inner
                .caregivers()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a caregiver.
    ///
    /// Changes the fields you send and returns the updated caregiver. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: CaregiverUpdate) -> Result<Caregiver> {
        self.client
            .block_on(self.client.inner.caregivers().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: CaregiverUpdate,
        options: RequestOptions,
    ) -> Result<Caregiver> {
        self.client.block_on(
            self.client
                .inner
                .caregivers()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a caregiver.
    ///
    /// Deletes a caregiver. The caregiver's past visits and other related records are not deleted.
    /// Available to keys belonging to office staff. Requires a key with `write` scope.
    pub fn delete(&self, id: &str) -> Result<CaregiverDeleted> {
        self.client
            .block_on(self.client.inner.caregivers().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CaregiverDeleted> {
        self.client.block_on(
            self.client
                .inner
                .caregivers()
                .delete_with_options(id, options),
        )
    }
}

impl AveeCare {
    /// Notes your staff keep on a caregiver's profile.
    pub fn caregiver_notes(&self) -> CaregiverNotes<'_> {
        CaregiverNotes { client: self }
    }
}

/// Notes your staff keep on a caregiver's profile. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct CaregiverNotes<'a> {
    client: &'a AveeCare,
}

impl CaregiverNotes<'_> {
    /// List caregiver notes.
    ///
    /// Returns a page of caregiver notes. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: CaregiverNoteListParams) -> Result<Page<CaregiverNote>> {
        self.client
            .block_on(self.client.inner.caregiver_notes().list(params))
    }

    pub fn list_with_options(
        &self,
        params: CaregiverNoteListParams,
        options: RequestOptions,
    ) -> Result<Page<CaregiverNote>> {
        self.client.block_on(
            self.client
                .inner
                .caregiver_notes()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: CaregiverNoteListParams) -> BlockingIter<CaregiverNote> {
        self.client
            .iter(self.client.inner.caregiver_notes().list_auto_paging(params))
    }

    /// Create a caregiver note.
    ///
    /// Creates a caregiver note and returns it. Send an `Idempotency-Key` header to make the
    /// request safe to retry. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn create(&self, params: CaregiverNoteCreate) -> Result<CaregiverNote> {
        self.client
            .block_on(self.client.inner.caregiver_notes().create(params))
    }

    pub fn create_with_options(
        &self,
        params: CaregiverNoteCreate,
        options: RequestOptions,
    ) -> Result<CaregiverNote> {
        self.client.block_on(
            self.client
                .inner
                .caregiver_notes()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a caregiver note.
    ///
    /// Returns one caregiver note by ID. Available to keys belonging to office staff and
    /// caregivers.
    pub fn retrieve(&self, id: &str) -> Result<CaregiverNote> {
        self.client
            .block_on(self.client.inner.caregiver_notes().retrieve(id))
    }

    pub fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CaregiverNote> {
        self.client.block_on(
            self.client
                .inner
                .caregiver_notes()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a caregiver note.
    ///
    /// Changes the fields you send and returns the updated caregiver note. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: CaregiverNoteUpdate) -> Result<CaregiverNote> {
        self.client
            .block_on(self.client.inner.caregiver_notes().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: CaregiverNoteUpdate,
        options: RequestOptions,
    ) -> Result<CaregiverNote> {
        self.client.block_on(
            self.client
                .inner
                .caregiver_notes()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a caregiver note.
    ///
    /// Deletes a caregiver note. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<CaregiverNoteDeleted> {
        self.client
            .block_on(self.client.inner.caregiver_notes().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CaregiverNoteDeleted> {
        self.client.block_on(
            self.client
                .inner
                .caregiver_notes()
                .delete_with_options(id, options),
        )
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
/// dates. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Certifications<'a> {
    client: &'a AveeCare,
}

impl Certifications<'_> {
    /// List certifications.
    ///
    /// Returns a page of certifications. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: CertificationListParams) -> Result<Page<Certification>> {
        self.client
            .block_on(self.client.inner.certifications().list(params))
    }

    pub fn list_with_options(
        &self,
        params: CertificationListParams,
        options: RequestOptions,
    ) -> Result<Page<Certification>> {
        self.client.block_on(
            self.client
                .inner
                .certifications()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: CertificationListParams) -> BlockingIter<Certification> {
        self.client
            .iter(self.client.inner.certifications().list_auto_paging(params))
    }

    /// Create a certification.
    ///
    /// Creates a certification and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn create(&self, params: CertificationCreate) -> Result<Certification> {
        self.client
            .block_on(self.client.inner.certifications().create(params))
    }

    pub fn create_with_options(
        &self,
        params: CertificationCreate,
        options: RequestOptions,
    ) -> Result<Certification> {
        self.client.block_on(
            self.client
                .inner
                .certifications()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a certification.
    ///
    /// Returns one certification by ID. Available to keys belonging to office staff and caregivers.
    pub fn retrieve(&self, id: &str) -> Result<Certification> {
        self.client
            .block_on(self.client.inner.certifications().retrieve(id))
    }

    pub fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Certification> {
        self.client.block_on(
            self.client
                .inner
                .certifications()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a certification.
    ///
    /// Changes the fields you send and returns the updated certification. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: CertificationUpdate) -> Result<Certification> {
        self.client
            .block_on(self.client.inner.certifications().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: CertificationUpdate,
        options: RequestOptions,
    ) -> Result<Certification> {
        self.client.block_on(
            self.client
                .inner
                .certifications()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a certification.
    ///
    /// Deletes a certification. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<CertificationDeleted> {
        self.client
            .block_on(self.client.inner.certifications().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CertificationDeleted> {
        self.client.block_on(
            self.client
                .inner
                .certifications()
                .delete_with_options(id, options),
        )
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
/// or a one-off date range such as time off. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct AvailabilityResource<'a> {
    client: &'a AveeCare,
}

impl AvailabilityResource<'_> {
    /// List availability windows.
    ///
    /// Returns a page of availability windows. Filters combine with AND. Use `next_cursor` to fetch
    /// the next page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: AvailabilityListParams) -> Result<Page<Availability>> {
        self.client
            .block_on(self.client.inner.availability().list(params))
    }

    pub fn list_with_options(
        &self,
        params: AvailabilityListParams,
        options: RequestOptions,
    ) -> Result<Page<Availability>> {
        self.client.block_on(
            self.client
                .inner
                .availability()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: AvailabilityListParams) -> BlockingIter<Availability> {
        self.client
            .iter(self.client.inner.availability().list_auto_paging(params))
    }

    /// Create an availability window.
    ///
    /// Creates an availability window and returns it. Send an `Idempotency-Key` header to make the
    /// request safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    pub fn create(&self, params: AvailabilityCreate) -> Result<Availability> {
        self.client
            .block_on(self.client.inner.availability().create(params))
    }

    pub fn create_with_options(
        &self,
        params: AvailabilityCreate,
        options: RequestOptions,
    ) -> Result<Availability> {
        self.client.block_on(
            self.client
                .inner
                .availability()
                .create_with_options(params, options),
        )
    }

    /// Retrieve an availability window.
    ///
    /// Returns one availability window by ID. Available to keys belonging to office staff and
    /// caregivers.
    pub fn retrieve(&self, id: &str) -> Result<Availability> {
        self.client
            .block_on(self.client.inner.availability().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Availability> {
        self.client.block_on(
            self.client
                .inner
                .availability()
                .retrieve_with_options(id, options),
        )
    }

    /// Update an availability window.
    ///
    /// Changes the fields you send and returns the updated availability window. Send `null` to
    /// clear an optional field. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    pub fn update(&self, id: &str, params: AvailabilityUpdate) -> Result<Availability> {
        self.client
            .block_on(self.client.inner.availability().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: AvailabilityUpdate,
        options: RequestOptions,
    ) -> Result<Availability> {
        self.client.block_on(
            self.client
                .inner
                .availability()
                .update_with_options(id, params, options),
        )
    }

    /// Delete an availability window.
    ///
    /// Deletes an availability window. Available to keys belonging to office staff and caregivers.
    /// Requires a key with `write` scope.
    pub fn delete(&self, id: &str) -> Result<AvailabilityDeleted> {
        self.client
            .block_on(self.client.inner.availability().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<AvailabilityDeleted> {
        self.client.block_on(
            self.client
                .inner
                .availability()
                .delete_with_options(id, options),
        )
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
/// (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Specializations<'a> {
    client: &'a AveeCare,
}

impl Specializations<'_> {
    /// List specializations.
    ///
    /// Returns a page of specializations. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: SpecializationListParams) -> Result<Page<Specialization>> {
        self.client
            .block_on(self.client.inner.specializations().list(params))
    }

    pub fn list_with_options(
        &self,
        params: SpecializationListParams,
        options: RequestOptions,
    ) -> Result<Page<Specialization>> {
        self.client.block_on(
            self.client
                .inner
                .specializations()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(
        &self,
        params: SpecializationListParams,
    ) -> BlockingIter<Specialization> {
        self.client
            .iter(self.client.inner.specializations().list_auto_paging(params))
    }

    /// Create a specialization.
    ///
    /// Creates a specialization and returns it. Send an `Idempotency-Key` header to make the
    /// request safe to retry. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn create(&self, params: SpecializationCreate) -> Result<Specialization> {
        self.client
            .block_on(self.client.inner.specializations().create(params))
    }

    pub fn create_with_options(
        &self,
        params: SpecializationCreate,
        options: RequestOptions,
    ) -> Result<Specialization> {
        self.client.block_on(
            self.client
                .inner
                .specializations()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a specialization.
    ///
    /// Returns one specialization by ID. Available to keys belonging to office staff and
    /// caregivers.
    pub fn retrieve(&self, id: &str) -> Result<Specialization> {
        self.client
            .block_on(self.client.inner.specializations().retrieve(id))
    }

    pub fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Specialization> {
        self.client.block_on(
            self.client
                .inner
                .specializations()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a specialization.
    ///
    /// Changes the fields you send and returns the updated specialization. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: SpecializationUpdate) -> Result<Specialization> {
        self.client
            .block_on(self.client.inner.specializations().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: SpecializationUpdate,
        options: RequestOptions,
    ) -> Result<Specialization> {
        self.client.block_on(
            self.client
                .inner
                .specializations()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a specialization.
    ///
    /// Deletes a specialization. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<SpecializationDeleted> {
        self.client
            .block_on(self.client.inner.specializations().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<SpecializationDeleted> {
        self.client.block_on(
            self.client
                .inner
                .specializations()
                .delete_with_options(id, options),
        )
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
/// hired as caregivers. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Applicants<'a> {
    client: &'a AveeCare,
}

impl Applicants<'_> {
    /// List applicants.
    ///
    /// Returns a page of applicants. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff.
    pub fn list(&self, params: ApplicantListParams) -> Result<Page<Applicant>> {
        self.client
            .block_on(self.client.inner.applicants().list(params))
    }

    pub fn list_with_options(
        &self,
        params: ApplicantListParams,
        options: RequestOptions,
    ) -> Result<Page<Applicant>> {
        self.client.block_on(
            self.client
                .inner
                .applicants()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: ApplicantListParams) -> BlockingIter<Applicant> {
        self.client
            .iter(self.client.inner.applicants().list_auto_paging(params))
    }

    /// Create an applicant.
    ///
    /// Creates an applicant and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn create(&self, params: ApplicantCreate) -> Result<Applicant> {
        self.client
            .block_on(self.client.inner.applicants().create(params))
    }

    pub fn create_with_options(
        &self,
        params: ApplicantCreate,
        options: RequestOptions,
    ) -> Result<Applicant> {
        self.client.block_on(
            self.client
                .inner
                .applicants()
                .create_with_options(params, options),
        )
    }

    /// Retrieve an applicant.
    ///
    /// Returns one applicant by ID. Available to keys belonging to office staff.
    pub fn retrieve(&self, id: &str) -> Result<Applicant> {
        self.client
            .block_on(self.client.inner.applicants().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Applicant> {
        self.client.block_on(
            self.client
                .inner
                .applicants()
                .retrieve_with_options(id, options),
        )
    }

    /// Update an applicant.
    ///
    /// Changes the fields you send and returns the updated applicant. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: ApplicantUpdate) -> Result<Applicant> {
        self.client
            .block_on(self.client.inner.applicants().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: ApplicantUpdate,
        options: RequestOptions,
    ) -> Result<Applicant> {
        self.client.block_on(
            self.client
                .inner
                .applicants()
                .update_with_options(id, params, options),
        )
    }

    /// Delete an applicant.
    ///
    /// Deletes an applicant. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<ApplicantDeleted> {
        self.client
            .block_on(self.client.inner.applicants().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ApplicantDeleted> {
        self.client.block_on(
            self.client
                .inner
                .applicants()
                .delete_with_options(id, options),
        )
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
/// AveeCare. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Visits<'a> {
    client: &'a AveeCare,
}

impl Visits<'_> {
    /// List visits.
    ///
    /// Returns a page of visits, ordered by scheduled start, earliest first. Filters combine with
    /// AND. Use `next_cursor` to fetch the next page.
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
    /// Creates a visit and returns it. Send an `Idempotency-Key` header to make the request safe to
    /// retry. Available to keys belonging to office staff. Requires a key with `write` scope.
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
    /// Returns one visit by ID.
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
    /// Changes the fields you send and returns the updated visit. Send `null` to clear an optional
    /// field. Available to keys belonging to office staff and caregivers. With a caregiver's or
    /// patient's key, a saved record that key cannot read back is answered with only `object` and
    /// `id`. Requires a key with `write` scope.
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
    /// Deletes a visit. The visit moves to the schedule's recycle bin in AveeCare, where staff can
    /// restore it. Available to keys belonging to office staff. Requires a key with `write` scope.
    pub fn delete(&self, id: &str) -> Result<VisitDeleted> {
        self.client.block_on(self.client.inner.visits().delete(id))
    }

    pub fn delete_with_options(&self, id: &str, options: RequestOptions) -> Result<VisitDeleted> {
        self.client
            .block_on(self.client.inner.visits().delete_with_options(id, options))
    }
}

impl AveeCare {
    /// Notes written about a visit by the caregiver or office staff.
    pub fn visit_notes(&self) -> VisitNotes<'_> {
        VisitNotes { client: self }
    }
}

/// Notes written about a visit by the caregiver or office staff. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct VisitNotes<'a> {
    client: &'a AveeCare,
}

impl VisitNotes<'_> {
    /// List visit notes.
    ///
    /// Returns a page of visit notes. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: VisitNoteListParams) -> Result<Page<VisitNote>> {
        self.client
            .block_on(self.client.inner.visit_notes().list(params))
    }

    pub fn list_with_options(
        &self,
        params: VisitNoteListParams,
        options: RequestOptions,
    ) -> Result<Page<VisitNote>> {
        self.client.block_on(
            self.client
                .inner
                .visit_notes()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: VisitNoteListParams) -> BlockingIter<VisitNote> {
        self.client
            .iter(self.client.inner.visit_notes().list_auto_paging(params))
    }

    /// Create a visit note.
    ///
    /// Creates a visit note and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    pub fn create(&self, params: VisitNoteCreate) -> Result<VisitNote> {
        self.client
            .block_on(self.client.inner.visit_notes().create(params))
    }

    pub fn create_with_options(
        &self,
        params: VisitNoteCreate,
        options: RequestOptions,
    ) -> Result<VisitNote> {
        self.client.block_on(
            self.client
                .inner
                .visit_notes()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a visit note.
    ///
    /// Returns one visit note by ID. Available to keys belonging to office staff and caregivers.
    pub fn retrieve(&self, id: &str) -> Result<VisitNote> {
        self.client
            .block_on(self.client.inner.visit_notes().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<VisitNote> {
        self.client.block_on(
            self.client
                .inner
                .visit_notes()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a visit note.
    ///
    /// Changes the fields you send and returns the updated visit note. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    pub fn update(&self, id: &str, params: VisitNoteUpdate) -> Result<VisitNote> {
        self.client
            .block_on(self.client.inner.visit_notes().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: VisitNoteUpdate,
        options: RequestOptions,
    ) -> Result<VisitNote> {
        self.client.block_on(
            self.client
                .inner
                .visit_notes()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a visit note.
    ///
    /// Deletes a visit note. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<VisitNoteDeleted> {
        self.client
            .block_on(self.client.inner.visit_notes().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<VisitNoteDeleted> {
        self.client.block_on(
            self.client
                .inner
                .visit_notes()
                .delete_with_options(id, options),
        )
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
/// (blocking)
#[derive(Debug, Clone, Copy)]
pub struct VisitRequests<'a> {
    client: &'a AveeCare,
}

impl VisitRequests<'_> {
    /// List visit requests.
    ///
    /// Returns a page of visit requests. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: VisitRequestListParams) -> Result<Page<VisitRequest>> {
        self.client
            .block_on(self.client.inner.visit_requests().list(params))
    }

    pub fn list_with_options(
        &self,
        params: VisitRequestListParams,
        options: RequestOptions,
    ) -> Result<Page<VisitRequest>> {
        self.client.block_on(
            self.client
                .inner
                .visit_requests()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: VisitRequestListParams) -> BlockingIter<VisitRequest> {
        self.client
            .iter(self.client.inner.visit_requests().list_auto_paging(params))
    }

    /// Create a visit request.
    ///
    /// Creates a visit request and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    pub fn create(&self, params: VisitRequestCreate) -> Result<VisitRequest> {
        self.client
            .block_on(self.client.inner.visit_requests().create(params))
    }

    pub fn create_with_options(
        &self,
        params: VisitRequestCreate,
        options: RequestOptions,
    ) -> Result<VisitRequest> {
        self.client.block_on(
            self.client
                .inner
                .visit_requests()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a visit request.
    ///
    /// Returns one visit request by ID. Available to keys belonging to office staff and caregivers.
    pub fn retrieve(&self, id: &str) -> Result<VisitRequest> {
        self.client
            .block_on(self.client.inner.visit_requests().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<VisitRequest> {
        self.client.block_on(
            self.client
                .inner
                .visit_requests()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a visit request.
    ///
    /// Changes the fields you send and returns the updated visit request. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    pub fn update(&self, id: &str, params: VisitRequestUpdate) -> Result<VisitRequest> {
        self.client
            .block_on(self.client.inner.visit_requests().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: VisitRequestUpdate,
        options: RequestOptions,
    ) -> Result<VisitRequest> {
        self.client.block_on(
            self.client
                .inner
                .visit_requests()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a visit request.
    ///
    /// Deletes a visit request. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<VisitRequestDeleted> {
        self.client
            .block_on(self.client.inner.visit_requests().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<VisitRequestDeleted> {
        self.client.block_on(
            self.client
                .inner
                .visit_requests()
                .delete_with_options(id, options),
        )
    }
}

impl AveeCare {
    /// Open shifts are unassigned visits offered to caregivers so one of them can claim it.
    pub fn open_shifts(&self) -> OpenShifts<'_> {
        OpenShifts { client: self }
    }
}

/// Open shifts are unassigned visits offered to caregivers so one of them can claim it. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct OpenShifts<'a> {
    client: &'a AveeCare,
}

impl OpenShifts<'_> {
    /// List open shifts.
    ///
    /// Returns a page of open shifts. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: OpenShiftListParams) -> Result<Page<OpenShift>> {
        self.client
            .block_on(self.client.inner.open_shifts().list(params))
    }

    pub fn list_with_options(
        &self,
        params: OpenShiftListParams,
        options: RequestOptions,
    ) -> Result<Page<OpenShift>> {
        self.client.block_on(
            self.client
                .inner
                .open_shifts()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: OpenShiftListParams) -> BlockingIter<OpenShift> {
        self.client
            .iter(self.client.inner.open_shifts().list_auto_paging(params))
    }

    /// Create an open shift.
    ///
    /// Creates an open shift and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn create(&self, params: OpenShiftCreate) -> Result<OpenShift> {
        self.client
            .block_on(self.client.inner.open_shifts().create(params))
    }

    pub fn create_with_options(
        &self,
        params: OpenShiftCreate,
        options: RequestOptions,
    ) -> Result<OpenShift> {
        self.client.block_on(
            self.client
                .inner
                .open_shifts()
                .create_with_options(params, options),
        )
    }

    /// Retrieve an open shift.
    ///
    /// Returns one open shift by ID. Available to keys belonging to office staff and caregivers.
    pub fn retrieve(&self, id: &str) -> Result<OpenShift> {
        self.client
            .block_on(self.client.inner.open_shifts().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<OpenShift> {
        self.client.block_on(
            self.client
                .inner
                .open_shifts()
                .retrieve_with_options(id, options),
        )
    }

    /// Update an open shift.
    ///
    /// Changes the fields you send and returns the updated open shift. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: OpenShiftUpdate) -> Result<OpenShift> {
        self.client
            .block_on(self.client.inner.open_shifts().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: OpenShiftUpdate,
        options: RequestOptions,
    ) -> Result<OpenShift> {
        self.client.block_on(
            self.client
                .inner
                .open_shifts()
                .update_with_options(id, params, options),
        )
    }

    /// Delete an open shift.
    ///
    /// Deletes an open shift. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<OpenShiftDeleted> {
        self.client
            .block_on(self.client.inner.open_shifts().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<OpenShiftDeleted> {
        self.client.block_on(
            self.client
                .inner
                .open_shifts()
                .delete_with_options(id, options),
        )
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
/// office approve. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct ShiftSwaps<'a> {
    client: &'a AveeCare,
}

impl ShiftSwaps<'_> {
    /// List shift swaps.
    ///
    /// Returns a page of shift swaps. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: ShiftSwapListParams) -> Result<Page<ShiftSwap>> {
        self.client
            .block_on(self.client.inner.shift_swaps().list(params))
    }

    pub fn list_with_options(
        &self,
        params: ShiftSwapListParams,
        options: RequestOptions,
    ) -> Result<Page<ShiftSwap>> {
        self.client.block_on(
            self.client
                .inner
                .shift_swaps()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: ShiftSwapListParams) -> BlockingIter<ShiftSwap> {
        self.client
            .iter(self.client.inner.shift_swaps().list_auto_paging(params))
    }

    /// Create a shift swap.
    ///
    /// Creates a shift swap and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    pub fn create(&self, params: ShiftSwapCreate) -> Result<ShiftSwap> {
        self.client
            .block_on(self.client.inner.shift_swaps().create(params))
    }

    pub fn create_with_options(
        &self,
        params: ShiftSwapCreate,
        options: RequestOptions,
    ) -> Result<ShiftSwap> {
        self.client.block_on(
            self.client
                .inner
                .shift_swaps()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a shift swap.
    ///
    /// Returns one shift swap by ID. Available to keys belonging to office staff and caregivers.
    pub fn retrieve(&self, id: &str) -> Result<ShiftSwap> {
        self.client
            .block_on(self.client.inner.shift_swaps().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<ShiftSwap> {
        self.client.block_on(
            self.client
                .inner
                .shift_swaps()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a shift swap.
    ///
    /// Changes the fields you send and returns the updated shift swap. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    pub fn update(&self, id: &str, params: ShiftSwapUpdate) -> Result<ShiftSwap> {
        self.client
            .block_on(self.client.inner.shift_swaps().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: ShiftSwapUpdate,
        options: RequestOptions,
    ) -> Result<ShiftSwap> {
        self.client.block_on(
            self.client
                .inner
                .shift_swaps()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a shift swap.
    ///
    /// Deletes a shift swap. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<ShiftSwapDeleted> {
        self.client
            .block_on(self.client.inner.shift_swaps().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ShiftSwapDeleted> {
        self.client.block_on(
            self.client
                .inner
                .shift_swaps()
                .delete_with_options(id, options),
        )
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
/// (blocking)
#[derive(Debug, Clone, Copy)]
pub struct CarePlans<'a> {
    client: &'a AveeCare,
}

impl CarePlans<'_> {
    /// List care plans.
    ///
    /// Returns a page of care plans. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: CarePlanListParams) -> Result<Page<CarePlan>> {
        self.client
            .block_on(self.client.inner.care_plans().list(params))
    }

    pub fn list_with_options(
        &self,
        params: CarePlanListParams,
        options: RequestOptions,
    ) -> Result<Page<CarePlan>> {
        self.client.block_on(
            self.client
                .inner
                .care_plans()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: CarePlanListParams) -> BlockingIter<CarePlan> {
        self.client
            .iter(self.client.inner.care_plans().list_auto_paging(params))
    }

    /// Create a care plan.
    ///
    /// Creates a care plan and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn create(&self, params: CarePlanCreate) -> Result<CarePlan> {
        self.client
            .block_on(self.client.inner.care_plans().create(params))
    }

    pub fn create_with_options(
        &self,
        params: CarePlanCreate,
        options: RequestOptions,
    ) -> Result<CarePlan> {
        self.client.block_on(
            self.client
                .inner
                .care_plans()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a care plan.
    ///
    /// Returns one care plan by ID. Available to keys belonging to office staff and caregivers.
    pub fn retrieve(&self, id: &str) -> Result<CarePlan> {
        self.client
            .block_on(self.client.inner.care_plans().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<CarePlan> {
        self.client.block_on(
            self.client
                .inner
                .care_plans()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a care plan.
    ///
    /// Changes the fields you send and returns the updated care plan. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: CarePlanUpdate) -> Result<CarePlan> {
        self.client
            .block_on(self.client.inner.care_plans().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: CarePlanUpdate,
        options: RequestOptions,
    ) -> Result<CarePlan> {
        self.client.block_on(
            self.client
                .inner
                .care_plans()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a care plan.
    ///
    /// Deletes a care plan. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<CarePlanDeleted> {
        self.client
            .block_on(self.client.inner.care_plans().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<CarePlanDeleted> {
        self.client.block_on(
            self.client
                .inner
                .care_plans()
                .delete_with_options(id, options),
        )
    }
}

impl AveeCare {
    /// Progress notes are the clinical summaries caregivers write after a visit.
    pub fn progress_notes(&self) -> ProgressNotes<'_> {
        ProgressNotes { client: self }
    }
}

/// Progress notes are the clinical summaries caregivers write after a visit. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct ProgressNotes<'a> {
    client: &'a AveeCare,
}

impl ProgressNotes<'_> {
    /// List progress notes.
    ///
    /// Returns a page of progress notes. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: ProgressNoteListParams) -> Result<Page<ProgressNote>> {
        self.client
            .block_on(self.client.inner.progress_notes().list(params))
    }

    pub fn list_with_options(
        &self,
        params: ProgressNoteListParams,
        options: RequestOptions,
    ) -> Result<Page<ProgressNote>> {
        self.client.block_on(
            self.client
                .inner
                .progress_notes()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: ProgressNoteListParams) -> BlockingIter<ProgressNote> {
        self.client
            .iter(self.client.inner.progress_notes().list_auto_paging(params))
    }

    /// Create a progress note.
    ///
    /// Creates a progress note and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    pub fn create(&self, params: ProgressNoteCreate) -> Result<ProgressNote> {
        self.client
            .block_on(self.client.inner.progress_notes().create(params))
    }

    pub fn create_with_options(
        &self,
        params: ProgressNoteCreate,
        options: RequestOptions,
    ) -> Result<ProgressNote> {
        self.client.block_on(
            self.client
                .inner
                .progress_notes()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a progress note.
    ///
    /// Returns one progress note by ID. Available to keys belonging to office staff and caregivers.
    pub fn retrieve(&self, id: &str) -> Result<ProgressNote> {
        self.client
            .block_on(self.client.inner.progress_notes().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<ProgressNote> {
        self.client.block_on(
            self.client
                .inner
                .progress_notes()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a progress note.
    ///
    /// Changes the fields you send and returns the updated progress note. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: ProgressNoteUpdate) -> Result<ProgressNote> {
        self.client
            .block_on(self.client.inner.progress_notes().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: ProgressNoteUpdate,
        options: RequestOptions,
    ) -> Result<ProgressNote> {
        self.client.block_on(
            self.client
                .inner
                .progress_notes()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a progress note.
    ///
    /// Deletes a progress note. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<ProgressNoteDeleted> {
        self.client
            .block_on(self.client.inner.progress_notes().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ProgressNoteDeleted> {
        self.client.block_on(
            self.client
                .inner
                .progress_notes()
                .delete_with_options(id, options),
        )
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
/// what dates. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Authorizations<'a> {
    client: &'a AveeCare,
}

impl Authorizations<'_> {
    /// List authorizations.
    ///
    /// Returns a page of authorizations. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff.
    pub fn list(&self, params: AuthorizationListParams) -> Result<Page<Authorization>> {
        self.client
            .block_on(self.client.inner.authorizations().list(params))
    }

    pub fn list_with_options(
        &self,
        params: AuthorizationListParams,
        options: RequestOptions,
    ) -> Result<Page<Authorization>> {
        self.client.block_on(
            self.client
                .inner
                .authorizations()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: AuthorizationListParams) -> BlockingIter<Authorization> {
        self.client
            .iter(self.client.inner.authorizations().list_auto_paging(params))
    }

    /// Create an authorization.
    ///
    /// Creates an authorization and returns it. Send an `Idempotency-Key` header to make the
    /// request safe to retry. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn create(&self, params: AuthorizationCreate) -> Result<Authorization> {
        self.client
            .block_on(self.client.inner.authorizations().create(params))
    }

    pub fn create_with_options(
        &self,
        params: AuthorizationCreate,
        options: RequestOptions,
    ) -> Result<Authorization> {
        self.client.block_on(
            self.client
                .inner
                .authorizations()
                .create_with_options(params, options),
        )
    }

    /// Retrieve an authorization.
    ///
    /// Returns one authorization by ID. Available to keys belonging to office staff.
    pub fn retrieve(&self, id: &str) -> Result<Authorization> {
        self.client
            .block_on(self.client.inner.authorizations().retrieve(id))
    }

    pub fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<Authorization> {
        self.client.block_on(
            self.client
                .inner
                .authorizations()
                .retrieve_with_options(id, options),
        )
    }

    /// Update an authorization.
    ///
    /// Changes the fields you send and returns the updated authorization. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: AuthorizationUpdate) -> Result<Authorization> {
        self.client
            .block_on(self.client.inner.authorizations().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: AuthorizationUpdate,
        options: RequestOptions,
    ) -> Result<Authorization> {
        self.client.block_on(
            self.client
                .inner
                .authorizations()
                .update_with_options(id, params, options),
        )
    }

    /// Delete an authorization.
    ///
    /// Deletes an authorization. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<AuthorizationDeleted> {
        self.client
            .block_on(self.client.inner.authorizations().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<AuthorizationDeleted> {
        self.client.block_on(
            self.client
                .inner
                .authorizations()
                .delete_with_options(id, options),
        )
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
/// for regulatory reporting. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Incidents<'a> {
    client: &'a AveeCare,
}

impl Incidents<'_> {
    /// List incidents.
    ///
    /// Returns a page of incidents. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: IncidentListParams) -> Result<Page<Incident>> {
        self.client
            .block_on(self.client.inner.incidents().list(params))
    }

    pub fn list_with_options(
        &self,
        params: IncidentListParams,
        options: RequestOptions,
    ) -> Result<Page<Incident>> {
        self.client.block_on(
            self.client
                .inner
                .incidents()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: IncidentListParams) -> BlockingIter<Incident> {
        self.client
            .iter(self.client.inner.incidents().list_auto_paging(params))
    }

    /// Create an incident.
    ///
    /// Creates an incident and returns it. Send an `Idempotency-Key` header to make the request
    /// safe to retry. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    pub fn create(&self, params: IncidentCreate) -> Result<Incident> {
        self.client
            .block_on(self.client.inner.incidents().create(params))
    }

    pub fn create_with_options(
        &self,
        params: IncidentCreate,
        options: RequestOptions,
    ) -> Result<Incident> {
        self.client.block_on(
            self.client
                .inner
                .incidents()
                .create_with_options(params, options),
        )
    }

    /// Retrieve an incident.
    ///
    /// Returns one incident by ID. Available to keys belonging to office staff and caregivers.
    pub fn retrieve(&self, id: &str) -> Result<Incident> {
        self.client
            .block_on(self.client.inner.incidents().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Incident> {
        self.client.block_on(
            self.client
                .inner
                .incidents()
                .retrieve_with_options(id, options),
        )
    }

    /// Update an incident.
    ///
    /// Changes the fields you send and returns the updated incident. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff and caregivers. With a
    /// caregiver's or patient's key, a saved record that key cannot read back is answered with only
    /// `object` and `id`. Requires a key with `write` scope.
    pub fn update(&self, id: &str, params: IncidentUpdate) -> Result<Incident> {
        self.client
            .block_on(self.client.inner.incidents().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: IncidentUpdate,
        options: RequestOptions,
    ) -> Result<Incident> {
        self.client.block_on(
            self.client
                .inner
                .incidents()
                .update_with_options(id, params, options),
        )
    }

    /// Delete an incident.
    ///
    /// Deletes an incident. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<IncidentDeleted> {
        self.client
            .block_on(self.client.inner.incidents().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<IncidentDeleted> {
        self.client.block_on(
            self.client
                .inner
                .incidents()
                .delete_with_options(id, options),
        )
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
/// compliment or an attendance issue. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Events<'a> {
    client: &'a AveeCare,
}

impl Events<'_> {
    /// List events.
    ///
    /// Returns a page of events. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff.
    pub fn list(&self, params: EventListParams) -> Result<Page<Event>> {
        self.client
            .block_on(self.client.inner.events().list(params))
    }

    pub fn list_with_options(
        &self,
        params: EventListParams,
        options: RequestOptions,
    ) -> Result<Page<Event>> {
        self.client.block_on(
            self.client
                .inner
                .events()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: EventListParams) -> BlockingIter<Event> {
        self.client
            .iter(self.client.inner.events().list_auto_paging(params))
    }

    /// Create an event.
    ///
    /// Creates an event and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
    pub fn create(&self, params: EventCreate) -> Result<Event> {
        self.client
            .block_on(self.client.inner.events().create(params))
    }

    pub fn create_with_options(
        &self,
        params: EventCreate,
        options: RequestOptions,
    ) -> Result<Event> {
        self.client.block_on(
            self.client
                .inner
                .events()
                .create_with_options(params, options),
        )
    }

    /// Retrieve an event.
    ///
    /// Returns one event by ID. Available to keys belonging to office staff.
    pub fn retrieve(&self, id: &str) -> Result<Event> {
        self.client
            .block_on(self.client.inner.events().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Event> {
        self.client.block_on(
            self.client
                .inner
                .events()
                .retrieve_with_options(id, options),
        )
    }

    /// Update an event.
    ///
    /// Changes the fields you send and returns the updated event. Send `null` to clear an optional
    /// field. Available to keys belonging to office staff. Requires a key with `write` scope.
    pub fn update(&self, id: &str, params: EventUpdate) -> Result<Event> {
        self.client
            .block_on(self.client.inner.events().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: EventUpdate,
        options: RequestOptions,
    ) -> Result<Event> {
        self.client.block_on(
            self.client
                .inner
                .events()
                .update_with_options(id, params, options),
        )
    }

    /// Delete an event.
    ///
    /// Deletes an event. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn delete(&self, id: &str) -> Result<EventDeleted> {
        self.client.block_on(self.client.inner.events().delete(id))
    }

    pub fn delete_with_options(&self, id: &str, options: RequestOptions) -> Result<EventDeleted> {
        self.client
            .block_on(self.client.inner.events().delete_with_options(id, options))
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
/// (blocking)
#[derive(Debug, Clone, Copy)]
pub struct ReferralSources<'a> {
    client: &'a AveeCare,
}

impl ReferralSources<'_> {
    /// List referral sources.
    ///
    /// Returns a page of referral sources. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff.
    pub fn list(&self, params: ReferralSourceListParams) -> Result<Page<ReferralSource>> {
        self.client
            .block_on(self.client.inner.referral_sources().list(params))
    }

    pub fn list_with_options(
        &self,
        params: ReferralSourceListParams,
        options: RequestOptions,
    ) -> Result<Page<ReferralSource>> {
        self.client.block_on(
            self.client
                .inner
                .referral_sources()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(
        &self,
        params: ReferralSourceListParams,
    ) -> BlockingIter<ReferralSource> {
        self.client.iter(
            self.client
                .inner
                .referral_sources()
                .list_auto_paging(params),
        )
    }

    /// Create a referral source.
    ///
    /// Creates a referral source and returns it. Send an `Idempotency-Key` header to make the
    /// request safe to retry. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn create(&self, params: ReferralSourceCreate) -> Result<ReferralSource> {
        self.client
            .block_on(self.client.inner.referral_sources().create(params))
    }

    pub fn create_with_options(
        &self,
        params: ReferralSourceCreate,
        options: RequestOptions,
    ) -> Result<ReferralSource> {
        self.client.block_on(
            self.client
                .inner
                .referral_sources()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a referral source.
    ///
    /// Returns one referral source by ID. Available to keys belonging to office staff.
    pub fn retrieve(&self, id: &str) -> Result<ReferralSource> {
        self.client
            .block_on(self.client.inner.referral_sources().retrieve(id))
    }

    pub fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ReferralSource> {
        self.client.block_on(
            self.client
                .inner
                .referral_sources()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a referral source.
    ///
    /// Changes the fields you send and returns the updated referral source. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: ReferralSourceUpdate) -> Result<ReferralSource> {
        self.client
            .block_on(self.client.inner.referral_sources().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: ReferralSourceUpdate,
        options: RequestOptions,
    ) -> Result<ReferralSource> {
        self.client.block_on(
            self.client
                .inner
                .referral_sources()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a referral source.
    ///
    /// Deletes a referral source. Available to keys belonging to office staff. Requires a key with
    /// `write` scope.
    pub fn delete(&self, id: &str) -> Result<ReferralSourceDeleted> {
        self.client
            .block_on(self.client.inner.referral_sources().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ReferralSourceDeleted> {
        self.client.block_on(
            self.client
                .inner
                .referral_sources()
                .delete_with_options(id, options),
        )
    }
}

impl AveeCare {
    /// Individual referrals of a prospective client, tracked from first contact to admission.
    pub fn referrals(&self) -> Referrals<'_> {
        Referrals { client: self }
    }
}

/// Individual referrals of a prospective client, tracked from first contact to admission.
/// (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Referrals<'a> {
    client: &'a AveeCare,
}

impl Referrals<'_> {
    /// List referrals.
    ///
    /// Returns a page of referrals. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff.
    pub fn list(&self, params: ReferralListParams) -> Result<Page<Referral>> {
        self.client
            .block_on(self.client.inner.referrals().list(params))
    }

    pub fn list_with_options(
        &self,
        params: ReferralListParams,
        options: RequestOptions,
    ) -> Result<Page<Referral>> {
        self.client.block_on(
            self.client
                .inner
                .referrals()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: ReferralListParams) -> BlockingIter<Referral> {
        self.client
            .iter(self.client.inner.referrals().list_auto_paging(params))
    }

    /// Create a referral.
    ///
    /// Creates a referral and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
    pub fn create(&self, params: ReferralCreate) -> Result<Referral> {
        self.client
            .block_on(self.client.inner.referrals().create(params))
    }

    pub fn create_with_options(
        &self,
        params: ReferralCreate,
        options: RequestOptions,
    ) -> Result<Referral> {
        self.client.block_on(
            self.client
                .inner
                .referrals()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a referral.
    ///
    /// Returns one referral by ID. Available to keys belonging to office staff.
    pub fn retrieve(&self, id: &str) -> Result<Referral> {
        self.client
            .block_on(self.client.inner.referrals().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Referral> {
        self.client.block_on(
            self.client
                .inner
                .referrals()
                .retrieve_with_options(id, options),
        )
    }

    /// Update a referral.
    ///
    /// Changes the fields you send and returns the updated referral. Send `null` to clear an
    /// optional field. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn update(&self, id: &str, params: ReferralUpdate) -> Result<Referral> {
        self.client
            .block_on(self.client.inner.referrals().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: ReferralUpdate,
        options: RequestOptions,
    ) -> Result<Referral> {
        self.client.block_on(
            self.client
                .inner
                .referrals()
                .update_with_options(id, params, options),
        )
    }

    /// Delete a referral.
    ///
    /// Deletes a referral. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn delete(&self, id: &str) -> Result<ReferralDeleted> {
        self.client
            .block_on(self.client.inner.referrals().delete(id))
    }

    pub fn delete_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<ReferralDeleted> {
        self.client.block_on(
            self.client
                .inner
                .referrals()
                .delete_with_options(id, options),
        )
    }
}

impl AveeCare {
    /// Invoices billed to patients and their families for private-pay care. Read-only.
    pub fn invoices(&self) -> Invoices<'_> {
        Invoices { client: self }
    }
}

/// Invoices billed to patients and their families for private-pay care. Read-only. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Invoices<'a> {
    client: &'a AveeCare,
}

impl Invoices<'_> {
    /// List invoices.
    ///
    /// Returns a page of invoices. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff.
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

    /// Retrieve an invoice.
    ///
    /// Returns one invoice by ID. Available to keys belonging to office staff.
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
    /// The individual charges on an invoice. Read-only.
    pub fn invoice_line_items(&self) -> InvoiceLineItems<'_> {
        InvoiceLineItems { client: self }
    }
}

/// The individual charges on an invoice. Read-only. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct InvoiceLineItems<'a> {
    client: &'a AveeCare,
}

impl InvoiceLineItems<'_> {
    /// List invoice line items.
    ///
    /// Returns a page of invoice line items. Filters combine with AND. Use `next_cursor` to fetch
    /// the next page. Available to keys belonging to office staff.
    pub fn list(&self, params: InvoiceLineItemListParams) -> Result<Page<InvoiceLineItem>> {
        self.client
            .block_on(self.client.inner.invoice_line_items().list(params))
    }

    pub fn list_with_options(
        &self,
        params: InvoiceLineItemListParams,
        options: RequestOptions,
    ) -> Result<Page<InvoiceLineItem>> {
        self.client.block_on(
            self.client
                .inner
                .invoice_line_items()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(
        &self,
        params: InvoiceLineItemListParams,
    ) -> BlockingIter<InvoiceLineItem> {
        self.client.iter(
            self.client
                .inner
                .invoice_line_items()
                .list_auto_paging(params),
        )
    }

    /// Retrieve an invoice line item.
    ///
    /// Returns one invoice line item by ID. Available to keys belonging to office staff.
    pub fn retrieve(&self, id: &str) -> Result<InvoiceLineItem> {
        self.client
            .block_on(self.client.inner.invoice_line_items().retrieve(id))
    }

    pub fn retrieve_with_options(
        &self,
        id: &str,
        options: RequestOptions,
    ) -> Result<InvoiceLineItem> {
        self.client.block_on(
            self.client
                .inner
                .invoice_line_items()
                .retrieve_with_options(id, options),
        )
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
/// Read-only. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Claims<'a> {
    client: &'a AveeCare,
}

impl Claims<'_> {
    /// List claims.
    ///
    /// Returns a page of claims. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff.
    pub fn list(&self, params: ClaimListParams) -> Result<Page<Claim>> {
        self.client
            .block_on(self.client.inner.claims().list(params))
    }

    pub fn list_with_options(
        &self,
        params: ClaimListParams,
        options: RequestOptions,
    ) -> Result<Page<Claim>> {
        self.client.block_on(
            self.client
                .inner
                .claims()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: ClaimListParams) -> BlockingIter<Claim> {
        self.client
            .iter(self.client.inner.claims().list_auto_paging(params))
    }

    /// Retrieve a claim.
    ///
    /// Returns one claim by ID. Available to keys belonging to office staff.
    pub fn retrieve(&self, id: &str) -> Result<Claim> {
        self.client
            .block_on(self.client.inner.claims().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Claim> {
        self.client.block_on(
            self.client
                .inner
                .claims()
                .retrieve_with_options(id, options),
        )
    }
}

impl AveeCare {
    /// One caregiver's hours and pay for one pay period. Read-only.
    pub fn payroll_entries(&self) -> PayrollEntries<'_> {
        PayrollEntries { client: self }
    }
}

/// One caregiver's hours and pay for one pay period. Read-only. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct PayrollEntries<'a> {
    client: &'a AveeCare,
}

impl PayrollEntries<'_> {
    /// List payroll entries.
    ///
    /// Returns a page of payroll entries. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page. Available to keys belonging to office staff.
    pub fn list(&self, params: PayrollEntryListParams) -> Result<Page<PayrollEntry>> {
        self.client
            .block_on(self.client.inner.payroll_entries().list(params))
    }

    pub fn list_with_options(
        &self,
        params: PayrollEntryListParams,
        options: RequestOptions,
    ) -> Result<Page<PayrollEntry>> {
        self.client.block_on(
            self.client
                .inner
                .payroll_entries()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: PayrollEntryListParams) -> BlockingIter<PayrollEntry> {
        self.client
            .iter(self.client.inner.payroll_entries().list_auto_paging(params))
    }

    /// Retrieve a payroll entry.
    ///
    /// Returns one payroll entry by ID. Available to keys belonging to office staff.
    pub fn retrieve(&self, id: &str) -> Result<PayrollEntry> {
        self.client
            .block_on(self.client.inner.payroll_entries().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<PayrollEntry> {
        self.client.block_on(
            self.client
                .inner
                .payroll_entries()
                .retrieve_with_options(id, options),
        )
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
/// Read-only; start conversations in AveeCare. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Conversations<'a> {
    client: &'a AveeCare,
}

impl Conversations<'_> {
    /// List conversations.
    ///
    /// Returns a page of conversations. Filters combine with AND. Use `next_cursor` to fetch the
    /// next page.
    pub fn list(&self, params: ConversationListParams) -> Result<Page<Conversation>> {
        self.client
            .block_on(self.client.inner.conversations().list(params))
    }

    pub fn list_with_options(
        &self,
        params: ConversationListParams,
        options: RequestOptions,
    ) -> Result<Page<Conversation>> {
        self.client.block_on(
            self.client
                .inner
                .conversations()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: ConversationListParams) -> BlockingIter<Conversation> {
        self.client
            .iter(self.client.inner.conversations().list_auto_paging(params))
    }

    /// Retrieve a conversation.
    ///
    /// Returns one conversation by ID.
    pub fn retrieve(&self, id: &str) -> Result<Conversation> {
        self.client
            .block_on(self.client.inner.conversations().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Conversation> {
        self.client.block_on(
            self.client
                .inner
                .conversations()
                .retrieve_with_options(id, options),
        )
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
/// (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Messages<'a> {
    client: &'a AveeCare,
}

impl Messages<'_> {
    /// List messages.
    ///
    /// Returns a page of messages. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page.
    pub fn list(&self, params: MessageListParams) -> Result<Page<Message>> {
        self.client
            .block_on(self.client.inner.messages().list(params))
    }

    pub fn list_with_options(
        &self,
        params: MessageListParams,
        options: RequestOptions,
    ) -> Result<Page<Message>> {
        self.client.block_on(
            self.client
                .inner
                .messages()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: MessageListParams) -> BlockingIter<Message> {
        self.client
            .iter(self.client.inner.messages().list_auto_paging(params))
    }

    /// Create a message.
    ///
    /// Creates a message and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. With a caregiver's or patient's key, a saved record that key cannot read back is
    /// answered with only `object` and `id`. Requires a key with `write` scope.
    pub fn create(&self, params: MessageCreate) -> Result<Message> {
        self.client
            .block_on(self.client.inner.messages().create(params))
    }

    pub fn create_with_options(
        &self,
        params: MessageCreate,
        options: RequestOptions,
    ) -> Result<Message> {
        self.client.block_on(
            self.client
                .inner
                .messages()
                .create_with_options(params, options),
        )
    }

    /// Retrieve a message.
    ///
    /// Returns one message by ID.
    pub fn retrieve(&self, id: &str) -> Result<Message> {
        self.client
            .block_on(self.client.inner.messages().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Message> {
        self.client.block_on(
            self.client
                .inner
                .messages()
                .retrieve_with_options(id, options),
        )
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
/// and announcements staff post for caregivers. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Alerts<'a> {
    client: &'a AveeCare,
}

impl Alerts<'_> {
    /// List alerts.
    ///
    /// Returns a page of alerts. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff and caregivers.
    pub fn list(&self, params: AlertListParams) -> Result<Page<Alert>> {
        self.client
            .block_on(self.client.inner.alerts().list(params))
    }

    pub fn list_with_options(
        &self,
        params: AlertListParams,
        options: RequestOptions,
    ) -> Result<Page<Alert>> {
        self.client.block_on(
            self.client
                .inner
                .alerts()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: AlertListParams) -> BlockingIter<Alert> {
        self.client
            .iter(self.client.inner.alerts().list_auto_paging(params))
    }

    /// Create an alert.
    ///
    /// Creates an alert and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
    pub fn create(&self, params: AlertCreate) -> Result<Alert> {
        self.client
            .block_on(self.client.inner.alerts().create(params))
    }

    pub fn create_with_options(
        &self,
        params: AlertCreate,
        options: RequestOptions,
    ) -> Result<Alert> {
        self.client.block_on(
            self.client
                .inner
                .alerts()
                .create_with_options(params, options),
        )
    }

    /// Retrieve an alert.
    ///
    /// Returns one alert by ID. Available to keys belonging to office staff and caregivers.
    pub fn retrieve(&self, id: &str) -> Result<Alert> {
        self.client
            .block_on(self.client.inner.alerts().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Alert> {
        self.client.block_on(
            self.client
                .inner
                .alerts()
                .retrieve_with_options(id, options),
        )
    }

    /// Update an alert.
    ///
    /// Changes the fields you send and returns the updated alert. Send `null` to clear an optional
    /// field. Available to keys belonging to office staff. Requires a key with `write` scope.
    pub fn update(&self, id: &str, params: AlertUpdate) -> Result<Alert> {
        self.client
            .block_on(self.client.inner.alerts().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: AlertUpdate,
        options: RequestOptions,
    ) -> Result<Alert> {
        self.client.block_on(
            self.client
                .inner
                .alerts()
                .update_with_options(id, params, options),
        )
    }

    /// Delete an alert.
    ///
    /// Deletes an alert. Available to keys belonging to office staff. Requires a key with `write`
    /// scope.
    pub fn delete(&self, id: &str) -> Result<AlertDeleted> {
        self.client.block_on(self.client.inner.alerts().delete(id))
    }

    pub fn delete_with_options(&self, id: &str, options: RequestOptions) -> Result<AlertDeleted> {
        self.client
            .block_on(self.client.inner.alerts().delete_with_options(id, options))
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
/// office. There is always exactly one default office. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct Offices<'a> {
    client: &'a AveeCare,
}

impl Offices<'_> {
    /// List offices.
    ///
    /// Returns a page of offices. Filters combine with AND. Use `next_cursor` to fetch the next
    /// page. Available to keys belonging to office staff.
    pub fn list(&self, params: OfficeListParams) -> Result<Page<Office>> {
        self.client
            .block_on(self.client.inner.offices().list(params))
    }

    pub fn list_with_options(
        &self,
        params: OfficeListParams,
        options: RequestOptions,
    ) -> Result<Page<Office>> {
        self.client.block_on(
            self.client
                .inner
                .offices()
                .list_with_options(params, options),
        )
    }

    /// Iterates every item across every page.
    pub fn list_auto_paging(&self, params: OfficeListParams) -> BlockingIter<Office> {
        self.client
            .iter(self.client.inner.offices().list_auto_paging(params))
    }

    /// Create an office.
    ///
    /// Creates an office and returns it. Send an `Idempotency-Key` header to make the request safe
    /// to retry. Available to keys belonging to office staff. Requires a key with `write` scope.
    pub fn create(&self, params: OfficeCreate) -> Result<Office> {
        self.client
            .block_on(self.client.inner.offices().create(params))
    }

    pub fn create_with_options(
        &self,
        params: OfficeCreate,
        options: RequestOptions,
    ) -> Result<Office> {
        self.client.block_on(
            self.client
                .inner
                .offices()
                .create_with_options(params, options),
        )
    }

    /// Retrieve an office.
    ///
    /// Returns one office by ID. Available to keys belonging to office staff.
    pub fn retrieve(&self, id: &str) -> Result<Office> {
        self.client
            .block_on(self.client.inner.offices().retrieve(id))
    }

    pub fn retrieve_with_options(&self, id: &str, options: RequestOptions) -> Result<Office> {
        self.client.block_on(
            self.client
                .inner
                .offices()
                .retrieve_with_options(id, options),
        )
    }

    /// Update an office.
    ///
    /// Changes the fields you send and returns the updated office. Send `null` to clear an optional
    /// field. Available to keys belonging to office staff. Requires a key with `write` scope.
    pub fn update(&self, id: &str, params: OfficeUpdate) -> Result<Office> {
        self.client
            .block_on(self.client.inner.offices().update(id, params))
    }

    pub fn update_with_options(
        &self,
        id: &str,
        params: OfficeUpdate,
        options: RequestOptions,
    ) -> Result<Office> {
        self.client.block_on(
            self.client
                .inner
                .offices()
                .update_with_options(id, params, options),
        )
    }

    /// Delete an office.
    ///
    /// Deletes an office. The default office, and an agency's only office, cannot be deleted.
    /// Available to keys belonging to office staff. Requires a key with `write` scope.
    pub fn delete(&self, id: &str) -> Result<OfficeDeleted> {
        self.client.block_on(self.client.inner.offices().delete(id))
    }

    pub fn delete_with_options(&self, id: &str, options: RequestOptions) -> Result<OfficeDeleted> {
        self.client
            .block_on(self.client.inner.offices().delete_with_options(id, options))
    }
}

impl AveeCare {
    /// The API key making the request: its scopes, its user's role and its agency.
    pub fn me(&self) -> MeResource<'_> {
        MeResource { client: self }
    }
}

/// The API key making the request: its scopes, its user's role and its agency. (blocking)
#[derive(Debug, Clone, Copy)]
pub struct MeResource<'a> {
    client: &'a AveeCare,
}

impl MeResource<'_> {
    /// Retrieve the current API key.
    ///
    /// Returns the API key making the request, its scopes, the role of the user it belongs to, and
    /// its agency. Useful to check a key works.
    pub fn retrieve(&self) -> Result<Me> {
        self.client.block_on(self.client.inner.me().retrieve())
    }

    pub fn retrieve_with_options(&self, options: RequestOptions) -> Result<Me> {
        self.client
            .block_on(self.client.inner.me().retrieve_with_options(options))
    }
}
