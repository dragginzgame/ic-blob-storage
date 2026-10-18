//! Explicit standalone endpoint attributes and lifecycle ownership.
#![expect(
    clippy::needless_pass_by_value,
    reason = "Endpoint macros own decoded inputs"
)]
use crate::ops;
use crate::ops::Workflow as workflow;
use ic_blob_storage_contracts::dto::configuration::ServiceInstallationInput;

fn context() -> UploadContext {
    UploadContext {
        service: ic_cdk::api::canister_self(),
        actor: ic_cdk::api::msg_caller(),
    }
}
#[ic_cdk::init(hidden = true, decode_with = "ops::decode_configuration")]
fn init(input: ServiceInstallationInput) {
    ops::install(&input);
}
#[ic_cdk::post_upgrade(decode_with = "ops::decode")]
fn post_upgrade() {
    ops::restore();
}
macro_rules! endpoint {
    (query, tenant_inspection, $($item:tt)*) => {
        #[ic_cdk::query(hidden = true, decode_with = "ops::decode_tenant_inspection", encode_with = "ops::encode_tenant_inspection")]
        $($item)*
    };
    (update, tenant_update, $($item:tt)*) => {
        #[ic_cdk::update(hidden = true, decode_with = "ops::decode_tenant_update", encode_with = "ops::encode_tenant_update")]
        $($item)*
    };
    (query, request, $($item:tt)*) => {
        #[ic_cdk::query(hidden = true, decode_with = "ops::decode", encode_with = "ops::encode")]
        $($item)*
    };
    (update, request, $($item:tt)*) => {
        #[ic_cdk::update(hidden = true, decode_with = "ops::decode", encode_with = "ops::encode")]
        $($item)*
    };
    (update, manifest, $($item:tt)*) => {
        #[ic_cdk::update(hidden = true, decode_with = "ops::decode_manifest", encode_with = "ops::encode")]
        $($item)*
    };
    (certificate, request, $wire_name:literal, $($item:tt)*) => {
        #[ic_cdk::update(name = $wire_name, hidden = true, decode_with = "ops::decode_certificate")]
        #[candid::candid_method(update, rename = $wire_name)]
        $($item)*

        // Keep provider registration and schema export in the same expansion pass.
        #[cfg(all(target_arch = "wasm32", feature = "provider-candid-export"))]
        ic_cdk::export_candid!();

        #[cfg(not(target_arch = "wasm32"))]
        candid::export_service!();

        #[cfg(any(not(target_arch = "wasm32"), feature = "provider-candid-export"))]
        pub(crate) fn interface() -> String {
            __export_service()
        }


    };
}
ic_blob_storage_contracts::mount_service_endpoints!(
    emit = endpoint,
    workflow = workflow,
    context = context,
    contracts = ic_blob_storage_contracts,
    clock = ic_cdk::api::time,
    caller = ic_cdk::api::msg_caller,
    trap = ic_cdk::trap,
);
