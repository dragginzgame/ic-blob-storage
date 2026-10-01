//! Managed host qualification facts; installed policy does not qualify Caffeine.
use ic_blob_storage::{
    model::service::upload::UploadPermission, policy::upload::exposure::UploadExposureHostEvidence,
};

pub(crate) fn evidence(permission: UploadPermission, now: u64) -> UploadExposureHostEvidence {
    UploadExposureHostEvidence {
        permission,
        observed_at_ns: now,
        precharge_limits: false,
        provider_namespace: false,
        replay_charging: false,
        recovery_ready: false,
        // The host supports synchronous stable updates in an IC transaction.
        // Qualification remains false for both passive assessment and issuance.
        durable_commit: true,
    }
}
