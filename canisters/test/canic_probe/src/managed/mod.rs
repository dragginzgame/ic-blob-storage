//! Maintained Canic-generated endpoints and lifecycle, separate from fixture handlers.
#[cfg(canic_export_candid)]
use super::api::*;
#[cfg(canic_export_candid)]
use super::dto::TransportFailure;
#[cfg(canic_export_candid)]
use super::{
    CompositionSnapshot, EnrollmentForwardFailure, ManagedCallFailure, PreparationForwardFailure,
    Principal, ProbeExposureFailure,
};
use super::{canic_install, canic_setup, canic_upgrade};
canic::start!(lifecycle_participant(
    init = crate::install,
    post_upgrade = crate::restore
),);
canic::finish!();
