// Code generated from the AveeCare OpenAPI specification. DO NOT EDIT.
#![allow(clippy::new_without_default, clippy::too_many_arguments)]

use serde::{Deserialize, Serialize};

/// Whether this person is a prospective client or an admitted client. `null` means client.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PatientLifecycleStage {
    #[serde(rename = "Prospect")]
    Prospect,
    #[serde(rename = "Client")]
    Client,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl PatientLifecycleStage {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            PatientLifecycleStage::Prospect => "Prospect",
            PatientLifecycleStage::Client => "Client",
            PatientLifecycleStage::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for PatientLifecycleStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The main type of service the patient receives.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PatientServiceType {
    #[serde(rename = "SkilledNursing")]
    SkilledNursing,
    #[serde(rename = "PhysicalTherapy")]
    PhysicalTherapy,
    #[serde(rename = "Hospice")]
    Hospice,
    #[serde(rename = "StandardCare")]
    StandardCare,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl PatientServiceType {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            PatientServiceType::SkilledNursing => "SkilledNursing",
            PatientServiceType::PhysicalTherapy => "PhysicalTherapy",
            PatientServiceType::Hospice => "Hospice",
            PatientServiceType::StandardCare => "StandardCare",
            PatientServiceType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for PatientServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// How statements are delivered.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PatientStatementDelivery {
    #[serde(rename = "Email")]
    Email,
    #[serde(rename = "Mail")]
    Mail,
    #[serde(rename = "EmailAndMail")]
    EmailAndMail,
    #[serde(rename = "NoStatement")]
    NoStatement,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl PatientStatementDelivery {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            PatientStatementDelivery::Email => "Email",
            PatientStatementDelivery::Mail => "Mail",
            PatientStatementDelivery::EmailAndMail => "EmailAndMail",
            PatientStatementDelivery::NoStatement => "NoStatement",
            PatientStatementDelivery::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for PatientStatementDelivery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// How the patient usually pays.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PatientPaymentMethodPreference {
    #[serde(rename = "ACH")]
    Ach,
    #[serde(rename = "Card")]
    Card,
    #[serde(rename = "Check")]
    Check,
    #[serde(rename = "Cash")]
    Cash,
    #[serde(rename = "PayerOnly")]
    PayerOnly,
    #[serde(rename = "Other")]
    Other,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl PatientPaymentMethodPreference {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            PatientPaymentMethodPreference::Ach => "ACH",
            PatientPaymentMethodPreference::Card => "Card",
            PatientPaymentMethodPreference::Check => "Check",
            PatientPaymentMethodPreference::Cash => "Cash",
            PatientPaymentMethodPreference::PayerOnly => "PayerOnly",
            PatientPaymentMethodPreference::Other => "Other",
            PatientPaymentMethodPreference::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for PatientPaymentMethodPreference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The client's status as shown in AveeCare. `Inactive` and `Discharged` mean the person is no
/// longer receiving care. A prospective client (`lifecycleStage: Prospect`) reads `Prospect` even
/// if you send `Active`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PatientCreateStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Inactive")]
    Inactive,
    #[serde(rename = "Prospect")]
    Prospect,
    #[serde(rename = "Discharged")]
    Discharged,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl PatientCreateStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            PatientCreateStatus::Active => "Active",
            PatientCreateStatus::Inactive => "Inactive",
            PatientCreateStatus::Prospect => "Prospect",
            PatientCreateStatus::Discharged => "Discharged",
            PatientCreateStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for PatientCreateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Which updates the contact is notified about.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ContactNotificationPolicy {
    #[serde(rename = "All")]
    All,
    #[serde(rename = "Emergencies")]
    Emergencies,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl ContactNotificationPolicy {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            ContactNotificationPolicy::All => "All",
            ContactNotificationPolicy::Emergencies => "Emergencies",
            ContactNotificationPolicy::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for ContactNotificationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// `Scheduled` for regular doses, `PRN` for as-needed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MedicationScheduleType {
    #[serde(rename = "Scheduled")]
    Scheduled,
    #[serde(rename = "PRN")]
    Prn,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl MedicationScheduleType {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            MedicationScheduleType::Scheduled => "Scheduled",
            MedicationScheduleType::Prn => "PRN",
            MedicationScheduleType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for MedicationScheduleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Whether the medication is currently given.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MedicationStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "OnHold")]
    OnHold,
    #[serde(rename = "Discontinued")]
    Discontinued,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl MedicationStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            MedicationStatus::Active => "Active",
            MedicationStatus::OnHold => "OnHold",
            MedicationStatus::Discontinued => "Discontinued",
            MedicationStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for MedicationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// What happened to the dose.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MedicationAdministrationStatus {
    #[serde(rename = "Given")]
    Given,
    #[serde(rename = "Refused")]
    Refused,
    #[serde(rename = "Held")]
    Held,
    #[serde(rename = "Missed")]
    Missed,
    #[serde(rename = "SelfAdministered")]
    SelfAdministered,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl MedicationAdministrationStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            MedicationAdministrationStatus::Given => "Given",
            MedicationAdministrationStatus::Refused => "Refused",
            MedicationAdministrationStatus::Held => "Held",
            MedicationAdministrationStatus::Missed => "Missed",
            MedicationAdministrationStatus::SelfAdministered => "SelfAdministered",
            MedicationAdministrationStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for MedicationAdministrationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Which OASIS time point this assessment covers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum OasisAssessmentTimepoint {
    #[serde(rename = "SOC")]
    Soc,
    #[serde(rename = "ROC")]
    Roc,
    #[serde(rename = "Recertification")]
    Recertification,
    #[serde(rename = "Transfer")]
    Transfer,
    #[serde(rename = "Discharge")]
    Discharge,
    #[serde(rename = "DeathAtHome")]
    DeathAtHome,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl OasisAssessmentTimepoint {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            OasisAssessmentTimepoint::Soc => "SOC",
            OasisAssessmentTimepoint::Roc => "ROC",
            OasisAssessmentTimepoint::Recertification => "Recertification",
            OasisAssessmentTimepoint::Transfer => "Transfer",
            OasisAssessmentTimepoint::Discharge => "Discharge",
            OasisAssessmentTimepoint::DeathAtHome => "DeathAtHome",
            OasisAssessmentTimepoint::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for OasisAssessmentTimepoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Where the assessment is in its lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum OasisAssessmentStatus {
    #[serde(rename = "Draft")]
    Draft,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Exported")]
    Exported,
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Inactivated")]
    Inactivated,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl OasisAssessmentStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            OasisAssessmentStatus::Draft => "Draft",
            OasisAssessmentStatus::Completed => "Completed",
            OasisAssessmentStatus::Exported => "Exported",
            OasisAssessmentStatus::Accepted => "Accepted",
            OasisAssessmentStatus::Rejected => "Rejected",
            OasisAssessmentStatus::Inactivated => "Inactivated",
            OasisAssessmentStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for OasisAssessmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Employment status.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CaregiverStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "OnLeave")]
    OnLeave,
    #[serde(rename = "Inactive")]
    Inactive,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl CaregiverStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            CaregiverStatus::Active => "Active",
            CaregiverStatus::OnLeave => "OnLeave",
            CaregiverStatus::Inactive => "Inactive",
            CaregiverStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for CaregiverStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Where the visit is in its day. New visits are `Scheduled`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VisitStatus {
    #[serde(rename = "Scheduled")]
    Scheduled,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Late")]
    Late,
    #[serde(rename = "OverTime")]
    OverTime,
    #[serde(rename = "Incomplete")]
    Incomplete,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl VisitStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            VisitStatus::Scheduled => "Scheduled",
            VisitStatus::InProgress => "InProgress",
            VisitStatus::Cancelled => "Cancelled",
            VisitStatus::Completed => "Completed",
            VisitStatus::Late => "Late",
            VisitStatus::OverTime => "OverTime",
            VisitStatus::Incomplete => "Incomplete",
            VisitStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for VisitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Electronic visit verification status.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VisitEvvStatus {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Validated")]
    Validated,
    #[serde(rename = "Flagged")]
    Flagged,
    #[serde(rename = "Overridden")]
    Overridden,
    #[serde(rename = "Exported")]
    Exported,
    #[serde(rename = "Submitted")]
    Submitted,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl VisitEvvStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            VisitEvvStatus::Pending => "Pending",
            VisitEvvStatus::Validated => "Validated",
            VisitEvvStatus::Flagged => "Flagged",
            VisitEvvStatus::Overridden => "Overridden",
            VisitEvvStatus::Exported => "Exported",
            VisitEvvStatus::Submitted => "Submitted",
            VisitEvvStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for VisitEvvStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Whether the visit has been billed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VisitBillingStatus {
    #[serde(rename = "NotBilled")]
    NotBilled,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Invoiced")]
    Invoiced,
    #[serde(rename = "Paid")]
    Paid,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl VisitBillingStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            VisitBillingStatus::NotBilled => "NotBilled",
            VisitBillingStatus::Pending => "Pending",
            VisitBillingStatus::Invoiced => "Invoiced",
            VisitBillingStatus::Paid => "Paid",
            VisitBillingStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for VisitBillingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The caregiver's response to the assignment.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VisitCaregiverRsvp {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "declined")]
    Declined,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl VisitCaregiverRsvp {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            VisitCaregiverRsvp::Confirmed => "confirmed",
            VisitCaregiverRsvp::Declined => "declined",
            VisitCaregiverRsvp::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for VisitCaregiverRsvp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Whether office staff (`User`) or a caregiver wrote the note. Set from the API key's user.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VisitNoteAuthorType {
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Caregiver")]
    Caregiver,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl VisitNoteAuthorType {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            VisitNoteAuthorType::User => "User",
            VisitNoteAuthorType::Caregiver => "Caregiver",
            VisitNoteAuthorType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for VisitNoteAuthorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Where the request stands. A caregiver's key creates `Pending` requests and can only change its
/// own pending request to `Cancelled`; approving and denying is for office staff.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum VisitRequestStatus {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "Denied")]
    Denied,
    #[serde(rename = "Cancelled")]
    Cancelled,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl VisitRequestStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            VisitRequestStatus::Pending => "Pending",
            VisitRequestStatus::Approved => "Approved",
            VisitRequestStatus::Denied => "Denied",
            VisitRequestStatus::Cancelled => "Cancelled",
            VisitRequestStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for VisitRequestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Whether the offer is still open.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum OpenShiftStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Filled")]
    Filled,
    #[serde(rename = "Cancelled")]
    Cancelled,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl OpenShiftStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            OpenShiftStatus::Active => "Active",
            OpenShiftStatus::Filled => "Filled",
            OpenShiftStatus::Cancelled => "Cancelled",
            OpenShiftStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for OpenShiftStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Where the swap stands. With a caregiver's key: new swaps are `Pending`; the asked caregiver
/// changes a `Pending` swap to `Accepted` or `Declined`; the requesting caregiver can change it to
/// `Cancelled`. `AdminApproved` and `AdminDenied` are for office staff.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ShiftSwapStatus {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Declined")]
    Declined,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "AdminApproved")]
    AdminApproved,
    #[serde(rename = "AdminDenied")]
    AdminDenied,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl ShiftSwapStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            ShiftSwapStatus::Pending => "Pending",
            ShiftSwapStatus::Accepted => "Accepted",
            ShiftSwapStatus::Declined => "Declined",
            ShiftSwapStatus::Cancelled => "Cancelled",
            ShiftSwapStatus::AdminApproved => "AdminApproved",
            ShiftSwapStatus::AdminDenied => "AdminDenied",
            ShiftSwapStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for ShiftSwapStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Where the plan is in its lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CarePlanStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Draft")]
    Draft,
    #[serde(rename = "Archived")]
    Archived,
    #[serde(rename = "UnderReview")]
    UnderReview,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Discontinued")]
    Discontinued,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl CarePlanStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            CarePlanStatus::Active => "Active",
            CarePlanStatus::Draft => "Draft",
            CarePlanStatus::Archived => "Archived",
            CarePlanStatus::UnderReview => "UnderReview",
            CarePlanStatus::Completed => "Completed",
            CarePlanStatus::Discontinued => "Discontinued",
            CarePlanStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for CarePlanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Whether units count time, visits or days.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AuthorizationUnitBasis {
    #[serde(rename = "Time")]
    Time,
    #[serde(rename = "Visits")]
    Visits,
    #[serde(rename = "Days")]
    Days,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl AuthorizationUnitBasis {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            AuthorizationUnitBasis::Time => "Time",
            AuthorizationUnitBasis::Visits => "Visits",
            AuthorizationUnitBasis::Days => "Days",
            AuthorizationUnitBasis::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for AuthorizationUnitBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Whether the authorization is in effect.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AuthorizationStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Expired")]
    Expired,
    #[serde(rename = "PendingRenewal")]
    PendingRenewal,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl AuthorizationStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            AuthorizationStatus::Active => "Active",
            AuthorizationStatus::Expired => "Expired",
            AuthorizationStatus::PendingRenewal => "PendingRenewal",
            AuthorizationStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for AuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Kind of incident.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum IncidentCategory {
    #[serde(rename = "Fall")]
    Fall,
    #[serde(rename = "Medication")]
    Medication,
    #[serde(rename = "Behavioral")]
    Behavioral,
    #[serde(rename = "Injury")]
    Injury,
    #[serde(rename = "Equipment")]
    Equipment,
    #[serde(rename = "Attendance")]
    Attendance,
    #[serde(rename = "Conduct")]
    Conduct,
    #[serde(rename = "Other")]
    Other,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl IncidentCategory {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            IncidentCategory::Fall => "Fall",
            IncidentCategory::Medication => "Medication",
            IncidentCategory::Behavioral => "Behavioral",
            IncidentCategory::Injury => "Injury",
            IncidentCategory::Equipment => "Equipment",
            IncidentCategory::Attendance => "Attendance",
            IncidentCategory::Conduct => "Conduct",
            IncidentCategory::Other => "Other",
            IncidentCategory::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for IncidentCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// How serious it was.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum IncidentSeverity {
    #[serde(rename = "Low")]
    Low,
    #[serde(rename = "Medium")]
    Medium,
    #[serde(rename = "High")]
    High,
    #[serde(rename = "Critical")]
    Critical,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl IncidentSeverity {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            IncidentSeverity::Low => "Low",
            IncidentSeverity::Medium => "Medium",
            IncidentSeverity::High => "High",
            IncidentSeverity::Critical => "Critical",
            IncidentSeverity::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for IncidentSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Where follow-up stands. New incidents start `Open`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum IncidentStatus {
    #[serde(rename = "Open")]
    Open,
    #[serde(rename = "Monitoring")]
    Monitoring,
    #[serde(rename = "AwaitingFollowUp")]
    AwaitingFollowUp,
    #[serde(rename = "Resolved")]
    Resolved,
    #[serde(rename = "Closed")]
    Closed,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl IncidentStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            IncidentStatus::Open => "Open",
            IncidentStatus::Monitoring => "Monitoring",
            IncidentStatus::AwaitingFollowUp => "AwaitingFollowUp",
            IncidentStatus::Resolved => "Resolved",
            IncidentStatus::Closed => "Closed",
            IncidentStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for IncidentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Where the referral stands.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ReferralStatus {
    #[serde(rename = "New")]
    New,
    #[serde(rename = "Contacted")]
    Contacted,
    #[serde(rename = "AssessmentScheduled")]
    AssessmentScheduled,
    #[serde(rename = "AssessmentCompleted")]
    AssessmentCompleted,
    #[serde(rename = "ServiceAgreementSent")]
    ServiceAgreementSent,
    #[serde(rename = "Admitted")]
    Admitted,
    #[serde(rename = "Declined")]
    Declined,
    #[serde(rename = "Lost")]
    Lost,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl ReferralStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            ReferralStatus::New => "New",
            ReferralStatus::Contacted => "Contacted",
            ReferralStatus::AssessmentScheduled => "AssessmentScheduled",
            ReferralStatus::AssessmentCompleted => "AssessmentCompleted",
            ReferralStatus::ServiceAgreementSent => "ServiceAgreementSent",
            ReferralStatus::Admitted => "Admitted",
            ReferralStatus::Declined => "Declined",
            ReferralStatus::Lost => "Lost",
            ReferralStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for ReferralStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Payment status.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InvoiceStatus {
    #[serde(rename = "Draft")]
    Draft,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Processing")]
    Processing,
    #[serde(rename = "Paid")]
    Paid,
    #[serde(rename = "PartiallyPaid")]
    PartiallyPaid,
    #[serde(rename = "Overdue")]
    Overdue,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Refunded")]
    Refunded,
    #[serde(rename = "Failed")]
    Failed,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl InvoiceStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            InvoiceStatus::Draft => "Draft",
            InvoiceStatus::Pending => "Pending",
            InvoiceStatus::Processing => "Processing",
            InvoiceStatus::Paid => "Paid",
            InvoiceStatus::PartiallyPaid => "PartiallyPaid",
            InvoiceStatus::Overdue => "Overdue",
            InvoiceStatus::Cancelled => "Cancelled",
            InvoiceStatus::Refunded => "Refunded",
            InvoiceStatus::Failed => "Failed",
            InvoiceStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Claim format: professional (837P) or institutional (837I).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ClaimClaimType {
    #[serde(rename = "Professional837P")]
    Professional837P,
    #[serde(rename = "Institutional837I")]
    Institutional837I,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl ClaimClaimType {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            ClaimClaimType::Professional837P => "Professional837P",
            ClaimClaimType::Institutional837I => "Institutional837I",
            ClaimClaimType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for ClaimClaimType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Where the claim is in adjudication.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ClaimStatus {
    #[serde(rename = "Draft")]
    Draft,
    #[serde(rename = "Validated")]
    Validated,
    #[serde(rename = "ValidationFailed")]
    ValidationFailed,
    #[serde(rename = "Submitted")]
    Submitted,
    #[serde(rename = "Acknowledged")]
    Acknowledged,
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "InAdjudication")]
    InAdjudication,
    #[serde(rename = "Paid")]
    Paid,
    #[serde(rename = "PartiallyPaid")]
    PartiallyPaid,
    #[serde(rename = "Denied")]
    Denied,
    #[serde(rename = "Appealed")]
    Appealed,
    #[serde(rename = "Voided")]
    Voided,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl ClaimStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            ClaimStatus::Draft => "Draft",
            ClaimStatus::Validated => "Validated",
            ClaimStatus::ValidationFailed => "ValidationFailed",
            ClaimStatus::Submitted => "Submitted",
            ClaimStatus::Acknowledged => "Acknowledged",
            ClaimStatus::Accepted => "Accepted",
            ClaimStatus::Rejected => "Rejected",
            ClaimStatus::InAdjudication => "InAdjudication",
            ClaimStatus::Paid => "Paid",
            ClaimStatus::PartiallyPaid => "PartiallyPaid",
            ClaimStatus::Denied => "Denied",
            ClaimStatus::Appealed => "Appealed",
            ClaimStatus::Voided => "Voided",
            ClaimStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for ClaimStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Where the entry is in your payroll process.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PayrollEntryStatus {
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "Processed")]
    Processed,
    #[serde(rename = "Paid")]
    Paid,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "HoursSubmitted")]
    HoursSubmitted,
    #[serde(rename = "CheckCreated")]
    CheckCreated,
    #[serde(rename = "PaidConfirmed")]
    PaidConfirmed,
    #[serde(rename = "PaidExternally")]
    PaidExternally,
    #[serde(rename = "Voided")]
    Voided,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl PayrollEntryStatus {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            PayrollEntryStatus::Pending => "Pending",
            PayrollEntryStatus::Approved => "Approved",
            PayrollEntryStatus::Processed => "Processed",
            PayrollEntryStatus::Paid => "Paid",
            PayrollEntryStatus::Cancelled => "Cancelled",
            PayrollEntryStatus::Failed => "Failed",
            PayrollEntryStatus::HoursSubmitted => "HoursSubmitted",
            PayrollEntryStatus::CheckCreated => "CheckCreated",
            PayrollEntryStatus::PaidConfirmed => "PaidConfirmed",
            PayrollEntryStatus::PaidExternally => "PaidExternally",
            PayrollEntryStatus::Voided => "Voided",
            PayrollEntryStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for PayrollEntryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// How the caregiver is paid.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PayrollEntryPaymentMethod {
    #[serde(rename = "DirectDeposit")]
    DirectDeposit,
    #[serde(rename = "Check")]
    Check,
    #[serde(rename = "Manual")]
    Manual,
    #[serde(rename = "ACH")]
    Ach,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl PayrollEntryPaymentMethod {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            PayrollEntryPaymentMethod::DirectDeposit => "DirectDeposit",
            PayrollEntryPaymentMethod::Check => "Check",
            PayrollEntryPaymentMethod::Manual => "Manual",
            PayrollEntryPaymentMethod::Ach => "ACH",
            PayrollEntryPaymentMethod::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for PayrollEntryPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// How prominently the alert is shown.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AlertType {
    #[serde(rename = "Critical")]
    Critical,
    #[serde(rename = "Important")]
    Important,
    #[serde(rename = "Notice")]
    Notice,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl AlertType {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertType::Critical => "Critical",
            AlertType::Important => "Important",
            AlertType::Notice => "Notice",
            AlertType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Who sees the alert. `Caregiver`, `Patient`, `Visit` and `PatientCareTeam` need
/// `audienceTargetId`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AlertAudience {
    #[serde(rename = "CompanyWide")]
    CompanyWide,
    #[serde(rename = "StaffOnly")]
    StaffOnly,
    #[serde(rename = "CaregiversAll")]
    CaregiversAll,
    #[serde(rename = "PatientsAll")]
    PatientsAll,
    #[serde(rename = "Caregiver")]
    Caregiver,
    #[serde(rename = "Patient")]
    Patient,
    #[serde(rename = "Visit")]
    Visit,
    #[serde(rename = "PatientCareTeam")]
    PatientCareTeam,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl AlertAudience {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertAudience::CompanyWide => "CompanyWide",
            AlertAudience::StaffOnly => "StaffOnly",
            AlertAudience::CaregiversAll => "CaregiversAll",
            AlertAudience::PatientsAll => "PatientsAll",
            AlertAudience::Caregiver => "Caregiver",
            AlertAudience::Patient => "Patient",
            AlertAudience::Visit => "Visit",
            AlertAudience::PatientCareTeam => "PatientCareTeam",
            AlertAudience::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for AlertAudience {
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

/// Role of the person the key belongs to: `RootUser` (agency administrator), `User` (office staff),
/// `Caregiver` or `Patient`. It decides which resources the key can reach.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MeRole {
    #[serde(rename = "RootUser")]
    RootUser,
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Caregiver")]
    Caregiver,
    #[serde(rename = "Patient")]
    Patient,
    /// A value this version of the library does not know yet. Never send it.
    #[serde(other)]
    Unknown,
}

impl MeRole {
    /// The value as sent on the wire.
    pub fn as_str(&self) -> &'static str {
        match self {
            MeRole::RootUser => "RootUser",
            MeRole::User => "User",
            MeRole::Caregiver => "Caregiver",
            MeRole::Patient => "Patient",
            MeRole::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for MeRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Patients are the clients your agency provides care to, including prospective clients you have
/// not yet admitted. Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Patient {
    /// String representing the object's type. Always `patient`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the patient.
    #[serde(default)]
    pub id: String,
    /// Given name.
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Middle name.
    #[serde(
        rename = "middleName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub middle_name: Option<String>,
    /// Family name.
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The name the patient prefers to be called.
    #[serde(
        rename = "goesByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub goes_by_name: Option<String>,
    /// Date of birth.
    #[serde(
        rename = "dateOfBirth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_of_birth: Option<String>,
    /// Gender as recorded by the agency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Language the patient is most comfortable speaking.
    #[serde(
        rename = "primaryLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_language: Option<String>,
    /// Contact email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Whether the patient is an active client. Only active clients are scheduled and billed. A
    /// prospective client is always inactive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// The client's status as shown in AveeCare. `Inactive` and `Discharged` mean the person is no
    /// longer receiving care. A prospective client (`lifecycleStage: Prospect`) reads `Prospect`
    /// even if you send `Active`. Usually one of `Active`, `Inactive`, `Prospect`, `Discharged`;
    /// records imported from another system can hold other values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Whether this person is a prospective client or an admitted client. `null` means client.
    #[serde(
        rename = "lifecycleStage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub lifecycle_stage: Option<PatientLifecycleStage>,
    /// The main type of service the patient receives.
    #[serde(
        rename = "serviceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_type: Option<PatientServiceType>,
    /// Street address where care is delivered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// City of the service address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State of the service address, as a two-letter code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ZIP code of the service address.
    #[serde(rename = "zipCode", default, skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// The office (branch) that serves this patient.
    #[serde(rename = "officeId", default, skip_serializing_if = "Option::is_none")]
    pub office_id: Option<String>,
    /// The referral source that sent this patient to you.
    #[serde(
        rename = "referralSourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub referral_source_id: Option<String>,
    /// Primary diagnosis, in plain words.
    #[serde(
        rename = "primaryDiagnosisDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_diagnosis_description: Option<String>,
    /// Whether the patient is a fall risk.
    #[serde(rename = "fallRisk", default, skip_serializing_if = "Option::is_none")]
    pub fall_risk: Option<bool>,
    /// Caregiver gender the patient prefers, if any.
    #[serde(
        rename = "preferredCaregiverGender",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_caregiver_gender: Option<String>,
    /// Notes schedulers see when booking visits for this patient.
    #[serde(
        rename = "schedulingNotes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduling_notes: Option<String>,
    /// Person legally responsible for the patient, if not the patient.
    #[serde(
        rename = "responsiblePartyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub responsible_party_name: Option<String>,
    /// Medicaid member ID.
    #[serde(
        rename = "medicaidId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub medicaid_id: Option<String>,
    /// Medicare beneficiary identifier.
    #[serde(
        rename = "medicareId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub medicare_id: Option<String>,
    /// Primary insurance company name.
    #[serde(
        rename = "insuranceProvider",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insurance_provider: Option<String>,
    /// Member ID with the primary insurance.
    #[serde(
        rename = "insuranceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insurance_id: Option<String>,
    /// Group number with the primary insurance.
    #[serde(
        rename = "insuranceGroupNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insurance_group_number: Option<String>,
    /// Who receives the bills.
    #[serde(
        rename = "billingContactName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_contact_name: Option<String>,
    /// Email address bills are sent to.
    #[serde(
        rename = "billingContactEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_contact_email: Option<String>,
    /// Phone number of the billing contact.
    #[serde(
        rename = "billingContactPhone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_contact_phone: Option<String>,
    /// How statements are delivered.
    #[serde(
        rename = "statementDelivery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub statement_delivery: Option<PatientStatementDelivery>,
    /// How the patient usually pays.
    #[serde(
        rename = "paymentMethodPreference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_method_preference: Option<PatientPaymentMethodPreference>,
    /// Balance the patient currently owes. In cents.
    #[serde(
        rename = "outstandingBalanceCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub outstanding_balance_cents: Option<i64>,
    /// When the patient was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the patient was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new patient. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct PatientCreate {
    /// Given name.
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// Middle name.
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<Option<String>>,
    /// Family name.
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// The name the patient prefers to be called.
    #[serde(rename = "goesByName", skip_serializing_if = "Option::is_none")]
    pub goes_by_name: Option<Option<String>>,
    /// Date of birth.
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<Option<String>>,
    /// Gender as recorded by the agency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Option<String>>,
    /// Language the patient is most comfortable speaking.
    #[serde(rename = "primaryLanguage", skip_serializing_if = "Option::is_none")]
    pub primary_language: Option<Option<String>>,
    /// Contact email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// Whether the patient is an active client. Only active clients are scheduled and billed. A
    /// prospective client is always inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<Option<bool>>,
    /// The client's status as shown in AveeCare. `Inactive` and `Discharged` mean the person is no
    /// longer receiving care. A prospective client (`lifecycleStage: Prospect`) reads `Prospect`
    /// even if you send `Active`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<PatientCreateStatus>>,
    /// Whether this person is a prospective client or an admitted client. `null` means client.
    #[serde(rename = "lifecycleStage", skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<Option<PatientLifecycleStage>>,
    /// The main type of service the patient receives.
    #[serde(rename = "serviceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<Option<PatientServiceType>>,
    /// Street address where care is delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<String>>,
    /// City of the service address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// State of the service address, as a two-letter code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    /// ZIP code of the service address.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<Option<String>>,
    /// The office (branch) that serves this patient.
    #[serde(rename = "officeId", skip_serializing_if = "Option::is_none")]
    pub office_id: Option<Option<String>>,
    /// The referral source that sent this patient to you.
    #[serde(rename = "referralSourceId", skip_serializing_if = "Option::is_none")]
    pub referral_source_id: Option<Option<String>>,
    /// Primary diagnosis, in plain words.
    #[serde(
        rename = "primaryDiagnosisDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_diagnosis_description: Option<Option<String>>,
    /// Whether the patient is a fall risk.
    #[serde(rename = "fallRisk", skip_serializing_if = "Option::is_none")]
    pub fall_risk: Option<Option<bool>>,
    /// Caregiver gender the patient prefers, if any.
    #[serde(
        rename = "preferredCaregiverGender",
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_caregiver_gender: Option<Option<String>>,
    /// Notes schedulers see when booking visits for this patient.
    #[serde(rename = "schedulingNotes", skip_serializing_if = "Option::is_none")]
    pub scheduling_notes: Option<Option<String>>,
    /// Person legally responsible for the patient, if not the patient.
    #[serde(
        rename = "responsiblePartyName",
        skip_serializing_if = "Option::is_none"
    )]
    pub responsible_party_name: Option<Option<String>>,
    /// Medicaid member ID.
    #[serde(rename = "medicaidId", skip_serializing_if = "Option::is_none")]
    pub medicaid_id: Option<Option<String>>,
    /// Medicare beneficiary identifier.
    #[serde(rename = "medicareId", skip_serializing_if = "Option::is_none")]
    pub medicare_id: Option<Option<String>>,
    /// Primary insurance company name.
    #[serde(rename = "insuranceProvider", skip_serializing_if = "Option::is_none")]
    pub insurance_provider: Option<Option<String>>,
    /// Member ID with the primary insurance.
    #[serde(rename = "insuranceId", skip_serializing_if = "Option::is_none")]
    pub insurance_id: Option<Option<String>>,
    /// Group number with the primary insurance.
    #[serde(
        rename = "insuranceGroupNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub insurance_group_number: Option<Option<String>>,
    /// Who receives the bills.
    #[serde(rename = "billingContactName", skip_serializing_if = "Option::is_none")]
    pub billing_contact_name: Option<Option<String>>,
    /// Email address bills are sent to.
    #[serde(
        rename = "billingContactEmail",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_contact_email: Option<Option<String>>,
    /// Phone number of the billing contact.
    #[serde(
        rename = "billingContactPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_contact_phone: Option<Option<String>>,
    /// How statements are delivered.
    #[serde(rename = "statementDelivery", skip_serializing_if = "Option::is_none")]
    pub statement_delivery: Option<Option<PatientStatementDelivery>>,
    /// How the patient usually pays.
    #[serde(
        rename = "paymentMethodPreference",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_method_preference: Option<Option<PatientPaymentMethodPreference>>,
}

impl PatientCreate {
    pub fn new(first_name: impl Into<String>, last_name: impl Into<String>) -> Self {
        Self {
            first_name: first_name.into(),
            middle_name: None,
            last_name: last_name.into(),
            goes_by_name: None,
            date_of_birth: None,
            gender: None,
            primary_language: None,
            email: None,
            active: None,
            status: None,
            lifecycle_stage: None,
            service_type: None,
            address: None,
            city: None,
            state: None,
            zip_code: None,
            office_id: None,
            referral_source_id: None,
            primary_diagnosis_description: None,
            fall_risk: None,
            preferred_caregiver_gender: None,
            scheduling_notes: None,
            responsible_party_name: None,
            medicaid_id: None,
            medicare_id: None,
            insurance_provider: None,
            insurance_id: None,
            insurance_group_number: None,
            billing_contact_name: None,
            billing_contact_email: None,
            billing_contact_phone: None,
            statement_delivery: None,
            payment_method_preference: None,
        }
    }

    /// Middle name.
    pub fn middle_name(mut self, value: impl Into<String>) -> Self {
        self.middle_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `middleName`, clearing it.
    pub fn clear_middle_name(mut self) -> Self {
        self.middle_name = Some(None);
        self
    }

    /// The name the patient prefers to be called.
    pub fn goes_by_name(mut self, value: impl Into<String>) -> Self {
        self.goes_by_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `goesByName`, clearing it.
    pub fn clear_goes_by_name(mut self) -> Self {
        self.goes_by_name = Some(None);
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

    /// Gender as recorded by the agency.
    pub fn gender(mut self, value: impl Into<String>) -> Self {
        self.gender = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `gender`, clearing it.
    pub fn clear_gender(mut self) -> Self {
        self.gender = Some(None);
        self
    }

    /// Language the patient is most comfortable speaking.
    pub fn primary_language(mut self, value: impl Into<String>) -> Self {
        self.primary_language = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `primaryLanguage`, clearing it.
    pub fn clear_primary_language(mut self) -> Self {
        self.primary_language = Some(None);
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

    /// Whether the patient is an active client. Only active clients are scheduled and billed. A
    /// prospective client is always inactive.
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(Some(value));
        self
    }

    /// Sends `null` for `active`, clearing it.
    pub fn clear_active(mut self) -> Self {
        self.active = Some(None);
        self
    }

    /// The client's status as shown in AveeCare. `Inactive` and `Discharged` mean the person is no
    /// longer receiving care. A prospective client (`lifecycleStage: Prospect`) reads `Prospect`
    /// even if you send `Active`.
    pub fn status(mut self, value: PatientCreateStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Whether this person is a prospective client or an admitted client. `null` means client.
    pub fn lifecycle_stage(mut self, value: PatientLifecycleStage) -> Self {
        self.lifecycle_stage = Some(Some(value));
        self
    }

    /// Sends `null` for `lifecycleStage`, clearing it.
    pub fn clear_lifecycle_stage(mut self) -> Self {
        self.lifecycle_stage = Some(None);
        self
    }

    /// The main type of service the patient receives.
    pub fn service_type(mut self, value: PatientServiceType) -> Self {
        self.service_type = Some(Some(value));
        self
    }

    /// Sends `null` for `serviceType`, clearing it.
    pub fn clear_service_type(mut self) -> Self {
        self.service_type = Some(None);
        self
    }

    /// Street address where care is delivered.
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `address`, clearing it.
    pub fn clear_address(mut self) -> Self {
        self.address = Some(None);
        self
    }

    /// City of the service address.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `city`, clearing it.
    pub fn clear_city(mut self) -> Self {
        self.city = Some(None);
        self
    }

    /// State of the service address, as a two-letter code.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `state`, clearing it.
    pub fn clear_state(mut self) -> Self {
        self.state = Some(None);
        self
    }

    /// ZIP code of the service address.
    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `zipCode`, clearing it.
    pub fn clear_zip_code(mut self) -> Self {
        self.zip_code = Some(None);
        self
    }

    /// The office (branch) that serves this patient.
    pub fn office_id(mut self, value: impl Into<String>) -> Self {
        self.office_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `officeId`, clearing it.
    pub fn clear_office_id(mut self) -> Self {
        self.office_id = Some(None);
        self
    }

    /// The referral source that sent this patient to you.
    pub fn referral_source_id(mut self, value: impl Into<String>) -> Self {
        self.referral_source_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `referralSourceId`, clearing it.
    pub fn clear_referral_source_id(mut self) -> Self {
        self.referral_source_id = Some(None);
        self
    }

    /// Primary diagnosis, in plain words.
    pub fn primary_diagnosis_description(mut self, value: impl Into<String>) -> Self {
        self.primary_diagnosis_description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `primaryDiagnosisDescription`, clearing it.
    pub fn clear_primary_diagnosis_description(mut self) -> Self {
        self.primary_diagnosis_description = Some(None);
        self
    }

    /// Whether the patient is a fall risk.
    pub fn fall_risk(mut self, value: bool) -> Self {
        self.fall_risk = Some(Some(value));
        self
    }

    /// Sends `null` for `fallRisk`, clearing it.
    pub fn clear_fall_risk(mut self) -> Self {
        self.fall_risk = Some(None);
        self
    }

    /// Caregiver gender the patient prefers, if any.
    pub fn preferred_caregiver_gender(mut self, value: impl Into<String>) -> Self {
        self.preferred_caregiver_gender = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `preferredCaregiverGender`, clearing it.
    pub fn clear_preferred_caregiver_gender(mut self) -> Self {
        self.preferred_caregiver_gender = Some(None);
        self
    }

    /// Notes schedulers see when booking visits for this patient.
    pub fn scheduling_notes(mut self, value: impl Into<String>) -> Self {
        self.scheduling_notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `schedulingNotes`, clearing it.
    pub fn clear_scheduling_notes(mut self) -> Self {
        self.scheduling_notes = Some(None);
        self
    }

    /// Person legally responsible for the patient, if not the patient.
    pub fn responsible_party_name(mut self, value: impl Into<String>) -> Self {
        self.responsible_party_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `responsiblePartyName`, clearing it.
    pub fn clear_responsible_party_name(mut self) -> Self {
        self.responsible_party_name = Some(None);
        self
    }

    /// Medicaid member ID.
    pub fn medicaid_id(mut self, value: impl Into<String>) -> Self {
        self.medicaid_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `medicaidId`, clearing it.
    pub fn clear_medicaid_id(mut self) -> Self {
        self.medicaid_id = Some(None);
        self
    }

    /// Medicare beneficiary identifier.
    pub fn medicare_id(mut self, value: impl Into<String>) -> Self {
        self.medicare_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `medicareId`, clearing it.
    pub fn clear_medicare_id(mut self) -> Self {
        self.medicare_id = Some(None);
        self
    }

    /// Primary insurance company name.
    pub fn insurance_provider(mut self, value: impl Into<String>) -> Self {
        self.insurance_provider = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `insuranceProvider`, clearing it.
    pub fn clear_insurance_provider(mut self) -> Self {
        self.insurance_provider = Some(None);
        self
    }

    /// Member ID with the primary insurance.
    pub fn insurance_id(mut self, value: impl Into<String>) -> Self {
        self.insurance_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `insuranceId`, clearing it.
    pub fn clear_insurance_id(mut self) -> Self {
        self.insurance_id = Some(None);
        self
    }

    /// Group number with the primary insurance.
    pub fn insurance_group_number(mut self, value: impl Into<String>) -> Self {
        self.insurance_group_number = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `insuranceGroupNumber`, clearing it.
    pub fn clear_insurance_group_number(mut self) -> Self {
        self.insurance_group_number = Some(None);
        self
    }

    /// Who receives the bills.
    pub fn billing_contact_name(mut self, value: impl Into<String>) -> Self {
        self.billing_contact_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `billingContactName`, clearing it.
    pub fn clear_billing_contact_name(mut self) -> Self {
        self.billing_contact_name = Some(None);
        self
    }

    /// Email address bills are sent to.
    pub fn billing_contact_email(mut self, value: impl Into<String>) -> Self {
        self.billing_contact_email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `billingContactEmail`, clearing it.
    pub fn clear_billing_contact_email(mut self) -> Self {
        self.billing_contact_email = Some(None);
        self
    }

    /// Phone number of the billing contact.
    pub fn billing_contact_phone(mut self, value: impl Into<String>) -> Self {
        self.billing_contact_phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `billingContactPhone`, clearing it.
    pub fn clear_billing_contact_phone(mut self) -> Self {
        self.billing_contact_phone = Some(None);
        self
    }

    /// How statements are delivered.
    pub fn statement_delivery(mut self, value: PatientStatementDelivery) -> Self {
        self.statement_delivery = Some(Some(value));
        self
    }

    /// Sends `null` for `statementDelivery`, clearing it.
    pub fn clear_statement_delivery(mut self) -> Self {
        self.statement_delivery = Some(None);
        self
    }

    /// How the patient usually pays.
    pub fn payment_method_preference(mut self, value: PatientPaymentMethodPreference) -> Self {
        self.payment_method_preference = Some(Some(value));
        self
    }

    /// Sends `null` for `paymentMethodPreference`, clearing it.
    pub fn clear_payment_method_preference(mut self) -> Self {
        self.payment_method_preference = Some(None);
        self
    }
}

/// Fields to change on a patient. Send only the fields to change; any other field is refused with
/// `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct PatientUpdate {
    /// Given name.
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Middle name.
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<Option<String>>,
    /// Family name.
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The name the patient prefers to be called.
    #[serde(rename = "goesByName", skip_serializing_if = "Option::is_none")]
    pub goes_by_name: Option<Option<String>>,
    /// Date of birth.
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<Option<String>>,
    /// Gender as recorded by the agency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Option<String>>,
    /// Language the patient is most comfortable speaking.
    #[serde(rename = "primaryLanguage", skip_serializing_if = "Option::is_none")]
    pub primary_language: Option<Option<String>>,
    /// Contact email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// Whether the patient is an active client. Only active clients are scheduled and billed. A
    /// prospective client is always inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<Option<bool>>,
    /// The client's status as shown in AveeCare. `Inactive` and `Discharged` mean the person is no
    /// longer receiving care. A prospective client (`lifecycleStage: Prospect`) reads `Prospect`
    /// even if you send `Active`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<PatientCreateStatus>>,
    /// Whether this person is a prospective client or an admitted client. `null` means client.
    #[serde(rename = "lifecycleStage", skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<Option<PatientLifecycleStage>>,
    /// The main type of service the patient receives.
    #[serde(rename = "serviceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<Option<PatientServiceType>>,
    /// Street address where care is delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<String>>,
    /// City of the service address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// State of the service address, as a two-letter code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    /// ZIP code of the service address.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<Option<String>>,
    /// The office (branch) that serves this patient.
    #[serde(rename = "officeId", skip_serializing_if = "Option::is_none")]
    pub office_id: Option<Option<String>>,
    /// The referral source that sent this patient to you.
    #[serde(rename = "referralSourceId", skip_serializing_if = "Option::is_none")]
    pub referral_source_id: Option<Option<String>>,
    /// Primary diagnosis, in plain words.
    #[serde(
        rename = "primaryDiagnosisDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_diagnosis_description: Option<Option<String>>,
    /// Whether the patient is a fall risk.
    #[serde(rename = "fallRisk", skip_serializing_if = "Option::is_none")]
    pub fall_risk: Option<Option<bool>>,
    /// Caregiver gender the patient prefers, if any.
    #[serde(
        rename = "preferredCaregiverGender",
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_caregiver_gender: Option<Option<String>>,
    /// Notes schedulers see when booking visits for this patient.
    #[serde(rename = "schedulingNotes", skip_serializing_if = "Option::is_none")]
    pub scheduling_notes: Option<Option<String>>,
    /// Person legally responsible for the patient, if not the patient.
    #[serde(
        rename = "responsiblePartyName",
        skip_serializing_if = "Option::is_none"
    )]
    pub responsible_party_name: Option<Option<String>>,
    /// Medicaid member ID.
    #[serde(rename = "medicaidId", skip_serializing_if = "Option::is_none")]
    pub medicaid_id: Option<Option<String>>,
    /// Medicare beneficiary identifier.
    #[serde(rename = "medicareId", skip_serializing_if = "Option::is_none")]
    pub medicare_id: Option<Option<String>>,
    /// Primary insurance company name.
    #[serde(rename = "insuranceProvider", skip_serializing_if = "Option::is_none")]
    pub insurance_provider: Option<Option<String>>,
    /// Member ID with the primary insurance.
    #[serde(rename = "insuranceId", skip_serializing_if = "Option::is_none")]
    pub insurance_id: Option<Option<String>>,
    /// Group number with the primary insurance.
    #[serde(
        rename = "insuranceGroupNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub insurance_group_number: Option<Option<String>>,
    /// Who receives the bills.
    #[serde(rename = "billingContactName", skip_serializing_if = "Option::is_none")]
    pub billing_contact_name: Option<Option<String>>,
    /// Email address bills are sent to.
    #[serde(
        rename = "billingContactEmail",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_contact_email: Option<Option<String>>,
    /// Phone number of the billing contact.
    #[serde(
        rename = "billingContactPhone",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_contact_phone: Option<Option<String>>,
    /// How statements are delivered.
    #[serde(rename = "statementDelivery", skip_serializing_if = "Option::is_none")]
    pub statement_delivery: Option<Option<PatientStatementDelivery>>,
    /// How the patient usually pays.
    #[serde(
        rename = "paymentMethodPreference",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_method_preference: Option<Option<PatientPaymentMethodPreference>>,
}

impl PatientUpdate {
    pub fn new() -> Self {
        Self {
            first_name: None,
            middle_name: None,
            last_name: None,
            goes_by_name: None,
            date_of_birth: None,
            gender: None,
            primary_language: None,
            email: None,
            active: None,
            status: None,
            lifecycle_stage: None,
            service_type: None,
            address: None,
            city: None,
            state: None,
            zip_code: None,
            office_id: None,
            referral_source_id: None,
            primary_diagnosis_description: None,
            fall_risk: None,
            preferred_caregiver_gender: None,
            scheduling_notes: None,
            responsible_party_name: None,
            medicaid_id: None,
            medicare_id: None,
            insurance_provider: None,
            insurance_id: None,
            insurance_group_number: None,
            billing_contact_name: None,
            billing_contact_email: None,
            billing_contact_phone: None,
            statement_delivery: None,
            payment_method_preference: None,
        }
    }

    /// Given name.
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    /// Middle name.
    pub fn middle_name(mut self, value: impl Into<String>) -> Self {
        self.middle_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `middleName`, clearing it.
    pub fn clear_middle_name(mut self) -> Self {
        self.middle_name = Some(None);
        self
    }

    /// Family name.
    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// The name the patient prefers to be called.
    pub fn goes_by_name(mut self, value: impl Into<String>) -> Self {
        self.goes_by_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `goesByName`, clearing it.
    pub fn clear_goes_by_name(mut self) -> Self {
        self.goes_by_name = Some(None);
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

    /// Gender as recorded by the agency.
    pub fn gender(mut self, value: impl Into<String>) -> Self {
        self.gender = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `gender`, clearing it.
    pub fn clear_gender(mut self) -> Self {
        self.gender = Some(None);
        self
    }

    /// Language the patient is most comfortable speaking.
    pub fn primary_language(mut self, value: impl Into<String>) -> Self {
        self.primary_language = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `primaryLanguage`, clearing it.
    pub fn clear_primary_language(mut self) -> Self {
        self.primary_language = Some(None);
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

    /// Whether the patient is an active client. Only active clients are scheduled and billed. A
    /// prospective client is always inactive.
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(Some(value));
        self
    }

    /// Sends `null` for `active`, clearing it.
    pub fn clear_active(mut self) -> Self {
        self.active = Some(None);
        self
    }

    /// The client's status as shown in AveeCare. `Inactive` and `Discharged` mean the person is no
    /// longer receiving care. A prospective client (`lifecycleStage: Prospect`) reads `Prospect`
    /// even if you send `Active`.
    pub fn status(mut self, value: PatientCreateStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Whether this person is a prospective client or an admitted client. `null` means client.
    pub fn lifecycle_stage(mut self, value: PatientLifecycleStage) -> Self {
        self.lifecycle_stage = Some(Some(value));
        self
    }

    /// Sends `null` for `lifecycleStage`, clearing it.
    pub fn clear_lifecycle_stage(mut self) -> Self {
        self.lifecycle_stage = Some(None);
        self
    }

    /// The main type of service the patient receives.
    pub fn service_type(mut self, value: PatientServiceType) -> Self {
        self.service_type = Some(Some(value));
        self
    }

    /// Sends `null` for `serviceType`, clearing it.
    pub fn clear_service_type(mut self) -> Self {
        self.service_type = Some(None);
        self
    }

    /// Street address where care is delivered.
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `address`, clearing it.
    pub fn clear_address(mut self) -> Self {
        self.address = Some(None);
        self
    }

    /// City of the service address.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `city`, clearing it.
    pub fn clear_city(mut self) -> Self {
        self.city = Some(None);
        self
    }

    /// State of the service address, as a two-letter code.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `state`, clearing it.
    pub fn clear_state(mut self) -> Self {
        self.state = Some(None);
        self
    }

    /// ZIP code of the service address.
    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `zipCode`, clearing it.
    pub fn clear_zip_code(mut self) -> Self {
        self.zip_code = Some(None);
        self
    }

    /// The office (branch) that serves this patient.
    pub fn office_id(mut self, value: impl Into<String>) -> Self {
        self.office_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `officeId`, clearing it.
    pub fn clear_office_id(mut self) -> Self {
        self.office_id = Some(None);
        self
    }

    /// The referral source that sent this patient to you.
    pub fn referral_source_id(mut self, value: impl Into<String>) -> Self {
        self.referral_source_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `referralSourceId`, clearing it.
    pub fn clear_referral_source_id(mut self) -> Self {
        self.referral_source_id = Some(None);
        self
    }

    /// Primary diagnosis, in plain words.
    pub fn primary_diagnosis_description(mut self, value: impl Into<String>) -> Self {
        self.primary_diagnosis_description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `primaryDiagnosisDescription`, clearing it.
    pub fn clear_primary_diagnosis_description(mut self) -> Self {
        self.primary_diagnosis_description = Some(None);
        self
    }

    /// Whether the patient is a fall risk.
    pub fn fall_risk(mut self, value: bool) -> Self {
        self.fall_risk = Some(Some(value));
        self
    }

    /// Sends `null` for `fallRisk`, clearing it.
    pub fn clear_fall_risk(mut self) -> Self {
        self.fall_risk = Some(None);
        self
    }

    /// Caregiver gender the patient prefers, if any.
    pub fn preferred_caregiver_gender(mut self, value: impl Into<String>) -> Self {
        self.preferred_caregiver_gender = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `preferredCaregiverGender`, clearing it.
    pub fn clear_preferred_caregiver_gender(mut self) -> Self {
        self.preferred_caregiver_gender = Some(None);
        self
    }

    /// Notes schedulers see when booking visits for this patient.
    pub fn scheduling_notes(mut self, value: impl Into<String>) -> Self {
        self.scheduling_notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `schedulingNotes`, clearing it.
    pub fn clear_scheduling_notes(mut self) -> Self {
        self.scheduling_notes = Some(None);
        self
    }

    /// Person legally responsible for the patient, if not the patient.
    pub fn responsible_party_name(mut self, value: impl Into<String>) -> Self {
        self.responsible_party_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `responsiblePartyName`, clearing it.
    pub fn clear_responsible_party_name(mut self) -> Self {
        self.responsible_party_name = Some(None);
        self
    }

    /// Medicaid member ID.
    pub fn medicaid_id(mut self, value: impl Into<String>) -> Self {
        self.medicaid_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `medicaidId`, clearing it.
    pub fn clear_medicaid_id(mut self) -> Self {
        self.medicaid_id = Some(None);
        self
    }

    /// Medicare beneficiary identifier.
    pub fn medicare_id(mut self, value: impl Into<String>) -> Self {
        self.medicare_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `medicareId`, clearing it.
    pub fn clear_medicare_id(mut self) -> Self {
        self.medicare_id = Some(None);
        self
    }

    /// Primary insurance company name.
    pub fn insurance_provider(mut self, value: impl Into<String>) -> Self {
        self.insurance_provider = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `insuranceProvider`, clearing it.
    pub fn clear_insurance_provider(mut self) -> Self {
        self.insurance_provider = Some(None);
        self
    }

    /// Member ID with the primary insurance.
    pub fn insurance_id(mut self, value: impl Into<String>) -> Self {
        self.insurance_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `insuranceId`, clearing it.
    pub fn clear_insurance_id(mut self) -> Self {
        self.insurance_id = Some(None);
        self
    }

    /// Group number with the primary insurance.
    pub fn insurance_group_number(mut self, value: impl Into<String>) -> Self {
        self.insurance_group_number = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `insuranceGroupNumber`, clearing it.
    pub fn clear_insurance_group_number(mut self) -> Self {
        self.insurance_group_number = Some(None);
        self
    }

    /// Who receives the bills.
    pub fn billing_contact_name(mut self, value: impl Into<String>) -> Self {
        self.billing_contact_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `billingContactName`, clearing it.
    pub fn clear_billing_contact_name(mut self) -> Self {
        self.billing_contact_name = Some(None);
        self
    }

    /// Email address bills are sent to.
    pub fn billing_contact_email(mut self, value: impl Into<String>) -> Self {
        self.billing_contact_email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `billingContactEmail`, clearing it.
    pub fn clear_billing_contact_email(mut self) -> Self {
        self.billing_contact_email = Some(None);
        self
    }

    /// Phone number of the billing contact.
    pub fn billing_contact_phone(mut self, value: impl Into<String>) -> Self {
        self.billing_contact_phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `billingContactPhone`, clearing it.
    pub fn clear_billing_contact_phone(mut self) -> Self {
        self.billing_contact_phone = Some(None);
        self
    }

    /// How statements are delivered.
    pub fn statement_delivery(mut self, value: PatientStatementDelivery) -> Self {
        self.statement_delivery = Some(Some(value));
        self
    }

    /// Sends `null` for `statementDelivery`, clearing it.
    pub fn clear_statement_delivery(mut self) -> Self {
        self.statement_delivery = Some(None);
        self
    }

    /// How the patient usually pays.
    pub fn payment_method_preference(mut self, value: PatientPaymentMethodPreference) -> Self {
        self.payment_method_preference = Some(Some(value));
        self
    }

    /// Sends `null` for `paymentMethodPreference`, clearing it.
    pub fn clear_payment_method_preference(mut self) -> Self {
        self.payment_method_preference = Some(None);
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
    /// ID of the deleted patient.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Contacts are family members and other people connected to a patient, such as an emergency
/// contact. Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Contact {
    /// String representing the object's type. Always `contact`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the contact.
    #[serde(default)]
    pub id: String,
    /// The patient this contact belongs to.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// Full name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Relationship to the patient.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
    /// Phone number.
    #[serde(
        rename = "phoneNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_number: Option<String>,
    /// Email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Whether the contact prefers email over phone.
    #[serde(
        rename = "preferEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prefer_email: Option<bool>,
    /// Whether this person is an emergency contact.
    #[serde(
        rename = "emergencyContact",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub emergency_contact: Option<bool>,
    /// Which updates the contact is notified about.
    #[serde(
        rename = "notificationPolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_policy: Option<ContactNotificationPolicy>,
    /// When the contact was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the contact was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new contact. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ContactCreate {
    /// The patient this contact belongs to.
    #[serde(rename = "patientId")]
    pub patient_id: String,
    /// Full name.
    pub name: String,
    /// Relationship to the patient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Option<String>>,
    /// Phone number.
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Option<String>>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// Whether the contact prefers email over phone.
    #[serde(rename = "preferEmail", skip_serializing_if = "Option::is_none")]
    pub prefer_email: Option<Option<bool>>,
    /// Whether this person is an emergency contact.
    #[serde(rename = "emergencyContact", skip_serializing_if = "Option::is_none")]
    pub emergency_contact: Option<Option<bool>>,
    /// Which updates the contact is notified about.
    #[serde(rename = "notificationPolicy", skip_serializing_if = "Option::is_none")]
    pub notification_policy: Option<Option<ContactNotificationPolicy>>,
}

impl ContactCreate {
    pub fn new(patient_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            patient_id: patient_id.into(),
            name: name.into(),
            relationship: None,
            phone_number: None,
            email: None,
            prefer_email: None,
            emergency_contact: None,
            notification_policy: None,
        }
    }

    /// Relationship to the patient.
    pub fn relationship(mut self, value: impl Into<String>) -> Self {
        self.relationship = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `relationship`, clearing it.
    pub fn clear_relationship(mut self) -> Self {
        self.relationship = Some(None);
        self
    }

    /// Phone number.
    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `phoneNumber`, clearing it.
    pub fn clear_phone_number(mut self) -> Self {
        self.phone_number = Some(None);
        self
    }

    /// Email address.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `email`, clearing it.
    pub fn clear_email(mut self) -> Self {
        self.email = Some(None);
        self
    }

    /// Whether the contact prefers email over phone.
    pub fn prefer_email(mut self, value: bool) -> Self {
        self.prefer_email = Some(Some(value));
        self
    }

    /// Sends `null` for `preferEmail`, clearing it.
    pub fn clear_prefer_email(mut self) -> Self {
        self.prefer_email = Some(None);
        self
    }

    /// Whether this person is an emergency contact.
    pub fn emergency_contact(mut self, value: bool) -> Self {
        self.emergency_contact = Some(Some(value));
        self
    }

    /// Sends `null` for `emergencyContact`, clearing it.
    pub fn clear_emergency_contact(mut self) -> Self {
        self.emergency_contact = Some(None);
        self
    }

    /// Which updates the contact is notified about.
    pub fn notification_policy(mut self, value: ContactNotificationPolicy) -> Self {
        self.notification_policy = Some(Some(value));
        self
    }

    /// Sends `null` for `notificationPolicy`, clearing it.
    pub fn clear_notification_policy(mut self) -> Self {
        self.notification_policy = Some(None);
        self
    }
}

/// Fields to change on a contact. Send only the fields to change; any other field is refused with
/// `unknown_field`. `patientId` is set when the contact is created and cannot be changed
/// afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ContactUpdate {
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Relationship to the patient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Option<String>>,
    /// Phone number.
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Option<String>>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// Whether the contact prefers email over phone.
    #[serde(rename = "preferEmail", skip_serializing_if = "Option::is_none")]
    pub prefer_email: Option<Option<bool>>,
    /// Whether this person is an emergency contact.
    #[serde(rename = "emergencyContact", skip_serializing_if = "Option::is_none")]
    pub emergency_contact: Option<Option<bool>>,
    /// Which updates the contact is notified about.
    #[serde(rename = "notificationPolicy", skip_serializing_if = "Option::is_none")]
    pub notification_policy: Option<Option<ContactNotificationPolicy>>,
}

impl ContactUpdate {
    pub fn new() -> Self {
        Self {
            name: None,
            relationship: None,
            phone_number: None,
            email: None,
            prefer_email: None,
            emergency_contact: None,
            notification_policy: None,
        }
    }

    /// Full name.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Relationship to the patient.
    pub fn relationship(mut self, value: impl Into<String>) -> Self {
        self.relationship = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `relationship`, clearing it.
    pub fn clear_relationship(mut self) -> Self {
        self.relationship = Some(None);
        self
    }

    /// Phone number.
    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `phoneNumber`, clearing it.
    pub fn clear_phone_number(mut self) -> Self {
        self.phone_number = Some(None);
        self
    }

    /// Email address.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `email`, clearing it.
    pub fn clear_email(mut self) -> Self {
        self.email = Some(None);
        self
    }

    /// Whether the contact prefers email over phone.
    pub fn prefer_email(mut self, value: bool) -> Self {
        self.prefer_email = Some(Some(value));
        self
    }

    /// Sends `null` for `preferEmail`, clearing it.
    pub fn clear_prefer_email(mut self) -> Self {
        self.prefer_email = Some(None);
        self
    }

    /// Whether this person is an emergency contact.
    pub fn emergency_contact(mut self, value: bool) -> Self {
        self.emergency_contact = Some(Some(value));
        self
    }

    /// Sends `null` for `emergencyContact`, clearing it.
    pub fn clear_emergency_contact(mut self) -> Self {
        self.emergency_contact = Some(None);
        self
    }

    /// Which updates the contact is notified about.
    pub fn notification_policy(mut self, value: ContactNotificationPolicy) -> Self {
        self.notification_policy = Some(Some(value));
        self
    }

    /// Sends `null` for `notificationPolicy`, clearing it.
    pub fn clear_notification_policy(mut self) -> Self {
        self.notification_policy = Some(None);
        self
    }
}

/// Confirmation that a contact was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContactDeleted {
    /// Always `contact`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted contact.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Allergies recorded on a patient's profile. Every field is always present; a field with no value
/// is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Allergy {
    /// String representing the object's type. Always `allergy`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the allergy.
    #[serde(default)]
    pub id: String,
    /// The patient who has this allergy.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// What the patient is allergic to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Reaction and other details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When the allergy was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the allergy was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new allergy. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AllergyCreate {
    /// The patient who has this allergy.
    #[serde(rename = "patientId")]
    pub patient_id: String,
    /// What the patient is allergic to.
    pub name: String,
    /// Reaction and other details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

impl AllergyCreate {
    pub fn new(patient_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            patient_id: patient_id.into(),
            name: name.into(),
            description: None,
        }
    }

    /// Reaction and other details.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `description`, clearing it.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }
}

/// Fields to change on an allergy. Send only the fields to change; any other field is refused with
/// `unknown_field`. `patientId` is set when the allergy is created and cannot be changed
/// afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AllergyUpdate {
    /// What the patient is allergic to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Reaction and other details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

impl AllergyUpdate {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
        }
    }

    /// What the patient is allergic to.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Reaction and other details.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `description`, clearing it.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }
}

/// Confirmation that an allergy was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllergyDeleted {
    /// Always `allergy`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted allergy.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Medications on a patient's medication list, with the schedule caregivers follow when they give
/// or assist with them. Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Medication {
    /// String representing the object's type. Always `medication`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the medication.
    #[serde(default)]
    pub id: String,
    /// The patient who takes this medication.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// Drug name, without the strength.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Strength per dose unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strength: Option<String>,
    /// Dose form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    /// How it is taken.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Directions as written on the prescription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directions: Option<String>,
    /// What the medication is for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indication: Option<String>,
    /// `Scheduled` for regular doses, `PRN` for as-needed.
    #[serde(
        rename = "scheduleType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub schedule_type: Option<MedicationScheduleType>,
    /// For as-needed medications, when to give it.
    #[serde(rename = "prnReason", default, skip_serializing_if = "Option::is_none")]
    pub prn_reason: Option<String>,
    /// For as-needed medications, the most doses allowed in a day.
    #[serde(
        rename = "prnMaxDailyDoses",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prn_max_daily_doses: Option<i64>,
    /// For as-needed medications, the minimum hours between doses.
    #[serde(
        rename = "prnMinIntervalHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prn_min_interval_hours: Option<f64>,
    /// Date the patient started the medication.
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Whether the medication is currently given.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<MedicationStatus>,
    /// Date the medication was stopped.
    #[serde(
        rename = "discontinuedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub discontinued_date: Option<String>,
    /// Why the medication was stopped.
    #[serde(
        rename = "discontinuedReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub discontinued_reason: Option<String>,
    /// Prescribing clinician.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prescriber: Option<String>,
    /// Prescriber's phone number.
    #[serde(
        rename = "prescriberPhone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prescriber_phone: Option<String>,
    /// Pharmacy that fills the prescription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pharmacy: Option<String>,
    /// Pharmacy phone number.
    #[serde(
        rename = "pharmacyPhone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pharmacy_phone: Option<String>,
    /// Whether a second person must witness each dose.
    #[serde(
        rename = "requiresWitness",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_witness: Option<bool>,
    /// Doses currently on hand.
    #[serde(
        rename = "quantityOnHand",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quantity_on_hand: Option<f64>,
    /// Refills left on the prescription.
    #[serde(
        rename = "refillsRemaining",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refills_remaining: Option<i64>,
    /// Other notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// When the medication was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the medication was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new medication. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct MedicationCreate {
    /// The patient who takes this medication.
    #[serde(rename = "patientId")]
    pub patient_id: String,
    /// Drug name, without the strength.
    pub name: String,
    /// Strength per dose unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<Option<String>>,
    /// Dose form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<Option<String>>,
    /// How it is taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<Option<String>>,
    /// Directions as written on the prescription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directions: Option<Option<String>>,
    /// What the medication is for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indication: Option<Option<String>>,
    /// `Scheduled` for regular doses, `PRN` for as-needed.
    #[serde(rename = "scheduleType", skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<Option<MedicationScheduleType>>,
    /// For as-needed medications, when to give it.
    #[serde(rename = "prnReason", skip_serializing_if = "Option::is_none")]
    pub prn_reason: Option<Option<String>>,
    /// For as-needed medications, the most doses allowed in a day.
    #[serde(rename = "prnMaxDailyDoses", skip_serializing_if = "Option::is_none")]
    pub prn_max_daily_doses: Option<Option<i64>>,
    /// For as-needed medications, the minimum hours between doses.
    #[serde(
        rename = "prnMinIntervalHours",
        skip_serializing_if = "Option::is_none"
    )]
    pub prn_min_interval_hours: Option<Option<f64>>,
    /// Date the patient started the medication.
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// Whether the medication is currently given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<MedicationStatus>>,
    /// Date the medication was stopped.
    #[serde(rename = "discontinuedDate", skip_serializing_if = "Option::is_none")]
    pub discontinued_date: Option<Option<String>>,
    /// Why the medication was stopped.
    #[serde(rename = "discontinuedReason", skip_serializing_if = "Option::is_none")]
    pub discontinued_reason: Option<Option<String>>,
    /// Prescribing clinician.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber: Option<Option<String>>,
    /// Prescriber's phone number.
    #[serde(rename = "prescriberPhone", skip_serializing_if = "Option::is_none")]
    pub prescriber_phone: Option<Option<String>>,
    /// Pharmacy that fills the prescription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy: Option<Option<String>>,
    /// Pharmacy phone number.
    #[serde(rename = "pharmacyPhone", skip_serializing_if = "Option::is_none")]
    pub pharmacy_phone: Option<Option<String>>,
    /// Whether a second person must witness each dose.
    #[serde(rename = "requiresWitness", skip_serializing_if = "Option::is_none")]
    pub requires_witness: Option<Option<bool>>,
    /// Doses currently on hand.
    #[serde(rename = "quantityOnHand", skip_serializing_if = "Option::is_none")]
    pub quantity_on_hand: Option<Option<f64>>,
    /// Refills left on the prescription.
    #[serde(rename = "refillsRemaining", skip_serializing_if = "Option::is_none")]
    pub refills_remaining: Option<Option<i64>>,
    /// Other notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
}

impl MedicationCreate {
    pub fn new(patient_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            patient_id: patient_id.into(),
            name: name.into(),
            strength: None,
            form: None,
            route: None,
            directions: None,
            indication: None,
            schedule_type: None,
            prn_reason: None,
            prn_max_daily_doses: None,
            prn_min_interval_hours: None,
            start_date: None,
            status: None,
            discontinued_date: None,
            discontinued_reason: None,
            prescriber: None,
            prescriber_phone: None,
            pharmacy: None,
            pharmacy_phone: None,
            requires_witness: None,
            quantity_on_hand: None,
            refills_remaining: None,
            notes: None,
        }
    }

    /// Strength per dose unit.
    pub fn strength(mut self, value: impl Into<String>) -> Self {
        self.strength = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `strength`, clearing it.
    pub fn clear_strength(mut self) -> Self {
        self.strength = Some(None);
        self
    }

    /// Dose form.
    pub fn form(mut self, value: impl Into<String>) -> Self {
        self.form = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `form`, clearing it.
    pub fn clear_form(mut self) -> Self {
        self.form = Some(None);
        self
    }

    /// How it is taken.
    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `route`, clearing it.
    pub fn clear_route(mut self) -> Self {
        self.route = Some(None);
        self
    }

    /// Directions as written on the prescription.
    pub fn directions(mut self, value: impl Into<String>) -> Self {
        self.directions = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `directions`, clearing it.
    pub fn clear_directions(mut self) -> Self {
        self.directions = Some(None);
        self
    }

    /// What the medication is for.
    pub fn indication(mut self, value: impl Into<String>) -> Self {
        self.indication = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `indication`, clearing it.
    pub fn clear_indication(mut self) -> Self {
        self.indication = Some(None);
        self
    }

    /// `Scheduled` for regular doses, `PRN` for as-needed.
    pub fn schedule_type(mut self, value: MedicationScheduleType) -> Self {
        self.schedule_type = Some(Some(value));
        self
    }

    /// Sends `null` for `scheduleType`, clearing it.
    pub fn clear_schedule_type(mut self) -> Self {
        self.schedule_type = Some(None);
        self
    }

    /// For as-needed medications, when to give it.
    pub fn prn_reason(mut self, value: impl Into<String>) -> Self {
        self.prn_reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `prnReason`, clearing it.
    pub fn clear_prn_reason(mut self) -> Self {
        self.prn_reason = Some(None);
        self
    }

    /// For as-needed medications, the most doses allowed in a day.
    pub fn prn_max_daily_doses(mut self, value: i64) -> Self {
        self.prn_max_daily_doses = Some(Some(value));
        self
    }

    /// Sends `null` for `prnMaxDailyDoses`, clearing it.
    pub fn clear_prn_max_daily_doses(mut self) -> Self {
        self.prn_max_daily_doses = Some(None);
        self
    }

    /// For as-needed medications, the minimum hours between doses.
    pub fn prn_min_interval_hours(mut self, value: f64) -> Self {
        self.prn_min_interval_hours = Some(Some(value));
        self
    }

    /// Sends `null` for `prnMinIntervalHours`, clearing it.
    pub fn clear_prn_min_interval_hours(mut self) -> Self {
        self.prn_min_interval_hours = Some(None);
        self
    }

    /// Date the patient started the medication.
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `startDate`, clearing it.
    pub fn clear_start_date(mut self) -> Self {
        self.start_date = Some(None);
        self
    }

    /// Whether the medication is currently given.
    pub fn status(mut self, value: MedicationStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Date the medication was stopped.
    pub fn discontinued_date(mut self, value: impl Into<String>) -> Self {
        self.discontinued_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `discontinuedDate`, clearing it.
    pub fn clear_discontinued_date(mut self) -> Self {
        self.discontinued_date = Some(None);
        self
    }

    /// Why the medication was stopped.
    pub fn discontinued_reason(mut self, value: impl Into<String>) -> Self {
        self.discontinued_reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `discontinuedReason`, clearing it.
    pub fn clear_discontinued_reason(mut self) -> Self {
        self.discontinued_reason = Some(None);
        self
    }

    /// Prescribing clinician.
    pub fn prescriber(mut self, value: impl Into<String>) -> Self {
        self.prescriber = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `prescriber`, clearing it.
    pub fn clear_prescriber(mut self) -> Self {
        self.prescriber = Some(None);
        self
    }

    /// Prescriber's phone number.
    pub fn prescriber_phone(mut self, value: impl Into<String>) -> Self {
        self.prescriber_phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `prescriberPhone`, clearing it.
    pub fn clear_prescriber_phone(mut self) -> Self {
        self.prescriber_phone = Some(None);
        self
    }

    /// Pharmacy that fills the prescription.
    pub fn pharmacy(mut self, value: impl Into<String>) -> Self {
        self.pharmacy = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `pharmacy`, clearing it.
    pub fn clear_pharmacy(mut self) -> Self {
        self.pharmacy = Some(None);
        self
    }

    /// Pharmacy phone number.
    pub fn pharmacy_phone(mut self, value: impl Into<String>) -> Self {
        self.pharmacy_phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `pharmacyPhone`, clearing it.
    pub fn clear_pharmacy_phone(mut self) -> Self {
        self.pharmacy_phone = Some(None);
        self
    }

    /// Whether a second person must witness each dose.
    pub fn requires_witness(mut self, value: bool) -> Self {
        self.requires_witness = Some(Some(value));
        self
    }

    /// Sends `null` for `requiresWitness`, clearing it.
    pub fn clear_requires_witness(mut self) -> Self {
        self.requires_witness = Some(None);
        self
    }

    /// Doses currently on hand.
    pub fn quantity_on_hand(mut self, value: f64) -> Self {
        self.quantity_on_hand = Some(Some(value));
        self
    }

    /// Sends `null` for `quantityOnHand`, clearing it.
    pub fn clear_quantity_on_hand(mut self) -> Self {
        self.quantity_on_hand = Some(None);
        self
    }

    /// Refills left on the prescription.
    pub fn refills_remaining(mut self, value: i64) -> Self {
        self.refills_remaining = Some(Some(value));
        self
    }

    /// Sends `null` for `refillsRemaining`, clearing it.
    pub fn clear_refills_remaining(mut self) -> Self {
        self.refills_remaining = Some(None);
        self
    }

    /// Other notes.
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

/// Fields to change on a medication. Send only the fields to change; any other field is refused
/// with `unknown_field`. `patientId` is set when the medication is created and cannot be changed
/// afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct MedicationUpdate {
    /// Drug name, without the strength.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Strength per dose unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<Option<String>>,
    /// Dose form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<Option<String>>,
    /// How it is taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<Option<String>>,
    /// Directions as written on the prescription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directions: Option<Option<String>>,
    /// What the medication is for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indication: Option<Option<String>>,
    /// `Scheduled` for regular doses, `PRN` for as-needed.
    #[serde(rename = "scheduleType", skip_serializing_if = "Option::is_none")]
    pub schedule_type: Option<Option<MedicationScheduleType>>,
    /// For as-needed medications, when to give it.
    #[serde(rename = "prnReason", skip_serializing_if = "Option::is_none")]
    pub prn_reason: Option<Option<String>>,
    /// For as-needed medications, the most doses allowed in a day.
    #[serde(rename = "prnMaxDailyDoses", skip_serializing_if = "Option::is_none")]
    pub prn_max_daily_doses: Option<Option<i64>>,
    /// For as-needed medications, the minimum hours between doses.
    #[serde(
        rename = "prnMinIntervalHours",
        skip_serializing_if = "Option::is_none"
    )]
    pub prn_min_interval_hours: Option<Option<f64>>,
    /// Date the patient started the medication.
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// Whether the medication is currently given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<MedicationStatus>>,
    /// Date the medication was stopped.
    #[serde(rename = "discontinuedDate", skip_serializing_if = "Option::is_none")]
    pub discontinued_date: Option<Option<String>>,
    /// Why the medication was stopped.
    #[serde(rename = "discontinuedReason", skip_serializing_if = "Option::is_none")]
    pub discontinued_reason: Option<Option<String>>,
    /// Prescribing clinician.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber: Option<Option<String>>,
    /// Prescriber's phone number.
    #[serde(rename = "prescriberPhone", skip_serializing_if = "Option::is_none")]
    pub prescriber_phone: Option<Option<String>>,
    /// Pharmacy that fills the prescription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy: Option<Option<String>>,
    /// Pharmacy phone number.
    #[serde(rename = "pharmacyPhone", skip_serializing_if = "Option::is_none")]
    pub pharmacy_phone: Option<Option<String>>,
    /// Whether a second person must witness each dose.
    #[serde(rename = "requiresWitness", skip_serializing_if = "Option::is_none")]
    pub requires_witness: Option<Option<bool>>,
    /// Doses currently on hand.
    #[serde(rename = "quantityOnHand", skip_serializing_if = "Option::is_none")]
    pub quantity_on_hand: Option<Option<f64>>,
    /// Refills left on the prescription.
    #[serde(rename = "refillsRemaining", skip_serializing_if = "Option::is_none")]
    pub refills_remaining: Option<Option<i64>>,
    /// Other notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
}

impl MedicationUpdate {
    pub fn new() -> Self {
        Self {
            name: None,
            strength: None,
            form: None,
            route: None,
            directions: None,
            indication: None,
            schedule_type: None,
            prn_reason: None,
            prn_max_daily_doses: None,
            prn_min_interval_hours: None,
            start_date: None,
            status: None,
            discontinued_date: None,
            discontinued_reason: None,
            prescriber: None,
            prescriber_phone: None,
            pharmacy: None,
            pharmacy_phone: None,
            requires_witness: None,
            quantity_on_hand: None,
            refills_remaining: None,
            notes: None,
        }
    }

    /// Drug name, without the strength.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Strength per dose unit.
    pub fn strength(mut self, value: impl Into<String>) -> Self {
        self.strength = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `strength`, clearing it.
    pub fn clear_strength(mut self) -> Self {
        self.strength = Some(None);
        self
    }

    /// Dose form.
    pub fn form(mut self, value: impl Into<String>) -> Self {
        self.form = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `form`, clearing it.
    pub fn clear_form(mut self) -> Self {
        self.form = Some(None);
        self
    }

    /// How it is taken.
    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `route`, clearing it.
    pub fn clear_route(mut self) -> Self {
        self.route = Some(None);
        self
    }

    /// Directions as written on the prescription.
    pub fn directions(mut self, value: impl Into<String>) -> Self {
        self.directions = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `directions`, clearing it.
    pub fn clear_directions(mut self) -> Self {
        self.directions = Some(None);
        self
    }

    /// What the medication is for.
    pub fn indication(mut self, value: impl Into<String>) -> Self {
        self.indication = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `indication`, clearing it.
    pub fn clear_indication(mut self) -> Self {
        self.indication = Some(None);
        self
    }

    /// `Scheduled` for regular doses, `PRN` for as-needed.
    pub fn schedule_type(mut self, value: MedicationScheduleType) -> Self {
        self.schedule_type = Some(Some(value));
        self
    }

    /// Sends `null` for `scheduleType`, clearing it.
    pub fn clear_schedule_type(mut self) -> Self {
        self.schedule_type = Some(None);
        self
    }

    /// For as-needed medications, when to give it.
    pub fn prn_reason(mut self, value: impl Into<String>) -> Self {
        self.prn_reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `prnReason`, clearing it.
    pub fn clear_prn_reason(mut self) -> Self {
        self.prn_reason = Some(None);
        self
    }

    /// For as-needed medications, the most doses allowed in a day.
    pub fn prn_max_daily_doses(mut self, value: i64) -> Self {
        self.prn_max_daily_doses = Some(Some(value));
        self
    }

    /// Sends `null` for `prnMaxDailyDoses`, clearing it.
    pub fn clear_prn_max_daily_doses(mut self) -> Self {
        self.prn_max_daily_doses = Some(None);
        self
    }

    /// For as-needed medications, the minimum hours between doses.
    pub fn prn_min_interval_hours(mut self, value: f64) -> Self {
        self.prn_min_interval_hours = Some(Some(value));
        self
    }

    /// Sends `null` for `prnMinIntervalHours`, clearing it.
    pub fn clear_prn_min_interval_hours(mut self) -> Self {
        self.prn_min_interval_hours = Some(None);
        self
    }

    /// Date the patient started the medication.
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `startDate`, clearing it.
    pub fn clear_start_date(mut self) -> Self {
        self.start_date = Some(None);
        self
    }

    /// Whether the medication is currently given.
    pub fn status(mut self, value: MedicationStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Date the medication was stopped.
    pub fn discontinued_date(mut self, value: impl Into<String>) -> Self {
        self.discontinued_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `discontinuedDate`, clearing it.
    pub fn clear_discontinued_date(mut self) -> Self {
        self.discontinued_date = Some(None);
        self
    }

    /// Why the medication was stopped.
    pub fn discontinued_reason(mut self, value: impl Into<String>) -> Self {
        self.discontinued_reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `discontinuedReason`, clearing it.
    pub fn clear_discontinued_reason(mut self) -> Self {
        self.discontinued_reason = Some(None);
        self
    }

    /// Prescribing clinician.
    pub fn prescriber(mut self, value: impl Into<String>) -> Self {
        self.prescriber = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `prescriber`, clearing it.
    pub fn clear_prescriber(mut self) -> Self {
        self.prescriber = Some(None);
        self
    }

    /// Prescriber's phone number.
    pub fn prescriber_phone(mut self, value: impl Into<String>) -> Self {
        self.prescriber_phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `prescriberPhone`, clearing it.
    pub fn clear_prescriber_phone(mut self) -> Self {
        self.prescriber_phone = Some(None);
        self
    }

    /// Pharmacy that fills the prescription.
    pub fn pharmacy(mut self, value: impl Into<String>) -> Self {
        self.pharmacy = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `pharmacy`, clearing it.
    pub fn clear_pharmacy(mut self) -> Self {
        self.pharmacy = Some(None);
        self
    }

    /// Pharmacy phone number.
    pub fn pharmacy_phone(mut self, value: impl Into<String>) -> Self {
        self.pharmacy_phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `pharmacyPhone`, clearing it.
    pub fn clear_pharmacy_phone(mut self) -> Self {
        self.pharmacy_phone = Some(None);
        self
    }

    /// Whether a second person must witness each dose.
    pub fn requires_witness(mut self, value: bool) -> Self {
        self.requires_witness = Some(Some(value));
        self
    }

    /// Sends `null` for `requiresWitness`, clearing it.
    pub fn clear_requires_witness(mut self) -> Self {
        self.requires_witness = Some(None);
        self
    }

    /// Doses currently on hand.
    pub fn quantity_on_hand(mut self, value: f64) -> Self {
        self.quantity_on_hand = Some(Some(value));
        self
    }

    /// Sends `null` for `quantityOnHand`, clearing it.
    pub fn clear_quantity_on_hand(mut self) -> Self {
        self.quantity_on_hand = Some(None);
        self
    }

    /// Refills left on the prescription.
    pub fn refills_remaining(mut self, value: i64) -> Self {
        self.refills_remaining = Some(Some(value));
        self
    }

    /// Sends `null` for `refillsRemaining`, clearing it.
    pub fn clear_refills_remaining(mut self) -> Self {
        self.refills_remaining = Some(None);
        self
    }

    /// Other notes.
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

/// Confirmation that a medication was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MedicationDeleted {
    /// Always `medication`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted medication.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Goals of care set for a patient, such as walking to the mailbox unaided. Every field is always
/// present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CareGoal {
    /// String representing the object's type. Always `care_goal`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the care goal.
    #[serde(default)]
    pub id: String,
    /// The patient this goal is for.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// Short name of the goal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// What success looks like.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When the care goal was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the care goal was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new care goal. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CareGoalCreate {
    /// The patient this goal is for.
    #[serde(rename = "patientId")]
    pub patient_id: String,
    /// Short name of the goal.
    pub name: String,
    /// What success looks like.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

impl CareGoalCreate {
    pub fn new(patient_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            patient_id: patient_id.into(),
            name: name.into(),
            description: None,
        }
    }

    /// What success looks like.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `description`, clearing it.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }
}

/// Fields to change on a care goal. Send only the fields to change; any other field is refused with
/// `unknown_field`. `patientId` is set when the care goal is created and cannot be changed
/// afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct CareGoalUpdate {
    /// Short name of the goal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// What success looks like.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

impl CareGoalUpdate {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
        }
    }

    /// Short name of the goal.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// What success looks like.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `description`, clearing it.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }
}

/// Confirmation that a care goal was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CareGoalDeleted {
    /// Always `care_goal`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted care goal.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Notes your staff and caregivers keep on a patient's profile. Every field is always present; a
/// field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PatientNote {
    /// String representing the object's type. Always `patient_note`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the patient note.
    #[serde(default)]
    pub id: String,
    /// The patient the note is about.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// The note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The person who wrote the note. Set to the API key's user.
    #[serde(rename = "authorId", default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    /// Display name of the author.
    #[serde(
        rename = "authorName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub author_name: Option<String>,
    /// When the patient note was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the patient note was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new patient note. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct PatientNoteCreate {
    /// The patient the note is about.
    #[serde(rename = "patientId")]
    pub patient_id: String,
    /// The note.
    pub content: String,
}

impl PatientNoteCreate {
    pub fn new(patient_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            patient_id: patient_id.into(),
            content: content.into(),
        }
    }
}

/// Fields to change on a patient note. Send only the fields to change; any other field is refused
/// with `unknown_field`. `patientId` is set when the patient note is created and cannot be changed
/// afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct PatientNoteUpdate {
    /// The note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl PatientNoteUpdate {
    pub fn new() -> Self {
        Self { content: None }
    }

    /// The note.
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }
}

/// Confirmation that a patient note was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PatientNoteDeleted {
    /// Always `patient_note`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted patient note.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// The medication administration record (MAR): each dose a caregiver gave, or the reason it was not
/// given. Read-only. Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MedicationAdministration {
    /// String representing the object's type. Always `medication_administration`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the medication administration.
    #[serde(default)]
    pub id: String,
    /// The medication this dose was for.
    #[serde(
        rename = "medicationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub medication_id: Option<String>,
    /// The patient.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// The visit during which the dose was recorded.
    #[serde(rename = "visitId", default, skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<String>,
    /// Date the dose was due.
    #[serde(
        rename = "scheduledDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_date: Option<String>,
    /// Time slot the dose was due, as `HH:MM`.
    #[serde(
        rename = "scheduledSlot",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_slot: Option<String>,
    /// What happened to the dose.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<MedicationAdministrationStatus>,
    /// When the dose was given.
    #[serde(
        rename = "administeredAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub administered_at: Option<String>,
    /// Who gave or recorded the dose.
    #[serde(
        rename = "administeredByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub administered_by_name: Option<String>,
    /// Why the dose was refused, held or missed.
    #[serde(
        rename = "reasonText",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reason_text: Option<String>,
    /// Note recorded with the dose.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Witness, for medications that require one.
    #[serde(
        rename = "witnessName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub witness_name: Option<String>,
    /// For as-needed doses, why it was given.
    #[serde(rename = "prnReason", default, skip_serializing_if = "Option::is_none")]
    pub prn_reason: Option<String>,
    /// Whether this entry was later corrected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amended: Option<bool>,
    /// When the medication administration was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the medication administration was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// OASIS assessments recorded for Medicare home health patients. This lists each assessment and its
/// status; the item-level answers are not included. Read-only. Every field is always present; a
/// field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OasisAssessment {
    /// String representing the object's type. Always `oasis_assessment`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the OASIS assessment.
    #[serde(default)]
    pub id: String,
    /// The patient assessed.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// Which OASIS time point this assessment covers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timepoint: Option<OasisAssessmentTimepoint>,
    /// OASIS data set version.
    #[serde(
        rename = "specVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub spec_version: Option<String>,
    /// Where the assessment is in its lifecycle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OasisAssessmentStatus>,
    /// Date the assessment was completed (OASIS item M0090).
    #[serde(
        rename = "m0090CompletedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub m0090_completed_date: Option<String>,
    /// Start of the care episode.
    #[serde(
        rename = "episodeStartDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub episode_start_date: Option<String>,
    /// Date the assessment is due.
    #[serde(
        rename = "targetDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_date: Option<String>,
    /// Clinician who performed the assessment.
    #[serde(
        rename = "assessorName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assessor_name: Option<String>,
    /// When the assessment was marked complete.
    #[serde(
        rename = "completedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_at: Option<String>,
    /// When the assessment was exported for submission.
    #[serde(
        rename = "exportedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub exported_at: Option<String>,
    /// Correction number; 0 for the original.
    #[serde(
        rename = "correctionNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub correction_number: Option<i64>,
    /// Notes on the assessment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// When the OASIS assessment was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the OASIS assessment was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Caregivers are the aides, nurses and therapists your agency schedules to visit patients. Every
/// field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Caregiver {
    /// String representing the object's type. Always `caregiver`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the caregiver.
    #[serde(default)]
    pub id: String,
    /// Given name.
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Middle name.
    #[serde(
        rename = "middleName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub middle_name: Option<String>,
    /// Family name.
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The name the caregiver prefers to be called.
    #[serde(
        rename = "goesByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub goes_by_name: Option<String>,
    /// Work email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Employment status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CaregiverStatus>,
    /// Whether the caregiver can be scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Date of birth.
    #[serde(
        rename = "dateOfBirth",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_of_birth: Option<String>,
    /// Gender as recorded by the agency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Language the caregiver is most comfortable speaking.
    #[serde(
        rename = "primaryLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_language: Option<String>,
    /// Home street address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// City.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State, as a two-letter code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ZIP code.
    #[serde(rename = "zipCode", default, skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// The office (branch) the caregiver works from.
    #[serde(rename = "officeId", default, skip_serializing_if = "Option::is_none")]
    pub office_id: Option<String>,
    /// License or credential type.
    #[serde(
        rename = "licenseType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub license_type: Option<String>,
    /// National Provider Identifier, for licensed clinicians.
    #[serde(rename = "npiNumber", default, skip_serializing_if = "Option::is_none")]
    pub npi_number: Option<String>,
    /// State license number.
    #[serde(
        rename = "stateLicenseNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_license_number: Option<String>,
    /// State that issued the license.
    #[serde(
        rename = "stateLicenseState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_license_state: Option<String>,
    /// License expiry date, as `YYYY-MM-DD`.
    #[serde(
        rename = "stateLicenseExpiry",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_license_expiry: Option<String>,
    /// Whether the caregiver works live-in shifts.
    #[serde(
        rename = "isLiveInCaregiver",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_live_in_caregiver: Option<bool>,
    /// Notes schedulers see when booking this caregiver.
    #[serde(
        rename = "schedulingNotes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduling_notes: Option<String>,
    /// Date of the caregiver's most recent visit.
    #[serde(rename = "lastVisit", default, skip_serializing_if = "Option::is_none")]
    pub last_visit: Option<String>,
    /// When the caregiver was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the caregiver was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new caregiver. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CaregiverCreate {
    /// Given name.
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// Middle name.
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<Option<String>>,
    /// Family name.
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// The name the caregiver prefers to be called.
    #[serde(rename = "goesByName", skip_serializing_if = "Option::is_none")]
    pub goes_by_name: Option<Option<String>>,
    /// Work email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// Employment status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<CaregiverStatus>>,
    /// Whether the caregiver can be scheduled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<Option<bool>>,
    /// Date of birth.
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<Option<String>>,
    /// Gender as recorded by the agency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Option<String>>,
    /// Language the caregiver is most comfortable speaking.
    #[serde(rename = "primaryLanguage", skip_serializing_if = "Option::is_none")]
    pub primary_language: Option<Option<String>>,
    /// Home street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<String>>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// State, as a two-letter code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    /// ZIP code.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<Option<String>>,
    /// The office (branch) the caregiver works from.
    #[serde(rename = "officeId", skip_serializing_if = "Option::is_none")]
    pub office_id: Option<Option<String>>,
    /// License or credential type.
    #[serde(rename = "licenseType", skip_serializing_if = "Option::is_none")]
    pub license_type: Option<Option<String>>,
    /// National Provider Identifier, for licensed clinicians.
    #[serde(rename = "npiNumber", skip_serializing_if = "Option::is_none")]
    pub npi_number: Option<Option<String>>,
    /// State license number.
    #[serde(rename = "stateLicenseNumber", skip_serializing_if = "Option::is_none")]
    pub state_license_number: Option<Option<String>>,
    /// State that issued the license.
    #[serde(rename = "stateLicenseState", skip_serializing_if = "Option::is_none")]
    pub state_license_state: Option<Option<String>>,
    /// License expiry date, as `YYYY-MM-DD`.
    #[serde(rename = "stateLicenseExpiry", skip_serializing_if = "Option::is_none")]
    pub state_license_expiry: Option<Option<String>>,
    /// Whether the caregiver works live-in shifts.
    #[serde(rename = "isLiveInCaregiver", skip_serializing_if = "Option::is_none")]
    pub is_live_in_caregiver: Option<Option<bool>>,
    /// Notes schedulers see when booking this caregiver.
    #[serde(rename = "schedulingNotes", skip_serializing_if = "Option::is_none")]
    pub scheduling_notes: Option<Option<String>>,
}

impl CaregiverCreate {
    pub fn new(first_name: impl Into<String>, last_name: impl Into<String>) -> Self {
        Self {
            first_name: first_name.into(),
            middle_name: None,
            last_name: last_name.into(),
            goes_by_name: None,
            email: None,
            status: None,
            active: None,
            date_of_birth: None,
            gender: None,
            primary_language: None,
            address: None,
            city: None,
            state: None,
            zip_code: None,
            office_id: None,
            license_type: None,
            npi_number: None,
            state_license_number: None,
            state_license_state: None,
            state_license_expiry: None,
            is_live_in_caregiver: None,
            scheduling_notes: None,
        }
    }

    /// Middle name.
    pub fn middle_name(mut self, value: impl Into<String>) -> Self {
        self.middle_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `middleName`, clearing it.
    pub fn clear_middle_name(mut self) -> Self {
        self.middle_name = Some(None);
        self
    }

    /// The name the caregiver prefers to be called.
    pub fn goes_by_name(mut self, value: impl Into<String>) -> Self {
        self.goes_by_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `goesByName`, clearing it.
    pub fn clear_goes_by_name(mut self) -> Self {
        self.goes_by_name = Some(None);
        self
    }

    /// Work email address.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `email`, clearing it.
    pub fn clear_email(mut self) -> Self {
        self.email = Some(None);
        self
    }

    /// Employment status.
    pub fn status(mut self, value: CaregiverStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Whether the caregiver can be scheduled.
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(Some(value));
        self
    }

    /// Sends `null` for `active`, clearing it.
    pub fn clear_active(mut self) -> Self {
        self.active = Some(None);
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

    /// Gender as recorded by the agency.
    pub fn gender(mut self, value: impl Into<String>) -> Self {
        self.gender = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `gender`, clearing it.
    pub fn clear_gender(mut self) -> Self {
        self.gender = Some(None);
        self
    }

    /// Language the caregiver is most comfortable speaking.
    pub fn primary_language(mut self, value: impl Into<String>) -> Self {
        self.primary_language = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `primaryLanguage`, clearing it.
    pub fn clear_primary_language(mut self) -> Self {
        self.primary_language = Some(None);
        self
    }

    /// Home street address.
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `address`, clearing it.
    pub fn clear_address(mut self) -> Self {
        self.address = Some(None);
        self
    }

    /// City.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `city`, clearing it.
    pub fn clear_city(mut self) -> Self {
        self.city = Some(None);
        self
    }

    /// State, as a two-letter code.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `state`, clearing it.
    pub fn clear_state(mut self) -> Self {
        self.state = Some(None);
        self
    }

    /// ZIP code.
    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `zipCode`, clearing it.
    pub fn clear_zip_code(mut self) -> Self {
        self.zip_code = Some(None);
        self
    }

    /// The office (branch) the caregiver works from.
    pub fn office_id(mut self, value: impl Into<String>) -> Self {
        self.office_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `officeId`, clearing it.
    pub fn clear_office_id(mut self) -> Self {
        self.office_id = Some(None);
        self
    }

    /// License or credential type.
    pub fn license_type(mut self, value: impl Into<String>) -> Self {
        self.license_type = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `licenseType`, clearing it.
    pub fn clear_license_type(mut self) -> Self {
        self.license_type = Some(None);
        self
    }

    /// National Provider Identifier, for licensed clinicians.
    pub fn npi_number(mut self, value: impl Into<String>) -> Self {
        self.npi_number = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `npiNumber`, clearing it.
    pub fn clear_npi_number(mut self) -> Self {
        self.npi_number = Some(None);
        self
    }

    /// State license number.
    pub fn state_license_number(mut self, value: impl Into<String>) -> Self {
        self.state_license_number = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `stateLicenseNumber`, clearing it.
    pub fn clear_state_license_number(mut self) -> Self {
        self.state_license_number = Some(None);
        self
    }

    /// State that issued the license.
    pub fn state_license_state(mut self, value: impl Into<String>) -> Self {
        self.state_license_state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `stateLicenseState`, clearing it.
    pub fn clear_state_license_state(mut self) -> Self {
        self.state_license_state = Some(None);
        self
    }

    /// License expiry date, as `YYYY-MM-DD`.
    pub fn state_license_expiry(mut self, value: impl Into<String>) -> Self {
        self.state_license_expiry = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `stateLicenseExpiry`, clearing it.
    pub fn clear_state_license_expiry(mut self) -> Self {
        self.state_license_expiry = Some(None);
        self
    }

    /// Whether the caregiver works live-in shifts.
    pub fn is_live_in_caregiver(mut self, value: bool) -> Self {
        self.is_live_in_caregiver = Some(Some(value));
        self
    }

    /// Sends `null` for `isLiveInCaregiver`, clearing it.
    pub fn clear_is_live_in_caregiver(mut self) -> Self {
        self.is_live_in_caregiver = Some(None);
        self
    }

    /// Notes schedulers see when booking this caregiver.
    pub fn scheduling_notes(mut self, value: impl Into<String>) -> Self {
        self.scheduling_notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `schedulingNotes`, clearing it.
    pub fn clear_scheduling_notes(mut self) -> Self {
        self.scheduling_notes = Some(None);
        self
    }
}

/// Fields to change on a caregiver. Send only the fields to change; any other field is refused with
/// `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct CaregiverUpdate {
    /// Given name.
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Middle name.
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<Option<String>>,
    /// Family name.
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The name the caregiver prefers to be called.
    #[serde(rename = "goesByName", skip_serializing_if = "Option::is_none")]
    pub goes_by_name: Option<Option<String>>,
    /// Work email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// Employment status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<CaregiverStatus>>,
    /// Whether the caregiver can be scheduled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<Option<bool>>,
    /// Date of birth.
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<Option<String>>,
    /// Gender as recorded by the agency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Option<String>>,
    /// Language the caregiver is most comfortable speaking.
    #[serde(rename = "primaryLanguage", skip_serializing_if = "Option::is_none")]
    pub primary_language: Option<Option<String>>,
    /// Home street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<String>>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// State, as a two-letter code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    /// ZIP code.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<Option<String>>,
    /// The office (branch) the caregiver works from.
    #[serde(rename = "officeId", skip_serializing_if = "Option::is_none")]
    pub office_id: Option<Option<String>>,
    /// License or credential type.
    #[serde(rename = "licenseType", skip_serializing_if = "Option::is_none")]
    pub license_type: Option<Option<String>>,
    /// National Provider Identifier, for licensed clinicians.
    #[serde(rename = "npiNumber", skip_serializing_if = "Option::is_none")]
    pub npi_number: Option<Option<String>>,
    /// State license number.
    #[serde(rename = "stateLicenseNumber", skip_serializing_if = "Option::is_none")]
    pub state_license_number: Option<Option<String>>,
    /// State that issued the license.
    #[serde(rename = "stateLicenseState", skip_serializing_if = "Option::is_none")]
    pub state_license_state: Option<Option<String>>,
    /// License expiry date, as `YYYY-MM-DD`.
    #[serde(rename = "stateLicenseExpiry", skip_serializing_if = "Option::is_none")]
    pub state_license_expiry: Option<Option<String>>,
    /// Whether the caregiver works live-in shifts.
    #[serde(rename = "isLiveInCaregiver", skip_serializing_if = "Option::is_none")]
    pub is_live_in_caregiver: Option<Option<bool>>,
    /// Notes schedulers see when booking this caregiver.
    #[serde(rename = "schedulingNotes", skip_serializing_if = "Option::is_none")]
    pub scheduling_notes: Option<Option<String>>,
}

impl CaregiverUpdate {
    pub fn new() -> Self {
        Self {
            first_name: None,
            middle_name: None,
            last_name: None,
            goes_by_name: None,
            email: None,
            status: None,
            active: None,
            date_of_birth: None,
            gender: None,
            primary_language: None,
            address: None,
            city: None,
            state: None,
            zip_code: None,
            office_id: None,
            license_type: None,
            npi_number: None,
            state_license_number: None,
            state_license_state: None,
            state_license_expiry: None,
            is_live_in_caregiver: None,
            scheduling_notes: None,
        }
    }

    /// Given name.
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    /// Middle name.
    pub fn middle_name(mut self, value: impl Into<String>) -> Self {
        self.middle_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `middleName`, clearing it.
    pub fn clear_middle_name(mut self) -> Self {
        self.middle_name = Some(None);
        self
    }

    /// Family name.
    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// The name the caregiver prefers to be called.
    pub fn goes_by_name(mut self, value: impl Into<String>) -> Self {
        self.goes_by_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `goesByName`, clearing it.
    pub fn clear_goes_by_name(mut self) -> Self {
        self.goes_by_name = Some(None);
        self
    }

    /// Work email address.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `email`, clearing it.
    pub fn clear_email(mut self) -> Self {
        self.email = Some(None);
        self
    }

    /// Employment status.
    pub fn status(mut self, value: CaregiverStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Whether the caregiver can be scheduled.
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(Some(value));
        self
    }

    /// Sends `null` for `active`, clearing it.
    pub fn clear_active(mut self) -> Self {
        self.active = Some(None);
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

    /// Gender as recorded by the agency.
    pub fn gender(mut self, value: impl Into<String>) -> Self {
        self.gender = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `gender`, clearing it.
    pub fn clear_gender(mut self) -> Self {
        self.gender = Some(None);
        self
    }

    /// Language the caregiver is most comfortable speaking.
    pub fn primary_language(mut self, value: impl Into<String>) -> Self {
        self.primary_language = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `primaryLanguage`, clearing it.
    pub fn clear_primary_language(mut self) -> Self {
        self.primary_language = Some(None);
        self
    }

    /// Home street address.
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `address`, clearing it.
    pub fn clear_address(mut self) -> Self {
        self.address = Some(None);
        self
    }

    /// City.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `city`, clearing it.
    pub fn clear_city(mut self) -> Self {
        self.city = Some(None);
        self
    }

    /// State, as a two-letter code.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `state`, clearing it.
    pub fn clear_state(mut self) -> Self {
        self.state = Some(None);
        self
    }

    /// ZIP code.
    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `zipCode`, clearing it.
    pub fn clear_zip_code(mut self) -> Self {
        self.zip_code = Some(None);
        self
    }

    /// The office (branch) the caregiver works from.
    pub fn office_id(mut self, value: impl Into<String>) -> Self {
        self.office_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `officeId`, clearing it.
    pub fn clear_office_id(mut self) -> Self {
        self.office_id = Some(None);
        self
    }

    /// License or credential type.
    pub fn license_type(mut self, value: impl Into<String>) -> Self {
        self.license_type = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `licenseType`, clearing it.
    pub fn clear_license_type(mut self) -> Self {
        self.license_type = Some(None);
        self
    }

    /// National Provider Identifier, for licensed clinicians.
    pub fn npi_number(mut self, value: impl Into<String>) -> Self {
        self.npi_number = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `npiNumber`, clearing it.
    pub fn clear_npi_number(mut self) -> Self {
        self.npi_number = Some(None);
        self
    }

    /// State license number.
    pub fn state_license_number(mut self, value: impl Into<String>) -> Self {
        self.state_license_number = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `stateLicenseNumber`, clearing it.
    pub fn clear_state_license_number(mut self) -> Self {
        self.state_license_number = Some(None);
        self
    }

    /// State that issued the license.
    pub fn state_license_state(mut self, value: impl Into<String>) -> Self {
        self.state_license_state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `stateLicenseState`, clearing it.
    pub fn clear_state_license_state(mut self) -> Self {
        self.state_license_state = Some(None);
        self
    }

    /// License expiry date, as `YYYY-MM-DD`.
    pub fn state_license_expiry(mut self, value: impl Into<String>) -> Self {
        self.state_license_expiry = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `stateLicenseExpiry`, clearing it.
    pub fn clear_state_license_expiry(mut self) -> Self {
        self.state_license_expiry = Some(None);
        self
    }

    /// Whether the caregiver works live-in shifts.
    pub fn is_live_in_caregiver(mut self, value: bool) -> Self {
        self.is_live_in_caregiver = Some(Some(value));
        self
    }

    /// Sends `null` for `isLiveInCaregiver`, clearing it.
    pub fn clear_is_live_in_caregiver(mut self) -> Self {
        self.is_live_in_caregiver = Some(None);
        self
    }

    /// Notes schedulers see when booking this caregiver.
    pub fn scheduling_notes(mut self, value: impl Into<String>) -> Self {
        self.scheduling_notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `schedulingNotes`, clearing it.
    pub fn clear_scheduling_notes(mut self) -> Self {
        self.scheduling_notes = Some(None);
        self
    }
}

/// Confirmation that a caregiver was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CaregiverDeleted {
    /// Always `caregiver`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted caregiver.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Notes your staff keep on a caregiver's profile. Every field is always present; a field with no
/// value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CaregiverNote {
    /// String representing the object's type. Always `caregiver_note`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the caregiver note.
    #[serde(default)]
    pub id: String,
    /// The caregiver the note is about.
    #[serde(
        rename = "caregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_id: Option<String>,
    /// The note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The person who wrote the note. Set to the API key's user.
    #[serde(rename = "authorId", default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    /// Display name of the author.
    #[serde(
        rename = "authorName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub author_name: Option<String>,
    /// When the caregiver note was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the caregiver note was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new caregiver note. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CaregiverNoteCreate {
    /// The caregiver the note is about.
    #[serde(rename = "caregiverId")]
    pub caregiver_id: String,
    /// The note.
    pub content: String,
}

impl CaregiverNoteCreate {
    pub fn new(caregiver_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            caregiver_id: caregiver_id.into(),
            content: content.into(),
        }
    }
}

/// Fields to change on a caregiver note. Send only the fields to change; any other field is refused
/// with `unknown_field`. `caregiverId` is set when the caregiver note is created and cannot be
/// changed afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct CaregiverNoteUpdate {
    /// The note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl CaregiverNoteUpdate {
    pub fn new() -> Self {
        Self { content: None }
    }

    /// The note.
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }
}

/// Confirmation that a caregiver note was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CaregiverNoteDeleted {
    /// Always `caregiver_note`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted caregiver note.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Certifications, licenses and background checks on a caregiver's profile, with their expiry
/// dates. Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Certification {
    /// String representing the object's type. Always `certification`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the certification.
    #[serde(default)]
    pub id: String,
    /// The caregiver who holds it.
    #[serde(
        rename = "caregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_id: Option<String>,
    /// Name of the certification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Kind of credential, such as a certification or a background check.
    #[serde(rename = "certKind", default, skip_serializing_if = "Option::is_none")]
    pub cert_kind: Option<String>,
    /// Date it was obtained.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obtained: Option<String>,
    /// Date it expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// For background checks, the agency that ran the check.
    #[serde(
        rename = "checkAgency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_agency: Option<String>,
    /// For background checks, the state the check covers.
    #[serde(
        rename = "checkState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_state: Option<String>,
    /// For background checks, the result.
    #[serde(
        rename = "checkResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_result: Option<String>,
    /// Who verified the credential.
    #[serde(
        rename = "verifiedByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verified_by_name: Option<String>,
    /// When the credential was verified.
    #[serde(
        rename = "verifiedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verified_at: Option<String>,
    /// When the certification was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the certification was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new certification. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CertificationCreate {
    /// The caregiver who holds it.
    #[serde(rename = "caregiverId")]
    pub caregiver_id: String,
    /// Name of the certification.
    pub name: String,
    /// Kind of credential, such as a certification or a background check.
    #[serde(rename = "certKind", skip_serializing_if = "Option::is_none")]
    pub cert_kind: Option<Option<String>>,
    /// Date it was obtained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obtained: Option<Option<String>>,
    /// Date it expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<Option<String>>,
    /// For background checks, the agency that ran the check.
    #[serde(rename = "checkAgency", skip_serializing_if = "Option::is_none")]
    pub check_agency: Option<Option<String>>,
    /// For background checks, the state the check covers.
    #[serde(rename = "checkState", skip_serializing_if = "Option::is_none")]
    pub check_state: Option<Option<String>>,
    /// For background checks, the result.
    #[serde(rename = "checkResult", skip_serializing_if = "Option::is_none")]
    pub check_result: Option<Option<String>>,
}

impl CertificationCreate {
    pub fn new(caregiver_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            caregiver_id: caregiver_id.into(),
            name: name.into(),
            cert_kind: None,
            obtained: None,
            expires: None,
            check_agency: None,
            check_state: None,
            check_result: None,
        }
    }

    /// Kind of credential, such as a certification or a background check.
    pub fn cert_kind(mut self, value: impl Into<String>) -> Self {
        self.cert_kind = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `certKind`, clearing it.
    pub fn clear_cert_kind(mut self) -> Self {
        self.cert_kind = Some(None);
        self
    }

    /// Date it was obtained.
    pub fn obtained(mut self, value: impl Into<String>) -> Self {
        self.obtained = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `obtained`, clearing it.
    pub fn clear_obtained(mut self) -> Self {
        self.obtained = Some(None);
        self
    }

    /// Date it expires.
    pub fn expires(mut self, value: impl Into<String>) -> Self {
        self.expires = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `expires`, clearing it.
    pub fn clear_expires(mut self) -> Self {
        self.expires = Some(None);
        self
    }

    /// For background checks, the agency that ran the check.
    pub fn check_agency(mut self, value: impl Into<String>) -> Self {
        self.check_agency = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `checkAgency`, clearing it.
    pub fn clear_check_agency(mut self) -> Self {
        self.check_agency = Some(None);
        self
    }

    /// For background checks, the state the check covers.
    pub fn check_state(mut self, value: impl Into<String>) -> Self {
        self.check_state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `checkState`, clearing it.
    pub fn clear_check_state(mut self) -> Self {
        self.check_state = Some(None);
        self
    }

    /// For background checks, the result.
    pub fn check_result(mut self, value: impl Into<String>) -> Self {
        self.check_result = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `checkResult`, clearing it.
    pub fn clear_check_result(mut self) -> Self {
        self.check_result = Some(None);
        self
    }
}

/// Fields to change on a certification. Send only the fields to change; any other field is refused
/// with `unknown_field`. `caregiverId` is set when the certification is created and cannot be
/// changed afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct CertificationUpdate {
    /// Name of the certification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Kind of credential, such as a certification or a background check.
    #[serde(rename = "certKind", skip_serializing_if = "Option::is_none")]
    pub cert_kind: Option<Option<String>>,
    /// Date it was obtained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obtained: Option<Option<String>>,
    /// Date it expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<Option<String>>,
    /// For background checks, the agency that ran the check.
    #[serde(rename = "checkAgency", skip_serializing_if = "Option::is_none")]
    pub check_agency: Option<Option<String>>,
    /// For background checks, the state the check covers.
    #[serde(rename = "checkState", skip_serializing_if = "Option::is_none")]
    pub check_state: Option<Option<String>>,
    /// For background checks, the result.
    #[serde(rename = "checkResult", skip_serializing_if = "Option::is_none")]
    pub check_result: Option<Option<String>>,
}

impl CertificationUpdate {
    pub fn new() -> Self {
        Self {
            name: None,
            cert_kind: None,
            obtained: None,
            expires: None,
            check_agency: None,
            check_state: None,
            check_result: None,
        }
    }

    /// Name of the certification.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Kind of credential, such as a certification or a background check.
    pub fn cert_kind(mut self, value: impl Into<String>) -> Self {
        self.cert_kind = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `certKind`, clearing it.
    pub fn clear_cert_kind(mut self) -> Self {
        self.cert_kind = Some(None);
        self
    }

    /// Date it was obtained.
    pub fn obtained(mut self, value: impl Into<String>) -> Self {
        self.obtained = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `obtained`, clearing it.
    pub fn clear_obtained(mut self) -> Self {
        self.obtained = Some(None);
        self
    }

    /// Date it expires.
    pub fn expires(mut self, value: impl Into<String>) -> Self {
        self.expires = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `expires`, clearing it.
    pub fn clear_expires(mut self) -> Self {
        self.expires = Some(None);
        self
    }

    /// For background checks, the agency that ran the check.
    pub fn check_agency(mut self, value: impl Into<String>) -> Self {
        self.check_agency = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `checkAgency`, clearing it.
    pub fn clear_check_agency(mut self) -> Self {
        self.check_agency = Some(None);
        self
    }

    /// For background checks, the state the check covers.
    pub fn check_state(mut self, value: impl Into<String>) -> Self {
        self.check_state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `checkState`, clearing it.
    pub fn clear_check_state(mut self) -> Self {
        self.check_state = Some(None);
        self
    }

    /// For background checks, the result.
    pub fn check_result(mut self, value: impl Into<String>) -> Self {
        self.check_result = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `checkResult`, clearing it.
    pub fn clear_check_result(mut self) -> Self {
        self.check_result = Some(None);
        self
    }
}

/// Confirmation that a certification was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CertificationDeleted {
    /// Always `certification`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted certification.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// When a caregiver can or cannot work. A window is either a weekly recurring time on one weekday,
/// or a one-off date range such as time off. Every field is always present; a field with no value
/// is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Availability {
    /// String representing the object's type. Always `availability`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the availability window.
    #[serde(default)]
    pub id: String,
    /// The caregiver.
    #[serde(
        rename = "caregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_id: Option<String>,
    /// `true` when the caregiver can work in this window, `false` for time off.
    #[serde(
        rename = "isAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_available: Option<bool>,
    /// Whether the window repeats weekly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring: Option<bool>,
    /// For weekly windows, the weekday: 0 is Sunday, 6 is Saturday.
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i64>,
    /// For weekly windows, the start time in minutes after midnight.
    #[serde(
        rename = "startMinute",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_minute: Option<i64>,
    /// For weekly windows, the end time in minutes after midnight.
    #[serde(rename = "endMinute", default, skip_serializing_if = "Option::is_none")]
    pub end_minute: Option<i64>,
    /// For weekly windows, repeat every this many weeks. Defaults to every week.
    #[serde(
        rename = "intervalWeeks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval_weeks: Option<i64>,
    /// First date a weekly window applies.
    #[serde(
        rename = "effectiveFrom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effective_from: Option<String>,
    /// Last date a weekly window applies; `null` for no end.
    #[serde(
        rename = "effectiveTo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub effective_to: Option<String>,
    /// For one-off windows, when it starts.
    #[serde(
        rename = "startDateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date_time: Option<String>,
    /// For one-off windows, when it ends.
    #[serde(
        rename = "endDateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date_time: Option<String>,
    /// Whether these are the caregiver's preferred hours.
    #[serde(
        rename = "isPreferred",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_preferred: Option<bool>,
    /// Why the caregiver is unavailable, for time off.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Whether the window is in effect.
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// When the availability window was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the availability window was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new availability window. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AvailabilityCreate {
    /// The caregiver.
    #[serde(rename = "caregiverId")]
    pub caregiver_id: String,
    /// `true` when the caregiver can work in this window, `false` for time off.
    #[serde(rename = "isAvailable", skip_serializing_if = "Option::is_none")]
    pub is_available: Option<Option<bool>>,
    /// Whether the window repeats weekly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Option<bool>>,
    /// For weekly windows, the weekday: 0 is Sunday, 6 is Saturday.
    #[serde(rename = "dayOfWeek", skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<Option<i64>>,
    /// For weekly windows, the start time in minutes after midnight.
    #[serde(rename = "startMinute", skip_serializing_if = "Option::is_none")]
    pub start_minute: Option<Option<i64>>,
    /// For weekly windows, the end time in minutes after midnight.
    #[serde(rename = "endMinute", skip_serializing_if = "Option::is_none")]
    pub end_minute: Option<Option<i64>>,
    /// For weekly windows, repeat every this many weeks. Defaults to every week.
    #[serde(rename = "intervalWeeks", skip_serializing_if = "Option::is_none")]
    pub interval_weeks: Option<Option<i64>>,
    /// First date a weekly window applies.
    #[serde(rename = "effectiveFrom", skip_serializing_if = "Option::is_none")]
    pub effective_from: Option<Option<String>>,
    /// Last date a weekly window applies; `null` for no end.
    #[serde(rename = "effectiveTo", skip_serializing_if = "Option::is_none")]
    pub effective_to: Option<Option<String>>,
    /// For one-off windows, when it starts.
    #[serde(rename = "startDateTime", skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<Option<String>>,
    /// For one-off windows, when it ends.
    #[serde(rename = "endDateTime", skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<Option<String>>,
    /// Whether these are the caregiver's preferred hours.
    #[serde(rename = "isPreferred", skip_serializing_if = "Option::is_none")]
    pub is_preferred: Option<Option<bool>>,
    /// Why the caregiver is unavailable, for time off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<String>>,
    /// Whether the window is in effect.
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<Option<bool>>,
}

impl AvailabilityCreate {
    pub fn new(caregiver_id: impl Into<String>) -> Self {
        Self {
            caregiver_id: caregiver_id.into(),
            is_available: None,
            recurring: None,
            day_of_week: None,
            start_minute: None,
            end_minute: None,
            interval_weeks: None,
            effective_from: None,
            effective_to: None,
            start_date_time: None,
            end_date_time: None,
            is_preferred: None,
            reason: None,
            is_active: None,
        }
    }

    /// `true` when the caregiver can work in this window, `false` for time off.
    pub fn is_available(mut self, value: bool) -> Self {
        self.is_available = Some(Some(value));
        self
    }

    /// Sends `null` for `isAvailable`, clearing it.
    pub fn clear_is_available(mut self) -> Self {
        self.is_available = Some(None);
        self
    }

    /// Whether the window repeats weekly.
    pub fn recurring(mut self, value: bool) -> Self {
        self.recurring = Some(Some(value));
        self
    }

    /// Sends `null` for `recurring`, clearing it.
    pub fn clear_recurring(mut self) -> Self {
        self.recurring = Some(None);
        self
    }

    /// For weekly windows, the weekday: 0 is Sunday, 6 is Saturday.
    pub fn day_of_week(mut self, value: i64) -> Self {
        self.day_of_week = Some(Some(value));
        self
    }

    /// Sends `null` for `dayOfWeek`, clearing it.
    pub fn clear_day_of_week(mut self) -> Self {
        self.day_of_week = Some(None);
        self
    }

    /// For weekly windows, the start time in minutes after midnight.
    pub fn start_minute(mut self, value: i64) -> Self {
        self.start_minute = Some(Some(value));
        self
    }

    /// Sends `null` for `startMinute`, clearing it.
    pub fn clear_start_minute(mut self) -> Self {
        self.start_minute = Some(None);
        self
    }

    /// For weekly windows, the end time in minutes after midnight.
    pub fn end_minute(mut self, value: i64) -> Self {
        self.end_minute = Some(Some(value));
        self
    }

    /// Sends `null` for `endMinute`, clearing it.
    pub fn clear_end_minute(mut self) -> Self {
        self.end_minute = Some(None);
        self
    }

    /// For weekly windows, repeat every this many weeks. Defaults to every week.
    pub fn interval_weeks(mut self, value: i64) -> Self {
        self.interval_weeks = Some(Some(value));
        self
    }

    /// Sends `null` for `intervalWeeks`, clearing it.
    pub fn clear_interval_weeks(mut self) -> Self {
        self.interval_weeks = Some(None);
        self
    }

    /// First date a weekly window applies.
    pub fn effective_from(mut self, value: impl Into<String>) -> Self {
        self.effective_from = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `effectiveFrom`, clearing it.
    pub fn clear_effective_from(mut self) -> Self {
        self.effective_from = Some(None);
        self
    }

    /// Last date a weekly window applies; `null` for no end.
    pub fn effective_to(mut self, value: impl Into<String>) -> Self {
        self.effective_to = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `effectiveTo`, clearing it.
    pub fn clear_effective_to(mut self) -> Self {
        self.effective_to = Some(None);
        self
    }

    /// For one-off windows, when it starts.
    pub fn start_date_time(mut self, value: impl Into<String>) -> Self {
        self.start_date_time = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `startDateTime`, clearing it.
    pub fn clear_start_date_time(mut self) -> Self {
        self.start_date_time = Some(None);
        self
    }

    /// For one-off windows, when it ends.
    pub fn end_date_time(mut self, value: impl Into<String>) -> Self {
        self.end_date_time = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `endDateTime`, clearing it.
    pub fn clear_end_date_time(mut self) -> Self {
        self.end_date_time = Some(None);
        self
    }

    /// Whether these are the caregiver's preferred hours.
    pub fn is_preferred(mut self, value: bool) -> Self {
        self.is_preferred = Some(Some(value));
        self
    }

    /// Sends `null` for `isPreferred`, clearing it.
    pub fn clear_is_preferred(mut self) -> Self {
        self.is_preferred = Some(None);
        self
    }

    /// Why the caregiver is unavailable, for time off.
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `reason`, clearing it.
    pub fn clear_reason(mut self) -> Self {
        self.reason = Some(None);
        self
    }

    /// Whether the window is in effect.
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(Some(value));
        self
    }

    /// Sends `null` for `isActive`, clearing it.
    pub fn clear_is_active(mut self) -> Self {
        self.is_active = Some(None);
        self
    }
}

/// Fields to change on an availability window. Send only the fields to change; any other field is
/// refused with `unknown_field`. `caregiverId` is set when the availability window is created and
/// cannot be changed afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AvailabilityUpdate {
    /// `true` when the caregiver can work in this window, `false` for time off.
    #[serde(rename = "isAvailable", skip_serializing_if = "Option::is_none")]
    pub is_available: Option<Option<bool>>,
    /// Whether the window repeats weekly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Option<bool>>,
    /// For weekly windows, the weekday: 0 is Sunday, 6 is Saturday.
    #[serde(rename = "dayOfWeek", skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<Option<i64>>,
    /// For weekly windows, the start time in minutes after midnight.
    #[serde(rename = "startMinute", skip_serializing_if = "Option::is_none")]
    pub start_minute: Option<Option<i64>>,
    /// For weekly windows, the end time in minutes after midnight.
    #[serde(rename = "endMinute", skip_serializing_if = "Option::is_none")]
    pub end_minute: Option<Option<i64>>,
    /// For weekly windows, repeat every this many weeks. Defaults to every week.
    #[serde(rename = "intervalWeeks", skip_serializing_if = "Option::is_none")]
    pub interval_weeks: Option<Option<i64>>,
    /// First date a weekly window applies.
    #[serde(rename = "effectiveFrom", skip_serializing_if = "Option::is_none")]
    pub effective_from: Option<Option<String>>,
    /// Last date a weekly window applies; `null` for no end.
    #[serde(rename = "effectiveTo", skip_serializing_if = "Option::is_none")]
    pub effective_to: Option<Option<String>>,
    /// For one-off windows, when it starts.
    #[serde(rename = "startDateTime", skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<Option<String>>,
    /// For one-off windows, when it ends.
    #[serde(rename = "endDateTime", skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<Option<String>>,
    /// Whether these are the caregiver's preferred hours.
    #[serde(rename = "isPreferred", skip_serializing_if = "Option::is_none")]
    pub is_preferred: Option<Option<bool>>,
    /// Why the caregiver is unavailable, for time off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<String>>,
    /// Whether the window is in effect.
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<Option<bool>>,
}

impl AvailabilityUpdate {
    pub fn new() -> Self {
        Self {
            is_available: None,
            recurring: None,
            day_of_week: None,
            start_minute: None,
            end_minute: None,
            interval_weeks: None,
            effective_from: None,
            effective_to: None,
            start_date_time: None,
            end_date_time: None,
            is_preferred: None,
            reason: None,
            is_active: None,
        }
    }

    /// `true` when the caregiver can work in this window, `false` for time off.
    pub fn is_available(mut self, value: bool) -> Self {
        self.is_available = Some(Some(value));
        self
    }

    /// Sends `null` for `isAvailable`, clearing it.
    pub fn clear_is_available(mut self) -> Self {
        self.is_available = Some(None);
        self
    }

    /// Whether the window repeats weekly.
    pub fn recurring(mut self, value: bool) -> Self {
        self.recurring = Some(Some(value));
        self
    }

    /// Sends `null` for `recurring`, clearing it.
    pub fn clear_recurring(mut self) -> Self {
        self.recurring = Some(None);
        self
    }

    /// For weekly windows, the weekday: 0 is Sunday, 6 is Saturday.
    pub fn day_of_week(mut self, value: i64) -> Self {
        self.day_of_week = Some(Some(value));
        self
    }

    /// Sends `null` for `dayOfWeek`, clearing it.
    pub fn clear_day_of_week(mut self) -> Self {
        self.day_of_week = Some(None);
        self
    }

    /// For weekly windows, the start time in minutes after midnight.
    pub fn start_minute(mut self, value: i64) -> Self {
        self.start_minute = Some(Some(value));
        self
    }

    /// Sends `null` for `startMinute`, clearing it.
    pub fn clear_start_minute(mut self) -> Self {
        self.start_minute = Some(None);
        self
    }

    /// For weekly windows, the end time in minutes after midnight.
    pub fn end_minute(mut self, value: i64) -> Self {
        self.end_minute = Some(Some(value));
        self
    }

    /// Sends `null` for `endMinute`, clearing it.
    pub fn clear_end_minute(mut self) -> Self {
        self.end_minute = Some(None);
        self
    }

    /// For weekly windows, repeat every this many weeks. Defaults to every week.
    pub fn interval_weeks(mut self, value: i64) -> Self {
        self.interval_weeks = Some(Some(value));
        self
    }

    /// Sends `null` for `intervalWeeks`, clearing it.
    pub fn clear_interval_weeks(mut self) -> Self {
        self.interval_weeks = Some(None);
        self
    }

    /// First date a weekly window applies.
    pub fn effective_from(mut self, value: impl Into<String>) -> Self {
        self.effective_from = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `effectiveFrom`, clearing it.
    pub fn clear_effective_from(mut self) -> Self {
        self.effective_from = Some(None);
        self
    }

    /// Last date a weekly window applies; `null` for no end.
    pub fn effective_to(mut self, value: impl Into<String>) -> Self {
        self.effective_to = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `effectiveTo`, clearing it.
    pub fn clear_effective_to(mut self) -> Self {
        self.effective_to = Some(None);
        self
    }

    /// For one-off windows, when it starts.
    pub fn start_date_time(mut self, value: impl Into<String>) -> Self {
        self.start_date_time = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `startDateTime`, clearing it.
    pub fn clear_start_date_time(mut self) -> Self {
        self.start_date_time = Some(None);
        self
    }

    /// For one-off windows, when it ends.
    pub fn end_date_time(mut self, value: impl Into<String>) -> Self {
        self.end_date_time = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `endDateTime`, clearing it.
    pub fn clear_end_date_time(mut self) -> Self {
        self.end_date_time = Some(None);
        self
    }

    /// Whether these are the caregiver's preferred hours.
    pub fn is_preferred(mut self, value: bool) -> Self {
        self.is_preferred = Some(Some(value));
        self
    }

    /// Sends `null` for `isPreferred`, clearing it.
    pub fn clear_is_preferred(mut self) -> Self {
        self.is_preferred = Some(None);
        self
    }

    /// Why the caregiver is unavailable, for time off.
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `reason`, clearing it.
    pub fn clear_reason(mut self) -> Self {
        self.reason = Some(None);
        self
    }

    /// Whether the window is in effect.
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(Some(value));
        self
    }

    /// Sends `null` for `isActive`, clearing it.
    pub fn clear_is_active(mut self) -> Self {
        self.is_active = Some(None);
        self
    }
}

/// Confirmation that an availability window was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AvailabilityDeleted {
    /// Always `availability`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted availability window.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Skills and specialties on a caregiver's profile, such as dementia care or Hoyer lift transfers.
/// Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Specialization {
    /// String representing the object's type. Always `specialization`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the specialization.
    #[serde(default)]
    pub id: String,
    /// The caregiver.
    #[serde(
        rename = "caregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_id: Option<String>,
    /// Name of the skill.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When the specialization was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the specialization was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new specialization. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SpecializationCreate {
    /// The caregiver.
    #[serde(rename = "caregiverId")]
    pub caregiver_id: String,
    /// Name of the skill.
    pub name: String,
    /// Details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

impl SpecializationCreate {
    pub fn new(caregiver_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            caregiver_id: caregiver_id.into(),
            name: name.into(),
            description: None,
        }
    }

    /// Details.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `description`, clearing it.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }
}

/// Fields to change on a specialization. Send only the fields to change; any other field is refused
/// with `unknown_field`. `caregiverId` is set when the specialization is created and cannot be
/// changed afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct SpecializationUpdate {
    /// Name of the skill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
}

impl SpecializationUpdate {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
        }
    }

    /// Name of the skill.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Details.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `description`, clearing it.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }
}

/// Confirmation that a specialization was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SpecializationDeleted {
    /// Always `specialization`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted specialization.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// People who applied to work for your agency, tracked through your hiring stages until they are
/// hired as caregivers. Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Applicant {
    /// String representing the object's type. Always `applicant`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the applicant.
    #[serde(default)]
    pub id: String,
    /// Given name.
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Middle name.
    #[serde(
        rename = "middleName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub middle_name: Option<String>,
    /// Family name.
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Hiring stage, as named in your pipeline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// When the applicant last moved stage.
    #[serde(
        rename = "stageChangedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stage_changed_at: Option<String>,
    /// When the application was received.
    #[serde(
        rename = "applicationDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_date: Option<String>,
    /// Where the applicant found you.
    #[serde(
        rename = "applicationSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_source: Option<String>,
    /// Who referred the applicant.
    #[serde(
        rename = "referredBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub referred_by: Option<String>,
    /// City.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State, as a two-letter code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ZIP code.
    #[serde(rename = "zipCode", default, skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// Earliest date the applicant can start.
    #[serde(
        rename = "desiredStartDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_start_date: Option<String>,
    /// Hourly pay the applicant is asking for, in dollars.
    #[serde(
        rename = "desiredPayRate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_pay_rate: Option<f64>,
    /// Hours per week the applicant can work.
    #[serde(
        rename = "availableHoursPerWeek",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub available_hours_per_week: Option<i64>,
    /// Years of caregiving experience.
    #[serde(
        rename = "yearsExperience",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub years_experience: Option<i64>,
    /// Primary language.
    #[serde(
        rename = "primaryLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_language: Option<String>,
    /// Whether the applicant has a driver's license.
    #[serde(
        rename = "hasDriversLicense",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_drivers_license: Option<bool>,
    /// Whether the applicant has reliable transportation.
    #[serde(
        rename = "hasReliableTransportation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_reliable_transportation: Option<bool>,
    /// Whether the applicant holds a current CPR certification.
    #[serde(
        rename = "hasCPRCertification",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_cpr_certification: Option<bool>,
    /// Scheduled or completed interview time.
    #[serde(
        rename = "interviewDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interview_date: Option<String>,
    /// Status of any job offer.
    #[serde(
        rename = "offerStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offer_status: Option<String>,
    /// The office the applicant applied to.
    #[serde(rename = "officeId", default, skip_serializing_if = "Option::is_none")]
    pub office_id: Option<String>,
    /// Recruiter notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The caregiver record created when the applicant was hired.
    #[serde(
        rename = "convertedToCaregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub converted_to_caregiver_id: Option<String>,
    /// When the applicant was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the applicant was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new applicant. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ApplicantCreate {
    /// Given name.
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// Middle name.
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<Option<String>>,
    /// Family name.
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Option<String>>,
    /// Hiring stage, as named in your pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<Option<String>>,
    /// When the application was received.
    #[serde(rename = "applicationDate", skip_serializing_if = "Option::is_none")]
    pub application_date: Option<Option<String>>,
    /// Where the applicant found you.
    #[serde(rename = "applicationSource", skip_serializing_if = "Option::is_none")]
    pub application_source: Option<Option<String>>,
    /// Who referred the applicant.
    #[serde(rename = "referredBy", skip_serializing_if = "Option::is_none")]
    pub referred_by: Option<Option<String>>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// State, as a two-letter code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    /// ZIP code.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<Option<String>>,
    /// Earliest date the applicant can start.
    #[serde(rename = "desiredStartDate", skip_serializing_if = "Option::is_none")]
    pub desired_start_date: Option<Option<String>>,
    /// Hourly pay the applicant is asking for, in dollars.
    #[serde(rename = "desiredPayRate", skip_serializing_if = "Option::is_none")]
    pub desired_pay_rate: Option<Option<f64>>,
    /// Hours per week the applicant can work.
    #[serde(
        rename = "availableHoursPerWeek",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_hours_per_week: Option<Option<i64>>,
    /// Years of caregiving experience.
    #[serde(rename = "yearsExperience", skip_serializing_if = "Option::is_none")]
    pub years_experience: Option<Option<i64>>,
    /// Primary language.
    #[serde(rename = "primaryLanguage", skip_serializing_if = "Option::is_none")]
    pub primary_language: Option<Option<String>>,
    /// Whether the applicant has a driver's license.
    #[serde(rename = "hasDriversLicense", skip_serializing_if = "Option::is_none")]
    pub has_drivers_license: Option<Option<bool>>,
    /// Whether the applicant has reliable transportation.
    #[serde(
        rename = "hasReliableTransportation",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_reliable_transportation: Option<Option<bool>>,
    /// Whether the applicant holds a current CPR certification.
    #[serde(
        rename = "hasCPRCertification",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_cpr_certification: Option<Option<bool>>,
    /// Scheduled or completed interview time.
    #[serde(rename = "interviewDate", skip_serializing_if = "Option::is_none")]
    pub interview_date: Option<Option<String>>,
    /// Status of any job offer.
    #[serde(rename = "offerStatus", skip_serializing_if = "Option::is_none")]
    pub offer_status: Option<Option<String>>,
    /// The office the applicant applied to.
    #[serde(rename = "officeId", skip_serializing_if = "Option::is_none")]
    pub office_id: Option<Option<String>>,
    /// Recruiter notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
}

impl ApplicantCreate {
    pub fn new(first_name: impl Into<String>, last_name: impl Into<String>) -> Self {
        Self {
            first_name: first_name.into(),
            middle_name: None,
            last_name: last_name.into(),
            email: None,
            phone: None,
            stage: None,
            application_date: None,
            application_source: None,
            referred_by: None,
            city: None,
            state: None,
            zip_code: None,
            desired_start_date: None,
            desired_pay_rate: None,
            available_hours_per_week: None,
            years_experience: None,
            primary_language: None,
            has_drivers_license: None,
            has_reliable_transportation: None,
            has_cpr_certification: None,
            interview_date: None,
            offer_status: None,
            office_id: None,
            notes: None,
        }
    }

    /// Middle name.
    pub fn middle_name(mut self, value: impl Into<String>) -> Self {
        self.middle_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `middleName`, clearing it.
    pub fn clear_middle_name(mut self) -> Self {
        self.middle_name = Some(None);
        self
    }

    /// Email address.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `email`, clearing it.
    pub fn clear_email(mut self) -> Self {
        self.email = Some(None);
        self
    }

    /// Phone number.
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `phone`, clearing it.
    pub fn clear_phone(mut self) -> Self {
        self.phone = Some(None);
        self
    }

    /// Hiring stage, as named in your pipeline.
    pub fn stage(mut self, value: impl Into<String>) -> Self {
        self.stage = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `stage`, clearing it.
    pub fn clear_stage(mut self) -> Self {
        self.stage = Some(None);
        self
    }

    /// When the application was received.
    pub fn application_date(mut self, value: impl Into<String>) -> Self {
        self.application_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `applicationDate`, clearing it.
    pub fn clear_application_date(mut self) -> Self {
        self.application_date = Some(None);
        self
    }

    /// Where the applicant found you.
    pub fn application_source(mut self, value: impl Into<String>) -> Self {
        self.application_source = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `applicationSource`, clearing it.
    pub fn clear_application_source(mut self) -> Self {
        self.application_source = Some(None);
        self
    }

    /// Who referred the applicant.
    pub fn referred_by(mut self, value: impl Into<String>) -> Self {
        self.referred_by = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `referredBy`, clearing it.
    pub fn clear_referred_by(mut self) -> Self {
        self.referred_by = Some(None);
        self
    }

    /// City.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `city`, clearing it.
    pub fn clear_city(mut self) -> Self {
        self.city = Some(None);
        self
    }

    /// State, as a two-letter code.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `state`, clearing it.
    pub fn clear_state(mut self) -> Self {
        self.state = Some(None);
        self
    }

    /// ZIP code.
    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `zipCode`, clearing it.
    pub fn clear_zip_code(mut self) -> Self {
        self.zip_code = Some(None);
        self
    }

    /// Earliest date the applicant can start.
    pub fn desired_start_date(mut self, value: impl Into<String>) -> Self {
        self.desired_start_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `desiredStartDate`, clearing it.
    pub fn clear_desired_start_date(mut self) -> Self {
        self.desired_start_date = Some(None);
        self
    }

    /// Hourly pay the applicant is asking for, in dollars.
    pub fn desired_pay_rate(mut self, value: f64) -> Self {
        self.desired_pay_rate = Some(Some(value));
        self
    }

    /// Sends `null` for `desiredPayRate`, clearing it.
    pub fn clear_desired_pay_rate(mut self) -> Self {
        self.desired_pay_rate = Some(None);
        self
    }

    /// Hours per week the applicant can work.
    pub fn available_hours_per_week(mut self, value: i64) -> Self {
        self.available_hours_per_week = Some(Some(value));
        self
    }

    /// Sends `null` for `availableHoursPerWeek`, clearing it.
    pub fn clear_available_hours_per_week(mut self) -> Self {
        self.available_hours_per_week = Some(None);
        self
    }

    /// Years of caregiving experience.
    pub fn years_experience(mut self, value: i64) -> Self {
        self.years_experience = Some(Some(value));
        self
    }

    /// Sends `null` for `yearsExperience`, clearing it.
    pub fn clear_years_experience(mut self) -> Self {
        self.years_experience = Some(None);
        self
    }

    /// Primary language.
    pub fn primary_language(mut self, value: impl Into<String>) -> Self {
        self.primary_language = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `primaryLanguage`, clearing it.
    pub fn clear_primary_language(mut self) -> Self {
        self.primary_language = Some(None);
        self
    }

    /// Whether the applicant has a driver's license.
    pub fn has_drivers_license(mut self, value: bool) -> Self {
        self.has_drivers_license = Some(Some(value));
        self
    }

    /// Sends `null` for `hasDriversLicense`, clearing it.
    pub fn clear_has_drivers_license(mut self) -> Self {
        self.has_drivers_license = Some(None);
        self
    }

    /// Whether the applicant has reliable transportation.
    pub fn has_reliable_transportation(mut self, value: bool) -> Self {
        self.has_reliable_transportation = Some(Some(value));
        self
    }

    /// Sends `null` for `hasReliableTransportation`, clearing it.
    pub fn clear_has_reliable_transportation(mut self) -> Self {
        self.has_reliable_transportation = Some(None);
        self
    }

    /// Whether the applicant holds a current CPR certification.
    pub fn has_cpr_certification(mut self, value: bool) -> Self {
        self.has_cpr_certification = Some(Some(value));
        self
    }

    /// Sends `null` for `hasCPRCertification`, clearing it.
    pub fn clear_has_cpr_certification(mut self) -> Self {
        self.has_cpr_certification = Some(None);
        self
    }

    /// Scheduled or completed interview time.
    pub fn interview_date(mut self, value: impl Into<String>) -> Self {
        self.interview_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `interviewDate`, clearing it.
    pub fn clear_interview_date(mut self) -> Self {
        self.interview_date = Some(None);
        self
    }

    /// Status of any job offer.
    pub fn offer_status(mut self, value: impl Into<String>) -> Self {
        self.offer_status = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `offerStatus`, clearing it.
    pub fn clear_offer_status(mut self) -> Self {
        self.offer_status = Some(None);
        self
    }

    /// The office the applicant applied to.
    pub fn office_id(mut self, value: impl Into<String>) -> Self {
        self.office_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `officeId`, clearing it.
    pub fn clear_office_id(mut self) -> Self {
        self.office_id = Some(None);
        self
    }

    /// Recruiter notes.
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

/// Fields to change on an applicant. Send only the fields to change; any other field is refused
/// with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ApplicantUpdate {
    /// Given name.
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Middle name.
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<Option<String>>,
    /// Family name.
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Option<String>>,
    /// Hiring stage, as named in your pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<Option<String>>,
    /// When the application was received.
    #[serde(rename = "applicationDate", skip_serializing_if = "Option::is_none")]
    pub application_date: Option<Option<String>>,
    /// Where the applicant found you.
    #[serde(rename = "applicationSource", skip_serializing_if = "Option::is_none")]
    pub application_source: Option<Option<String>>,
    /// Who referred the applicant.
    #[serde(rename = "referredBy", skip_serializing_if = "Option::is_none")]
    pub referred_by: Option<Option<String>>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// State, as a two-letter code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    /// ZIP code.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<Option<String>>,
    /// Earliest date the applicant can start.
    #[serde(rename = "desiredStartDate", skip_serializing_if = "Option::is_none")]
    pub desired_start_date: Option<Option<String>>,
    /// Hourly pay the applicant is asking for, in dollars.
    #[serde(rename = "desiredPayRate", skip_serializing_if = "Option::is_none")]
    pub desired_pay_rate: Option<Option<f64>>,
    /// Hours per week the applicant can work.
    #[serde(
        rename = "availableHoursPerWeek",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_hours_per_week: Option<Option<i64>>,
    /// Years of caregiving experience.
    #[serde(rename = "yearsExperience", skip_serializing_if = "Option::is_none")]
    pub years_experience: Option<Option<i64>>,
    /// Primary language.
    #[serde(rename = "primaryLanguage", skip_serializing_if = "Option::is_none")]
    pub primary_language: Option<Option<String>>,
    /// Whether the applicant has a driver's license.
    #[serde(rename = "hasDriversLicense", skip_serializing_if = "Option::is_none")]
    pub has_drivers_license: Option<Option<bool>>,
    /// Whether the applicant has reliable transportation.
    #[serde(
        rename = "hasReliableTransportation",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_reliable_transportation: Option<Option<bool>>,
    /// Whether the applicant holds a current CPR certification.
    #[serde(
        rename = "hasCPRCertification",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_cpr_certification: Option<Option<bool>>,
    /// Scheduled or completed interview time.
    #[serde(rename = "interviewDate", skip_serializing_if = "Option::is_none")]
    pub interview_date: Option<Option<String>>,
    /// Status of any job offer.
    #[serde(rename = "offerStatus", skip_serializing_if = "Option::is_none")]
    pub offer_status: Option<Option<String>>,
    /// The office the applicant applied to.
    #[serde(rename = "officeId", skip_serializing_if = "Option::is_none")]
    pub office_id: Option<Option<String>>,
    /// Recruiter notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
}

impl ApplicantUpdate {
    pub fn new() -> Self {
        Self {
            first_name: None,
            middle_name: None,
            last_name: None,
            email: None,
            phone: None,
            stage: None,
            application_date: None,
            application_source: None,
            referred_by: None,
            city: None,
            state: None,
            zip_code: None,
            desired_start_date: None,
            desired_pay_rate: None,
            available_hours_per_week: None,
            years_experience: None,
            primary_language: None,
            has_drivers_license: None,
            has_reliable_transportation: None,
            has_cpr_certification: None,
            interview_date: None,
            offer_status: None,
            office_id: None,
            notes: None,
        }
    }

    /// Given name.
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    /// Middle name.
    pub fn middle_name(mut self, value: impl Into<String>) -> Self {
        self.middle_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `middleName`, clearing it.
    pub fn clear_middle_name(mut self) -> Self {
        self.middle_name = Some(None);
        self
    }

    /// Family name.
    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    /// Email address.
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `email`, clearing it.
    pub fn clear_email(mut self) -> Self {
        self.email = Some(None);
        self
    }

    /// Phone number.
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `phone`, clearing it.
    pub fn clear_phone(mut self) -> Self {
        self.phone = Some(None);
        self
    }

    /// Hiring stage, as named in your pipeline.
    pub fn stage(mut self, value: impl Into<String>) -> Self {
        self.stage = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `stage`, clearing it.
    pub fn clear_stage(mut self) -> Self {
        self.stage = Some(None);
        self
    }

    /// When the application was received.
    pub fn application_date(mut self, value: impl Into<String>) -> Self {
        self.application_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `applicationDate`, clearing it.
    pub fn clear_application_date(mut self) -> Self {
        self.application_date = Some(None);
        self
    }

    /// Where the applicant found you.
    pub fn application_source(mut self, value: impl Into<String>) -> Self {
        self.application_source = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `applicationSource`, clearing it.
    pub fn clear_application_source(mut self) -> Self {
        self.application_source = Some(None);
        self
    }

    /// Who referred the applicant.
    pub fn referred_by(mut self, value: impl Into<String>) -> Self {
        self.referred_by = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `referredBy`, clearing it.
    pub fn clear_referred_by(mut self) -> Self {
        self.referred_by = Some(None);
        self
    }

    /// City.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `city`, clearing it.
    pub fn clear_city(mut self) -> Self {
        self.city = Some(None);
        self
    }

    /// State, as a two-letter code.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `state`, clearing it.
    pub fn clear_state(mut self) -> Self {
        self.state = Some(None);
        self
    }

    /// ZIP code.
    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `zipCode`, clearing it.
    pub fn clear_zip_code(mut self) -> Self {
        self.zip_code = Some(None);
        self
    }

    /// Earliest date the applicant can start.
    pub fn desired_start_date(mut self, value: impl Into<String>) -> Self {
        self.desired_start_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `desiredStartDate`, clearing it.
    pub fn clear_desired_start_date(mut self) -> Self {
        self.desired_start_date = Some(None);
        self
    }

    /// Hourly pay the applicant is asking for, in dollars.
    pub fn desired_pay_rate(mut self, value: f64) -> Self {
        self.desired_pay_rate = Some(Some(value));
        self
    }

    /// Sends `null` for `desiredPayRate`, clearing it.
    pub fn clear_desired_pay_rate(mut self) -> Self {
        self.desired_pay_rate = Some(None);
        self
    }

    /// Hours per week the applicant can work.
    pub fn available_hours_per_week(mut self, value: i64) -> Self {
        self.available_hours_per_week = Some(Some(value));
        self
    }

    /// Sends `null` for `availableHoursPerWeek`, clearing it.
    pub fn clear_available_hours_per_week(mut self) -> Self {
        self.available_hours_per_week = Some(None);
        self
    }

    /// Years of caregiving experience.
    pub fn years_experience(mut self, value: i64) -> Self {
        self.years_experience = Some(Some(value));
        self
    }

    /// Sends `null` for `yearsExperience`, clearing it.
    pub fn clear_years_experience(mut self) -> Self {
        self.years_experience = Some(None);
        self
    }

    /// Primary language.
    pub fn primary_language(mut self, value: impl Into<String>) -> Self {
        self.primary_language = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `primaryLanguage`, clearing it.
    pub fn clear_primary_language(mut self) -> Self {
        self.primary_language = Some(None);
        self
    }

    /// Whether the applicant has a driver's license.
    pub fn has_drivers_license(mut self, value: bool) -> Self {
        self.has_drivers_license = Some(Some(value));
        self
    }

    /// Sends `null` for `hasDriversLicense`, clearing it.
    pub fn clear_has_drivers_license(mut self) -> Self {
        self.has_drivers_license = Some(None);
        self
    }

    /// Whether the applicant has reliable transportation.
    pub fn has_reliable_transportation(mut self, value: bool) -> Self {
        self.has_reliable_transportation = Some(Some(value));
        self
    }

    /// Sends `null` for `hasReliableTransportation`, clearing it.
    pub fn clear_has_reliable_transportation(mut self) -> Self {
        self.has_reliable_transportation = Some(None);
        self
    }

    /// Whether the applicant holds a current CPR certification.
    pub fn has_cpr_certification(mut self, value: bool) -> Self {
        self.has_cpr_certification = Some(Some(value));
        self
    }

    /// Sends `null` for `hasCPRCertification`, clearing it.
    pub fn clear_has_cpr_certification(mut self) -> Self {
        self.has_cpr_certification = Some(None);
        self
    }

    /// Scheduled or completed interview time.
    pub fn interview_date(mut self, value: impl Into<String>) -> Self {
        self.interview_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `interviewDate`, clearing it.
    pub fn clear_interview_date(mut self) -> Self {
        self.interview_date = Some(None);
        self
    }

    /// Status of any job offer.
    pub fn offer_status(mut self, value: impl Into<String>) -> Self {
        self.offer_status = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `offerStatus`, clearing it.
    pub fn clear_offer_status(mut self) -> Self {
        self.offer_status = Some(None);
        self
    }

    /// The office the applicant applied to.
    pub fn office_id(mut self, value: impl Into<String>) -> Self {
        self.office_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `officeId`, clearing it.
    pub fn clear_office_id(mut self) -> Self {
        self.office_id = Some(None);
        self
    }

    /// Recruiter notes.
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

/// Confirmation that an applicant was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApplicantDeleted {
    /// Always `applicant`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted applicant.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Visits are scheduled or completed caregiver shifts at a patient's home. A visit's patient,
/// caregiver, times and status are set when it is created; reschedule or reassign a visit in
/// AveeCare. Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Visit {
    /// String representing the object's type. Always `visit`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the visit.
    #[serde(default)]
    pub id: String,
    /// The patient being visited.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// The caregiver assigned to the visit; `null` for an open shift.
    #[serde(
        rename = "caregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_id: Option<String>,
    /// Scheduled start.
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Scheduled end.
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Where the visit is in its day. New visits are `Scheduled`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<VisitStatus>,
    /// Short title shown on the schedule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Whether the visit spans the whole day, such as a live-in shift.
    #[serde(rename = "isAllDay", default, skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
    /// Tasks the caregiver should complete during the visit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<String>>,
    /// Why the visit is scheduled, if it is not routine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Notes for the caregiver about this visit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Billing service code for the visit.
    #[serde(
        rename = "serviceCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_code: Option<String>,
    /// Where the visit takes place, when it is not the patient's home address.
    #[serde(
        rename = "locationAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub location_address: Option<String>,
    /// When the caregiver clocked in.
    #[serde(
        rename = "actualStartTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_start_time: Option<String>,
    /// When the caregiver clocked out.
    #[serde(
        rename = "actualEndTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_end_time: Option<String>,
    /// Tasks the caregiver marked done.
    #[serde(
        rename = "completedTasks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_tasks: Option<Vec<String>>,
    /// Whether the visit passed electronic visit verification.
    #[serde(
        rename = "evvVerified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub evv_verified: Option<bool>,
    /// Electronic visit verification status.
    #[serde(rename = "evvStatus", default, skip_serializing_if = "Option::is_none")]
    pub evv_status: Option<VisitEvvStatus>,
    /// Whether the visit has been billed.
    #[serde(
        rename = "billingStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_status: Option<VisitBillingStatus>,
    /// The invoice the visit was billed on.
    #[serde(rename = "invoiceId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Hours billed for the visit.
    #[serde(
        rename = "billableHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub billable_hours: Option<f64>,
    /// Whether the visit belongs to a repeating series.
    #[serde(
        rename = "isRecurring",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_recurring: Option<bool>,
    /// The repeating series the visit belongs to.
    #[serde(
        rename = "recurrenceSeriesId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recurrence_series_id: Option<String>,
    /// The caregiver's response to the assignment.
    #[serde(
        rename = "caregiverRSVP",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_rsvp: Option<VisitCaregiverRsvp>,
    /// When the visit was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the visit was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new visit. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct VisitCreate {
    /// The patient being visited.
    #[serde(rename = "patientId")]
    pub patient_id: String,
    /// The caregiver assigned to the visit; `null` for an open shift.
    #[serde(rename = "caregiverId", skip_serializing_if = "Option::is_none")]
    pub caregiver_id: Option<Option<String>>,
    /// Scheduled start.
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// Scheduled end.
    #[serde(rename = "endTime")]
    pub end_time: String,
    /// Where the visit is in its day. New visits are `Scheduled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<VisitStatus>>,
    /// Short title shown on the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    /// Whether the visit spans the whole day, such as a live-in shift.
    #[serde(rename = "isAllDay", skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<Option<bool>>,
    /// Tasks the caregiver should complete during the visit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Option<Vec<String>>>,
    /// Why the visit is scheduled, if it is not routine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<String>>,
    /// Notes for the caregiver about this visit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
    /// Billing service code for the visit.
    #[serde(rename = "serviceCode", skip_serializing_if = "Option::is_none")]
    pub service_code: Option<Option<String>>,
    /// Where the visit takes place, when it is not the patient's home address.
    #[serde(rename = "locationAddress", skip_serializing_if = "Option::is_none")]
    pub location_address: Option<Option<String>>,
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
            title: None,
            is_all_day: None,
            tasks: None,
            reason: None,
            notes: None,
            service_code: None,
            location_address: None,
        }
    }

    /// The caregiver assigned to the visit; `null` for an open shift.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `caregiverId`, clearing it.
    pub fn clear_caregiver_id(mut self) -> Self {
        self.caregiver_id = Some(None);
        self
    }

    /// Where the visit is in its day. New visits are `Scheduled`.
    pub fn status(mut self, value: VisitStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Short title shown on the schedule.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `title`, clearing it.
    pub fn clear_title(mut self) -> Self {
        self.title = Some(None);
        self
    }

    /// Whether the visit spans the whole day, such as a live-in shift.
    pub fn is_all_day(mut self, value: bool) -> Self {
        self.is_all_day = Some(Some(value));
        self
    }

    /// Sends `null` for `isAllDay`, clearing it.
    pub fn clear_is_all_day(mut self) -> Self {
        self.is_all_day = Some(None);
        self
    }

    /// Tasks the caregiver should complete during the visit.
    pub fn tasks(mut self, value: Vec<String>) -> Self {
        self.tasks = Some(Some(value));
        self
    }

    /// Sends `null` for `tasks`, clearing it.
    pub fn clear_tasks(mut self) -> Self {
        self.tasks = Some(None);
        self
    }

    /// Why the visit is scheduled, if it is not routine.
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `reason`, clearing it.
    pub fn clear_reason(mut self) -> Self {
        self.reason = Some(None);
        self
    }

    /// Notes for the caregiver about this visit.
    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `notes`, clearing it.
    pub fn clear_notes(mut self) -> Self {
        self.notes = Some(None);
        self
    }

    /// Billing service code for the visit.
    pub fn service_code(mut self, value: impl Into<String>) -> Self {
        self.service_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `serviceCode`, clearing it.
    pub fn clear_service_code(mut self) -> Self {
        self.service_code = Some(None);
        self
    }

    /// Where the visit takes place, when it is not the patient's home address.
    pub fn location_address(mut self, value: impl Into<String>) -> Self {
        self.location_address = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `locationAddress`, clearing it.
    pub fn clear_location_address(mut self) -> Self {
        self.location_address = Some(None);
        self
    }
}

/// Fields to change on a visit. Send only the fields to change; any other field is refused with
/// `unknown_field`. `patientId`, `caregiverId`, `startTime`, `endTime`, `status` are set when the
/// visit is created and cannot be changed afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct VisitUpdate {
    /// Short title shown on the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    /// Whether the visit spans the whole day, such as a live-in shift.
    #[serde(rename = "isAllDay", skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<Option<bool>>,
    /// Tasks the caregiver should complete during the visit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Option<Vec<String>>>,
    /// Why the visit is scheduled, if it is not routine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<String>>,
    /// Notes for the caregiver about this visit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
    /// Billing service code for the visit.
    #[serde(rename = "serviceCode", skip_serializing_if = "Option::is_none")]
    pub service_code: Option<Option<String>>,
    /// Where the visit takes place, when it is not the patient's home address.
    #[serde(rename = "locationAddress", skip_serializing_if = "Option::is_none")]
    pub location_address: Option<Option<String>>,
}

impl VisitUpdate {
    pub fn new() -> Self {
        Self {
            title: None,
            is_all_day: None,
            tasks: None,
            reason: None,
            notes: None,
            service_code: None,
            location_address: None,
        }
    }

    /// Short title shown on the schedule.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `title`, clearing it.
    pub fn clear_title(mut self) -> Self {
        self.title = Some(None);
        self
    }

    /// Whether the visit spans the whole day, such as a live-in shift.
    pub fn is_all_day(mut self, value: bool) -> Self {
        self.is_all_day = Some(Some(value));
        self
    }

    /// Sends `null` for `isAllDay`, clearing it.
    pub fn clear_is_all_day(mut self) -> Self {
        self.is_all_day = Some(None);
        self
    }

    /// Tasks the caregiver should complete during the visit.
    pub fn tasks(mut self, value: Vec<String>) -> Self {
        self.tasks = Some(Some(value));
        self
    }

    /// Sends `null` for `tasks`, clearing it.
    pub fn clear_tasks(mut self) -> Self {
        self.tasks = Some(None);
        self
    }

    /// Why the visit is scheduled, if it is not routine.
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `reason`, clearing it.
    pub fn clear_reason(mut self) -> Self {
        self.reason = Some(None);
        self
    }

    /// Notes for the caregiver about this visit.
    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `notes`, clearing it.
    pub fn clear_notes(mut self) -> Self {
        self.notes = Some(None);
        self
    }

    /// Billing service code for the visit.
    pub fn service_code(mut self, value: impl Into<String>) -> Self {
        self.service_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `serviceCode`, clearing it.
    pub fn clear_service_code(mut self) -> Self {
        self.service_code = Some(None);
        self
    }

    /// Where the visit takes place, when it is not the patient's home address.
    pub fn location_address(mut self, value: impl Into<String>) -> Self {
        self.location_address = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `locationAddress`, clearing it.
    pub fn clear_location_address(mut self) -> Self {
        self.location_address = Some(None);
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
    /// ID of the deleted visit.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Notes written about a visit by the caregiver or office staff. Every field is always present; a
/// field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VisitNote {
    /// String representing the object's type. Always `visit_note`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the visit note.
    #[serde(default)]
    pub id: String,
    /// The visit the note is about.
    #[serde(rename = "visitId", default, skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<String>,
    /// The note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Whether office staff (`User`) or a caregiver wrote the note. Set from the API key's user.
    #[serde(
        rename = "authorType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub author_type: Option<VisitNoteAuthorType>,
    /// ID of the staff user or caregiver who wrote the note. Set from the API key's user.
    #[serde(rename = "authorId", default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    /// When the visit note was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the visit note was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new visit note. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct VisitNoteCreate {
    /// The visit the note is about.
    #[serde(rename = "visitId")]
    pub visit_id: String,
    /// The note.
    pub content: String,
}

impl VisitNoteCreate {
    pub fn new(visit_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            visit_id: visit_id.into(),
            content: content.into(),
        }
    }
}

/// Fields to change on a visit note. Send only the fields to change; any other field is refused
/// with `unknown_field`. `visitId` is set when the visit note is created and cannot be changed
/// afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct VisitNoteUpdate {
    /// The note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl VisitNoteUpdate {
    pub fn new() -> Self {
        Self { content: None }
    }

    /// The note.
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }
}

/// Confirmation that a visit note was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VisitNoteDeleted {
    /// Always `visit_note`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted visit note.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Requests from caregivers to pick up visits with a patient, which office staff approve or deny.
/// Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VisitRequest {
    /// String representing the object's type. Always `visit_request`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the visit request.
    #[serde(default)]
    pub id: String,
    /// The caregiver asking for the visit. With a caregiver's API key this is always that caregiver
    /// (leave it out); office staff must send it.
    #[serde(
        rename = "caregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_id: Option<String>,
    /// Caregiver's display name, taken from the caregiver's record.
    #[serde(
        rename = "caregiverName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_name: Option<String>,
    /// The patient the caregiver would visit.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// Patient's display name.
    #[serde(
        rename = "patientName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub patient_name: Option<String>,
    /// First date the caregiver could start.
    #[serde(
        rename = "preferredStartDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_start_date: Option<String>,
    /// Last date of the request.
    #[serde(
        rename = "preferredEndDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_end_date: Option<String>,
    /// Time of day the caregiver prefers.
    #[serde(
        rename = "preferredTimeOfDay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_time_of_day: Option<String>,
    /// Why the caregiver is asking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Where the request stands. A caregiver's key creates `Pending` requests and can only change
    /// its own pending request to `Cancelled`; approving and denying is for office staff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<VisitRequestStatus>,
    /// The office's reply. Set by office staff.
    #[serde(
        rename = "adminResponse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admin_response: Option<String>,
    /// Who answered the request.
    #[serde(
        rename = "respondedByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub responded_by_name: Option<String>,
    /// When the request was answered.
    #[serde(
        rename = "respondedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub responded_at: Option<String>,
    /// When the visit request was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the visit request was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new visit request. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct VisitRequestCreate {
    /// The caregiver asking for the visit. With a caregiver's API key this is always that caregiver
    /// (leave it out); office staff must send it.
    #[serde(rename = "caregiverId", skip_serializing_if = "Option::is_none")]
    pub caregiver_id: Option<Option<String>>,
    /// The patient the caregiver would visit.
    #[serde(rename = "patientId", skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<Option<String>>,
    /// Patient's display name.
    #[serde(rename = "patientName", skip_serializing_if = "Option::is_none")]
    pub patient_name: Option<Option<String>>,
    /// First date the caregiver could start.
    #[serde(rename = "preferredStartDate", skip_serializing_if = "Option::is_none")]
    pub preferred_start_date: Option<Option<String>>,
    /// Last date of the request.
    #[serde(rename = "preferredEndDate", skip_serializing_if = "Option::is_none")]
    pub preferred_end_date: Option<Option<String>>,
    /// Time of day the caregiver prefers.
    #[serde(rename = "preferredTimeOfDay", skip_serializing_if = "Option::is_none")]
    pub preferred_time_of_day: Option<Option<String>>,
    /// Why the caregiver is asking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<String>>,
    /// Where the request stands. A caregiver's key creates `Pending` requests and can only change
    /// its own pending request to `Cancelled`; approving and denying is for office staff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<VisitRequestStatus>>,
    /// The office's reply. Set by office staff.
    #[serde(rename = "adminResponse", skip_serializing_if = "Option::is_none")]
    pub admin_response: Option<Option<String>>,
}

impl VisitRequestCreate {
    pub fn new() -> Self {
        Self {
            caregiver_id: None,
            patient_id: None,
            patient_name: None,
            preferred_start_date: None,
            preferred_end_date: None,
            preferred_time_of_day: None,
            reason: None,
            status: None,
            admin_response: None,
        }
    }

    /// The caregiver asking for the visit. With a caregiver's API key this is always that caregiver
    /// (leave it out); office staff must send it.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `caregiverId`, clearing it.
    pub fn clear_caregiver_id(mut self) -> Self {
        self.caregiver_id = Some(None);
        self
    }

    /// The patient the caregiver would visit.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `patientId`, clearing it.
    pub fn clear_patient_id(mut self) -> Self {
        self.patient_id = Some(None);
        self
    }

    /// Patient's display name.
    pub fn patient_name(mut self, value: impl Into<String>) -> Self {
        self.patient_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `patientName`, clearing it.
    pub fn clear_patient_name(mut self) -> Self {
        self.patient_name = Some(None);
        self
    }

    /// First date the caregiver could start.
    pub fn preferred_start_date(mut self, value: impl Into<String>) -> Self {
        self.preferred_start_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `preferredStartDate`, clearing it.
    pub fn clear_preferred_start_date(mut self) -> Self {
        self.preferred_start_date = Some(None);
        self
    }

    /// Last date of the request.
    pub fn preferred_end_date(mut self, value: impl Into<String>) -> Self {
        self.preferred_end_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `preferredEndDate`, clearing it.
    pub fn clear_preferred_end_date(mut self) -> Self {
        self.preferred_end_date = Some(None);
        self
    }

    /// Time of day the caregiver prefers.
    pub fn preferred_time_of_day(mut self, value: impl Into<String>) -> Self {
        self.preferred_time_of_day = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `preferredTimeOfDay`, clearing it.
    pub fn clear_preferred_time_of_day(mut self) -> Self {
        self.preferred_time_of_day = Some(None);
        self
    }

    /// Why the caregiver is asking.
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `reason`, clearing it.
    pub fn clear_reason(mut self) -> Self {
        self.reason = Some(None);
        self
    }

    /// Where the request stands. A caregiver's key creates `Pending` requests and can only change
    /// its own pending request to `Cancelled`; approving and denying is for office staff.
    pub fn status(mut self, value: VisitRequestStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// The office's reply. Set by office staff.
    pub fn admin_response(mut self, value: impl Into<String>) -> Self {
        self.admin_response = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `adminResponse`, clearing it.
    pub fn clear_admin_response(mut self) -> Self {
        self.admin_response = Some(None);
        self
    }
}

/// Fields to change on a visit request. Send only the fields to change; any other field is refused
/// with `unknown_field`. `caregiverId` is set when the visit request is created and cannot be
/// changed afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct VisitRequestUpdate {
    /// The patient the caregiver would visit.
    #[serde(rename = "patientId", skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<Option<String>>,
    /// Patient's display name.
    #[serde(rename = "patientName", skip_serializing_if = "Option::is_none")]
    pub patient_name: Option<Option<String>>,
    /// First date the caregiver could start.
    #[serde(rename = "preferredStartDate", skip_serializing_if = "Option::is_none")]
    pub preferred_start_date: Option<Option<String>>,
    /// Last date of the request.
    #[serde(rename = "preferredEndDate", skip_serializing_if = "Option::is_none")]
    pub preferred_end_date: Option<Option<String>>,
    /// Time of day the caregiver prefers.
    #[serde(rename = "preferredTimeOfDay", skip_serializing_if = "Option::is_none")]
    pub preferred_time_of_day: Option<Option<String>>,
    /// Why the caregiver is asking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<String>>,
    /// Where the request stands. A caregiver's key creates `Pending` requests and can only change
    /// its own pending request to `Cancelled`; approving and denying is for office staff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<VisitRequestStatus>>,
    /// The office's reply. Set by office staff.
    #[serde(rename = "adminResponse", skip_serializing_if = "Option::is_none")]
    pub admin_response: Option<Option<String>>,
}

impl VisitRequestUpdate {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            patient_name: None,
            preferred_start_date: None,
            preferred_end_date: None,
            preferred_time_of_day: None,
            reason: None,
            status: None,
            admin_response: None,
        }
    }

    /// The patient the caregiver would visit.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `patientId`, clearing it.
    pub fn clear_patient_id(mut self) -> Self {
        self.patient_id = Some(None);
        self
    }

    /// Patient's display name.
    pub fn patient_name(mut self, value: impl Into<String>) -> Self {
        self.patient_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `patientName`, clearing it.
    pub fn clear_patient_name(mut self) -> Self {
        self.patient_name = Some(None);
        self
    }

    /// First date the caregiver could start.
    pub fn preferred_start_date(mut self, value: impl Into<String>) -> Self {
        self.preferred_start_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `preferredStartDate`, clearing it.
    pub fn clear_preferred_start_date(mut self) -> Self {
        self.preferred_start_date = Some(None);
        self
    }

    /// Last date of the request.
    pub fn preferred_end_date(mut self, value: impl Into<String>) -> Self {
        self.preferred_end_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `preferredEndDate`, clearing it.
    pub fn clear_preferred_end_date(mut self) -> Self {
        self.preferred_end_date = Some(None);
        self
    }

    /// Time of day the caregiver prefers.
    pub fn preferred_time_of_day(mut self, value: impl Into<String>) -> Self {
        self.preferred_time_of_day = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `preferredTimeOfDay`, clearing it.
    pub fn clear_preferred_time_of_day(mut self) -> Self {
        self.preferred_time_of_day = Some(None);
        self
    }

    /// Why the caregiver is asking.
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `reason`, clearing it.
    pub fn clear_reason(mut self) -> Self {
        self.reason = Some(None);
        self
    }

    /// Where the request stands. A caregiver's key creates `Pending` requests and can only change
    /// its own pending request to `Cancelled`; approving and denying is for office staff.
    pub fn status(mut self, value: VisitRequestStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// The office's reply. Set by office staff.
    pub fn admin_response(mut self, value: impl Into<String>) -> Self {
        self.admin_response = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `adminResponse`, clearing it.
    pub fn clear_admin_response(mut self) -> Self {
        self.admin_response = Some(None);
        self
    }
}

/// Confirmation that a visit request was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VisitRequestDeleted {
    /// Always `visit_request`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted visit request.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Open shifts are unassigned visits offered to caregivers so one of them can claim it. Every field
/// is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenShift {
    /// String representing the object's type. Always `open_shift`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the open shift.
    #[serde(default)]
    pub id: String,
    /// The unassigned visit being offered.
    #[serde(rename = "visitId", default, skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<String>,
    /// When the shift was offered.
    #[serde(
        rename = "broadcastDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub broadcast_date: Option<String>,
    /// Whether the offer is still open.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OpenShiftStatus>,
    /// The caregiver who claimed the shift.
    #[serde(
        rename = "acceptedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub accepted_by: Option<String>,
    /// When the open shift was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the open shift was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new open shift. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct OpenShiftCreate {
    /// The unassigned visit being offered.
    #[serde(rename = "visitId")]
    pub visit_id: String,
    /// When the shift was offered.
    #[serde(rename = "broadcastDate", skip_serializing_if = "Option::is_none")]
    pub broadcast_date: Option<Option<String>>,
    /// Whether the offer is still open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<OpenShiftStatus>>,
}

impl OpenShiftCreate {
    pub fn new(visit_id: impl Into<String>) -> Self {
        Self {
            visit_id: visit_id.into(),
            broadcast_date: None,
            status: None,
        }
    }

    /// When the shift was offered.
    pub fn broadcast_date(mut self, value: impl Into<String>) -> Self {
        self.broadcast_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `broadcastDate`, clearing it.
    pub fn clear_broadcast_date(mut self) -> Self {
        self.broadcast_date = Some(None);
        self
    }

    /// Whether the offer is still open.
    pub fn status(mut self, value: OpenShiftStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }
}

/// Fields to change on an open shift. Send only the fields to change; any other field is refused
/// with `unknown_field`. `visitId` is set when the open shift is created and cannot be changed
/// afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct OpenShiftUpdate {
    /// When the shift was offered.
    #[serde(rename = "broadcastDate", skip_serializing_if = "Option::is_none")]
    pub broadcast_date: Option<Option<String>>,
    /// Whether the offer is still open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<OpenShiftStatus>>,
}

impl OpenShiftUpdate {
    pub fn new() -> Self {
        Self {
            broadcast_date: None,
            status: None,
        }
    }

    /// When the shift was offered.
    pub fn broadcast_date(mut self, value: impl Into<String>) -> Self {
        self.broadcast_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `broadcastDate`, clearing it.
    pub fn clear_broadcast_date(mut self) -> Self {
        self.broadcast_date = Some(None);
        self
    }

    /// Whether the offer is still open.
    pub fn status(mut self, value: OpenShiftStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }
}

/// Confirmation that an open shift was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenShiftDeleted {
    /// Always `open_shift`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted open shift.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Requests from one caregiver to hand a visit to another, which the other caregiver and then the
/// office approve. Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ShiftSwap {
    /// String representing the object's type. Always `shift_swap`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the shift swap.
    #[serde(default)]
    pub id: String,
    /// The visit to hand over.
    #[serde(rename = "visitId", default, skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<String>,
    /// The caregiver giving up the visit. With a caregiver's API key this is always that caregiver
    /// (leave it out), and the visit must be assigned to them; office staff must send it.
    #[serde(
        rename = "requestingCaregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requesting_caregiver_id: Option<String>,
    /// Display name of the requesting caregiver, taken from the caregiver's record.
    #[serde(
        rename = "requestingCaregiverName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requesting_caregiver_name: Option<String>,
    /// The caregiver asked to take the visit.
    #[serde(
        rename = "targetCaregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_caregiver_id: Option<String>,
    /// Display name of the target caregiver.
    #[serde(
        rename = "targetCaregiverName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_caregiver_name: Option<String>,
    /// Why the swap is requested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Where the swap stands. With a caregiver's key: new swaps are `Pending`; the asked caregiver
    /// changes a `Pending` swap to `Accepted` or `Declined`; the requesting caregiver can change it
    /// to `Cancelled`. `AdminApproved` and `AdminDenied` are for office staff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ShiftSwapStatus>,
    /// When the swap was requested.
    #[serde(
        rename = "requestDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_date: Option<String>,
    /// When the target caregiver answered.
    #[serde(
        rename = "responseDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_date: Option<String>,
    /// When the office answered.
    #[serde(
        rename = "adminResponseDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admin_response_date: Option<String>,
    /// When the shift swap was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the shift swap was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new shift swap. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ShiftSwapCreate {
    /// The visit to hand over.
    #[serde(rename = "visitId")]
    pub visit_id: String,
    /// The caregiver giving up the visit. With a caregiver's API key this is always that caregiver
    /// (leave it out), and the visit must be assigned to them; office staff must send it.
    #[serde(
        rename = "requestingCaregiverId",
        skip_serializing_if = "Option::is_none"
    )]
    pub requesting_caregiver_id: Option<Option<String>>,
    /// The caregiver asked to take the visit.
    #[serde(rename = "targetCaregiverId", skip_serializing_if = "Option::is_none")]
    pub target_caregiver_id: Option<Option<String>>,
    /// Display name of the target caregiver.
    #[serde(
        rename = "targetCaregiverName",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_caregiver_name: Option<Option<String>>,
    /// Why the swap is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<String>>,
    /// Where the swap stands. With a caregiver's key: new swaps are `Pending`; the asked caregiver
    /// changes a `Pending` swap to `Accepted` or `Declined`; the requesting caregiver can change it
    /// to `Cancelled`. `AdminApproved` and `AdminDenied` are for office staff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<ShiftSwapStatus>>,
    /// When the swap was requested.
    #[serde(rename = "requestDate", skip_serializing_if = "Option::is_none")]
    pub request_date: Option<Option<String>>,
}

impl ShiftSwapCreate {
    pub fn new(visit_id: impl Into<String>) -> Self {
        Self {
            visit_id: visit_id.into(),
            requesting_caregiver_id: None,
            target_caregiver_id: None,
            target_caregiver_name: None,
            reason: None,
            status: None,
            request_date: None,
        }
    }

    /// The caregiver giving up the visit. With a caregiver's API key this is always that caregiver
    /// (leave it out), and the visit must be assigned to them; office staff must send it.
    pub fn requesting_caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.requesting_caregiver_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `requestingCaregiverId`, clearing it.
    pub fn clear_requesting_caregiver_id(mut self) -> Self {
        self.requesting_caregiver_id = Some(None);
        self
    }

    /// The caregiver asked to take the visit.
    pub fn target_caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.target_caregiver_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `targetCaregiverId`, clearing it.
    pub fn clear_target_caregiver_id(mut self) -> Self {
        self.target_caregiver_id = Some(None);
        self
    }

    /// Display name of the target caregiver.
    pub fn target_caregiver_name(mut self, value: impl Into<String>) -> Self {
        self.target_caregiver_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `targetCaregiverName`, clearing it.
    pub fn clear_target_caregiver_name(mut self) -> Self {
        self.target_caregiver_name = Some(None);
        self
    }

    /// Why the swap is requested.
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `reason`, clearing it.
    pub fn clear_reason(mut self) -> Self {
        self.reason = Some(None);
        self
    }

    /// Where the swap stands. With a caregiver's key: new swaps are `Pending`; the asked caregiver
    /// changes a `Pending` swap to `Accepted` or `Declined`; the requesting caregiver can change it
    /// to `Cancelled`. `AdminApproved` and `AdminDenied` are for office staff.
    pub fn status(mut self, value: ShiftSwapStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// When the swap was requested.
    pub fn request_date(mut self, value: impl Into<String>) -> Self {
        self.request_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `requestDate`, clearing it.
    pub fn clear_request_date(mut self) -> Self {
        self.request_date = Some(None);
        self
    }
}

/// Fields to change on a shift swap. Send only the fields to change; any other field is refused
/// with `unknown_field`. `visitId`, `requestingCaregiverId` are set when the shift swap is created
/// and cannot be changed afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ShiftSwapUpdate {
    /// The caregiver asked to take the visit.
    #[serde(rename = "targetCaregiverId", skip_serializing_if = "Option::is_none")]
    pub target_caregiver_id: Option<Option<String>>,
    /// Display name of the target caregiver.
    #[serde(
        rename = "targetCaregiverName",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_caregiver_name: Option<Option<String>>,
    /// Why the swap is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Option<String>>,
    /// Where the swap stands. With a caregiver's key: new swaps are `Pending`; the asked caregiver
    /// changes a `Pending` swap to `Accepted` or `Declined`; the requesting caregiver can change it
    /// to `Cancelled`. `AdminApproved` and `AdminDenied` are for office staff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<ShiftSwapStatus>>,
    /// When the swap was requested.
    #[serde(rename = "requestDate", skip_serializing_if = "Option::is_none")]
    pub request_date: Option<Option<String>>,
}

impl ShiftSwapUpdate {
    pub fn new() -> Self {
        Self {
            target_caregiver_id: None,
            target_caregiver_name: None,
            reason: None,
            status: None,
            request_date: None,
        }
    }

    /// The caregiver asked to take the visit.
    pub fn target_caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.target_caregiver_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `targetCaregiverId`, clearing it.
    pub fn clear_target_caregiver_id(mut self) -> Self {
        self.target_caregiver_id = Some(None);
        self
    }

    /// Display name of the target caregiver.
    pub fn target_caregiver_name(mut self, value: impl Into<String>) -> Self {
        self.target_caregiver_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `targetCaregiverName`, clearing it.
    pub fn clear_target_caregiver_name(mut self) -> Self {
        self.target_caregiver_name = Some(None);
        self
    }

    /// Why the swap is requested.
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `reason`, clearing it.
    pub fn clear_reason(mut self) -> Self {
        self.reason = Some(None);
        self
    }

    /// Where the swap stands. With a caregiver's key: new swaps are `Pending`; the asked caregiver
    /// changes a `Pending` swap to `Accepted` or `Declined`; the requesting caregiver can change it
    /// to `Cancelled`. `AdminApproved` and `AdminDenied` are for office staff.
    pub fn status(mut self, value: ShiftSwapStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// When the swap was requested.
    pub fn request_date(mut self, value: impl Into<String>) -> Self {
        self.request_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `requestDate`, clearing it.
    pub fn clear_request_date(mut self) -> Self {
        self.request_date = Some(None);
        self
    }
}

/// Confirmation that a shift swap was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ShiftSwapDeleted {
    /// Always `shift_swap`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted shift swap.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Care plans describe the services a patient receives, how often, and when the plan is next
/// reviewed. The plan's goals and task lists are managed in AveeCare and are not included here.
/// Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CarePlan {
    /// String representing the object's type. Always `care_plan`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the care plan.
    #[serde(default)]
    pub id: String,
    /// The patient the plan is for.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// Plan title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Where the plan is in its lifecycle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CarePlanStatus>,
    /// Type of service the plan covers.
    #[serde(
        rename = "serviceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_type: Option<String>,
    /// How often visits happen under the plan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// Date the plan takes effect.
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Date the plan ends; `null` if open-ended.
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Date the plan is next due for review.
    #[serde(
        rename = "reviewDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub review_date: Option<String>,
    /// Days between reviews.
    #[serde(
        rename = "reviewIntervalDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub review_interval_days: Option<i64>,
    /// When the plan was last reviewed.
    #[serde(
        rename = "lastReviewedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_reviewed_at: Option<String>,
    /// Who last reviewed the plan.
    #[serde(
        rename = "lastReviewedByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_reviewed_by_name: Option<String>,
    /// Who created the plan.
    #[serde(
        rename = "createdByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by_name: Option<String>,
    /// When the care plan was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the care plan was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new care plan. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CarePlanCreate {
    /// The patient the plan is for.
    #[serde(rename = "patientId")]
    pub patient_id: String,
    /// Plan title.
    pub title: String,
    /// Where the plan is in its lifecycle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<CarePlanStatus>>,
    /// Type of service the plan covers.
    #[serde(rename = "serviceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<Option<String>>,
    /// How often visits happen under the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Option<String>>,
    /// Date the plan takes effect.
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// Date the plan ends; `null` if open-ended.
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    /// Date the plan is next due for review.
    #[serde(rename = "reviewDate", skip_serializing_if = "Option::is_none")]
    pub review_date: Option<Option<String>>,
    /// Days between reviews.
    #[serde(rename = "reviewIntervalDays", skip_serializing_if = "Option::is_none")]
    pub review_interval_days: Option<Option<i64>>,
}

impl CarePlanCreate {
    pub fn new(patient_id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            patient_id: patient_id.into(),
            title: title.into(),
            status: None,
            service_type: None,
            frequency: None,
            start_date: None,
            end_date: None,
            review_date: None,
            review_interval_days: None,
        }
    }

    /// Where the plan is in its lifecycle.
    pub fn status(mut self, value: CarePlanStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Type of service the plan covers.
    pub fn service_type(mut self, value: impl Into<String>) -> Self {
        self.service_type = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `serviceType`, clearing it.
    pub fn clear_service_type(mut self) -> Self {
        self.service_type = Some(None);
        self
    }

    /// How often visits happen under the plan.
    pub fn frequency(mut self, value: impl Into<String>) -> Self {
        self.frequency = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `frequency`, clearing it.
    pub fn clear_frequency(mut self) -> Self {
        self.frequency = Some(None);
        self
    }

    /// Date the plan takes effect.
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `startDate`, clearing it.
    pub fn clear_start_date(mut self) -> Self {
        self.start_date = Some(None);
        self
    }

    /// Date the plan ends; `null` if open-ended.
    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `endDate`, clearing it.
    pub fn clear_end_date(mut self) -> Self {
        self.end_date = Some(None);
        self
    }

    /// Date the plan is next due for review.
    pub fn review_date(mut self, value: impl Into<String>) -> Self {
        self.review_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `reviewDate`, clearing it.
    pub fn clear_review_date(mut self) -> Self {
        self.review_date = Some(None);
        self
    }

    /// Days between reviews.
    pub fn review_interval_days(mut self, value: i64) -> Self {
        self.review_interval_days = Some(Some(value));
        self
    }

    /// Sends `null` for `reviewIntervalDays`, clearing it.
    pub fn clear_review_interval_days(mut self) -> Self {
        self.review_interval_days = Some(None);
        self
    }
}

/// Fields to change on a care plan. Send only the fields to change; any other field is refused with
/// `unknown_field`. `patientId` is set when the care plan is created and cannot be changed
/// afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct CarePlanUpdate {
    /// Plan title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Where the plan is in its lifecycle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<CarePlanStatus>>,
    /// Type of service the plan covers.
    #[serde(rename = "serviceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<Option<String>>,
    /// How often visits happen under the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Option<String>>,
    /// Date the plan takes effect.
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// Date the plan ends; `null` if open-ended.
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    /// Date the plan is next due for review.
    #[serde(rename = "reviewDate", skip_serializing_if = "Option::is_none")]
    pub review_date: Option<Option<String>>,
    /// Days between reviews.
    #[serde(rename = "reviewIntervalDays", skip_serializing_if = "Option::is_none")]
    pub review_interval_days: Option<Option<i64>>,
}

impl CarePlanUpdate {
    pub fn new() -> Self {
        Self {
            title: None,
            status: None,
            service_type: None,
            frequency: None,
            start_date: None,
            end_date: None,
            review_date: None,
            review_interval_days: None,
        }
    }

    /// Plan title.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Where the plan is in its lifecycle.
    pub fn status(mut self, value: CarePlanStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Type of service the plan covers.
    pub fn service_type(mut self, value: impl Into<String>) -> Self {
        self.service_type = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `serviceType`, clearing it.
    pub fn clear_service_type(mut self) -> Self {
        self.service_type = Some(None);
        self
    }

    /// How often visits happen under the plan.
    pub fn frequency(mut self, value: impl Into<String>) -> Self {
        self.frequency = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `frequency`, clearing it.
    pub fn clear_frequency(mut self) -> Self {
        self.frequency = Some(None);
        self
    }

    /// Date the plan takes effect.
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `startDate`, clearing it.
    pub fn clear_start_date(mut self) -> Self {
        self.start_date = Some(None);
        self
    }

    /// Date the plan ends; `null` if open-ended.
    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `endDate`, clearing it.
    pub fn clear_end_date(mut self) -> Self {
        self.end_date = Some(None);
        self
    }

    /// Date the plan is next due for review.
    pub fn review_date(mut self, value: impl Into<String>) -> Self {
        self.review_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `reviewDate`, clearing it.
    pub fn clear_review_date(mut self) -> Self {
        self.review_date = Some(None);
        self
    }

    /// Days between reviews.
    pub fn review_interval_days(mut self, value: i64) -> Self {
        self.review_interval_days = Some(Some(value));
        self
    }

    /// Sends `null` for `reviewIntervalDays`, clearing it.
    pub fn clear_review_interval_days(mut self) -> Self {
        self.review_interval_days = Some(None);
        self
    }
}

/// Confirmation that a care plan was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CarePlanDeleted {
    /// Always `care_plan`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted care plan.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Progress notes are the clinical summaries caregivers write after a visit. Every field is always
/// present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProgressNote {
    /// String representing the object's type. Always `progress_note`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the progress note.
    #[serde(default)]
    pub id: String,
    /// The visit the note documents.
    #[serde(rename = "visitId", default, skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<String>,
    /// The patient.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// The caregiver who wrote the note. With a caregiver's API key this is always that caregiver
    /// (leave it out), and the visit must be assigned to them.
    #[serde(
        rename = "caregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_id: Option<String>,
    /// Display name of the caregiver, taken from the caregiver's record.
    #[serde(
        rename = "caregiverName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_name: Option<String>,
    /// The written summary of the visit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Whether the summary was drafted with AI assistance.
    #[serde(
        rename = "aiGenerated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ai_generated: Option<bool>,
    /// When the progress note was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the progress note was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new progress note. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ProgressNoteCreate {
    /// The visit the note documents.
    #[serde(rename = "visitId")]
    pub visit_id: String,
    /// The patient.
    #[serde(rename = "patientId")]
    pub patient_id: String,
    /// The caregiver who wrote the note. With a caregiver's API key this is always that caregiver
    /// (leave it out), and the visit must be assigned to them.
    #[serde(rename = "caregiverId", skip_serializing_if = "Option::is_none")]
    pub caregiver_id: Option<Option<String>>,
    /// The written summary of the visit.
    pub summary: String,
}

impl ProgressNoteCreate {
    pub fn new(
        visit_id: impl Into<String>,
        patient_id: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            visit_id: visit_id.into(),
            patient_id: patient_id.into(),
            caregiver_id: None,
            summary: summary.into(),
        }
    }

    /// The caregiver who wrote the note. With a caregiver's API key this is always that caregiver
    /// (leave it out), and the visit must be assigned to them.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `caregiverId`, clearing it.
    pub fn clear_caregiver_id(mut self) -> Self {
        self.caregiver_id = Some(None);
        self
    }
}

/// Fields to change on a progress note. Send only the fields to change; any other field is refused
/// with `unknown_field`. `visitId`, `patientId`, `caregiverId` are set when the progress note is
/// created and cannot be changed afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ProgressNoteUpdate {
    /// The written summary of the visit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

impl ProgressNoteUpdate {
    pub fn new() -> Self {
        Self { summary: None }
    }

    /// The written summary of the visit.
    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }
}

/// Confirmation that a progress note was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProgressNoteDeleted {
    /// Always `progress_note`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted progress note.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Payer authorizations: how many units of a service a payer has approved for a patient, and over
/// what dates. Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Authorization {
    /// String representing the object's type. Always `authorization`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the authorization.
    #[serde(default)]
    pub id: String,
    /// The patient the authorization covers.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// The payer that issued it.
    #[serde(rename = "payerName", default, skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<String>,
    /// The payer's authorization number.
    #[serde(
        rename = "authorizationNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_number: Option<String>,
    /// Service authorized.
    #[serde(
        rename = "serviceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_type: Option<String>,
    /// Procedure (HCPCS) code billed against it.
    #[serde(
        rename = "procedureCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub procedure_code: Option<String>,
    /// Procedure code modifiers, comma separated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<String>,
    /// First date the authorization covers.
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Last date the authorization covers.
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Units approved.
    #[serde(
        rename = "authorizedUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorized_units: Option<i64>,
    /// What a unit is, in the payer's words.
    #[serde(rename = "unitType", default, skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<String>,
    /// Whether units count time, visits or days.
    #[serde(rename = "unitBasis", default, skip_serializing_if = "Option::is_none")]
    pub unit_basis: Option<AuthorizationUnitBasis>,
    /// For time-based units, minutes in one unit.
    #[serde(
        rename = "minutesPerUnit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub minutes_per_unit: Option<i64>,
    /// Units used so far.
    #[serde(rename = "usedUnits", default, skip_serializing_if = "Option::is_none")]
    pub used_units: Option<i64>,
    /// Units left.
    #[serde(
        rename = "remainingUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remaining_units: Option<i64>,
    /// Frequency the payer approved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// Place-of-service code.
    #[serde(
        rename = "placeOfService",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub place_of_service: Option<String>,
    /// Whether the authorization is in effect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AuthorizationStatus>,
    /// Notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// When the authorization was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the authorization was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new authorization. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AuthorizationCreate {
    /// The patient the authorization covers.
    #[serde(rename = "patientId")]
    pub patient_id: String,
    /// The payer that issued it.
    #[serde(rename = "payerName", skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<Option<String>>,
    /// The payer's authorization number.
    #[serde(
        rename = "authorizationNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_number: Option<Option<String>>,
    /// Service authorized.
    #[serde(rename = "serviceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<Option<String>>,
    /// Procedure (HCPCS) code billed against it.
    #[serde(rename = "procedureCode", skip_serializing_if = "Option::is_none")]
    pub procedure_code: Option<Option<String>>,
    /// Procedure code modifiers, comma separated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Option<String>>,
    /// First date the authorization covers.
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// Last date the authorization covers.
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    /// Units approved.
    #[serde(rename = "authorizedUnits", skip_serializing_if = "Option::is_none")]
    pub authorized_units: Option<Option<i64>>,
    /// What a unit is, in the payer's words.
    #[serde(rename = "unitType", skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<Option<String>>,
    /// Whether units count time, visits or days.
    #[serde(rename = "unitBasis", skip_serializing_if = "Option::is_none")]
    pub unit_basis: Option<Option<AuthorizationUnitBasis>>,
    /// For time-based units, minutes in one unit.
    #[serde(rename = "minutesPerUnit", skip_serializing_if = "Option::is_none")]
    pub minutes_per_unit: Option<Option<i64>>,
    /// Frequency the payer approved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Option<String>>,
    /// Place-of-service code.
    #[serde(rename = "placeOfService", skip_serializing_if = "Option::is_none")]
    pub place_of_service: Option<Option<String>>,
    /// Whether the authorization is in effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<AuthorizationStatus>>,
    /// Notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
}

impl AuthorizationCreate {
    pub fn new(patient_id: impl Into<String>) -> Self {
        Self {
            patient_id: patient_id.into(),
            payer_name: None,
            authorization_number: None,
            service_type: None,
            procedure_code: None,
            modifiers: None,
            start_date: None,
            end_date: None,
            authorized_units: None,
            unit_type: None,
            unit_basis: None,
            minutes_per_unit: None,
            frequency: None,
            place_of_service: None,
            status: None,
            notes: None,
        }
    }

    /// The payer that issued it.
    pub fn payer_name(mut self, value: impl Into<String>) -> Self {
        self.payer_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `payerName`, clearing it.
    pub fn clear_payer_name(mut self) -> Self {
        self.payer_name = Some(None);
        self
    }

    /// The payer's authorization number.
    pub fn authorization_number(mut self, value: impl Into<String>) -> Self {
        self.authorization_number = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `authorizationNumber`, clearing it.
    pub fn clear_authorization_number(mut self) -> Self {
        self.authorization_number = Some(None);
        self
    }

    /// Service authorized.
    pub fn service_type(mut self, value: impl Into<String>) -> Self {
        self.service_type = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `serviceType`, clearing it.
    pub fn clear_service_type(mut self) -> Self {
        self.service_type = Some(None);
        self
    }

    /// Procedure (HCPCS) code billed against it.
    pub fn procedure_code(mut self, value: impl Into<String>) -> Self {
        self.procedure_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `procedureCode`, clearing it.
    pub fn clear_procedure_code(mut self) -> Self {
        self.procedure_code = Some(None);
        self
    }

    /// Procedure code modifiers, comma separated.
    pub fn modifiers(mut self, value: impl Into<String>) -> Self {
        self.modifiers = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `modifiers`, clearing it.
    pub fn clear_modifiers(mut self) -> Self {
        self.modifiers = Some(None);
        self
    }

    /// First date the authorization covers.
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `startDate`, clearing it.
    pub fn clear_start_date(mut self) -> Self {
        self.start_date = Some(None);
        self
    }

    /// Last date the authorization covers.
    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `endDate`, clearing it.
    pub fn clear_end_date(mut self) -> Self {
        self.end_date = Some(None);
        self
    }

    /// Units approved.
    pub fn authorized_units(mut self, value: i64) -> Self {
        self.authorized_units = Some(Some(value));
        self
    }

    /// Sends `null` for `authorizedUnits`, clearing it.
    pub fn clear_authorized_units(mut self) -> Self {
        self.authorized_units = Some(None);
        self
    }

    /// What a unit is, in the payer's words.
    pub fn unit_type(mut self, value: impl Into<String>) -> Self {
        self.unit_type = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `unitType`, clearing it.
    pub fn clear_unit_type(mut self) -> Self {
        self.unit_type = Some(None);
        self
    }

    /// Whether units count time, visits or days.
    pub fn unit_basis(mut self, value: AuthorizationUnitBasis) -> Self {
        self.unit_basis = Some(Some(value));
        self
    }

    /// Sends `null` for `unitBasis`, clearing it.
    pub fn clear_unit_basis(mut self) -> Self {
        self.unit_basis = Some(None);
        self
    }

    /// For time-based units, minutes in one unit.
    pub fn minutes_per_unit(mut self, value: i64) -> Self {
        self.minutes_per_unit = Some(Some(value));
        self
    }

    /// Sends `null` for `minutesPerUnit`, clearing it.
    pub fn clear_minutes_per_unit(mut self) -> Self {
        self.minutes_per_unit = Some(None);
        self
    }

    /// Frequency the payer approved.
    pub fn frequency(mut self, value: impl Into<String>) -> Self {
        self.frequency = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `frequency`, clearing it.
    pub fn clear_frequency(mut self) -> Self {
        self.frequency = Some(None);
        self
    }

    /// Place-of-service code.
    pub fn place_of_service(mut self, value: impl Into<String>) -> Self {
        self.place_of_service = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `placeOfService`, clearing it.
    pub fn clear_place_of_service(mut self) -> Self {
        self.place_of_service = Some(None);
        self
    }

    /// Whether the authorization is in effect.
    pub fn status(mut self, value: AuthorizationStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Notes.
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

/// Fields to change on an authorization. Send only the fields to change; any other field is refused
/// with `unknown_field`. `patientId` is set when the authorization is created and cannot be changed
/// afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AuthorizationUpdate {
    /// The payer that issued it.
    #[serde(rename = "payerName", skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<Option<String>>,
    /// The payer's authorization number.
    #[serde(
        rename = "authorizationNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_number: Option<Option<String>>,
    /// Service authorized.
    #[serde(rename = "serviceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<Option<String>>,
    /// Procedure (HCPCS) code billed against it.
    #[serde(rename = "procedureCode", skip_serializing_if = "Option::is_none")]
    pub procedure_code: Option<Option<String>>,
    /// Procedure code modifiers, comma separated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Option<String>>,
    /// First date the authorization covers.
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// Last date the authorization covers.
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    /// Units approved.
    #[serde(rename = "authorizedUnits", skip_serializing_if = "Option::is_none")]
    pub authorized_units: Option<Option<i64>>,
    /// What a unit is, in the payer's words.
    #[serde(rename = "unitType", skip_serializing_if = "Option::is_none")]
    pub unit_type: Option<Option<String>>,
    /// Whether units count time, visits or days.
    #[serde(rename = "unitBasis", skip_serializing_if = "Option::is_none")]
    pub unit_basis: Option<Option<AuthorizationUnitBasis>>,
    /// For time-based units, minutes in one unit.
    #[serde(rename = "minutesPerUnit", skip_serializing_if = "Option::is_none")]
    pub minutes_per_unit: Option<Option<i64>>,
    /// Frequency the payer approved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Option<String>>,
    /// Place-of-service code.
    #[serde(rename = "placeOfService", skip_serializing_if = "Option::is_none")]
    pub place_of_service: Option<Option<String>>,
    /// Whether the authorization is in effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<AuthorizationStatus>>,
    /// Notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
}

impl AuthorizationUpdate {
    pub fn new() -> Self {
        Self {
            payer_name: None,
            authorization_number: None,
            service_type: None,
            procedure_code: None,
            modifiers: None,
            start_date: None,
            end_date: None,
            authorized_units: None,
            unit_type: None,
            unit_basis: None,
            minutes_per_unit: None,
            frequency: None,
            place_of_service: None,
            status: None,
            notes: None,
        }
    }

    /// The payer that issued it.
    pub fn payer_name(mut self, value: impl Into<String>) -> Self {
        self.payer_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `payerName`, clearing it.
    pub fn clear_payer_name(mut self) -> Self {
        self.payer_name = Some(None);
        self
    }

    /// The payer's authorization number.
    pub fn authorization_number(mut self, value: impl Into<String>) -> Self {
        self.authorization_number = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `authorizationNumber`, clearing it.
    pub fn clear_authorization_number(mut self) -> Self {
        self.authorization_number = Some(None);
        self
    }

    /// Service authorized.
    pub fn service_type(mut self, value: impl Into<String>) -> Self {
        self.service_type = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `serviceType`, clearing it.
    pub fn clear_service_type(mut self) -> Self {
        self.service_type = Some(None);
        self
    }

    /// Procedure (HCPCS) code billed against it.
    pub fn procedure_code(mut self, value: impl Into<String>) -> Self {
        self.procedure_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `procedureCode`, clearing it.
    pub fn clear_procedure_code(mut self) -> Self {
        self.procedure_code = Some(None);
        self
    }

    /// Procedure code modifiers, comma separated.
    pub fn modifiers(mut self, value: impl Into<String>) -> Self {
        self.modifiers = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `modifiers`, clearing it.
    pub fn clear_modifiers(mut self) -> Self {
        self.modifiers = Some(None);
        self
    }

    /// First date the authorization covers.
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `startDate`, clearing it.
    pub fn clear_start_date(mut self) -> Self {
        self.start_date = Some(None);
        self
    }

    /// Last date the authorization covers.
    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `endDate`, clearing it.
    pub fn clear_end_date(mut self) -> Self {
        self.end_date = Some(None);
        self
    }

    /// Units approved.
    pub fn authorized_units(mut self, value: i64) -> Self {
        self.authorized_units = Some(Some(value));
        self
    }

    /// Sends `null` for `authorizedUnits`, clearing it.
    pub fn clear_authorized_units(mut self) -> Self {
        self.authorized_units = Some(None);
        self
    }

    /// What a unit is, in the payer's words.
    pub fn unit_type(mut self, value: impl Into<String>) -> Self {
        self.unit_type = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `unitType`, clearing it.
    pub fn clear_unit_type(mut self) -> Self {
        self.unit_type = Some(None);
        self
    }

    /// Whether units count time, visits or days.
    pub fn unit_basis(mut self, value: AuthorizationUnitBasis) -> Self {
        self.unit_basis = Some(Some(value));
        self
    }

    /// Sends `null` for `unitBasis`, clearing it.
    pub fn clear_unit_basis(mut self) -> Self {
        self.unit_basis = Some(None);
        self
    }

    /// For time-based units, minutes in one unit.
    pub fn minutes_per_unit(mut self, value: i64) -> Self {
        self.minutes_per_unit = Some(Some(value));
        self
    }

    /// Sends `null` for `minutesPerUnit`, clearing it.
    pub fn clear_minutes_per_unit(mut self) -> Self {
        self.minutes_per_unit = Some(None);
        self
    }

    /// Frequency the payer approved.
    pub fn frequency(mut self, value: impl Into<String>) -> Self {
        self.frequency = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `frequency`, clearing it.
    pub fn clear_frequency(mut self) -> Self {
        self.frequency = Some(None);
        self
    }

    /// Place-of-service code.
    pub fn place_of_service(mut self, value: impl Into<String>) -> Self {
        self.place_of_service = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `placeOfService`, clearing it.
    pub fn clear_place_of_service(mut self) -> Self {
        self.place_of_service = Some(None);
        self
    }

    /// Whether the authorization is in effect.
    pub fn status(mut self, value: AuthorizationStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Notes.
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

/// Confirmation that an authorization was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AuthorizationDeleted {
    /// Always `authorization`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted authorization.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Incident reports: falls, injuries, medication errors and other events that need follow-up, often
/// for regulatory reporting. Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Incident {
    /// String representing the object's type. Always `incident`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the incident.
    #[serde(default)]
    pub id: String,
    /// Short summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// What happened.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When the incident happened.
    #[serde(
        rename = "incidentDateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub incident_date_time: Option<String>,
    /// Kind of incident.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<IncidentCategory>,
    /// How serious it was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<IncidentSeverity>,
    /// Where follow-up stands. New incidents start `Open`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IncidentStatus>,
    /// The patient involved.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// The caregiver involved.
    #[serde(
        rename = "caregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_id: Option<String>,
    /// The visit during which it happened.
    #[serde(rename = "visitId", default, skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<String>,
    /// What was done at the time.
    #[serde(
        rename = "actionsTaken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actions_taken: Option<String>,
    /// Body part injured, if any.
    #[serde(
        rename = "injuryBodyPart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub injury_body_part: Option<String>,
    /// Witness, if any.
    #[serde(
        rename = "witnessName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub witness_name: Option<String>,
    /// Whether 911 or other emergency services were called.
    #[serde(
        rename = "emergencyServicesContacted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub emergency_services_contacted: Option<bool>,
    /// Whether the patient went to hospital.
    #[serde(
        rename = "hospitalTransport",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hospital_transport: Option<bool>,
    /// Hospital, if transported.
    #[serde(
        rename = "hospitalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hospital_name: Option<String>,
    /// Whether the patient's physician was told.
    #[serde(
        rename = "physicianNotified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub physician_notified: Option<bool>,
    /// Whether family was told.
    #[serde(
        rename = "familyNotified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub family_notified: Option<bool>,
    /// Whether the incident must be reported to a regulator.
    #[serde(
        rename = "regulatoryReportRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub regulatory_report_required: Option<bool>,
    /// Whether follow-up is needed.
    #[serde(
        rename = "followUpRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_up_required: Option<bool>,
    /// When follow-up is due.
    #[serde(
        rename = "followUpDueAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_up_due_at: Option<String>,
    /// Follow-up notes.
    #[serde(
        rename = "followUpNotes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_up_notes: Option<String>,
    /// Whether the incident is resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
    /// How it was resolved.
    #[serde(
        rename = "resolutionNotes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resolution_notes: Option<String>,
    /// Whether only administrators can see it.
    #[serde(
        rename = "isConfidential",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_confidential: Option<bool>,
    /// Who reported it.
    #[serde(
        rename = "reportedByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reported_by_name: Option<String>,
    /// When it was reported.
    #[serde(
        rename = "reportedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reported_at: Option<String>,
    /// When it was resolved.
    #[serde(
        rename = "resolvedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resolved_at: Option<String>,
    /// When the incident was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the incident was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new incident. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct IncidentCreate {
    /// Short summary.
    pub title: String,
    /// What happened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// When the incident happened.
    #[serde(rename = "incidentDateTime", skip_serializing_if = "Option::is_none")]
    pub incident_date_time: Option<Option<String>>,
    /// Kind of incident.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Option<IncidentCategory>>,
    /// How serious it was.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Option<IncidentSeverity>>,
    /// Where follow-up stands. New incidents start `Open`. Only keys belonging to office staff can
    /// set this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<IncidentStatus>>,
    /// The patient involved.
    #[serde(rename = "patientId", skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<Option<String>>,
    /// The caregiver involved.
    #[serde(rename = "caregiverId", skip_serializing_if = "Option::is_none")]
    pub caregiver_id: Option<Option<String>>,
    /// The visit during which it happened.
    #[serde(rename = "visitId", skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<Option<String>>,
    /// What was done at the time.
    #[serde(rename = "actionsTaken", skip_serializing_if = "Option::is_none")]
    pub actions_taken: Option<Option<String>>,
    /// Body part injured, if any.
    #[serde(rename = "injuryBodyPart", skip_serializing_if = "Option::is_none")]
    pub injury_body_part: Option<Option<String>>,
    /// Witness, if any.
    #[serde(rename = "witnessName", skip_serializing_if = "Option::is_none")]
    pub witness_name: Option<Option<String>>,
    /// Whether 911 or other emergency services were called.
    #[serde(
        rename = "emergencyServicesContacted",
        skip_serializing_if = "Option::is_none"
    )]
    pub emergency_services_contacted: Option<Option<bool>>,
    /// Whether the patient went to hospital.
    #[serde(rename = "hospitalTransport", skip_serializing_if = "Option::is_none")]
    pub hospital_transport: Option<Option<bool>>,
    /// Hospital, if transported.
    #[serde(rename = "hospitalName", skip_serializing_if = "Option::is_none")]
    pub hospital_name: Option<Option<String>>,
    /// Whether the patient's physician was told.
    #[serde(rename = "physicianNotified", skip_serializing_if = "Option::is_none")]
    pub physician_notified: Option<Option<bool>>,
    /// Whether family was told.
    #[serde(rename = "familyNotified", skip_serializing_if = "Option::is_none")]
    pub family_notified: Option<Option<bool>>,
    /// Whether the incident must be reported to a regulator.
    #[serde(
        rename = "regulatoryReportRequired",
        skip_serializing_if = "Option::is_none"
    )]
    pub regulatory_report_required: Option<Option<bool>>,
    /// Whether follow-up is needed.
    #[serde(rename = "followUpRequired", skip_serializing_if = "Option::is_none")]
    pub follow_up_required: Option<Option<bool>>,
    /// When follow-up is due. Only keys belonging to office staff can set this.
    #[serde(rename = "followUpDueAt", skip_serializing_if = "Option::is_none")]
    pub follow_up_due_at: Option<Option<String>>,
    /// Follow-up notes.
    #[serde(rename = "followUpNotes", skip_serializing_if = "Option::is_none")]
    pub follow_up_notes: Option<Option<String>>,
    /// Whether the incident is resolved. Only keys belonging to office staff can set this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<Option<bool>>,
    /// How it was resolved. Only keys belonging to office staff can set this.
    #[serde(rename = "resolutionNotes", skip_serializing_if = "Option::is_none")]
    pub resolution_notes: Option<Option<String>>,
    /// Whether only administrators can see it. Only keys belonging to office staff can set this.
    #[serde(rename = "isConfidential", skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<Option<bool>>,
}

impl IncidentCreate {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            incident_date_time: None,
            category: None,
            severity: None,
            status: None,
            patient_id: None,
            caregiver_id: None,
            visit_id: None,
            actions_taken: None,
            injury_body_part: None,
            witness_name: None,
            emergency_services_contacted: None,
            hospital_transport: None,
            hospital_name: None,
            physician_notified: None,
            family_notified: None,
            regulatory_report_required: None,
            follow_up_required: None,
            follow_up_due_at: None,
            follow_up_notes: None,
            resolved: None,
            resolution_notes: None,
            is_confidential: None,
        }
    }

    /// What happened.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `description`, clearing it.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }

    /// When the incident happened.
    pub fn incident_date_time(mut self, value: impl Into<String>) -> Self {
        self.incident_date_time = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `incidentDateTime`, clearing it.
    pub fn clear_incident_date_time(mut self) -> Self {
        self.incident_date_time = Some(None);
        self
    }

    /// Kind of incident.
    pub fn category(mut self, value: IncidentCategory) -> Self {
        self.category = Some(Some(value));
        self
    }

    /// Sends `null` for `category`, clearing it.
    pub fn clear_category(mut self) -> Self {
        self.category = Some(None);
        self
    }

    /// How serious it was.
    pub fn severity(mut self, value: IncidentSeverity) -> Self {
        self.severity = Some(Some(value));
        self
    }

    /// Sends `null` for `severity`, clearing it.
    pub fn clear_severity(mut self) -> Self {
        self.severity = Some(None);
        self
    }

    /// Where follow-up stands. New incidents start `Open`. Only keys belonging to office staff can
    /// set this.
    pub fn status(mut self, value: IncidentStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// The patient involved.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `patientId`, clearing it.
    pub fn clear_patient_id(mut self) -> Self {
        self.patient_id = Some(None);
        self
    }

    /// The caregiver involved.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `caregiverId`, clearing it.
    pub fn clear_caregiver_id(mut self) -> Self {
        self.caregiver_id = Some(None);
        self
    }

    /// The visit during which it happened.
    pub fn visit_id(mut self, value: impl Into<String>) -> Self {
        self.visit_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `visitId`, clearing it.
    pub fn clear_visit_id(mut self) -> Self {
        self.visit_id = Some(None);
        self
    }

    /// What was done at the time.
    pub fn actions_taken(mut self, value: impl Into<String>) -> Self {
        self.actions_taken = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `actionsTaken`, clearing it.
    pub fn clear_actions_taken(mut self) -> Self {
        self.actions_taken = Some(None);
        self
    }

    /// Body part injured, if any.
    pub fn injury_body_part(mut self, value: impl Into<String>) -> Self {
        self.injury_body_part = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `injuryBodyPart`, clearing it.
    pub fn clear_injury_body_part(mut self) -> Self {
        self.injury_body_part = Some(None);
        self
    }

    /// Witness, if any.
    pub fn witness_name(mut self, value: impl Into<String>) -> Self {
        self.witness_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `witnessName`, clearing it.
    pub fn clear_witness_name(mut self) -> Self {
        self.witness_name = Some(None);
        self
    }

    /// Whether 911 or other emergency services were called.
    pub fn emergency_services_contacted(mut self, value: bool) -> Self {
        self.emergency_services_contacted = Some(Some(value));
        self
    }

    /// Sends `null` for `emergencyServicesContacted`, clearing it.
    pub fn clear_emergency_services_contacted(mut self) -> Self {
        self.emergency_services_contacted = Some(None);
        self
    }

    /// Whether the patient went to hospital.
    pub fn hospital_transport(mut self, value: bool) -> Self {
        self.hospital_transport = Some(Some(value));
        self
    }

    /// Sends `null` for `hospitalTransport`, clearing it.
    pub fn clear_hospital_transport(mut self) -> Self {
        self.hospital_transport = Some(None);
        self
    }

    /// Hospital, if transported.
    pub fn hospital_name(mut self, value: impl Into<String>) -> Self {
        self.hospital_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `hospitalName`, clearing it.
    pub fn clear_hospital_name(mut self) -> Self {
        self.hospital_name = Some(None);
        self
    }

    /// Whether the patient's physician was told.
    pub fn physician_notified(mut self, value: bool) -> Self {
        self.physician_notified = Some(Some(value));
        self
    }

    /// Sends `null` for `physicianNotified`, clearing it.
    pub fn clear_physician_notified(mut self) -> Self {
        self.physician_notified = Some(None);
        self
    }

    /// Whether family was told.
    pub fn family_notified(mut self, value: bool) -> Self {
        self.family_notified = Some(Some(value));
        self
    }

    /// Sends `null` for `familyNotified`, clearing it.
    pub fn clear_family_notified(mut self) -> Self {
        self.family_notified = Some(None);
        self
    }

    /// Whether the incident must be reported to a regulator.
    pub fn regulatory_report_required(mut self, value: bool) -> Self {
        self.regulatory_report_required = Some(Some(value));
        self
    }

    /// Sends `null` for `regulatoryReportRequired`, clearing it.
    pub fn clear_regulatory_report_required(mut self) -> Self {
        self.regulatory_report_required = Some(None);
        self
    }

    /// Whether follow-up is needed.
    pub fn follow_up_required(mut self, value: bool) -> Self {
        self.follow_up_required = Some(Some(value));
        self
    }

    /// Sends `null` for `followUpRequired`, clearing it.
    pub fn clear_follow_up_required(mut self) -> Self {
        self.follow_up_required = Some(None);
        self
    }

    /// When follow-up is due. Only keys belonging to office staff can set this.
    pub fn follow_up_due_at(mut self, value: impl Into<String>) -> Self {
        self.follow_up_due_at = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `followUpDueAt`, clearing it.
    pub fn clear_follow_up_due_at(mut self) -> Self {
        self.follow_up_due_at = Some(None);
        self
    }

    /// Follow-up notes.
    pub fn follow_up_notes(mut self, value: impl Into<String>) -> Self {
        self.follow_up_notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `followUpNotes`, clearing it.
    pub fn clear_follow_up_notes(mut self) -> Self {
        self.follow_up_notes = Some(None);
        self
    }

    /// Whether the incident is resolved. Only keys belonging to office staff can set this.
    pub fn resolved(mut self, value: bool) -> Self {
        self.resolved = Some(Some(value));
        self
    }

    /// Sends `null` for `resolved`, clearing it.
    pub fn clear_resolved(mut self) -> Self {
        self.resolved = Some(None);
        self
    }

    /// How it was resolved. Only keys belonging to office staff can set this.
    pub fn resolution_notes(mut self, value: impl Into<String>) -> Self {
        self.resolution_notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `resolutionNotes`, clearing it.
    pub fn clear_resolution_notes(mut self) -> Self {
        self.resolution_notes = Some(None);
        self
    }

    /// Whether only administrators can see it. Only keys belonging to office staff can set this.
    pub fn is_confidential(mut self, value: bool) -> Self {
        self.is_confidential = Some(Some(value));
        self
    }

    /// Sends `null` for `isConfidential`, clearing it.
    pub fn clear_is_confidential(mut self) -> Self {
        self.is_confidential = Some(None);
        self
    }
}

/// Fields to change on an incident. Send only the fields to change; any other field is refused with
/// `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct IncidentUpdate {
    /// Short summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// What happened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// When the incident happened.
    #[serde(rename = "incidentDateTime", skip_serializing_if = "Option::is_none")]
    pub incident_date_time: Option<Option<String>>,
    /// Kind of incident.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Option<IncidentCategory>>,
    /// How serious it was.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Option<IncidentSeverity>>,
    /// Where follow-up stands. New incidents start `Open`. Only keys belonging to office staff can
    /// set this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<IncidentStatus>>,
    /// The patient involved.
    #[serde(rename = "patientId", skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<Option<String>>,
    /// The caregiver involved.
    #[serde(rename = "caregiverId", skip_serializing_if = "Option::is_none")]
    pub caregiver_id: Option<Option<String>>,
    /// The visit during which it happened.
    #[serde(rename = "visitId", skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<Option<String>>,
    /// What was done at the time.
    #[serde(rename = "actionsTaken", skip_serializing_if = "Option::is_none")]
    pub actions_taken: Option<Option<String>>,
    /// Body part injured, if any.
    #[serde(rename = "injuryBodyPart", skip_serializing_if = "Option::is_none")]
    pub injury_body_part: Option<Option<String>>,
    /// Witness, if any.
    #[serde(rename = "witnessName", skip_serializing_if = "Option::is_none")]
    pub witness_name: Option<Option<String>>,
    /// Whether 911 or other emergency services were called.
    #[serde(
        rename = "emergencyServicesContacted",
        skip_serializing_if = "Option::is_none"
    )]
    pub emergency_services_contacted: Option<Option<bool>>,
    /// Whether the patient went to hospital.
    #[serde(rename = "hospitalTransport", skip_serializing_if = "Option::is_none")]
    pub hospital_transport: Option<Option<bool>>,
    /// Hospital, if transported.
    #[serde(rename = "hospitalName", skip_serializing_if = "Option::is_none")]
    pub hospital_name: Option<Option<String>>,
    /// Whether the patient's physician was told.
    #[serde(rename = "physicianNotified", skip_serializing_if = "Option::is_none")]
    pub physician_notified: Option<Option<bool>>,
    /// Whether family was told.
    #[serde(rename = "familyNotified", skip_serializing_if = "Option::is_none")]
    pub family_notified: Option<Option<bool>>,
    /// Whether the incident must be reported to a regulator.
    #[serde(
        rename = "regulatoryReportRequired",
        skip_serializing_if = "Option::is_none"
    )]
    pub regulatory_report_required: Option<Option<bool>>,
    /// Whether follow-up is needed.
    #[serde(rename = "followUpRequired", skip_serializing_if = "Option::is_none")]
    pub follow_up_required: Option<Option<bool>>,
    /// When follow-up is due. Only keys belonging to office staff can set this.
    #[serde(rename = "followUpDueAt", skip_serializing_if = "Option::is_none")]
    pub follow_up_due_at: Option<Option<String>>,
    /// Follow-up notes.
    #[serde(rename = "followUpNotes", skip_serializing_if = "Option::is_none")]
    pub follow_up_notes: Option<Option<String>>,
    /// Whether the incident is resolved. Only keys belonging to office staff can set this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<Option<bool>>,
    /// How it was resolved. Only keys belonging to office staff can set this.
    #[serde(rename = "resolutionNotes", skip_serializing_if = "Option::is_none")]
    pub resolution_notes: Option<Option<String>>,
    /// Whether only administrators can see it. Only keys belonging to office staff can set this.
    #[serde(rename = "isConfidential", skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<Option<bool>>,
}

impl IncidentUpdate {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            incident_date_time: None,
            category: None,
            severity: None,
            status: None,
            patient_id: None,
            caregiver_id: None,
            visit_id: None,
            actions_taken: None,
            injury_body_part: None,
            witness_name: None,
            emergency_services_contacted: None,
            hospital_transport: None,
            hospital_name: None,
            physician_notified: None,
            family_notified: None,
            regulatory_report_required: None,
            follow_up_required: None,
            follow_up_due_at: None,
            follow_up_notes: None,
            resolved: None,
            resolution_notes: None,
            is_confidential: None,
        }
    }

    /// Short summary.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// What happened.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `description`, clearing it.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }

    /// When the incident happened.
    pub fn incident_date_time(mut self, value: impl Into<String>) -> Self {
        self.incident_date_time = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `incidentDateTime`, clearing it.
    pub fn clear_incident_date_time(mut self) -> Self {
        self.incident_date_time = Some(None);
        self
    }

    /// Kind of incident.
    pub fn category(mut self, value: IncidentCategory) -> Self {
        self.category = Some(Some(value));
        self
    }

    /// Sends `null` for `category`, clearing it.
    pub fn clear_category(mut self) -> Self {
        self.category = Some(None);
        self
    }

    /// How serious it was.
    pub fn severity(mut self, value: IncidentSeverity) -> Self {
        self.severity = Some(Some(value));
        self
    }

    /// Sends `null` for `severity`, clearing it.
    pub fn clear_severity(mut self) -> Self {
        self.severity = Some(None);
        self
    }

    /// Where follow-up stands. New incidents start `Open`. Only keys belonging to office staff can
    /// set this.
    pub fn status(mut self, value: IncidentStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// The patient involved.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `patientId`, clearing it.
    pub fn clear_patient_id(mut self) -> Self {
        self.patient_id = Some(None);
        self
    }

    /// The caregiver involved.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `caregiverId`, clearing it.
    pub fn clear_caregiver_id(mut self) -> Self {
        self.caregiver_id = Some(None);
        self
    }

    /// The visit during which it happened.
    pub fn visit_id(mut self, value: impl Into<String>) -> Self {
        self.visit_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `visitId`, clearing it.
    pub fn clear_visit_id(mut self) -> Self {
        self.visit_id = Some(None);
        self
    }

    /// What was done at the time.
    pub fn actions_taken(mut self, value: impl Into<String>) -> Self {
        self.actions_taken = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `actionsTaken`, clearing it.
    pub fn clear_actions_taken(mut self) -> Self {
        self.actions_taken = Some(None);
        self
    }

    /// Body part injured, if any.
    pub fn injury_body_part(mut self, value: impl Into<String>) -> Self {
        self.injury_body_part = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `injuryBodyPart`, clearing it.
    pub fn clear_injury_body_part(mut self) -> Self {
        self.injury_body_part = Some(None);
        self
    }

    /// Witness, if any.
    pub fn witness_name(mut self, value: impl Into<String>) -> Self {
        self.witness_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `witnessName`, clearing it.
    pub fn clear_witness_name(mut self) -> Self {
        self.witness_name = Some(None);
        self
    }

    /// Whether 911 or other emergency services were called.
    pub fn emergency_services_contacted(mut self, value: bool) -> Self {
        self.emergency_services_contacted = Some(Some(value));
        self
    }

    /// Sends `null` for `emergencyServicesContacted`, clearing it.
    pub fn clear_emergency_services_contacted(mut self) -> Self {
        self.emergency_services_contacted = Some(None);
        self
    }

    /// Whether the patient went to hospital.
    pub fn hospital_transport(mut self, value: bool) -> Self {
        self.hospital_transport = Some(Some(value));
        self
    }

    /// Sends `null` for `hospitalTransport`, clearing it.
    pub fn clear_hospital_transport(mut self) -> Self {
        self.hospital_transport = Some(None);
        self
    }

    /// Hospital, if transported.
    pub fn hospital_name(mut self, value: impl Into<String>) -> Self {
        self.hospital_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `hospitalName`, clearing it.
    pub fn clear_hospital_name(mut self) -> Self {
        self.hospital_name = Some(None);
        self
    }

    /// Whether the patient's physician was told.
    pub fn physician_notified(mut self, value: bool) -> Self {
        self.physician_notified = Some(Some(value));
        self
    }

    /// Sends `null` for `physicianNotified`, clearing it.
    pub fn clear_physician_notified(mut self) -> Self {
        self.physician_notified = Some(None);
        self
    }

    /// Whether family was told.
    pub fn family_notified(mut self, value: bool) -> Self {
        self.family_notified = Some(Some(value));
        self
    }

    /// Sends `null` for `familyNotified`, clearing it.
    pub fn clear_family_notified(mut self) -> Self {
        self.family_notified = Some(None);
        self
    }

    /// Whether the incident must be reported to a regulator.
    pub fn regulatory_report_required(mut self, value: bool) -> Self {
        self.regulatory_report_required = Some(Some(value));
        self
    }

    /// Sends `null` for `regulatoryReportRequired`, clearing it.
    pub fn clear_regulatory_report_required(mut self) -> Self {
        self.regulatory_report_required = Some(None);
        self
    }

    /// Whether follow-up is needed.
    pub fn follow_up_required(mut self, value: bool) -> Self {
        self.follow_up_required = Some(Some(value));
        self
    }

    /// Sends `null` for `followUpRequired`, clearing it.
    pub fn clear_follow_up_required(mut self) -> Self {
        self.follow_up_required = Some(None);
        self
    }

    /// When follow-up is due. Only keys belonging to office staff can set this.
    pub fn follow_up_due_at(mut self, value: impl Into<String>) -> Self {
        self.follow_up_due_at = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `followUpDueAt`, clearing it.
    pub fn clear_follow_up_due_at(mut self) -> Self {
        self.follow_up_due_at = Some(None);
        self
    }

    /// Follow-up notes.
    pub fn follow_up_notes(mut self, value: impl Into<String>) -> Self {
        self.follow_up_notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `followUpNotes`, clearing it.
    pub fn clear_follow_up_notes(mut self) -> Self {
        self.follow_up_notes = Some(None);
        self
    }

    /// Whether the incident is resolved. Only keys belonging to office staff can set this.
    pub fn resolved(mut self, value: bool) -> Self {
        self.resolved = Some(Some(value));
        self
    }

    /// Sends `null` for `resolved`, clearing it.
    pub fn clear_resolved(mut self) -> Self {
        self.resolved = Some(None);
        self
    }

    /// How it was resolved. Only keys belonging to office staff can set this.
    pub fn resolution_notes(mut self, value: impl Into<String>) -> Self {
        self.resolution_notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `resolutionNotes`, clearing it.
    pub fn clear_resolution_notes(mut self) -> Self {
        self.resolution_notes = Some(None);
        self
    }

    /// Whether only administrators can see it. Only keys belonging to office staff can set this.
    pub fn is_confidential(mut self, value: bool) -> Self {
        self.is_confidential = Some(Some(value));
        self
    }

    /// Sends `null` for `isConfidential`, clearing it.
    pub fn clear_is_confidential(mut self) -> Self {
        self.is_confidential = Some(None);
        self
    }
}

/// Confirmation that an incident was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IncidentDeleted {
    /// Always `incident`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted incident.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Events your agency documents about a patient, caregiver or visit, such as a complaint, a
/// compliment or an attendance issue. Every field is always present; a field with no value is
/// `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Event {
    /// String representing the object's type. Always `event`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the event.
    #[serde(default)]
    pub id: String,
    /// Short summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// What happened.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Category, as named in your agency's event types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// How serious it was.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<IncidentSeverity>,
    /// Where follow-up stands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<IncidentStatus>,
    /// When it happened.
    #[serde(
        rename = "occurredAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub occurred_at: Option<String>,
    /// The patient involved.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// The caregiver involved.
    #[serde(
        rename = "caregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_id: Option<String>,
    /// The visit involved.
    #[serde(rename = "visitId", default, skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<String>,
    /// Attendance points this event counts for, if your agency uses a points policy.
    #[serde(
        rename = "pointValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub point_value: Option<f64>,
    /// What was done.
    #[serde(
        rename = "actionsTaken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actions_taken: Option<String>,
    /// Outcome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    /// Whether follow-up is needed.
    #[serde(
        rename = "followUpRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_up_required: Option<bool>,
    /// When follow-up is due.
    #[serde(
        rename = "followUpDueAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_up_due_at: Option<String>,
    /// Follow-up notes.
    #[serde(
        rename = "followUpNotes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub follow_up_notes: Option<String>,
    /// How it was resolved.
    #[serde(
        rename = "resolutionNotes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resolution_notes: Option<String>,
    /// Whether only administrators can see it.
    #[serde(
        rename = "isConfidential",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_confidential: Option<bool>,
    /// Who documented it.
    #[serde(
        rename = "reportedByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reported_by_name: Option<String>,
    /// When it was documented.
    #[serde(
        rename = "reportedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reported_at: Option<String>,
    /// When it was resolved.
    #[serde(
        rename = "resolvedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resolved_at: Option<String>,
    /// When the event was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the event was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new event. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct EventCreate {
    /// Short summary.
    pub title: String,
    /// What happened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// Category, as named in your agency's event types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Option<String>>,
    /// How serious it was.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Option<IncidentSeverity>>,
    /// Where follow-up stands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<IncidentStatus>>,
    /// When it happened.
    #[serde(rename = "occurredAt", skip_serializing_if = "Option::is_none")]
    pub occurred_at: Option<Option<String>>,
    /// The patient involved.
    #[serde(rename = "patientId", skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<Option<String>>,
    /// The caregiver involved.
    #[serde(rename = "caregiverId", skip_serializing_if = "Option::is_none")]
    pub caregiver_id: Option<Option<String>>,
    /// The visit involved.
    #[serde(rename = "visitId", skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<Option<String>>,
    /// Attendance points this event counts for, if your agency uses a points policy.
    #[serde(rename = "pointValue", skip_serializing_if = "Option::is_none")]
    pub point_value: Option<Option<f64>>,
    /// What was done.
    #[serde(rename = "actionsTaken", skip_serializing_if = "Option::is_none")]
    pub actions_taken: Option<Option<String>>,
    /// Outcome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<Option<String>>,
    /// Whether follow-up is needed.
    #[serde(rename = "followUpRequired", skip_serializing_if = "Option::is_none")]
    pub follow_up_required: Option<Option<bool>>,
    /// When follow-up is due.
    #[serde(rename = "followUpDueAt", skip_serializing_if = "Option::is_none")]
    pub follow_up_due_at: Option<Option<String>>,
    /// Follow-up notes.
    #[serde(rename = "followUpNotes", skip_serializing_if = "Option::is_none")]
    pub follow_up_notes: Option<Option<String>>,
    /// How it was resolved.
    #[serde(rename = "resolutionNotes", skip_serializing_if = "Option::is_none")]
    pub resolution_notes: Option<Option<String>>,
    /// Whether only administrators can see it.
    #[serde(rename = "isConfidential", skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<Option<bool>>,
}

impl EventCreate {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            category: None,
            severity: None,
            status: None,
            occurred_at: None,
            patient_id: None,
            caregiver_id: None,
            visit_id: None,
            point_value: None,
            actions_taken: None,
            outcome: None,
            follow_up_required: None,
            follow_up_due_at: None,
            follow_up_notes: None,
            resolution_notes: None,
            is_confidential: None,
        }
    }

    /// What happened.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `description`, clearing it.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }

    /// Category, as named in your agency's event types.
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `category`, clearing it.
    pub fn clear_category(mut self) -> Self {
        self.category = Some(None);
        self
    }

    /// How serious it was.
    pub fn severity(mut self, value: IncidentSeverity) -> Self {
        self.severity = Some(Some(value));
        self
    }

    /// Sends `null` for `severity`, clearing it.
    pub fn clear_severity(mut self) -> Self {
        self.severity = Some(None);
        self
    }

    /// Where follow-up stands.
    pub fn status(mut self, value: IncidentStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// When it happened.
    pub fn occurred_at(mut self, value: impl Into<String>) -> Self {
        self.occurred_at = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `occurredAt`, clearing it.
    pub fn clear_occurred_at(mut self) -> Self {
        self.occurred_at = Some(None);
        self
    }

    /// The patient involved.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `patientId`, clearing it.
    pub fn clear_patient_id(mut self) -> Self {
        self.patient_id = Some(None);
        self
    }

    /// The caregiver involved.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `caregiverId`, clearing it.
    pub fn clear_caregiver_id(mut self) -> Self {
        self.caregiver_id = Some(None);
        self
    }

    /// The visit involved.
    pub fn visit_id(mut self, value: impl Into<String>) -> Self {
        self.visit_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `visitId`, clearing it.
    pub fn clear_visit_id(mut self) -> Self {
        self.visit_id = Some(None);
        self
    }

    /// Attendance points this event counts for, if your agency uses a points policy.
    pub fn point_value(mut self, value: f64) -> Self {
        self.point_value = Some(Some(value));
        self
    }

    /// Sends `null` for `pointValue`, clearing it.
    pub fn clear_point_value(mut self) -> Self {
        self.point_value = Some(None);
        self
    }

    /// What was done.
    pub fn actions_taken(mut self, value: impl Into<String>) -> Self {
        self.actions_taken = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `actionsTaken`, clearing it.
    pub fn clear_actions_taken(mut self) -> Self {
        self.actions_taken = Some(None);
        self
    }

    /// Outcome.
    pub fn outcome(mut self, value: impl Into<String>) -> Self {
        self.outcome = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `outcome`, clearing it.
    pub fn clear_outcome(mut self) -> Self {
        self.outcome = Some(None);
        self
    }

    /// Whether follow-up is needed.
    pub fn follow_up_required(mut self, value: bool) -> Self {
        self.follow_up_required = Some(Some(value));
        self
    }

    /// Sends `null` for `followUpRequired`, clearing it.
    pub fn clear_follow_up_required(mut self) -> Self {
        self.follow_up_required = Some(None);
        self
    }

    /// When follow-up is due.
    pub fn follow_up_due_at(mut self, value: impl Into<String>) -> Self {
        self.follow_up_due_at = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `followUpDueAt`, clearing it.
    pub fn clear_follow_up_due_at(mut self) -> Self {
        self.follow_up_due_at = Some(None);
        self
    }

    /// Follow-up notes.
    pub fn follow_up_notes(mut self, value: impl Into<String>) -> Self {
        self.follow_up_notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `followUpNotes`, clearing it.
    pub fn clear_follow_up_notes(mut self) -> Self {
        self.follow_up_notes = Some(None);
        self
    }

    /// How it was resolved.
    pub fn resolution_notes(mut self, value: impl Into<String>) -> Self {
        self.resolution_notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `resolutionNotes`, clearing it.
    pub fn clear_resolution_notes(mut self) -> Self {
        self.resolution_notes = Some(None);
        self
    }

    /// Whether only administrators can see it.
    pub fn is_confidential(mut self, value: bool) -> Self {
        self.is_confidential = Some(Some(value));
        self
    }

    /// Sends `null` for `isConfidential`, clearing it.
    pub fn clear_is_confidential(mut self) -> Self {
        self.is_confidential = Some(None);
        self
    }
}

/// Fields to change on an event. Send only the fields to change; any other field is refused with
/// `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct EventUpdate {
    /// Short summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// What happened.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// Category, as named in your agency's event types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Option<String>>,
    /// How serious it was.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Option<IncidentSeverity>>,
    /// Where follow-up stands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<IncidentStatus>>,
    /// When it happened.
    #[serde(rename = "occurredAt", skip_serializing_if = "Option::is_none")]
    pub occurred_at: Option<Option<String>>,
    /// The patient involved.
    #[serde(rename = "patientId", skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<Option<String>>,
    /// The caregiver involved.
    #[serde(rename = "caregiverId", skip_serializing_if = "Option::is_none")]
    pub caregiver_id: Option<Option<String>>,
    /// The visit involved.
    #[serde(rename = "visitId", skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<Option<String>>,
    /// Attendance points this event counts for, if your agency uses a points policy.
    #[serde(rename = "pointValue", skip_serializing_if = "Option::is_none")]
    pub point_value: Option<Option<f64>>,
    /// What was done.
    #[serde(rename = "actionsTaken", skip_serializing_if = "Option::is_none")]
    pub actions_taken: Option<Option<String>>,
    /// Outcome.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<Option<String>>,
    /// Whether follow-up is needed.
    #[serde(rename = "followUpRequired", skip_serializing_if = "Option::is_none")]
    pub follow_up_required: Option<Option<bool>>,
    /// When follow-up is due.
    #[serde(rename = "followUpDueAt", skip_serializing_if = "Option::is_none")]
    pub follow_up_due_at: Option<Option<String>>,
    /// Follow-up notes.
    #[serde(rename = "followUpNotes", skip_serializing_if = "Option::is_none")]
    pub follow_up_notes: Option<Option<String>>,
    /// How it was resolved.
    #[serde(rename = "resolutionNotes", skip_serializing_if = "Option::is_none")]
    pub resolution_notes: Option<Option<String>>,
    /// Whether only administrators can see it.
    #[serde(rename = "isConfidential", skip_serializing_if = "Option::is_none")]
    pub is_confidential: Option<Option<bool>>,
}

impl EventUpdate {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            category: None,
            severity: None,
            status: None,
            occurred_at: None,
            patient_id: None,
            caregiver_id: None,
            visit_id: None,
            point_value: None,
            actions_taken: None,
            outcome: None,
            follow_up_required: None,
            follow_up_due_at: None,
            follow_up_notes: None,
            resolution_notes: None,
            is_confidential: None,
        }
    }

    /// Short summary.
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// What happened.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `description`, clearing it.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }

    /// Category, as named in your agency's event types.
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `category`, clearing it.
    pub fn clear_category(mut self) -> Self {
        self.category = Some(None);
        self
    }

    /// How serious it was.
    pub fn severity(mut self, value: IncidentSeverity) -> Self {
        self.severity = Some(Some(value));
        self
    }

    /// Sends `null` for `severity`, clearing it.
    pub fn clear_severity(mut self) -> Self {
        self.severity = Some(None);
        self
    }

    /// Where follow-up stands.
    pub fn status(mut self, value: IncidentStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// When it happened.
    pub fn occurred_at(mut self, value: impl Into<String>) -> Self {
        self.occurred_at = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `occurredAt`, clearing it.
    pub fn clear_occurred_at(mut self) -> Self {
        self.occurred_at = Some(None);
        self
    }

    /// The patient involved.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `patientId`, clearing it.
    pub fn clear_patient_id(mut self) -> Self {
        self.patient_id = Some(None);
        self
    }

    /// The caregiver involved.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `caregiverId`, clearing it.
    pub fn clear_caregiver_id(mut self) -> Self {
        self.caregiver_id = Some(None);
        self
    }

    /// The visit involved.
    pub fn visit_id(mut self, value: impl Into<String>) -> Self {
        self.visit_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `visitId`, clearing it.
    pub fn clear_visit_id(mut self) -> Self {
        self.visit_id = Some(None);
        self
    }

    /// Attendance points this event counts for, if your agency uses a points policy.
    pub fn point_value(mut self, value: f64) -> Self {
        self.point_value = Some(Some(value));
        self
    }

    /// Sends `null` for `pointValue`, clearing it.
    pub fn clear_point_value(mut self) -> Self {
        self.point_value = Some(None);
        self
    }

    /// What was done.
    pub fn actions_taken(mut self, value: impl Into<String>) -> Self {
        self.actions_taken = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `actionsTaken`, clearing it.
    pub fn clear_actions_taken(mut self) -> Self {
        self.actions_taken = Some(None);
        self
    }

    /// Outcome.
    pub fn outcome(mut self, value: impl Into<String>) -> Self {
        self.outcome = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `outcome`, clearing it.
    pub fn clear_outcome(mut self) -> Self {
        self.outcome = Some(None);
        self
    }

    /// Whether follow-up is needed.
    pub fn follow_up_required(mut self, value: bool) -> Self {
        self.follow_up_required = Some(Some(value));
        self
    }

    /// Sends `null` for `followUpRequired`, clearing it.
    pub fn clear_follow_up_required(mut self) -> Self {
        self.follow_up_required = Some(None);
        self
    }

    /// When follow-up is due.
    pub fn follow_up_due_at(mut self, value: impl Into<String>) -> Self {
        self.follow_up_due_at = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `followUpDueAt`, clearing it.
    pub fn clear_follow_up_due_at(mut self) -> Self {
        self.follow_up_due_at = Some(None);
        self
    }

    /// Follow-up notes.
    pub fn follow_up_notes(mut self, value: impl Into<String>) -> Self {
        self.follow_up_notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `followUpNotes`, clearing it.
    pub fn clear_follow_up_notes(mut self) -> Self {
        self.follow_up_notes = Some(None);
        self
    }

    /// How it was resolved.
    pub fn resolution_notes(mut self, value: impl Into<String>) -> Self {
        self.resolution_notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `resolutionNotes`, clearing it.
    pub fn clear_resolution_notes(mut self) -> Self {
        self.resolution_notes = Some(None);
        self
    }

    /// Whether only administrators can see it.
    pub fn is_confidential(mut self, value: bool) -> Self {
        self.is_confidential = Some(Some(value));
        self
    }

    /// Sends `null` for `isConfidential`, clearing it.
    pub fn clear_is_confidential(mut self) -> Self {
        self.is_confidential = Some(None);
        self
    }
}

/// Confirmation that an event was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventDeleted {
    /// Always `event`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted event.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Hospitals, physicians, case managers and other organizations that refer clients to your agency.
/// Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReferralSource {
    /// String representing the object's type. Always `referral_source`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the referral source.
    #[serde(default)]
    pub id: String,
    /// Name of the source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Kind of source, such as Hospital or Physician.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Organization the contact works for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Main contact.
    #[serde(
        rename = "contactName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_name: Option<String>,
    /// Contact's job title.
    #[serde(
        rename = "contactTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_title: Option<String>,
    /// Contact's phone number.
    #[serde(
        rename = "contactPhone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_phone: Option<String>,
    /// Contact's email address.
    #[serde(
        rename = "contactEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_email: Option<String>,
    /// Street address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// City.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State, as a two-letter code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ZIP code.
    #[serde(rename = "zipCode", default, skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// Notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Whether you still receive referrals from this source.
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Referrals received from this source.
    #[serde(
        rename = "totalReferrals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_referrals: Option<i64>,
    /// Referrals from this source that became clients.
    #[serde(
        rename = "totalConversions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_conversions: Option<i64>,
    /// When the most recent referral arrived.
    #[serde(
        rename = "lastReferralDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_referral_date: Option<String>,
    /// When the referral source was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the referral source was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new referral source. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ReferralSourceCreate {
    /// Name of the source.
    pub name: String,
    /// Kind of source, such as Hospital or Physician.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
    /// Organization the contact works for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Option<String>>,
    /// Main contact.
    #[serde(rename = "contactName", skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<Option<String>>,
    /// Contact's job title.
    #[serde(rename = "contactTitle", skip_serializing_if = "Option::is_none")]
    pub contact_title: Option<Option<String>>,
    /// Contact's phone number.
    #[serde(rename = "contactPhone", skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<Option<String>>,
    /// Contact's email address.
    #[serde(rename = "contactEmail", skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<Option<String>>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<String>>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// State, as a two-letter code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    /// ZIP code.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<Option<String>>,
    /// Notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
    /// Whether you still receive referrals from this source.
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<Option<bool>>,
}

impl ReferralSourceCreate {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            r#type: None,
            organization: None,
            contact_name: None,
            contact_title: None,
            contact_phone: None,
            contact_email: None,
            address: None,
            city: None,
            state: None,
            zip_code: None,
            notes: None,
            is_active: None,
        }
    }

    /// Kind of source, such as Hospital or Physician.
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `type`, clearing it.
    pub fn clear_type(mut self) -> Self {
        self.r#type = Some(None);
        self
    }

    /// Organization the contact works for.
    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.organization = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `organization`, clearing it.
    pub fn clear_organization(mut self) -> Self {
        self.organization = Some(None);
        self
    }

    /// Main contact.
    pub fn contact_name(mut self, value: impl Into<String>) -> Self {
        self.contact_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `contactName`, clearing it.
    pub fn clear_contact_name(mut self) -> Self {
        self.contact_name = Some(None);
        self
    }

    /// Contact's job title.
    pub fn contact_title(mut self, value: impl Into<String>) -> Self {
        self.contact_title = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `contactTitle`, clearing it.
    pub fn clear_contact_title(mut self) -> Self {
        self.contact_title = Some(None);
        self
    }

    /// Contact's phone number.
    pub fn contact_phone(mut self, value: impl Into<String>) -> Self {
        self.contact_phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `contactPhone`, clearing it.
    pub fn clear_contact_phone(mut self) -> Self {
        self.contact_phone = Some(None);
        self
    }

    /// Contact's email address.
    pub fn contact_email(mut self, value: impl Into<String>) -> Self {
        self.contact_email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `contactEmail`, clearing it.
    pub fn clear_contact_email(mut self) -> Self {
        self.contact_email = Some(None);
        self
    }

    /// Street address.
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `address`, clearing it.
    pub fn clear_address(mut self) -> Self {
        self.address = Some(None);
        self
    }

    /// City.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `city`, clearing it.
    pub fn clear_city(mut self) -> Self {
        self.city = Some(None);
        self
    }

    /// State, as a two-letter code.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `state`, clearing it.
    pub fn clear_state(mut self) -> Self {
        self.state = Some(None);
        self
    }

    /// ZIP code.
    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `zipCode`, clearing it.
    pub fn clear_zip_code(mut self) -> Self {
        self.zip_code = Some(None);
        self
    }

    /// Notes.
    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `notes`, clearing it.
    pub fn clear_notes(mut self) -> Self {
        self.notes = Some(None);
        self
    }

    /// Whether you still receive referrals from this source.
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(Some(value));
        self
    }

    /// Sends `null` for `isActive`, clearing it.
    pub fn clear_is_active(mut self) -> Self {
        self.is_active = Some(None);
        self
    }
}

/// Fields to change on a referral source. Send only the fields to change; any other field is
/// refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ReferralSourceUpdate {
    /// Name of the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Kind of source, such as Hospital or Physician.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
    /// Organization the contact works for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Option<String>>,
    /// Main contact.
    #[serde(rename = "contactName", skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<Option<String>>,
    /// Contact's job title.
    #[serde(rename = "contactTitle", skip_serializing_if = "Option::is_none")]
    pub contact_title: Option<Option<String>>,
    /// Contact's phone number.
    #[serde(rename = "contactPhone", skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<Option<String>>,
    /// Contact's email address.
    #[serde(rename = "contactEmail", skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<Option<String>>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<String>>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// State, as a two-letter code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    /// ZIP code.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<Option<String>>,
    /// Notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
    /// Whether you still receive referrals from this source.
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<Option<bool>>,
}

impl ReferralSourceUpdate {
    pub fn new() -> Self {
        Self {
            name: None,
            r#type: None,
            organization: None,
            contact_name: None,
            contact_title: None,
            contact_phone: None,
            contact_email: None,
            address: None,
            city: None,
            state: None,
            zip_code: None,
            notes: None,
            is_active: None,
        }
    }

    /// Name of the source.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Kind of source, such as Hospital or Physician.
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `type`, clearing it.
    pub fn clear_type(mut self) -> Self {
        self.r#type = Some(None);
        self
    }

    /// Organization the contact works for.
    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.organization = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `organization`, clearing it.
    pub fn clear_organization(mut self) -> Self {
        self.organization = Some(None);
        self
    }

    /// Main contact.
    pub fn contact_name(mut self, value: impl Into<String>) -> Self {
        self.contact_name = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `contactName`, clearing it.
    pub fn clear_contact_name(mut self) -> Self {
        self.contact_name = Some(None);
        self
    }

    /// Contact's job title.
    pub fn contact_title(mut self, value: impl Into<String>) -> Self {
        self.contact_title = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `contactTitle`, clearing it.
    pub fn clear_contact_title(mut self) -> Self {
        self.contact_title = Some(None);
        self
    }

    /// Contact's phone number.
    pub fn contact_phone(mut self, value: impl Into<String>) -> Self {
        self.contact_phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `contactPhone`, clearing it.
    pub fn clear_contact_phone(mut self) -> Self {
        self.contact_phone = Some(None);
        self
    }

    /// Contact's email address.
    pub fn contact_email(mut self, value: impl Into<String>) -> Self {
        self.contact_email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `contactEmail`, clearing it.
    pub fn clear_contact_email(mut self) -> Self {
        self.contact_email = Some(None);
        self
    }

    /// Street address.
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `address`, clearing it.
    pub fn clear_address(mut self) -> Self {
        self.address = Some(None);
        self
    }

    /// City.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `city`, clearing it.
    pub fn clear_city(mut self) -> Self {
        self.city = Some(None);
        self
    }

    /// State, as a two-letter code.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `state`, clearing it.
    pub fn clear_state(mut self) -> Self {
        self.state = Some(None);
        self
    }

    /// ZIP code.
    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `zipCode`, clearing it.
    pub fn clear_zip_code(mut self) -> Self {
        self.zip_code = Some(None);
        self
    }

    /// Notes.
    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `notes`, clearing it.
    pub fn clear_notes(mut self) -> Self {
        self.notes = Some(None);
        self
    }

    /// Whether you still receive referrals from this source.
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(Some(value));
        self
    }

    /// Sends `null` for `isActive`, clearing it.
    pub fn clear_is_active(mut self) -> Self {
        self.is_active = Some(None);
        self
    }
}

/// Confirmation that a referral source was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReferralSourceDeleted {
    /// Always `referral_source`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted referral source.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Individual referrals of a prospective client, tracked from first contact to admission. Every
/// field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Referral {
    /// String representing the object's type. Always `referral`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the referral.
    #[serde(default)]
    pub id: String,
    /// Who made the referral.
    #[serde(
        rename = "referralSourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub referral_source_id: Option<String>,
    /// The patient record, once one exists.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// Name of the person referred.
    #[serde(
        rename = "patientName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub patient_name: Option<String>,
    /// Phone number to reach the person or their family.
    #[serde(
        rename = "contactPhone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_phone: Option<String>,
    /// Email address to reach the person or their family.
    #[serde(
        rename = "contactEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_email: Option<String>,
    /// When the referral arrived.
    #[serde(
        rename = "referralDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub referral_date: Option<String>,
    /// Where the referral stands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReferralStatus>,
    /// Care the person is expected to need.
    #[serde(
        rename = "estimatedCareNeeds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_care_needs: Option<String>,
    /// Expected hours of care per week.
    #[serde(
        rename = "estimatedWeeklyHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_weekly_hours: Option<f64>,
    /// How soon care is needed.
    #[serde(
        rename = "urgencyLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub urgency_level: Option<String>,
    /// Why the referral was declined or lost.
    #[serde(
        rename = "declineReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub decline_reason: Option<String>,
    /// Notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// When the person was admitted as a client.
    #[serde(
        rename = "convertedDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub converted_date: Option<String>,
    /// When the referral was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the referral was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new referral. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ReferralCreate {
    /// Who made the referral.
    #[serde(rename = "referralSourceId", skip_serializing_if = "Option::is_none")]
    pub referral_source_id: Option<Option<String>>,
    /// The patient record, once one exists.
    #[serde(rename = "patientId", skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<Option<String>>,
    /// Name of the person referred.
    #[serde(rename = "patientName")]
    pub patient_name: String,
    /// Phone number to reach the person or their family.
    #[serde(rename = "contactPhone", skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<Option<String>>,
    /// Email address to reach the person or their family.
    #[serde(rename = "contactEmail", skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<Option<String>>,
    /// When the referral arrived.
    #[serde(rename = "referralDate", skip_serializing_if = "Option::is_none")]
    pub referral_date: Option<Option<String>>,
    /// Where the referral stands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<ReferralStatus>>,
    /// Care the person is expected to need.
    #[serde(rename = "estimatedCareNeeds", skip_serializing_if = "Option::is_none")]
    pub estimated_care_needs: Option<Option<String>>,
    /// Expected hours of care per week.
    #[serde(
        rename = "estimatedWeeklyHours",
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_weekly_hours: Option<Option<f64>>,
    /// How soon care is needed.
    #[serde(rename = "urgencyLevel", skip_serializing_if = "Option::is_none")]
    pub urgency_level: Option<Option<String>>,
    /// Why the referral was declined or lost.
    #[serde(rename = "declineReason", skip_serializing_if = "Option::is_none")]
    pub decline_reason: Option<Option<String>>,
    /// Notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
}

impl ReferralCreate {
    pub fn new(patient_name: impl Into<String>) -> Self {
        Self {
            referral_source_id: None,
            patient_id: None,
            patient_name: patient_name.into(),
            contact_phone: None,
            contact_email: None,
            referral_date: None,
            status: None,
            estimated_care_needs: None,
            estimated_weekly_hours: None,
            urgency_level: None,
            decline_reason: None,
            notes: None,
        }
    }

    /// Who made the referral.
    pub fn referral_source_id(mut self, value: impl Into<String>) -> Self {
        self.referral_source_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `referralSourceId`, clearing it.
    pub fn clear_referral_source_id(mut self) -> Self {
        self.referral_source_id = Some(None);
        self
    }

    /// The patient record, once one exists.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `patientId`, clearing it.
    pub fn clear_patient_id(mut self) -> Self {
        self.patient_id = Some(None);
        self
    }

    /// Phone number to reach the person or their family.
    pub fn contact_phone(mut self, value: impl Into<String>) -> Self {
        self.contact_phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `contactPhone`, clearing it.
    pub fn clear_contact_phone(mut self) -> Self {
        self.contact_phone = Some(None);
        self
    }

    /// Email address to reach the person or their family.
    pub fn contact_email(mut self, value: impl Into<String>) -> Self {
        self.contact_email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `contactEmail`, clearing it.
    pub fn clear_contact_email(mut self) -> Self {
        self.contact_email = Some(None);
        self
    }

    /// When the referral arrived.
    pub fn referral_date(mut self, value: impl Into<String>) -> Self {
        self.referral_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `referralDate`, clearing it.
    pub fn clear_referral_date(mut self) -> Self {
        self.referral_date = Some(None);
        self
    }

    /// Where the referral stands.
    pub fn status(mut self, value: ReferralStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Care the person is expected to need.
    pub fn estimated_care_needs(mut self, value: impl Into<String>) -> Self {
        self.estimated_care_needs = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `estimatedCareNeeds`, clearing it.
    pub fn clear_estimated_care_needs(mut self) -> Self {
        self.estimated_care_needs = Some(None);
        self
    }

    /// Expected hours of care per week.
    pub fn estimated_weekly_hours(mut self, value: f64) -> Self {
        self.estimated_weekly_hours = Some(Some(value));
        self
    }

    /// Sends `null` for `estimatedWeeklyHours`, clearing it.
    pub fn clear_estimated_weekly_hours(mut self) -> Self {
        self.estimated_weekly_hours = Some(None);
        self
    }

    /// How soon care is needed.
    pub fn urgency_level(mut self, value: impl Into<String>) -> Self {
        self.urgency_level = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `urgencyLevel`, clearing it.
    pub fn clear_urgency_level(mut self) -> Self {
        self.urgency_level = Some(None);
        self
    }

    /// Why the referral was declined or lost.
    pub fn decline_reason(mut self, value: impl Into<String>) -> Self {
        self.decline_reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `declineReason`, clearing it.
    pub fn clear_decline_reason(mut self) -> Self {
        self.decline_reason = Some(None);
        self
    }

    /// Notes.
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

/// Fields to change on a referral. Send only the fields to change; any other field is refused with
/// `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct ReferralUpdate {
    /// Who made the referral.
    #[serde(rename = "referralSourceId", skip_serializing_if = "Option::is_none")]
    pub referral_source_id: Option<Option<String>>,
    /// The patient record, once one exists.
    #[serde(rename = "patientId", skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<Option<String>>,
    /// Name of the person referred.
    #[serde(rename = "patientName", skip_serializing_if = "Option::is_none")]
    pub patient_name: Option<String>,
    /// Phone number to reach the person or their family.
    #[serde(rename = "contactPhone", skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<Option<String>>,
    /// Email address to reach the person or their family.
    #[serde(rename = "contactEmail", skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<Option<String>>,
    /// When the referral arrived.
    #[serde(rename = "referralDate", skip_serializing_if = "Option::is_none")]
    pub referral_date: Option<Option<String>>,
    /// Where the referral stands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<ReferralStatus>>,
    /// Care the person is expected to need.
    #[serde(rename = "estimatedCareNeeds", skip_serializing_if = "Option::is_none")]
    pub estimated_care_needs: Option<Option<String>>,
    /// Expected hours of care per week.
    #[serde(
        rename = "estimatedWeeklyHours",
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_weekly_hours: Option<Option<f64>>,
    /// How soon care is needed.
    #[serde(rename = "urgencyLevel", skip_serializing_if = "Option::is_none")]
    pub urgency_level: Option<Option<String>>,
    /// Why the referral was declined or lost.
    #[serde(rename = "declineReason", skip_serializing_if = "Option::is_none")]
    pub decline_reason: Option<Option<String>>,
    /// Notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
}

impl ReferralUpdate {
    pub fn new() -> Self {
        Self {
            referral_source_id: None,
            patient_id: None,
            patient_name: None,
            contact_phone: None,
            contact_email: None,
            referral_date: None,
            status: None,
            estimated_care_needs: None,
            estimated_weekly_hours: None,
            urgency_level: None,
            decline_reason: None,
            notes: None,
        }
    }

    /// Who made the referral.
    pub fn referral_source_id(mut self, value: impl Into<String>) -> Self {
        self.referral_source_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `referralSourceId`, clearing it.
    pub fn clear_referral_source_id(mut self) -> Self {
        self.referral_source_id = Some(None);
        self
    }

    /// The patient record, once one exists.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `patientId`, clearing it.
    pub fn clear_patient_id(mut self) -> Self {
        self.patient_id = Some(None);
        self
    }

    /// Name of the person referred.
    pub fn patient_name(mut self, value: impl Into<String>) -> Self {
        self.patient_name = Some(value.into());
        self
    }

    /// Phone number to reach the person or their family.
    pub fn contact_phone(mut self, value: impl Into<String>) -> Self {
        self.contact_phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `contactPhone`, clearing it.
    pub fn clear_contact_phone(mut self) -> Self {
        self.contact_phone = Some(None);
        self
    }

    /// Email address to reach the person or their family.
    pub fn contact_email(mut self, value: impl Into<String>) -> Self {
        self.contact_email = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `contactEmail`, clearing it.
    pub fn clear_contact_email(mut self) -> Self {
        self.contact_email = Some(None);
        self
    }

    /// When the referral arrived.
    pub fn referral_date(mut self, value: impl Into<String>) -> Self {
        self.referral_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `referralDate`, clearing it.
    pub fn clear_referral_date(mut self) -> Self {
        self.referral_date = Some(None);
        self
    }

    /// Where the referral stands.
    pub fn status(mut self, value: ReferralStatus) -> Self {
        self.status = Some(Some(value));
        self
    }

    /// Sends `null` for `status`, clearing it.
    pub fn clear_status(mut self) -> Self {
        self.status = Some(None);
        self
    }

    /// Care the person is expected to need.
    pub fn estimated_care_needs(mut self, value: impl Into<String>) -> Self {
        self.estimated_care_needs = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `estimatedCareNeeds`, clearing it.
    pub fn clear_estimated_care_needs(mut self) -> Self {
        self.estimated_care_needs = Some(None);
        self
    }

    /// Expected hours of care per week.
    pub fn estimated_weekly_hours(mut self, value: f64) -> Self {
        self.estimated_weekly_hours = Some(Some(value));
        self
    }

    /// Sends `null` for `estimatedWeeklyHours`, clearing it.
    pub fn clear_estimated_weekly_hours(mut self) -> Self {
        self.estimated_weekly_hours = Some(None);
        self
    }

    /// How soon care is needed.
    pub fn urgency_level(mut self, value: impl Into<String>) -> Self {
        self.urgency_level = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `urgencyLevel`, clearing it.
    pub fn clear_urgency_level(mut self) -> Self {
        self.urgency_level = Some(None);
        self
    }

    /// Why the referral was declined or lost.
    pub fn decline_reason(mut self, value: impl Into<String>) -> Self {
        self.decline_reason = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `declineReason`, clearing it.
    pub fn clear_decline_reason(mut self) -> Self {
        self.decline_reason = Some(None);
        self
    }

    /// Notes.
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

/// Confirmation that a referral was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReferralDeleted {
    /// Always `referral`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted referral.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Invoices billed to patients and their families for private-pay care. Read-only. Every field is
/// always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Invoice {
    /// String representing the object's type. Always `invoice`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the invoice.
    #[serde(default)]
    pub id: String,
    /// Invoice number shown to the patient.
    #[serde(
        rename = "invoiceNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invoice_number: Option<String>,
    /// The patient billed.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// Payment status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoiceStatus>,
    /// Date of the invoice.
    #[serde(
        rename = "invoiceDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invoice_date: Option<String>,
    /// Date payment is due.
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// First day of service billed.
    #[serde(
        rename = "periodStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub period_start: Option<String>,
    /// Last day of service billed.
    #[serde(rename = "periodEnd", default, skip_serializing_if = "Option::is_none")]
    pub period_end: Option<String>,
    /// Description shown on the invoice.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Three-letter currency code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Total before tax, discounts and credits. In cents.
    #[serde(
        rename = "subtotalCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subtotal_cents: Option<i64>,
    /// Tax. In cents.
    #[serde(rename = "taxCents", default, skip_serializing_if = "Option::is_none")]
    pub tax_cents: Option<i64>,
    /// Discounts applied. In cents.
    #[serde(
        rename = "discountCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub discount_cents: Option<i64>,
    /// Account credit applied. In cents.
    #[serde(
        rename = "creditAppliedCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credit_applied_cents: Option<i64>,
    /// Invoice total. In cents.
    #[serde(
        rename = "totalCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_cents: Option<i64>,
    /// Amount paid so far. In cents.
    #[serde(
        rename = "amountPaidCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub amount_paid_cents: Option<i64>,
    /// Amount still owed. In cents.
    #[serde(
        rename = "amountDueCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub amount_due_cents: Option<i64>,
    /// Visits billed on this invoice.
    #[serde(rename = "visitIds", default, skip_serializing_if = "Option::is_none")]
    pub visit_ids: Option<Vec<String>>,
    /// Whether AveeCare created the invoice automatically when a visit was completed.
    #[serde(
        rename = "isAutoBilled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_auto_billed: Option<bool>,
    /// When the invoice was sent.
    #[serde(rename = "sentAt", default, skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    /// When the invoice was paid in full.
    #[serde(rename = "paidDate", default, skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<String>,
    /// When the invoice was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the invoice was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// The individual charges on an invoice. Read-only. Every field is always present; a field with no
/// value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvoiceLineItem {
    /// String representing the object's type. Always `invoice_line_item`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the invoice line item.
    #[serde(default)]
    pub id: String,
    /// The invoice this line belongs to.
    #[serde(rename = "invoiceId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Line description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Quantity, such as hours.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    /// Price per unit. In cents.
    #[serde(
        rename = "unitPriceCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unit_price_cents: Option<i64>,
    /// Line total. In cents.
    #[serde(
        rename = "totalCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_cents: Option<i64>,
    /// Service code.
    #[serde(
        rename = "serviceCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_code: Option<String>,
    /// Service category.
    #[serde(
        rename = "serviceCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_category: Option<String>,
    /// The visit this line bills.
    #[serde(rename = "visitId", default, skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<String>,
    /// Date of the visit.
    #[serde(rename = "visitDate", default, skip_serializing_if = "Option::is_none")]
    pub visit_date: Option<String>,
    /// Position of the line on the invoice.
    #[serde(rename = "sortOrder", default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
    /// When the invoice line item was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the invoice line item was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Insurance claims billed to Medicaid, Medicare and other payers, with their adjudication status.
/// Read-only. Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Claim {
    /// String representing the object's type. Always `claim`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the claim.
    #[serde(default)]
    pub id: String,
    /// The patient the claim is for.
    #[serde(rename = "patientId", default, skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    /// The caregiver who provided the service.
    #[serde(
        rename = "caregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_id: Option<String>,
    /// The visit billed, for single-visit claims.
    #[serde(rename = "visitId", default, skip_serializing_if = "Option::is_none")]
    pub visit_id: Option<String>,
    /// The related invoice, if any.
    #[serde(rename = "invoiceId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Your claim number.
    #[serde(
        rename = "claimNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub claim_number: Option<String>,
    /// Claim format: professional (837P) or institutional (837I).
    #[serde(rename = "claimType", default, skip_serializing_if = "Option::is_none")]
    pub claim_type: Option<ClaimClaimType>,
    /// The payer billed.
    #[serde(rename = "payerName", default, skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<String>,
    /// The payer's electronic payer ID.
    #[serde(rename = "payerId", default, skip_serializing_if = "Option::is_none")]
    pub payer_id: Option<String>,
    /// First date of service, as `YYYY-MM-DD`.
    #[serde(
        rename = "serviceDateStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_date_start: Option<String>,
    /// Last date of service, as `YYYY-MM-DD`.
    #[serde(
        rename = "serviceDateEnd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_date_end: Option<String>,
    /// Total billed. In cents.
    #[serde(
        rename = "totalChargeAmountCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_charge_amount_cents: Option<i64>,
    /// Where the claim is in adjudication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ClaimStatus>,
    /// Latest status detail from the payer or clearinghouse.
    #[serde(
        rename = "statusMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_message: Option<String>,
    /// When the status last changed, ISO 8601.
    #[serde(
        rename = "statusUpdatedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_updated_at: Option<String>,
    /// Amount the payer paid. In cents.
    #[serde(
        rename = "paidAmountCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub paid_amount_cents: Option<i64>,
    /// Amount the patient owes after the payer. In cents.
    #[serde(
        rename = "patientResponsibilityCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub patient_responsibility_cents: Option<i64>,
    /// Authorization number sent on the claim.
    #[serde(
        rename = "priorAuthNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prior_auth_number: Option<String>,
    /// Payment check or EFT trace number.
    #[serde(
        rename = "checkNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub check_number: Option<String>,
    /// When the claim was submitted, ISO 8601.
    #[serde(
        rename = "submittedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub submitted_at: Option<String>,
    /// When the claim was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the claim was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// One caregiver's hours and pay for one pay period. Read-only. Every field is always present; a
/// field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PayrollEntry {
    /// String representing the object's type. Always `payroll_entry`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the payroll entry.
    #[serde(default)]
    pub id: String,
    /// The caregiver paid.
    #[serde(
        rename = "caregiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub caregiver_id: Option<String>,
    /// First day of the pay period.
    #[serde(
        rename = "payPeriodStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pay_period_start: Option<String>,
    /// Last day of the pay period.
    #[serde(
        rename = "payPeriodEnd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pay_period_end: Option<String>,
    /// Where the entry is in your payroll process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PayrollEntryStatus>,
    /// Regular hours.
    #[serde(
        rename = "regularHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub regular_hours: Option<f64>,
    /// Overtime hours.
    #[serde(
        rename = "overtimeHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub overtime_hours: Option<f64>,
    /// Holiday hours.
    #[serde(
        rename = "holidayHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub holiday_hours: Option<f64>,
    /// Paid time off hours.
    #[serde(rename = "ptoHours", default, skip_serializing_if = "Option::is_none")]
    pub pto_hours: Option<f64>,
    /// Sick leave hours.
    #[serde(
        rename = "sickLeaveHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sick_leave_hours: Option<f64>,
    /// Gross pay. In cents.
    #[serde(
        rename = "grossPayCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gross_pay_cents: Option<i64>,
    /// Total deductions. In cents.
    #[serde(
        rename = "deductionsCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deductions_cents: Option<i64>,
    /// Net pay. In cents.
    #[serde(
        rename = "netPayCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub net_pay_cents: Option<i64>,
    /// Mileage reimbursement. In cents.
    #[serde(
        rename = "mileageCents",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mileage_cents: Option<i64>,
    /// Visits in the pay period.
    #[serde(
        rename = "visitCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visit_count: Option<i64>,
    /// Visits paid on this entry.
    #[serde(rename = "visitIds", default, skip_serializing_if = "Option::is_none")]
    pub visit_ids: Option<Vec<String>>,
    /// How the caregiver is paid.
    #[serde(
        rename = "paymentMethod",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_method: Option<PayrollEntryPaymentMethod>,
    /// Who approved the entry.
    #[serde(
        rename = "approvedByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approved_by_name: Option<String>,
    /// When it was approved.
    #[serde(
        rename = "approvedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approved_at: Option<String>,
    /// When it was paid.
    #[serde(rename = "paidAt", default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    /// Notes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// When the payroll entry was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the payroll entry was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// In-app message threads between people at your agency: one-to-one chats and named group chats.
/// Read-only; start conversations in AveeCare. Every field is always present; a field with no value
/// is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Conversation {
    /// String representing the object's type. Always `conversation`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the conversation.
    #[serde(default)]
    pub id: String,
    /// User IDs of everyone in the conversation.
    #[serde(
        rename = "participantIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub participant_ids: Option<Vec<String>>,
    /// Whether this is a group chat.
    #[serde(rename = "isGroup", default, skip_serializing_if = "Option::is_none")]
    pub is_group: Option<bool>,
    /// Group chat name; `null` for one-to-one chats.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// When the most recent message was sent.
    #[serde(
        rename = "lastMessageTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_message_time: Option<String>,
    /// Preview of the most recent message.
    #[serde(
        rename = "lastMessageContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_message_content: Option<String>,
    /// User ID of whoever sent the most recent message.
    #[serde(
        rename = "lastMessageSenderId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_message_sender_id: Option<String>,
    /// When the conversation was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the conversation was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Messages in a conversation. Messages you send through the API are sent as the API key's user.
/// Every field is always present; a field with no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Message {
    /// String representing the object's type. Always `message`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the message.
    #[serde(default)]
    pub id: String,
    /// The conversation the message belongs to.
    #[serde(
        rename = "conversationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conversation_id: Option<String>,
    /// Message text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// User ID of the sender. Set to the API key's user.
    #[serde(rename = "senderId", default, skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    /// When the message was sent.
    #[serde(rename = "sentAt", default, skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    /// When the message was last edited.
    #[serde(rename = "editedAt", default, skip_serializing_if = "Option::is_none")]
    pub edited_at: Option<String>,
    /// Whether the sender deleted the message. Deleted messages keep their place in the thread.
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// When the message was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the message was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new message. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct MessageCreate {
    /// The conversation the message belongs to.
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    /// Message text.
    pub content: String,
}

impl MessageCreate {
    pub fn new(conversation_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            conversation_id: conversation_id.into(),
            content: content.into(),
        }
    }
}

/// Alerts on your agency's alert board, such as a credential about to expire or a missed clock-in,
/// and announcements staff post for caregivers. Every field is always present; a field with no
/// value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Alert {
    /// String representing the object's type. Always `alert`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the alert.
    #[serde(default)]
    pub id: String,
    /// Headline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// How prominently the alert is shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AlertType>,
    /// Who sees the alert. `Caregiver`, `Patient`, `Visit` and `PatientCareTeam` need
    /// `audienceTargetId`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<AlertAudience>,
    /// The caregiver, patient or visit the alert is for, when the audience names one.
    #[serde(
        rename = "audienceTargetId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audience_target_id: Option<String>,
    /// When the alert takes effect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// When the alert should be reviewed or cleared.
    #[serde(
        rename = "reviewDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub review_date: Option<String>,
    /// When the alert was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the alert was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new alert. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AlertCreate {
    /// Headline.
    pub name: String,
    /// Details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// How prominently the alert is shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<AlertType>>,
    /// Who sees the alert. `Caregiver`, `Patient`, `Visit` and `PatientCareTeam` need
    /// `audienceTargetId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Option<AlertAudience>>,
    /// The caregiver, patient or visit the alert is for, when the audience names one.
    #[serde(rename = "audienceTargetId", skip_serializing_if = "Option::is_none")]
    pub audience_target_id: Option<Option<String>>,
    /// When the alert takes effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<Option<String>>,
    /// When the alert should be reviewed or cleared.
    #[serde(rename = "reviewDate", skip_serializing_if = "Option::is_none")]
    pub review_date: Option<Option<String>>,
}

impl AlertCreate {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            r#type: None,
            audience: None,
            audience_target_id: None,
            time: None,
            review_date: None,
        }
    }

    /// Details.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `description`, clearing it.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }

    /// How prominently the alert is shown.
    pub fn r#type(mut self, value: AlertType) -> Self {
        self.r#type = Some(Some(value));
        self
    }

    /// Sends `null` for `type`, clearing it.
    pub fn clear_type(mut self) -> Self {
        self.r#type = Some(None);
        self
    }

    /// Who sees the alert. `Caregiver`, `Patient`, `Visit` and `PatientCareTeam` need
    /// `audienceTargetId`.
    pub fn audience(mut self, value: AlertAudience) -> Self {
        self.audience = Some(Some(value));
        self
    }

    /// Sends `null` for `audience`, clearing it.
    pub fn clear_audience(mut self) -> Self {
        self.audience = Some(None);
        self
    }

    /// The caregiver, patient or visit the alert is for, when the audience names one.
    pub fn audience_target_id(mut self, value: impl Into<String>) -> Self {
        self.audience_target_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `audienceTargetId`, clearing it.
    pub fn clear_audience_target_id(mut self) -> Self {
        self.audience_target_id = Some(None);
        self
    }

    /// When the alert takes effect.
    pub fn time(mut self, value: impl Into<String>) -> Self {
        self.time = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `time`, clearing it.
    pub fn clear_time(mut self) -> Self {
        self.time = Some(None);
        self
    }

    /// When the alert should be reviewed or cleared.
    pub fn review_date(mut self, value: impl Into<String>) -> Self {
        self.review_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `reviewDate`, clearing it.
    pub fn clear_review_date(mut self) -> Self {
        self.review_date = Some(None);
        self
    }
}

/// Fields to change on an alert. Send only the fields to change; any other field is refused with
/// `unknown_field`. `time` is set when the alert is created and cannot be changed afterwards.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct AlertUpdate {
    /// Headline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// How prominently the alert is shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<AlertType>>,
    /// Who sees the alert. `Caregiver`, `Patient`, `Visit` and `PatientCareTeam` need
    /// `audienceTargetId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Option<AlertAudience>>,
    /// The caregiver, patient or visit the alert is for, when the audience names one.
    #[serde(rename = "audienceTargetId", skip_serializing_if = "Option::is_none")]
    pub audience_target_id: Option<Option<String>>,
    /// When the alert should be reviewed or cleared.
    #[serde(rename = "reviewDate", skip_serializing_if = "Option::is_none")]
    pub review_date: Option<Option<String>>,
}

impl AlertUpdate {
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            r#type: None,
            audience: None,
            audience_target_id: None,
            review_date: None,
        }
    }

    /// Headline.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Details.
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `description`, clearing it.
    pub fn clear_description(mut self) -> Self {
        self.description = Some(None);
        self
    }

    /// How prominently the alert is shown.
    pub fn r#type(mut self, value: AlertType) -> Self {
        self.r#type = Some(Some(value));
        self
    }

    /// Sends `null` for `type`, clearing it.
    pub fn clear_type(mut self) -> Self {
        self.r#type = Some(None);
        self
    }

    /// Who sees the alert. `Caregiver`, `Patient`, `Visit` and `PatientCareTeam` need
    /// `audienceTargetId`.
    pub fn audience(mut self, value: AlertAudience) -> Self {
        self.audience = Some(Some(value));
        self
    }

    /// Sends `null` for `audience`, clearing it.
    pub fn clear_audience(mut self) -> Self {
        self.audience = Some(None);
        self
    }

    /// The caregiver, patient or visit the alert is for, when the audience names one.
    pub fn audience_target_id(mut self, value: impl Into<String>) -> Self {
        self.audience_target_id = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `audienceTargetId`, clearing it.
    pub fn clear_audience_target_id(mut self) -> Self {
        self.audience_target_id = Some(None);
        self
    }

    /// When the alert should be reviewed or cleared.
    pub fn review_date(mut self, value: impl Into<String>) -> Self {
        self.review_date = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `reviewDate`, clearing it.
    pub fn clear_review_date(mut self) -> Self {
        self.review_date = Some(None);
        self
    }
}

/// Confirmation that an alert was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AlertDeleted {
    /// Always `alert`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted alert.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// Your agency's offices (branches). Patients, caregivers and applicants can each belong to one
/// office. There is always exactly one default office. Every field is always present; a field with
/// no value is `null`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Office {
    /// String representing the object's type. Always `office`.
    #[serde(default)]
    pub object: String,
    /// Unique identifier of the office.
    #[serde(default)]
    pub id: String,
    /// Office name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Street address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// City.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State, as a two-letter code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ZIP code.
    #[serde(rename = "zipCode", default, skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// Main phone number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Whether this is the default office. Making another office the default clears this one.
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// When the office was created.
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the office was last changed.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Fields for a new office. Any other field is refused with `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct OfficeCreate {
    /// Office name.
    pub name: String,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<String>>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// State, as a two-letter code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    /// ZIP code.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<Option<String>>,
    /// Main phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Option<String>>,
    /// Whether this is the default office. Making another office the default clears this one.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<Option<bool>>,
}

impl OfficeCreate {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            address: None,
            city: None,
            state: None,
            zip_code: None,
            phone: None,
            is_default: None,
        }
    }

    /// Street address.
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `address`, clearing it.
    pub fn clear_address(mut self) -> Self {
        self.address = Some(None);
        self
    }

    /// City.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `city`, clearing it.
    pub fn clear_city(mut self) -> Self {
        self.city = Some(None);
        self
    }

    /// State, as a two-letter code.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `state`, clearing it.
    pub fn clear_state(mut self) -> Self {
        self.state = Some(None);
        self
    }

    /// ZIP code.
    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `zipCode`, clearing it.
    pub fn clear_zip_code(mut self) -> Self {
        self.zip_code = Some(None);
        self
    }

    /// Main phone number.
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `phone`, clearing it.
    pub fn clear_phone(mut self) -> Self {
        self.phone = Some(None);
        self
    }

    /// Whether this is the default office. Making another office the default clears this one.
    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(Some(value));
        self
    }

    /// Sends `null` for `isDefault`, clearing it.
    pub fn clear_is_default(mut self) -> Self {
        self.is_default = Some(None);
        self
    }
}

/// Fields to change on an office. Send only the fields to change; any other field is refused with
/// `unknown_field`.
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
#[non_exhaustive]
pub struct OfficeUpdate {
    /// Office name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<String>>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Option<String>>,
    /// State, as a two-letter code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Option<String>>,
    /// ZIP code.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<Option<String>>,
    /// Main phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Option<String>>,
    /// Whether this is the default office. Making another office the default clears this one.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<Option<bool>>,
}

impl OfficeUpdate {
    pub fn new() -> Self {
        Self {
            name: None,
            address: None,
            city: None,
            state: None,
            zip_code: None,
            phone: None,
            is_default: None,
        }
    }

    /// Office name.
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Street address.
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `address`, clearing it.
    pub fn clear_address(mut self) -> Self {
        self.address = Some(None);
        self
    }

    /// City.
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `city`, clearing it.
    pub fn clear_city(mut self) -> Self {
        self.city = Some(None);
        self
    }

    /// State, as a two-letter code.
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `state`, clearing it.
    pub fn clear_state(mut self) -> Self {
        self.state = Some(None);
        self
    }

    /// ZIP code.
    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `zipCode`, clearing it.
    pub fn clear_zip_code(mut self) -> Self {
        self.zip_code = Some(None);
        self
    }

    /// Main phone number.
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(Some(value.into()));
        self
    }

    /// Sends `null` for `phone`, clearing it.
    pub fn clear_phone(mut self) -> Self {
        self.phone = Some(None);
        self
    }

    /// Whether this is the default office. Making another office the default clears this one.
    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(Some(value));
        self
    }

    /// Sends `null` for `isDefault`, clearing it.
    pub fn clear_is_default(mut self) -> Self {
        self.is_default = Some(None);
        self
    }
}

/// Confirmation that an office was deleted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OfficeDeleted {
    /// Always `office`.
    #[serde(default)]
    pub object: String,
    /// ID of the deleted office.
    #[serde(default)]
    pub id: String,
    /// Always `true`.
    #[serde(default)]
    pub deleted: bool,
}

/// The API key making the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Me {
    /// Always `me`.
    #[serde(default)]
    pub object: String,
    /// The key's public identifier: the part of the key between `ak_` and the second underscore.
    #[serde(default)]
    pub id: String,
    /// The name given to the key when it was created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// What the key may do. A key without `write` can only read.
    #[serde(default)]
    pub scopes: Vec<MeScope>,
    /// Role of the person the key belongs to: `RootUser` (agency administrator), `User` (office
    /// staff), `Caregiver` or `Patient`. It decides which resources the key can reach.
    pub role: MeRole,
    /// Email address of the person the key belongs to.
    #[serde(default)]
    pub email: String,
    /// When the key stops working.
    #[serde(rename = "expiresAt", default)]
    pub expires_at: String,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Query parameters for `patients.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct PatientListParams {
    /// Only return patients with this status.
    pub status: Option<String>,
    /// Only return active (`true`) or inactive (`false`) patients.
    pub active: Option<bool>,
    /// Only return prospects or clients.
    pub lifecycle_stage: Option<PatientLifecycleStage>,
    /// Only return patients served by this office.
    pub office_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl PatientListParams {
    pub fn new() -> Self {
        Self {
            status: None,
            active: None,
            lifecycle_stage: None,
            office_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return patients with this status.
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Only return active (`true`) or inactive (`false`) patients.
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    /// Only return prospects or clients.
    pub fn lifecycle_stage(mut self, value: PatientLifecycleStage) -> Self {
        self.lifecycle_stage = Some(value);
        self
    }

    /// Only return patients served by this office.
    pub fn office_id(mut self, value: impl Into<String>) -> Self {
        self.office_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.status {
            query.push(("status", value.clone()));
        }
        if let Some(value) = &self.active {
            query.push(("active", value.to_string()));
        }
        if let Some(value) = &self.lifecycle_stage {
            query.push(("lifecycle_stage", value.as_str().to_owned()));
        }
        if let Some(value) = &self.office_id {
            query.push(("office_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `contacts.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct ContactListParams {
    /// Only return contacts of this patient.
    pub patient_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl ContactListParams {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return contacts of this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `allergies.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct AllergyListParams {
    /// Only return allergies of this patient.
    pub patient_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl AllergyListParams {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return allergies of this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `medications.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct MedicationListParams {
    /// Only return medications of this patient.
    pub patient_id: Option<String>,
    /// Only return medications with this status.
    pub status: Option<MedicationStatus>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl MedicationListParams {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            status: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return medications of this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return medications with this status.
    pub fn status(mut self, value: MedicationStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.status {
            query.push(("status", value.as_str().to_owned()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `careGoals.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct CareGoalListParams {
    /// Only return goals of this patient.
    pub patient_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl CareGoalListParams {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return goals of this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `patientNotes.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct PatientNoteListParams {
    /// Only return notes about this patient.
    pub patient_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl PatientNoteListParams {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return notes about this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `medicationAdministrations.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct MedicationAdministrationListParams {
    /// Only return doses for this patient.
    pub patient_id: Option<String>,
    /// Only return doses of this medication.
    pub medication_id: Option<String>,
    /// Only return doses due on this date.
    pub scheduled_date: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl MedicationAdministrationListParams {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            medication_id: None,
            scheduled_date: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return doses for this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return doses of this medication.
    pub fn medication_id(mut self, value: impl Into<String>) -> Self {
        self.medication_id = Some(value.into());
        self
    }

    /// Only return doses due on this date.
    pub fn scheduled_date(mut self, value: impl Into<String>) -> Self {
        self.scheduled_date = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.medication_id {
            query.push(("medication_id", value.clone()));
        }
        if let Some(value) = &self.scheduled_date {
            query.push(("scheduled_date", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `oasisAssessments.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct OasisAssessmentListParams {
    /// Only return assessments of this patient.
    pub patient_id: Option<String>,
    /// Only return assessments with this status.
    pub status: Option<OasisAssessmentStatus>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl OasisAssessmentListParams {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            status: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return assessments of this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return assessments with this status.
    pub fn status(mut self, value: OasisAssessmentStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.status {
            query.push(("status", value.as_str().to_owned()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `caregivers.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct CaregiverListParams {
    /// Only return caregivers with this employment status.
    pub status: Option<CaregiverStatus>,
    /// Only return caregivers who can (`true`) or cannot (`false`) be scheduled.
    pub active: Option<bool>,
    /// Only return caregivers from this office.
    pub office_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl CaregiverListParams {
    pub fn new() -> Self {
        Self {
            status: None,
            active: None,
            office_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return caregivers with this employment status.
    pub fn status(mut self, value: CaregiverStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return caregivers who can (`true`) or cannot (`false`) be scheduled.
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    /// Only return caregivers from this office.
    pub fn office_id(mut self, value: impl Into<String>) -> Self {
        self.office_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
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
        if let Some(value) = &self.active {
            query.push(("active", value.to_string()));
        }
        if let Some(value) = &self.office_id {
            query.push(("office_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `caregiverNotes.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct CaregiverNoteListParams {
    /// Only return notes about this caregiver.
    pub caregiver_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl CaregiverNoteListParams {
    pub fn new() -> Self {
        Self {
            caregiver_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return notes about this caregiver.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.caregiver_id {
            query.push(("caregiver_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `certifications.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct CertificationListParams {
    /// Only return certifications of this caregiver.
    pub caregiver_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl CertificationListParams {
    pub fn new() -> Self {
        Self {
            caregiver_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return certifications of this caregiver.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.caregiver_id {
            query.push(("caregiver_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `availability.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct AvailabilityListParams {
    /// Only return windows of this caregiver.
    pub caregiver_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl AvailabilityListParams {
    pub fn new() -> Self {
        Self {
            caregiver_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return windows of this caregiver.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.caregiver_id {
            query.push(("caregiver_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `specializations.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct SpecializationListParams {
    /// Only return specializations of this caregiver.
    pub caregiver_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl SpecializationListParams {
    pub fn new() -> Self {
        Self {
            caregiver_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return specializations of this caregiver.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.caregiver_id {
            query.push(("caregiver_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `applicants.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct ApplicantListParams {
    /// Only return applicants in this hiring stage.
    pub stage: Option<String>,
    /// Only return applicants to this office.
    pub office_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl ApplicantListParams {
    pub fn new() -> Self {
        Self {
            stage: None,
            office_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return applicants in this hiring stage.
    pub fn stage(mut self, value: impl Into<String>) -> Self {
        self.stage = Some(value.into());
        self
    }

    /// Only return applicants to this office.
    pub fn office_id(mut self, value: impl Into<String>) -> Self {
        self.office_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.stage {
            query.push(("stage", value.clone()));
        }
        if let Some(value) = &self.office_id {
            query.push(("office_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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
    /// Start of the window (inclusive), ISO 8601 with a time zone. Visits are listed by scheduled
    /// start. Both `start_from` and `start_to` are required, and the window can be at most 93 days.
    pub start_from: String,
    /// End of the window (exclusive), ISO 8601 with a time zone. At most 93 days after
    /// `start_from`.
    pub start_to: String,
    /// Only return visits for this patient.
    pub patient_id: Option<String>,
    /// Only return visits assigned to this caregiver.
    pub caregiver_id: Option<String>,
    /// Only return visits with this status.
    pub status: Option<VisitStatus>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
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
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return visits for this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return visits assigned to this caregiver.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(value.into());
        self
    }

    /// Only return visits with this status.
    pub fn status(mut self, value: VisitStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
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
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `visitNotes.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct VisitNoteListParams {
    /// Only return notes about this visit.
    pub visit_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl VisitNoteListParams {
    pub fn new() -> Self {
        Self {
            visit_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return notes about this visit.
    pub fn visit_id(mut self, value: impl Into<String>) -> Self {
        self.visit_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.visit_id {
            query.push(("visit_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `visitRequests.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct VisitRequestListParams {
    /// Only return requests from this caregiver.
    pub caregiver_id: Option<String>,
    /// Only return requests with this status.
    pub status: Option<VisitRequestStatus>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl VisitRequestListParams {
    pub fn new() -> Self {
        Self {
            caregiver_id: None,
            status: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return requests from this caregiver.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(value.into());
        self
    }

    /// Only return requests with this status.
    pub fn status(mut self, value: VisitRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.caregiver_id {
            query.push(("caregiver_id", value.clone()));
        }
        if let Some(value) = &self.status {
            query.push(("status", value.as_str().to_owned()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `openShifts.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct OpenShiftListParams {
    /// Only return offers with this status.
    pub status: Option<OpenShiftStatus>,
    /// Only return offers for this visit.
    pub visit_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl OpenShiftListParams {
    pub fn new() -> Self {
        Self {
            status: None,
            visit_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return offers with this status.
    pub fn status(mut self, value: OpenShiftStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return offers for this visit.
    pub fn visit_id(mut self, value: impl Into<String>) -> Self {
        self.visit_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
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
        if let Some(value) = &self.visit_id {
            query.push(("visit_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `shiftSwaps.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct ShiftSwapListParams {
    /// Only return swaps with this status.
    pub status: Option<ShiftSwapStatus>,
    /// Only return swaps for this visit.
    pub visit_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl ShiftSwapListParams {
    pub fn new() -> Self {
        Self {
            status: None,
            visit_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return swaps with this status.
    pub fn status(mut self, value: ShiftSwapStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return swaps for this visit.
    pub fn visit_id(mut self, value: impl Into<String>) -> Self {
        self.visit_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
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
        if let Some(value) = &self.visit_id {
            query.push(("visit_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `carePlans.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct CarePlanListParams {
    /// Only return plans for this patient.
    pub patient_id: Option<String>,
    /// Only return plans with this status.
    pub status: Option<CarePlanStatus>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl CarePlanListParams {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            status: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return plans for this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return plans with this status.
    pub fn status(mut self, value: CarePlanStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.status {
            query.push(("status", value.as_str().to_owned()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `progressNotes.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct ProgressNoteListParams {
    /// Only return notes for this patient.
    pub patient_id: Option<String>,
    /// Only return notes for this visit.
    pub visit_id: Option<String>,
    /// Only return notes by this caregiver.
    pub caregiver_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl ProgressNoteListParams {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            visit_id: None,
            caregiver_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return notes for this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return notes for this visit.
    pub fn visit_id(mut self, value: impl Into<String>) -> Self {
        self.visit_id = Some(value.into());
        self
    }

    /// Only return notes by this caregiver.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.visit_id {
            query.push(("visit_id", value.clone()));
        }
        if let Some(value) = &self.caregiver_id {
            query.push(("caregiver_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `authorizations.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct AuthorizationListParams {
    /// Only return authorizations for this patient.
    pub patient_id: Option<String>,
    /// Only return authorizations with this status.
    pub status: Option<AuthorizationStatus>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl AuthorizationListParams {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            status: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return authorizations for this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return authorizations with this status.
    pub fn status(mut self, value: AuthorizationStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.status {
            query.push(("status", value.as_str().to_owned()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `incidents.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct IncidentListParams {
    /// Only return incidents with this follow-up status.
    pub status: Option<IncidentStatus>,
    /// Only return incidents involving this patient.
    pub patient_id: Option<String>,
    /// Only return incidents involving this caregiver.
    pub caregiver_id: Option<String>,
    /// Only return resolved (`true`) or unresolved (`false`) incidents.
    pub resolved: Option<bool>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl IncidentListParams {
    pub fn new() -> Self {
        Self {
            status: None,
            patient_id: None,
            caregiver_id: None,
            resolved: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return incidents with this follow-up status.
    pub fn status(mut self, value: IncidentStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return incidents involving this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return incidents involving this caregiver.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(value.into());
        self
    }

    /// Only return resolved (`true`) or unresolved (`false`) incidents.
    pub fn resolved(mut self, value: bool) -> Self {
        self.resolved = Some(value);
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
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
        if let Some(value) = &self.caregiver_id {
            query.push(("caregiver_id", value.clone()));
        }
        if let Some(value) = &self.resolved {
            query.push(("resolved", value.to_string()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `events.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct EventListParams {
    /// Only return events with this follow-up status.
    pub status: Option<IncidentStatus>,
    /// Only return events involving this patient.
    pub patient_id: Option<String>,
    /// Only return events involving this caregiver.
    pub caregiver_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl EventListParams {
    pub fn new() -> Self {
        Self {
            status: None,
            patient_id: None,
            caregiver_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return events with this follow-up status.
    pub fn status(mut self, value: IncidentStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return events involving this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return events involving this caregiver.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
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
        if let Some(value) = &self.caregiver_id {
            query.push(("caregiver_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `referralSources.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct ReferralSourceListParams {
    /// Only return active (`true`) or inactive (`false`) sources.
    pub is_active: Option<bool>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl ReferralSourceListParams {
    pub fn new() -> Self {
        Self {
            is_active: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return active (`true`) or inactive (`false`) sources.
    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.is_active {
            query.push(("is_active", value.to_string()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `referrals.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct ReferralListParams {
    /// Only return referrals with this status.
    pub status: Option<ReferralStatus>,
    /// Only return referrals from this source.
    pub referral_source_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl ReferralListParams {
    pub fn new() -> Self {
        Self {
            status: None,
            referral_source_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return referrals with this status.
    pub fn status(mut self, value: ReferralStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return referrals from this source.
    pub fn referral_source_id(mut self, value: impl Into<String>) -> Self {
        self.referral_source_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
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
        if let Some(value) = &self.referral_source_id {
            query.push(("referral_source_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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
    /// Only return invoices for this patient.
    pub patient_id: Option<String>,
    /// Only return invoices with this status.
    pub status: Option<InvoiceStatus>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl InvoiceListParams {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            status: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return invoices for this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return invoices with this status.
    pub fn status(mut self, value: InvoiceStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.status {
            query.push(("status", value.as_str().to_owned()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `invoiceLineItems.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct InvoiceLineItemListParams {
    /// Only return lines of this invoice.
    pub invoice_id: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl InvoiceLineItemListParams {
    pub fn new() -> Self {
        Self {
            invoice_id: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return lines of this invoice.
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.invoice_id {
            query.push(("invoice_id", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `claims.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct ClaimListParams {
    /// Only return claims for this patient.
    pub patient_id: Option<String>,
    /// Only return claims with this status.
    pub status: Option<ClaimStatus>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl ClaimListParams {
    pub fn new() -> Self {
        Self {
            patient_id: None,
            status: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return claims for this patient.
    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    /// Only return claims with this status.
    pub fn status(mut self, value: ClaimStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.patient_id {
            query.push(("patient_id", value.clone()));
        }
        if let Some(value) = &self.status {
            query.push(("status", value.as_str().to_owned()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `payrollEntries.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct PayrollEntryListParams {
    /// Only return entries for this caregiver.
    pub caregiver_id: Option<String>,
    /// Only return entries with this status.
    pub status: Option<PayrollEntryStatus>,
    /// Only return entries for the pay period starting on this date.
    pub pay_period_start: Option<String>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl PayrollEntryListParams {
    pub fn new() -> Self {
        Self {
            caregiver_id: None,
            status: None,
            pay_period_start: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return entries for this caregiver.
    pub fn caregiver_id(mut self, value: impl Into<String>) -> Self {
        self.caregiver_id = Some(value.into());
        self
    }

    /// Only return entries with this status.
    pub fn status(mut self, value: PayrollEntryStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Only return entries for the pay period starting on this date.
    pub fn pay_period_start(mut self, value: impl Into<String>) -> Self {
        self.pay_period_start = Some(value.into());
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.caregiver_id {
            query.push(("caregiver_id", value.clone()));
        }
        if let Some(value) = &self.status {
            query.push(("status", value.as_str().to_owned()));
        }
        if let Some(value) = &self.pay_period_start {
            query.push(("pay_period_start", value.clone()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `conversations.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct ConversationListParams {
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl ConversationListParams {
    pub fn new() -> Self {
        Self {
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `messages.list`.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct MessageListParams {
    /// The conversation to list messages from.
    pub conversation_id: String,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl MessageListParams {
    pub fn new(conversation_id: impl Into<String>) -> Self {
        Self {
            conversation_id: conversation_id.into(),
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        query.push(("conversation_id", self.conversation_id.clone()));
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `alerts.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct AlertListParams {
    /// Only return alerts of this type.
    pub r#type: Option<AlertType>,
    /// Only return alerts for this audience.
    pub audience: Option<AlertAudience>,
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl AlertListParams {
    pub fn new() -> Self {
        Self {
            r#type: None,
            audience: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return alerts of this type.
    pub fn r#type(mut self, value: AlertType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Only return alerts for this audience.
    pub fn audience(mut self, value: AlertAudience) -> Self {
        self.audience = Some(value);
        self
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.r#type {
            query.push(("type", value.as_str().to_owned()));
        }
        if let Some(value) = &self.audience {
            query.push(("audience", value.as_str().to_owned()));
        }
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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

/// Query parameters for `offices.list`.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct OfficeListParams {
    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub created_after: Option<String>,
    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub created_before: Option<String>,
    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub updated_after: Option<String>,
    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub updated_before: Option<String>,
    /// Number of objects to return, from 1 to 100.
    pub limit: Option<i64>,
    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub cursor: Option<String>,
}

impl OfficeListParams {
    pub fn new() -> Self {
        Self {
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            limit: None,
            cursor: None,
        }
    }

    /// Only return objects created after this time (exclusive). ISO 8601 with a time zone.
    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Only return objects created before this time (exclusive). ISO 8601 with a time zone.
    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    /// Only return objects last changed after this time (exclusive). ISO 8601 with a time zone. Use
    /// it for incremental sync: pass the newest `updatedAt` you already have.
    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    /// Only return objects last changed before this time (exclusive). ISO 8601 with a time zone.
    pub fn updated_before(mut self, value: impl Into<String>) -> Self {
        self.updated_before = Some(value.into());
        self
    }

    /// Number of objects to return, from 1 to 100.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// The `next_cursor` from the previous page. Send the same filters with it.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    #[allow(clippy::vec_init_then_push)]
    pub(crate) fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut query = Vec::new();
        if let Some(value) = &self.created_after {
            query.push(("created_after", value.clone()));
        }
        if let Some(value) = &self.created_before {
            query.push(("created_before", value.clone()));
        }
        if let Some(value) = &self.updated_after {
            query.push(("updated_after", value.clone()));
        }
        if let Some(value) = &self.updated_before {
            query.push(("updated_before", value.clone()));
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
