//! Controlled application inputs with Canic's canonical managed lifecycle.
//! This fixture is not a production endpoint artifact or provider observation.
mod api;
pub mod configuration;
mod context;
mod dto;
mod limits;
mod ops;
use candid::{CandidType, Deserialize, Principal};
pub use dto::ProbeExposureFailure;
use dto::TransportFailure;
use ic_blob_storage::{
    dto::{
        configuration::{HostFailure, ServiceConfigurationInput},
        tenant::{TenantEnrollmentResponse, TenantFailure, TenantUpdateRequest},
    },
    ic_memory::{self, ic_stable_structures::Memory},
};
use ic_blob_storage_canic::{ManagedCallFailure, lifecycle::ManagedInstallation};
use std::cell::{Cell, RefCell};

canic::memory::ic_memory_range!(
    authority = "blob-probe",
    start = 100,
    end = 116,
    mode = Allowed
);
ic_blob_storage_canic::declare_memories!(authority = "blob-probe");
canic::memory::ic_memory_range!(
    authority = "neighbor",
    start = 117,
    end = 117,
    mode = Allowed
);
ic_memory::ic_memory_declaration!(authority = "neighbor", key = "neighbor.data.v1");

thread_local! {
    static INSTALLATION: RefCell<Option<ManagedInstallation>> = const { RefCell::new(None) };
    static DEFERRED_RUNS: Cell<u32> = const { Cell::new(0) };
    // Test-only retained carrier for exact local management reinstallation tests.
    static INIT_ARGUMENTS: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}
fn neighbor() -> ic_blob_storage_canic::memory::ManagedMemory {
    ic_memory::open_default_memory_manager_memory_by_key("neighbor.data.v1").unwrap()
}
fn install() {
    let installation = context::install().unwrap();
    INIT_ARGUMENTS.set(ic_cdk::api::msg_arg_data());
    let memory = neighbor();
    assert_eq!(memory.grow(1), 0);
    memory.write(0, b"neighbor");
    INSTALLATION.with_borrow_mut(|owner| {
        assert!(owner.is_none());
        *owner = Some(installation);
    });
}
fn restore() {
    let installation = context::restore().unwrap();
    INSTALLATION.with_borrow_mut(|owner| {
        assert!(owner.is_none());
        *owner = Some(installation);
    });
}
fn deferred() {
    INSTALLATION
        .with_borrow(|owner| assert!(owner.is_some(), "installation precedes deferred work"));
    DEFERRED_RUNS.set(DEFERRED_RUNS.get() + 1);
}
async fn canic_setup() {
    deferred();
}
async fn canic_install(_: Option<Vec<u8>>) {
    deferred();
}
async fn canic_upgrade() {
    deferred();
}

/// Fixture-only passive observation before and after managed activation.
#[derive(Clone, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub struct CompositionSnapshot {
    /// Exact retained installation configuration.
    pub configuration: ServiceConfigurationInput,
    /// Explicit immutable project.
    pub project: String,
    /// Explicit immutable verifier.
    pub verifier: Principal,
    /// Canic-validated bound release, not a freshness authority.
    pub release: String,
    /// Upload, funding, gateway and read owners respectively.
    pub fences: [bool; 4],
    /// Deferred hooks run only after owner publication and activation.
    pub deferred_runs: u32,
    /// Neighbor data remains independently owned.
    pub neighbor: Vec<u8>,
}
#[ic_cdk::query]
fn probe_init_arguments() -> Result<Vec<u8>, HostFailure> {
    INSTALLATION.with_borrow(|owner| {
        if ic_cdk::api::msg_caller() != owner.as_ref().unwrap().configuration().operator {
            return Err(HostFailure::Denied);
        }
        Ok(INIT_ARGUMENTS.with_borrow(Clone::clone))
    })
}
#[ic_cdk::query]
fn probe_snapshot() -> CompositionSnapshot {
    INSTALLATION.with_borrow(|owner| {
        let owner = owner.as_ref().unwrap();
        let configuration = owner.configuration();
        let status = ic_blob_storage::workflow::operator::inspect(
            owner.stores().into(),
            ic_blob_storage::model::service::upload::UploadContext {
                service: configuration.service,
                actor: configuration.operator,
            },
            ic_blob_storage::dto::operator::OperatorScope {
                service: configuration.service,
                namespace: configuration.namespace,
                cashier: configuration.billing.cashier,
                payment_account: configuration.payment_account,
            },
        )
        .unwrap();
        let mut bytes = vec![0; 8];
        neighbor().read(0, &mut bytes);
        CompositionSnapshot {
            configuration: owner.configuration(),
            project: owner.download_scope().project().to_owned(),
            verifier: owner.completion_authority().verifier(),
            release: owner.release().to_owned(),
            fences: [
                status.uploads.fenced,
                status.funding.fenced,
                status.gateways.fenced,
                status.reads.fenced,
            ],
            deferred_runs: DEFERRED_RUNS.get(),
            neighbor: bytes,
        }
    })
}
#[ic_cdk::query]
fn probe_fleet_caller() -> Result<Principal, ManagedCallFailure> {
    context::fleet_context().map(|context| context.actor)
}

// Operator-only local exposure-state substitute. No certificate, provider effect
// or provider/recovery qualification fact is supplied by this test hook.
#[canic::canic_update(public, payload(max_bytes = 4096))]
fn probe_expose_upload(
    input: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest,
) -> Result<(), TransportFailure<ProbeExposureFailure>> {
    let actual = context::context().unwrap_or_else(|_| ic_cdk::trap("managed service is inactive"));
    ops::fixture::expose(actual, input).map_err(TransportFailure)
}
#[ic_cdk::update]
async fn probe_forward_enroll(
    input: TenantUpdateRequest,
) -> Result<TenantEnrollmentResponse, EnrollmentForwardFailure> {
    let operator = ops::read(|owner| owner.configuration().operator);
    if ic_cdk::api::msg_caller() != operator {
        return Err(EnrollmentForwardFailure::Denied);
    }
    ic_cdk::call::Call::unbounded_wait(ic_cdk::api::canister_self(), "blob_update_tenant")
        .with_arg(input)
        .await
        .map_err(|_| EnrollmentForwardFailure::Transport)?
        .candid::<Result<TenantEnrollmentResponse, TenantFailure>>()
        .expect("fixture typed reply")
        .map_err(EnrollmentForwardFailure::Tenant)
}

/// Fixture-only local forwarding; it carries no operator delegation to its callee.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum EnrollmentForwardFailure {
    /// Only the installed operator may invoke this helper.
    Denied,
    /// Actual inter-canister transport was refused before a shared handler reply.
    Transport,
    /// The maintained tenant handler refused the canister caller.
    Tenant(TenantFailure),
}

/// Fixture-only forwarding outcome; it cannot authorize provider or tenant effects.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum PreparationForwardFailure {
    /// Only the installed operator can use this local test forwarding helper.
    Denied,
    /// The actual inter-canister call was rejected.
    Transport,
    /// The shared manifest handler returned a typed refusal.
    Manifest(ic_blob_storage::dto::upload::manifest::UploadManifestFailure),
}

#[canic::canic_update(public, payload(max_bytes = 262_144))]
async fn probe_forward_prepare(
    arguments: Vec<u8>,
) -> Result<ic_blob_storage::dto::upload::manifest::UploadManifestMutation, PreparationForwardFailure>
{
    let operator =
        INSTALLATION.with_borrow(|owner| owner.as_ref().unwrap().configuration().operator);
    if ic_cdk::api::msg_caller() != operator {
        return Err(PreparationForwardFailure::Denied);
    }
    let response =
        ic_cdk::call::Call::unbounded_wait(ic_cdk::api::canister_self(), "blob_prepare_upload")
            .with_raw_args(&arguments)
            .await
            .map_err(|_| PreparationForwardFailure::Transport)?;
    response
        .candid::<Result<
            ic_blob_storage::dto::upload::manifest::UploadManifestMutation,
            ic_blob_storage::dto::upload::manifest::UploadManifestFailure,
        >>()
        .expect("fixture typed manifest reply")
        .map_err(PreparationForwardFailure::Manifest)
}

// Expand Canic's finish only after every application endpoint is registered.
mod managed;
