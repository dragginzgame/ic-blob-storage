//! Host-owned qualification facts. Installed configuration is not provider evidence.
use ic_blob_storage::{
    model::service::upload::UploadPermission, policy::upload::exposure::UploadExposureHostEvidence,
};

pub(crate) fn evidence(permission: UploadPermission, now: u64) -> UploadExposureHostEvidence {
    UploadExposureHostEvidence {
        permission,
        observed_at_ns: now,
        // The retained upstream source specifies the root-only reply, not deployed
        // pre-charge enforcement, namespace provisioning or replay charges.
        precharge_limits: false,
        provider_namespace: false,
        replay_charging: false,
        // An unfenced local store is necessary but does not prove that all provider
        // obligations and independent old-backup freshness authority survive.
        recovery_ready: false,
        // The synchronous ingress handler propagates traps and returns only after
        // the shared stable exposure commit, in the same IC message transaction.
        durable_commit: true,
    }
}
