//! Controlled application inputs with the real published Canic managed lifecycle.
//! This fixture is not a production endpoint artifact or provider observation.
pub mod configuration;
mod context;
use candid::{CandidType, Deserialize, Principal};
use ic_blob_storage::{
    dto::{
        configuration::ServiceConfigurationInput,
        tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
    },
    ic_memory::{self, ic_stable_structures::Memory},
    workflow::tenants,
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
fn probe_init_arguments() -> Result<Vec<u8>, ProbeFailure> {
    INSTALLATION.with_borrow(|owner| {
        if ic_cdk::api::msg_caller() != owner.as_ref().unwrap().configuration().operator {
            return Err(ProbeFailure::Tenant(TenantFailure::Denied));
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
/// Typed fixture endpoint refusal preserves managed and service authority separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ProbeFailure {
    /// Managed activation or transport admission refused.
    Managed(ManagedCallFailure),
    /// Shared tenant handler refused.
    Tenant(TenantFailure),
}
#[ic_cdk::update]
fn probe_enroll(input: TenantUpdateRequest) -> Result<TenantEnrollmentResponse, ProbeFailure> {
    let context = context::context().map_err(ProbeFailure::Managed)?;
    INSTALLATION.with_borrow_mut(|owner| {
        tenants::update(
            &mut owner.as_mut().unwrap().stores_mut().uploads,
            context,
            input,
        )
        .map_err(ProbeFailure::Tenant)
    })
}
#[ic_cdk::query]
fn probe_tenant(input: TenantScope) -> Result<TenantEnrollmentResponse, ProbeFailure> {
    let context = context::context().map_err(ProbeFailure::Managed)?;
    INSTALLATION.with_borrow(|owner| {
        tenants::inspect(&owner.as_ref().unwrap().stores().uploads, context, input)
            .map_err(ProbeFailure::Tenant)
    })
}
#[ic_cdk::query]
fn probe_fleet_caller() -> Result<Principal, ManagedCallFailure> {
    context::fleet_context().map(|context| context.actor)
}
#[ic_cdk::update]
async fn probe_forward_enroll(
    target: Principal,
    input: TenantUpdateRequest,
) -> Result<TenantEnrollmentResponse, ProbeFailure> {
    ic_cdk::call::Call::unbounded_wait(target, "probe_enroll")
        .with_arg(input)
        .await
        .expect("fixture inter-canister reply")
        .candid()
        .expect("fixture typed reply")
}

// Expand Canic's finish only after every application endpoint is registered.
mod managed;
