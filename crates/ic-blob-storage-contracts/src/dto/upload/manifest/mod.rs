//! Bounded upload declarations and historical recovery; no content bytes or certificates.
use crate::dto::upload::admission::UploadAdmissionFailure;
use crate::dto::upload::admission::UploadAdmissionRequest;
use candid::CandidType;
use candid::Deserialize;
/// Original metadata spelling and value; the boundary does not rewrite either.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadManifestHeader {
    /// ASCII HTTP token, unique ignoring ASCII case.
    pub name: String,
    /// Exact value, without controls or surrounding whitespace.
    pub value: String,
}
/// Ordered leaf hashes and metadata, never complete content or raw-digest evidence.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadManifestDeclaration {
    /// Ordered domain-separated chunk identities.
    pub chunks: Vec<[u8; 32]>,
    /// Requires canonical Content-Length matching the immutable reservation.
    pub headers: Vec<UploadManifestHeader>,
}
/// Save this complete intent before preparation dispatch; adapters bound ingress decoding.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadManifestRequest {
    /// Full original permission, including uploader and exclusive expiry.
    pub permission: UploadAdmissionRequest,
    /// Declaration to validate against the admitted root and length.
    pub declaration: UploadManifestDeclaration,
}
/// Explicit absence prevents incompatible Candid optional data becoming unprepared.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum UploadManifestInspection {
    /// No declaration is retained; this does not authorize repeating uncertain effects.
    Unprepared,
    /// Original accepted declaration, even after revocation, expiry or completion.
    Prepared(UploadManifestDeclaration),
}
/// Authenticated historical observation, not issuance or publication authority.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadManifestResponse {
    /// Exact original permission.
    pub permission: UploadAdmissionRequest,
    /// First accepted declaration; equivalent retries do not replace its metadata.
    pub manifest: UploadManifestInspection,
}
/// Successful local preparation, with no provider call or file transfer.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct UploadManifestMutation {
    /// Original accepted declaration, always Prepared.
    pub observation: UploadManifestResponse,
    /// False when an equivalent declaration was already bound.
    pub changed: bool,
}
/// Failed preparation/inspection; uncertainty must preserve the original intent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum UploadManifestFailure {
    /// Permission identity, authority, expiry, activation or restoration failure.
    Permission(UploadAdmissionFailure),
    /// Local issuance permission was withdrawn.
    Revoked,
    /// Preparation is no longer permitted in the current phase or clock context.
    Phase,
    /// Leaves, metadata or declared root/length are inconsistent.
    Declaration,
    /// Raw declaration exceeds configured count or byte bounds.
    Limit,
}
