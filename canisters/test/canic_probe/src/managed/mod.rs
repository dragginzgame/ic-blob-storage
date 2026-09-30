//! Maintained Canic-generated endpoints and lifecycle, separate from fixture handlers.
#[cfg(canic_export_candid)]
use super::{
    CompositionSnapshot, ManagedCallFailure, Principal, ProbeFailure, TenantEnrollmentResponse,
    TenantScope, TenantUpdateRequest,
};
use super::{canic_install, canic_setup, canic_upgrade};
canic::start!(lifecycle_participant(
    init = crate::install,
    post_upgrade = crate::restore
),);
canic::finish!();
