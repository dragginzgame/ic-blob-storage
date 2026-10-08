//! Trusted verifier statements about observed bytes, separate from provider durability.
use crate::dto::upload::admission::UploadAdmissionFailure;
use crate::dto::upload::admission::UploadAdmissionRequest;
use candid::CandidType;
use candid::Deserialize;
use candid::Principal;
/// One authenticated observation for the installed verifier to inspect exposed bytes.
/// No origin, credential, upload certificate or retry permission is supplied.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadVerificationPlan {
    /// Full original permission; independent identities must remain intact.
    pub permission: UploadAdmissionRequest,
    /// Explicit installed verifier, independent of uploader and operator.
    pub verifier: Principal,
    /// Installed Caffeine owner, equal to this service.
    pub owner: Principal,
    /// Installed provider project, never inferred from namespace or tenant.
    pub project: String,
    /// Original admission time; a verifier clock before this cannot attest.
    pub admitted_at_ns: u64,
    /// Original root-consistent leaves and metadata, not provider response headers.
    pub declaration: super::manifest::UploadManifestDeclaration,
}
/// Save this exact statement before sending. Changed statements cannot replace a receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadAttestationRequest {
    /// Full original permission, root, length and object/reference identities.
    pub permission: UploadAdmissionRequest,
    /// Raw SHA-256 of independently fetched and root-verified content.
    pub content_digest: [u8; 32],
    /// Verifier's observation time in IC nanoseconds, not a retention deadline.
    pub observed_at_ns: u64,
}
/// First accepted verifier evidence. Historical confirmation does not grant current liveness.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadAttestationReceipt {
    /// Exact original statement.
    pub request: UploadAttestationRequest,
    /// Authenticated installed verifier that supplied it.
    pub verifier: Principal,
    /// Service acceptance time; exact retry preserves this value.
    pub accepted_at_ns: u64,
}
/// Read-only exact permission lookup, available through restore fencing.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
#[expect(
    clippy::large_enum_variant,
    reason = "One bounded receipt; preserve allocation-free value semantics"
)]
pub enum UploadAttestationLookup {
    /// No verifier statement retained; not retry or exposure authority.
    Absent,
    /// Immutable statement, including after reference release or settlement.
    Found(UploadAttestationReceipt),
}
/// Attestation observation and the upload owner's restore fence.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadAttestationResponse {
    /// Full exact permission echo, also when evidence is absent.
    pub permission: UploadAdmissionRequest,
    /// Original receipt, never synthesized from current lifecycle state.
    pub attestation: UploadAttestationLookup,
    /// Read-only restore fence; a false value does not grant dispatch authority.
    pub fenced: bool,
}
/// Successful atomic confirmation or exact replay.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadAttestationMutation {
    /// First accepted immutable evidence.
    pub receipt: UploadAttestationReceipt,
    /// False for an exact replay; no reference is reactivated.
    pub changed: bool,
}
/// No failure authorizes a new upload or discards uncertain provider effects.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum UploadAttestationFailure {
    /// Original permission or store rejection.
    Permission(UploadAdmissionFailure),
    /// Caller is not the explicit verifier (or an allowed receipt observer).
    Denied,
    /// Upload is not exposed with a prepared manifest, or existing host-only confirmation lacks this receipt.
    Phase,
    /// A different statement was already accepted for this exact upload.
    Conflict,
    /// Observation predates admission or lies in the future.
    Observation,
}
