//! Exposure boundary failures; no certificate or provider response schema.
use crate::dto::upload::admission::UploadAdmissionFailure;
use serde::Deserialize;

/// Refused local exposure or historical permission inspection.
#[cfg_attr(
    all(feature = "wire-schema", not(target_arch = "wasm32")),
    derive(ic_wire::Wire)
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum UploadExposureFailure {
    /// Full permission, authority, activation, deadline or restore fence failed.
    Permission(UploadAdmissionFailure),
    /// No validated manifest is bound to the original permission.
    Unprepared,
    /// Local issuance permission has been withdrawn.
    Revoked,
    /// Exposure already happened or the operation/clock no longer allows it.
    Phase,
}
