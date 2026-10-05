// Code generated from the AveeCare OpenAPI specification. DO NOT EDIT.
#![allow(clippy::new_without_default, clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

/// Current care status.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PatientStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "discharged")]
    Discharged,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl PatientStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            PatientStatus::Active => "active",
            PatientStatus::Inactive => "inactive",
            PatientStatus::Discharged => "discharged",
            PatientStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for PatientStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Visit status.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VisitStatus {
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "missed")]
    Missed,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl VisitStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            VisitStatus::Scheduled => "scheduled",
            VisitStatus::InProgress => "in_progress",
            VisitStatus::Completed => "completed",
            VisitStatus::Cancelled => "cancelled",
            VisitStatus::Missed => "missed",
            VisitStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for VisitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Invoice status.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InvoiceStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "void")]
    Void,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl InvoiceStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            InvoiceStatus::Draft => "draft",
            InvoiceStatus::Open => "open",
            InvoiceStatus::Paid => "paid",
            InvoiceStatus::Void => "void",
            InvoiceStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// MeScope
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MeScope {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl MeScope {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            MeScope::Read => "read",
            MeScope::Write => "write",
            MeScope::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for MeScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A client receiving care from the agency.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Patient {
    /// String representing the object type. Always `patient`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier for the patient.
    #[serde(default)]
    pub id: String,
    /// Given name.
    #[serde(rename = "firstName", default)]
    pub first_name: String,
    /// Family name.
    #[serde(rename = "lastName", default)]
    pub last_name: String,
    /// Date of birth.
    #[serde(
        rename = "dateOfBirth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_of_birth: Option<String>,
    /// Contact email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Primary phone number in E.164 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Current care status.
    pub status: PatientStatus,
    /// Service address where care is delivered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<PatientAddress>,
    /// Time the patient was created, ISO 8601.
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
    /// Time the patient was last changed, ISO 8601.
    #[serde(rename = "updatedAt", default)]
    pub updated_at: String,
}

/// Service address where care is delivered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PatientAddress {
    /// Street address.
    #[serde(default)]
    pub line1: String,
    /// Apartment, suite or unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// City.
    #[serde(default)]
    pub city: String,
    /// Two-letter state code.
    #[serde(default)]
    pub state: String,
    /// ZIP code.
    #[serde(rename = "postalCode", default)]
    pub postal_code: String,
}

impl PatientAddress {
    pub fn new(
        line1: impl Into<String>,
        city: impl Into<String>,
        state: impl Into<String>,
        postal_code: impl Into<String>,
    ) -> Self {
        Self {
            line1: line1.into(),
            line2: None,
            city: city.into(),
            state: state.into(),
            postal_code: postal_code.into(),
        }
    }

    /// Apartment, suite or unit.
    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.line2 = Some(value.into());
        self
    }
}

/// Fields for a new patient.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct PatientCreate {
    /// Given name.
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// Family name.
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// Date of birth.
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<Option<String>>,
    /// Contact email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// Primary phone number in E.164 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Option<String>>,
    /// Current care status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PatientStatus>,
    /// Service address where care is delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<PatientAddress>>,
}

impl PatientCreate {
    pub fn new(first_name: impl Into<String>, last_name: impl Into<String>) -> Self {
        Self {
            first_name: first_name.into(),
            last_name: last_name.into(),
            date_of_birth: None,
            email: None,
            phone: None,
            status: None,
            address: None,
        }
    }

    /// Date of birth.
    pub fn date_of_birth(mut self, value: impl Into<String>) -> Self {
        self.date_of_birth = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `dateOfBirth`, clearing it.
    pub fn clear_date_of_birth(mut self) -> Self {
        self.date_of_birth = Some(None);
        self
    }

    /// Contact email address.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `email`, clearing it.
    pub fn clear_email(mut self) -> Self {
        self.email = Some(None);
        self
    }

    /// Primary phone number in E.164 format.
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `phone`, clearing it.
    pub fn clear_phone(mut self) -> Self {
        self.phone = Some(None);
        self
    }

    /// Current care status.
    pub fn status(mut self, value: PatientStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Service address where care is delivered.
    pub fn address(mut self, value: PatientAddress) -> Self {
        self.address = Some(Some(value));
        self
    }

    /// Sends `null` for `address`, clearing it.
    pub fn clear_address(mut self) -> Self {
        self.address = Some(None);
        self
    }
}

/// Fields to change on a patient.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct PatientUpdate {
    /// Given name.
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Family name.
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Date of birth.
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<Option<String>>,
    /// Contact email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// Primary phone number in E.164 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Option<String>>,
    /// Current care status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PatientStatus>,
    /// Service address where care is delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<PatientAddress>>,
}

impl PatientUpdate {
    pub fn new() -> Self {
        Self {
            first_name: None,
            last_name: None,
            date_of_birth: None,
            email: None,
            phone: None,
            status: None,
            address: None,
        }
    }

    /// Given name.
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    /// Family name.
    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Date of birth.
    pub fn date_of_birth(mut self, value: impl Into<String>) -> Self {
        self.date_of_birth = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `dateOfBirth`, clearing it.
    pub fn clear_date_of_birth(mut self) -> Self {
        self.date_of_birth = Some(None);
        self
    }

    /// Contact email address.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `email`, clearing it.
    pub fn clear_email(mut self) -> Self {
        self.email = Some(None);
        self
    }

    /// Primary phone number in E.164 format.
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `phone`, clearing it.
    pub fn clear_phone(mut self) -> Self {
        self.phone = Some(None);
        self
    }

    /// Current care status.
    pub fn status(mut self, value: PatientStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Service address where care is delivered.
    pub fn address(mut self, value: PatientAddress) -> Self {
        self.address = Some(Some(value));
        self
    }

    /// Sends `null` for `address`, clearing it.
    pub fn clear_address(mut self) -> Self {
        self.address = Some(None);
        self
    }
}

/// Confirmation that a patient was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PatientDeleted {
    /// Always `patient`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted object.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// A scheduled or completed caregiver shift.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Visit {
    /// String representing the object type. Always `visit`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier for the visit.
    #[serde(default)]
    pub id: String,
    /// ID of the patient receiving care.
    #[serde(rename = "patientId", default)]
    pub patient_id: String,
    /// ID of the assigned caregiver, or null when unassigned.
    #[serde(
        rename = "caregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_id: Option<String>,
    /// Scheduled start, ISO 8601.
    #[serde(rename = "startTime", default)]
    pub start_time: String,
    /// Scheduled end, ISO 8601.
    #[serde(rename = "endTime", default)]
    pub end_time: String,
    /// Visit status.
    pub status: VisitStatus,
    /// Scheduling notes visible to the caregiver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Time the visit was created, ISO 8601.
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
    /// Time the visit was last changed, ISO 8601.
    #[serde(rename = "updatedAt", default)]
    pub updated_at: String,
}

/// Fields for a new visit.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct VisitCreate {
    /// ID of the patient receiving care.
    #[serde(rename = "patientId")]
    pub patient_id: String,
    /// ID of the assigned caregiver, or null when unassigned.
    #[serde(rename = "caregiverId", skip_serializing_if = "Option::is_none")]
    pub caregiver_id: Option<Option<String>>,
    /// Scheduled start, ISO 8601.
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// Scheduled end, ISO 8601.
    #[serde(rename = "endTime")]
    pub end_time: String,
    /// Visit status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VisitStatus>,
    /// Scheduling notes visible to the caregiver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
}

impl VisitCreate {
    pub fn new(
        patient_id: impl Into<String>,
        start_time: impl Into<String>,
        end_time: impl Into<String>,
    ) -> Self {
        Self {
            patient_id: patient_id.into(),
            caregiver_id: None,
            start_time: start_time.into(),
            end_time: end_time.into(),
            status: None,
            notes: None,
        }
    }

    /// ID of the assigned caregiver, or null when unassigned.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `caregiverId`, clearing it.
    pub fn clear_caregiver_id(mut self) -> Self {
        self.caregiver_id = Some(None);
        self
    }

    /// Visit status.
    pub fn status(mut self, value: VisitStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Scheduling notes visible to the caregiver.
    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `notes`, clearing it.
    pub fn clear_notes(mut self) -> Self {
        self.notes = Some(None);
        self
    }
}

/// Fields to change on a visit.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct VisitUpdate {
    /// ID of the patient receiving care.
    #[serde(rename = "patientId", skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// ID of the assigned caregiver, or null when unassigned.
    #[serde(rename = "caregiverId", skip_serializing_if = "Option::is_none")]
    pub caregiver_id: Option<Option<String>>,
    /// Scheduled start, ISO 8601.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Scheduled end, ISO 8601.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Visit status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<VisitStatus>,
    /// Scheduling notes visible to the caregiver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
}

impl VisitUpdate {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            caregiver_id: None,
            start_time: None,
            end_time: None,
            status: None,
            notes: None,
        }
    }

    /// ID of the patient receiving care.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// ID of the assigned caregiver, or null when unassigned.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `caregiverId`, clearing it.
    pub fn clear_caregiver_id(mut self) -> Self {
        self.caregiver_id = Some(None);
        self
    }

    /// Scheduled start, ISO 8601.
    pub fn start_time(mut self, value: impl Into<String>) -> Self {
        self.start_time = Some(value.into());
        self
    }

    /// Scheduled end, ISO 8601.
    pub fn end_time(mut self, value: impl Into<String>) -> Self {
        self.end_time = Some(value.into());
        self
    }

    /// Visit status.
    pub fn status(mut self, value: VisitStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Scheduling notes visible to the caregiver.
    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `notes`, clearing it.
    pub fn clear_notes(mut self) -> Self {
        self.notes = Some(None);
        self
    }
}

/// Confirmation that a visit was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VisitDeleted {
    /// Always `visit`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted object.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// An invoice for care delivered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Invoice {
    /// String representing the object type. Always `invoice`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier for the invoice.
    #[serde(default)]
    pub id: String,
    /// ID of the billed patient.
    #[serde(rename = "patientId", default)]
    pub patient_id: String,
    /// Human-readable invoice number.
    #[serde(default)]
    pub number: String,
    /// Invoice status.
    pub status: InvoiceStatus,
    /// Total in cents.
    #[serde(rename = "totalCents", default)]
    pub total_cents: i64,
    /// Amount still owed in cents.
    #[serde(rename = "balanceCents", default)]
    pub balance_cents: i64,
    /// Date issued.
    #[serde(rename = "issuedOn", default, skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,
    /// Date payment is due.
    #[serde(rename = "dueOn", default, skip_serializing_if = "Option::is_none")]
    pub due_on: Option<String>,
    /// Line items on the invoice.
    #[serde(rename = "lineItems", default)]
    pub line_items: Vec<InvoiceLineItem>,
    /// Time the invoice was created, ISO 8601.
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
    /// Time the invoice was last changed, ISO 8601.
    #[serde(rename = "updatedAt", default)]
    pub updated_at: String,
}

/// One billed line on an invoice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvoiceLineItem {
    /// What was billed.
    #[serde(default)]
    pub description: String,
    /// Units billed, such as hours.
    #[serde(default)]
    pub quantity: f64,
    /// Price per unit in cents.
    #[serde(rename = "unitPriceCents", default)]
    pub unit_price_cents: i64,
    /// Line total in cents.
    #[serde(rename = "amountCents", default)]
    pub amount_cents: i64,
}

/// The API key making the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Me {
    /// String representing the object type. Always `me`.
    #[serde(default)]
    pub object: String,
    /// ID of the API key.
    #[serde(default)]
    pub id: String,
    /// Name given to the API key.
    #[serde(default)]
    pub name: String,
    /// Scopes granted to the key.
    #[serde(default)]
    pub scopes: Vec<MeScope>,
    /// The agency the key belongs to.
    pub organization: MeOrganization,
}

/// The agency the key belongs to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MeOrganization {
    /// ID of the agency.
    #[serde(default)]
    pub id: String,
    /// Agency name.
    #[serde(default)]
    pub name: String,
}

/// Query parameters for `patients.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct PatientListParams {
    /// Only return patients with this status.
    pub status: Option<PatientStatus>,
    /// Only return patients changed after this time, ISO 8601.
    pub updated_after: Option<String>,
    /// Number of objects to return, between 1 and 100.
    pub limit: Option<i64>,
    /// Opaque cursor from a previous response `next_cursor`.
    pub cursor: Option<String>,
}

impl PatientListParams {
    pub fn new() -> Self {
        Self {
            status: None,
            updated_after: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return patients with this status.
    pub fn status(mut self, value: PatientStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return patients changed after this time, ISO 8601.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Number of objects to return, between 1 and 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Opaque cursor from a previous response `next_cursor`.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.status {
            query.push(("status", value.as_str().to_owned()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.limit {
            query.push(("limit", value.to_string()));
        }
        if let Some(value) = &self.cursor {
            query.push(("cursor", value.clone()));
        }
        query
    }
}

/// Query parameters for `visits.list`.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct VisitListParams {
    /// Start of the window (inclusive), ISO 8601. The window may span at most 93 days.
    pub start_from: String,
    /// End of the window (exclusive), ISO 8601.
    pub start_to: String,
    /// Only return visits for this patient.
    pub patient_id: Option<String>,
    /// Only return visits for this caregiver.
    pub caregiver_id: Option<String>,
    /// Only return visits with this status.
    pub status: Option<VisitStatus>,
    /// Number of objects to return, between 1 and 100.
    pub limit: Option<i64>,
    /// Opaque cursor from a previous response `next_cursor`.
    pub cursor: Option<String>,
}

impl VisitListParams {
    pub fn new(start_from: impl Into<String>, start_to: impl Into<String>) -> Self {
        Self {
            start_from: start_from.into(),
            start_to: start_to.into(),
            patient_id: None,
            caregiver_id: None,
            status: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return visits for this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return visits for this caregiver.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(value.into());
        self
    }

    /// Only return visits with this status.
    pub fn status(mut self, value: VisitStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Number of objects to return, between 1 and 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Opaque cursor from a previous response `next_cursor`.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        query.push(("start_from", self.start_from.clone()));
        query.push(("start_to", self.start_to.clone()));
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.caregiver_id {
            query.push(("caregiver_id", value.clone()));
        }
        if let Some(value) = &self.status {
            query.push(("status", value.as_str().to_owned()));
        }
        if let Some(value) = &self.limit {
            query.push(("limit", value.to_string()));
        }
        if let Some(value) = &self.cursor {
            query.push(("cursor", value.clone()));
        }
        query
    }
}

/// Query parameters for `invoices.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct InvoiceListParams {
    /// Only return invoices with this status.
    pub status: Option<InvoiceStatus>,
    /// Only return invoices for this patient.
    pub patient_id: Option<String>,
    /// Only return invoices changed after this time, ISO 8601.
    pub updated_after: Option<String>,
    /// Number of objects to return, between 1 and 100.
    pub limit: Option<i64>,
    /// Opaque cursor from a previous response `next_cursor`.
    pub cursor: Option<String>,
}

impl InvoiceListParams {
    pub fn new() -> Self {
        Self {
            status: None,
            patient_id: None,
            updated_after: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return invoices with this status.
    pub fn status(mut self, value: InvoiceStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return invoices for this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return invoices changed after this time, ISO 8601.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Number of objects to return, between 1 and 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Opaque cursor from a previous response `next_cursor`.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.status {
            query.push(("status", value.as_str().to_owned()));
        }
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.limit {
            query.push(("limit", value.to_string()));
        }
        if let Some(value) = &self.cursor {
            query.push(("cursor", value.clone()));
        }
        query
    }
}
