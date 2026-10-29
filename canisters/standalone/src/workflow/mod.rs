//! Standalone response attribution and authenticated reporting; storage policy stays host-owned.
use crate::ops::{
    self,
    metrics::{Outcome, Started},
};
use ic_blob_storage_contracts::{
    dto::{
        configuration::HostFailure,
        gateway::sync::{GatewaySyncFailure, GatewaySyncResponse},
        reference::{ReferenceFailure, ReferenceMutationResponse},
        upload::{
            admission::{UploadAdmissionFailure, UploadAdmissionMutation},
            manifest::{UploadManifestFailure, UploadManifestMutation},
        },
    },
    standalone::{MetricOperation, MetricsReport},
};

pub(crate) fn begin(operation: MetricOperation, bytes: usize) -> Started {
    ops::metrics::begin(operation, bytes)
}
pub(crate) fn complete(started: Started, outcome: Outcome) {
    ops::metrics::complete(started, outcome);
}
pub(crate) fn registered(started: Started) {
    complete(started, Outcome::Applied);
}
pub(crate) fn acknowledged(started: Started) {
    complete(started, Outcome::Acknowledged);
}
pub(crate) fn admission(
    started: Started,
    result: &Result<UploadAdmissionMutation, UploadAdmissionFailure>,
) {
    complete(
        started,
        match result {
            Ok(value) if value.replayed => Outcome::Replayed,
            Ok(_) => Outcome::Applied,
            Err(_) => Outcome::Refused,
        },
    );
}
pub(crate) fn manifest(
    started: Started,
    result: &Result<UploadManifestMutation, UploadManifestFailure>,
) {
    complete(
        started,
        match result {
            Ok(value) if !value.changed => Outcome::Replayed,
            Ok(_) => Outcome::Applied,
            Err(_) => Outcome::Refused,
        },
    );
}
pub(crate) fn reference(
    started: Started,
    result: &Result<ReferenceMutationResponse, ReferenceFailure>,
) {
    complete(
        started,
        match result {
            Ok(value) if value.replayed => Outcome::Replayed,
            Ok(value) if value.receipt.result.is_ok() => Outcome::Applied,
            _ => Outcome::Refused,
        },
    );
}
pub(crate) fn gateway_sync(
    started: Started,
    result: &Result<GatewaySyncResponse, GatewaySyncFailure>,
) {
    complete(
        started,
        match result {
            Ok(_) => Outcome::Acknowledged,
            Err(
                GatewaySyncFailure::NotEnqueued
                | GatewaySyncFailure::Rejected(_)
                | GatewaySyncFailure::InvalidReply
                | GatewaySyncFailure::ReplyTooLarge,
            ) => Outcome::ProviderFailed,
            Err(_) => Outcome::Refused,
        },
    );
}
pub(crate) fn metrics(actor: ic_platform::Principal) -> Result<MetricsReport, HostFailure> {
    let installed = ops::Workflow::configuration(actor)?;
    Ok(ops::metrics::report(
        installed.configuration.service,
        installed.configuration.namespace,
    ))
}
