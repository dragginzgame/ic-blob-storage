//! Synchronous installation, bounded ingress and borrowing of shared service owners.
pub(crate) mod account;
mod configuration;
pub(crate) mod gateways;
mod memory;
use crate::{
    dto::{HostConfigurationView, HostFailure, HostInstallationInput},
    model::ConfigurationRecord,
};
use candid::{CandidType, DecoderConfig, Deserialize, Principal};
use ic_blob_storage::model::service::upload::completion::CompletionAuthority;
use ic_blob_storage::{
    ic_memory::{
        MemoryRuntime,
        ic_stable_structures::{BTreeMap, DefaultMemoryImpl, Memory as _},
    },
    model::service::read::download::CaffeineDownloadScope,
    ops::service::{configuration::validate_candidate, stores::ServiceStores},
};
use memory::{Grants, Memory};
use std::cell::RefCell;
struct Host {
    _runtime: MemoryRuntime<DefaultMemoryImpl>,
    configuration: ConfigurationRecord,
    download_scope: CaffeineDownloadScope,
    completion: CompletionAuthority,
    stores: ServiceStores<Memory>,
}
thread_local! {
    static HOST: RefCell<Option<Host>> = const { RefCell::new(None) };
}
pub(crate) fn install(input: &HostInstallationInput) {
    // Validate the whole candidate before memory runtime bootstrap or allocation.
    let config = validate_candidate(ic_cdk::api::canister_self(), input.configuration)
        .expect("invalid installation configuration");
    let download_scope = CaffeineDownloadScope::new(
        ic_cdk::api::canister_self(),
        input
            .configuration
            .namespace
            .try_into()
            .expect("validated namespace"),
        &input.project,
    )
    .expect("invalid installation project");
    let completion = CompletionAuthority::new(
        config.service().bindings().service,
        config.service().bindings().namespace,
        input.completion_verifier,
    )
    .expect("invalid completion verifier");
    assert_uninitialized();
    let Grants {
        runtime,
        configuration: memory,
        stores,
    } = memory::open(true);
    assert_eq!(memory.size(), 0, "configuration memory already allocated");
    let record = configuration::record(input);
    let stores = ServiceStores::install(stores, config).expect("service installation");
    let mut records = BTreeMap::new(memory);
    records.insert(0u8, record.clone());
    publish(Host {
        _runtime: runtime,
        configuration: record,
        download_scope,
        completion,
        stores,
    });
}
pub(crate) fn restore() {
    // Upgrades cannot supply replacement configuration or initialize missing stores.
    assert_eq!(
        ic_cdk::api::msg_arg_data(),
        b"DIDL\0\0",
        "upgrade takes no arguments"
    );
    assert_uninitialized();
    let Grants {
        runtime,
        configuration: memory,
        stores,
    } = memory::open(false);
    assert!(memory.size() > 0, "missing installation configuration");
    let records: BTreeMap<u8, ConfigurationRecord, Memory> = BTreeMap::load(memory);
    assert_eq!(records.len(), 1, "configuration record count");
    let record = records.get(&0).expect("missing installation record");
    record.check_binding(ic_cdk::api::canister_self(), env!("CARGO_PKG_VERSION"));
    let config = validate_candidate(ic_cdk::api::canister_self(), configuration::input(&record))
        .expect("retained configuration");
    let download_scope = CaffeineDownloadScope::new(
        ic_cdk::api::canister_self(),
        record.namespace.try_into().expect("validated namespace"),
        &record.project,
    )
    .expect("retained project");
    let completion = CompletionAuthority::new(
        config.service().bindings().service,
        config.service().bindings().namespace,
        record.completion_verifier,
    )
    .expect("retained completion verifier");
    let stores = ServiceStores::open(stores, config).expect("service restoration");
    publish(Host {
        _runtime: runtime,
        configuration: record,
        download_scope,
        completion,
        stores,
    });
}
fn assert_uninitialized() {
    HOST.with_borrow(|host| assert!(host.is_none(), "host initialization is not reset"));
}
fn publish(host: Host) {
    HOST.with_borrow_mut(|state| *state = Some(host));
}
pub(crate) fn read<R>(f: impl FnOnce(&ServiceStores<Memory>) -> R) -> R {
    HOST.with_borrow(|host| f(&host.as_ref().expect("initialized host").stores))
}
pub(crate) fn mutate<R>(f: impl FnOnce(&mut ServiceStores<Memory>) -> R) -> R {
    HOST.with_borrow_mut(|host| f(&mut host.as_mut().expect("initialized host").stores))
}
pub(crate) fn with_completion<R>(
    f: impl FnOnce(&mut ServiceStores<Memory>, CompletionAuthority) -> R,
) -> R {
    HOST.with_borrow_mut(|state| {
        let host = state.as_mut().expect("initialized host");
        f(&mut host.stores, host.completion)
    })
}
pub(crate) fn with_download<R>(
    f: impl FnOnce(
        &ic_blob_storage::ops::service::uploads::StableUploads<Memory>,
        &CaffeineDownloadScope,
    ) -> R,
) -> R {
    HOST.with_borrow(|host| {
        let host = host.as_ref().expect("initialized host");
        f(&host.stores.uploads, &host.download_scope)
    })
}
pub(crate) fn configuration(actor: Principal) -> Result<HostConfigurationView, HostFailure> {
    HOST.with_borrow(|host| {
        let host = host.as_ref().expect("initialized host");
        if actor != host.configuration.operator {
            return Err(HostFailure::Denied);
        }
        Ok(HostConfigurationView {
            configuration: configuration::input(&host.configuration),
            project: host.configuration.project.clone(),
            completion_verifier: host.configuration.completion_verifier,
            release: host.configuration.release.clone(),
            fenced: host.stores.uploads.is_fenced(),
        })
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
