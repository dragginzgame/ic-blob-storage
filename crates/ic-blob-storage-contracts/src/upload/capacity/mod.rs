//! Bounded capacity observations after independent service authentication.
pub mod reply;

use crate::dto::upload::capacity::UploadCapacityResponse;

/// Report aggregate demand exceeding independently observed capacity.
///
/// Stable diagnostic codes cover fresh objects, retained chunks, the three byte
/// dimensions and one sequential upload slot. Callers count only unseen roots
/// and retain per-object, enrollment, fence and recovery checks. These sequential
/// observations neither reserve capacity nor authorize admission.
pub fn demand_blockers(
    capacity: &UploadCapacityResponse,
    objects: u64,
    bytes: u128,
    chunks: u64,
) -> impl Iterator<Item = &'static str> {
    [
        (objects > capacity.remaining_objects).then_some("object_history_capacity"),
        (chunks > capacity.remaining_manifest_chunks).then_some("manifest_capacity"),
        (bytes > capacity.remaining_logical_bytes).then_some("logical_byte_capacity"),
        (bytes > capacity.remaining_physical_bytes).then_some("physical_byte_capacity"),
        (bytes > capacity.remaining_liability_bytes).then_some("liability_byte_capacity"),
        (objects != 0 && capacity.remaining_active_uploads == 0).then_some("no_active_upload_slot"),
    ]
    .into_iter()
    .flatten()
}
