//! Synchronous host lifecycle and borrowing of the shared installation owner.
mod memory;
use ic_blob_storage::ic_memory::DefaultMemoryImpl;
use ic_blob_storage::ic_memory::MemoryRuntime;
use ic_blob_storage::ops::service::installation::ServiceInstallation;
use ic_blob_storage::ops::service::installation::ServiceInstallationMemories;
use ic_blob_storage::ops::service::installation::ValidatedServiceInstallation;
use ic_blob_storage_contracts::configuration::ServiceInstallationCandidate;
use ic_blob_storage_contracts::dto::configuration::ServiceInstallationInput;
use memory::{Grants, Memory};
use std::cell::RefCell;
pub(crate) struct Host {
    _runtime: MemoryRuntime<DefaultMemoryImpl>,
    installation: ServiceInstallation<Memory>,
}
thread_local! {
    static HOST: RefCell<Option<Host>> = const { RefCell::new(None) };
}
pub(crate) fn install(input: &ServiceInstallationInput) {
    // The complete candidate is checked before this host bootstraps memory.
    let candidate = ValidatedServiceInstallation::new(
        ic_cdk::api::canister_self(),
        ServiceInstallationCandidate {
            configuration: input.configuration,
            project: &input.project,
            completion_verifier: input.completion_verifier,
            release: ic_blob_storage::LIBRARY_VERSION,
            platform_installation_version: ic_cdk::api::canister_version(),
        },
    )
    .expect("invalid installation configuration");
    assert_uninitialized();
    let Grants {
        runtime,
        configuration,
        stores,
    } = memory::open(true);
    let installation = ServiceInstallation::install(
        ServiceInstallationMemories {
            configuration,
            stores,
        },
        candidate,
    )
    .expect("service installation");
    publish(Host {
        _runtime: runtime,
        installation,
    });
}
pub(crate) fn restore() {
    // The lifecycle adapter has already decoded one exact CBOR unit value.
    assert_uninitialized();
    let Grants {
        runtime,
        configuration,
        stores,
    } = memory::open(false);
    let installation = ServiceInstallation::open(
        ServiceInstallationMemories {
            configuration,
            stores,
        },
        ic_cdk::api::canister_self(),
        ic_blob_storage::LIBRARY_VERSION,
    )
    .expect("service restoration");
    publish(Host {
        _runtime: runtime,
        installation,
    });
}
fn assert_uninitialized() {
    HOST.with_borrow(|host| assert!(host.is_none(), "host initialization is not reset"));
}
fn publish(host: Host) {
    HOST.with_borrow_mut(|state| *state = Some(host));
}
fn bounded<T: serde::de::DeserializeOwned>(bytes: &[u8], max: usize) -> T {
    ic_blob_storage_contracts::wire::decode(bytes, max).expect("invalid Blob CBOR ingress")
}
/// Keep only the external provider certificate method on its mandated Candid codec.
#[expect(clippy::needless_pass_by_value, reason = "CDK owns the ingress buffer")]
pub(crate) fn decode_certificate(bytes: Vec<u8>) -> String {
    assert!(bytes.len() <= 4096, "certificate ingress byte bound");
    let config = ic_blob_storage_contracts::decoding::limits(2_000_000, 1024, 64, Some(4096));
    candid::decode_one_with_config(&bytes, &config).expect("invalid certificate ingress")
}
pub(crate) fn encode<T: serde::Serialize>(value: T) -> Vec<u8> {
    ic_blob_storage_contracts::wire::encode(value).expect("Blob CBOR reply encoding failed")
}
#[expect(clippy::needless_pass_by_value, reason = "CDK owns the ingress buffer")]
pub(crate) fn decode_tenant_inspection(
    bytes: Vec<u8>,
) -> ic_blob_storage_contracts::dto::tenant::TenantScope {
    ic_blob_storage_contracts::wire::tenant::decode_inspection_request(&bytes)
        .expect("invalid tenant inspection ingress")
}
/// Endpoint-owned trap on invalid method-bound tenant mutation ingress.
#[expect(clippy::needless_pass_by_value, reason = "CDK owns the ingress buffer")]
pub(crate) fn decode_tenant_update(
    bytes: Vec<u8>,
) -> ic_blob_storage_contracts::dto::tenant::TenantUpdateRequest {
    ic_blob_storage_contracts::wire::tenant::decode_update_request(&bytes)
        .expect("invalid tenant update ingress")
}
/// Fail with a trap if a response cannot be encoded.
pub(crate) fn encode_tenant_inspection(
    value: Result<
        ic_blob_storage_contracts::dto::tenant::TenantEnrollmentResponse,
        ic_blob_storage_contracts::dto::tenant::TenantFailure,
    >,
) -> Vec<u8> {
    ic_blob_storage_contracts::wire::tenant::inspection_response(value)
        .expect("tenant inspection response encoding")
}
/// A trap preserves IC rollback for synchronous response encoding failure.
pub(crate) fn encode_tenant_update(
    value: Result<
        ic_blob_storage_contracts::dto::tenant::TenantEnrollmentResponse,
        ic_blob_storage_contracts::dto::tenant::TenantFailure,
    >,
) -> Vec<u8> {
    ic_blob_storage_contracts::wire::tenant::update_response(value)
        .expect("tenant update response encoding")
}
/// Explicit endpoint catalog binding; this alias registers nothing.
pub(crate) type Workflow = ic_blob_storage::workflow::service::ServiceWorkflow<Host>;
impl ic_blob_storage::workflow::service::ServiceHost for Host {
    type Memory = Memory;
    fn with_installation<R>(operation: impl FnOnce(&mut ServiceInstallation<Memory>) -> R) -> R {
        HOST.with_borrow_mut(|state| {
            operation(&mut state.as_mut().expect("initialized host").installation)
        })
    }
    fn platform_version() -> u64 {
        ic_cdk::api::canister_version()
    }
    fn time() -> u64 {
        ic_cdk::api::time()
    }
    fn limits() -> ic_blob_storage::workflow::service::ServiceExecutionLimits {
        use ic_blob_storage::ops::caffeine::decoding::CaffeineReplyLimits;
        ic_blob_storage::workflow::service::ServiceExecutionLimits {
            timeout: 30.try_into().unwrap(),
            gateways: CaffeineReplyLimits {
                max_bytes: 65_536.try_into().unwrap(),
                decoding_quota: 500_000.try_into().unwrap(),
                skipping_quota: 1000.try_into().unwrap(),
                max_type_entries: 32.try_into().unwrap(),
            },
            account: CaffeineReplyLimits {
                max_bytes: 4096.try_into().unwrap(),
                decoding_quota: 500_000.try_into().unwrap(),
                skipping_quota: 1000.try_into().unwrap(),
                max_type_entries: 64.try_into().unwrap(),
            },
            history_scan: 64.try_into().unwrap(),
            history_results: 32.try_into().unwrap(),
            funding_results: 32.try_into().unwrap(),
        }
    }
}
#[expect(clippy::needless_pass_by_value, reason = "CDK owns ingress buffers")]
pub(crate) fn decode<T: serde::de::DeserializeOwned>(bytes: Vec<u8>) -> T {
    bounded(&bytes, 4096)
}
#[expect(clippy::needless_pass_by_value, reason = "CDK owns ingress buffers")]
pub(crate) fn decode_configuration(bytes: Vec<u8>) -> ServiceInstallationInput {
    // Initialization and ordinary requests reject unknown fields and trailing
    // values before entering the service. Installation has its separate byte bound.
    bounded(&bytes, 16_384)
}
#[expect(clippy::needless_pass_by_value, reason = "CDK owns ingress buffers")]
pub(crate) fn decode_manifest(
    bytes: Vec<u8>,
) -> ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest {
    bounded(&bytes, 131_072)
}
