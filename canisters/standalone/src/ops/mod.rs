//! Synchronous host lifecycle and borrowing of the shared installation owner.
pub(crate) mod account;
pub(crate) mod gateways;
mod memory;
use ic_blob_storage::ic_memory::DefaultMemoryImpl;
use ic_blob_storage::ic_memory::MemoryRuntime;
use ic_blob_storage::ops::service::installation::ServiceInstallation;
use ic_blob_storage::ops::service::installation::ServiceInstallationMemories;
use ic_blob_storage::ops::service::installation::ValidatedServiceInstallation;
use ic_blob_storage::ops::service::stores::ServiceStores;
use ic_blob_storage_contracts::configuration::ServiceInstallationCandidate;
use ic_blob_storage_contracts::download::scope::CaffeineDownloadScope;
use ic_blob_storage_contracts::dto::configuration::ServiceInstallationInput;
use ic_blob_storage_contracts::upload::completion::CompletionAuthority;
use memory::{Grants, Memory};
use std::cell::RefCell;
struct Host {
    _runtime: MemoryRuntime<DefaultMemoryImpl>,
    installation: ServiceInstallation<Memory>,
    last_platform_version: u64,
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
        last_platform_version: ic_cdk::api::canister_version(),
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
        last_platform_version: ic_cdk::api::canister_version(),
    });
}
fn assert_uninitialized() {
    HOST.with_borrow(|host| assert!(host.is_none(), "host initialization is not reset"));
}
fn publish(host: Host) {
    HOST.with_borrow_mut(|state| *state = Some(host));
}
// Management changes can restore an old heap without a lifecycle hook. A gap
// enters a one-way fence; only fresh IC history can later prove continuity.
pub(crate) fn observe_version() {
    let version = ic_cdk::api::canister_version();
    HOST.with_borrow_mut(|state| {
        let host = state.as_mut().expect("initialized host");
        host.installation
            .observe_platform_version(&mut host.last_platform_version, version);
    });
}
pub(crate) fn read<R>(f: impl FnOnce(&ServiceStores<Memory>) -> R) -> R {
    observe_version();
    HOST.with_borrow(|host| {
        f(host
            .as_ref()
            .expect("initialized host")
            .installation
            .stores())
    })
}
pub(crate) fn mutate<R>(f: impl FnOnce(&mut ServiceStores<Memory>) -> R) -> R {
    observe_version();
    HOST.with_borrow_mut(|host| {
        f(host
            .as_mut()
            .expect("initialized host")
            .installation
            .stores_mut())
    })
}
pub(crate) fn with_completion<R>(
    f: impl FnOnce(&mut ServiceStores<Memory>, CompletionAuthority) -> R,
) -> R {
    observe_version();
    HOST.with_borrow_mut(|state| {
        let installation = &mut state.as_mut().expect("initialized host").installation;
        let authority = installation.completion_authority();
        f(installation.stores_mut(), authority)
    })
}
pub(crate) fn with_download<R>(
    f: impl FnOnce(
        &ic_blob_storage::ops::service::uploads::StableUploads<Memory>,
        &CaffeineDownloadScope,
    ) -> R,
) -> R {
    observe_version();
    HOST.with_borrow(|host| {
        let installation = &host.as_ref().expect("initialized host").installation;
        f(
            &installation.stores().uploads,
            installation.download_scope(),
        )
    })
}
pub(crate) fn with_verification<R>(
    f: impl FnOnce(
        &ic_blob_storage::ops::service::uploads::StableUploads<Memory>,
        CompletionAuthority,
        &CaffeineDownloadScope,
    ) -> R,
) -> R {
    observe_version();
    HOST.with_borrow(|host| {
        let installation = &host.as_ref().expect("initialized host").installation;
        f(
            &installation.stores().uploads,
            installation.completion_authority(),
            installation.download_scope(),
        )
    })
}
pub(crate) fn with_installation<R>(f: impl FnOnce(&ServiceInstallation<Memory>) -> R) -> R {
    observe_version();
    HOST.with_borrow(|host| {
        let installation = &host.as_ref().expect("initialized host").installation;
        f(installation)
    })
}
pub(crate) fn with_certificate<R>(f: impl FnOnce(&mut ServiceInstallation<Memory>) -> R) -> R {
    observe_version();
    HOST.with_borrow_mut(|host| f(&mut host.as_mut().expect("initialized host").installation))
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
pub(crate) struct RecoveryHost;
impl ic_blob_storage::ops::service::recovery::RecoveryInstallationAccess for RecoveryHost {
    type Memory = Memory;
    fn with_installation<R>(
        &self,
        operation: impl FnOnce(&mut ServiceInstallation<Memory>) -> R,
    ) -> R {
        with_certificate(operation)
    }
}
impl ic_blob_storage::ops::service::recovery::ContinuityInstallationAccess for RecoveryHost {
    fn observe_version(&self) {
        observe_version();
    }
    fn prepare_active_continuity(
        &self,
    ) -> Option<ic_blob_storage::ops::service::recovery::ActiveContinuityRequest> {
        let version = ic_cdk::api::canister_version();
        HOST.with_borrow_mut(|state| {
            let host = state.as_mut().expect("initialized host");
            host.installation
                .prepare_active_continuity(&mut host.last_platform_version, version)
        })
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

/// Endpoint-owned trap on invalid method-bound tenant ingress.
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
