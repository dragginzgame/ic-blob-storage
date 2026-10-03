//! Synchronous host lifecycle and borrowing of the shared installation owner.
pub(crate) mod account;
pub(crate) mod gateways;
mod memory;
use candid::{CandidType, DecoderConfig, Deserialize};
use ic_blob_storage::{
    dto::configuration::ServiceInstallationInput,
    ic_memory::{MemoryRuntime, ic_stable_structures::DefaultMemoryImpl},
    model::service::{
        read::download::CaffeineDownloadScope, upload::completion::CompletionAuthority,
    },
    ops::service::{
        installation::{
            ServiceInstallation, ServiceInstallationCandidate, ServiceInstallationMemories,
            ValidatedServiceInstallation,
        },
        stores::ServiceStores,
    },
};
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
            trusted_uploader: input.trusted_uploader,
            release: env!("CARGO_PKG_VERSION"),
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
    assert_eq!(
        ic_cdk::api::msg_arg_data(),
        b"DIDL\0\0",
        "upgrade takes no arguments"
    );
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
        env!("CARGO_PKG_VERSION"),
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
        if version < host.last_platform_version
            || version.saturating_sub(host.last_platform_version) > 1
        {
            host.installation.fence();
        }
        host.last_platform_version = version;
    });
}
// Replicated queries discard their heap checkpoint; failed asynchronous replies
// can also leave a gap. An active owner gets one independent continuity check
// before a later mutation. Already-fenced restoration never activates here.
pub(crate) struct ActiveContinuityRequest {
    pub(crate) installation_version: u64,
}
pub(crate) fn begin_update() -> Option<ActiveContinuityRequest> {
    let version = ic_cdk::api::canister_version();
    let (needs_check, was_active, anchor) = HOST.with_borrow(|state| {
        let host = state.as_ref().expect("initialized host");
        (
            version < host.last_platform_version
                || version.saturating_sub(host.last_platform_version) > 1,
            !host.installation.stores().uploads.is_fenced(),
            host.installation.platform_installation_version(),
        )
    });
    observe_version();
    (needs_check && was_active).then_some(ActiveContinuityRequest {
        installation_version: anchor,
    })
}
pub(crate) fn resume_continuous_active(
    proof: ic_blob_storage::ops::service::recovery::CurrentInstanceProof,
) -> Result<(), ic_blob_storage::ops::service::recovery::CurrentInstanceRecoveryError> {
    HOST.with_borrow_mut(|state| {
        state
            .as_mut()
            .expect("initialized host")
            .installation
            .resume_current_instance(proof)
    })
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
fn bounded<T: CandidType + for<'de> Deserialize<'de>>(bytes: &[u8], max: usize) -> T {
    assert!(bytes.len() <= max, "ingress byte bound");
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(2_000_000)
        .set_skipping_quota(1024)
        .set_max_type_len(64)
        .set_max_header_len(max)
        .set_full_error_message(false);
    candid::decode_one_with_config(bytes, &config).expect("invalid ingress")
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
#[expect(clippy::needless_pass_by_value, reason = "CDK owns ingress buffers")]
pub(crate) fn decode<T: CandidType + for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
    bounded(&bytes, 4096)
}
#[expect(clippy::needless_pass_by_value, reason = "CDK owns ingress buffers")]
pub(crate) fn decode_configuration(bytes: Vec<u8>) -> ServiceInstallationInput {
    bounded(&bytes, 16_384)
}
#[expect(clippy::needless_pass_by_value, reason = "CDK owns ingress buffers")]
pub(crate) fn decode_manifest(
    bytes: Vec<u8>,
) -> ic_blob_storage::dto::upload::manifest::UploadManifestRequest {
    bounded(&bytes, 131_072)
}
