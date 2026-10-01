//! Synchronous host lifecycle and borrowing of the shared installation owner.
pub(crate) mod account;
pub(crate) mod certificate;
pub(crate) mod gateways;
mod memory;
use crate::dto::HostInstallationInput;
use candid::{CandidType, DecoderConfig, Deserialize};
use ic_blob_storage::{
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
    runtime: MemoryRuntime<DefaultMemoryImpl>,
    installation: ServiceInstallation<Memory>,
}
thread_local! {
    static HOST: RefCell<Option<Host>> = const { RefCell::new(None) };
}
pub(crate) fn install(input: &HostInstallationInput) {
    // The complete candidate is checked before this host bootstraps memory.
    let candidate = ValidatedServiceInstallation::new(
        ic_cdk::api::canister_self(),
        ServiceInstallationCandidate {
            configuration: input.configuration,
            project: &input.project,
            completion_verifier: input.completion_verifier,
            release: env!("CARGO_PKG_VERSION"),
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
        runtime,
        installation,
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
        runtime,
        installation,
    });
}
fn assert_uninitialized() {
    HOST.with_borrow(|host| assert!(host.is_none(), "host initialization is not reset"));
}
fn publish(host: Host) {
    HOST.with_borrow_mut(|state| *state = Some(host));
}
pub(crate) fn read<R>(f: impl FnOnce(&ServiceStores<Memory>) -> R) -> R {
    HOST.with_borrow(|host| {
        f(host
            .as_ref()
            .expect("initialized host")
            .installation
            .stores())
    })
}
pub(crate) fn mutate<R>(f: impl FnOnce(&mut ServiceStores<Memory>) -> R) -> R {
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
    HOST.with_borrow(|host| {
        let installation = &host.as_ref().expect("initialized host").installation;
        f(installation)
    })
}
pub(crate) fn with_memory_host<R>(
    f: impl FnOnce(&ServiceInstallation<Memory>, &MemoryRuntime<DefaultMemoryImpl>) -> R,
) -> R {
    HOST.with_borrow(|host| {
        let host = host.as_ref().expect("initialized host");
        f(&host.installation, &host.runtime)
    })
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
#[expect(clippy::needless_pass_by_value, reason = "CDK owns ingress buffers")]
pub(crate) fn decode<T: CandidType + for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
    bounded(&bytes, 4096)
}
#[expect(clippy::needless_pass_by_value, reason = "CDK owns ingress buffers")]
pub(crate) fn decode_configuration(bytes: Vec<u8>) -> HostInstallationInput {
    bounded(&bytes, 16_384)
}
#[expect(clippy::needless_pass_by_value, reason = "CDK owns ingress buffers")]
pub(crate) fn decode_manifest(
    bytes: Vec<u8>,
) -> ic_blob_storage::dto::upload::manifest::UploadManifestRequest {
    bounded(&bytes, 131_072)
}
