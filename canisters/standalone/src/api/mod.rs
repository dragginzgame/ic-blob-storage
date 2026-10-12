//! Explicit standalone endpoint attributes and lifecycle ownership.
#![expect(
    clippy::needless_pass_by_value,
    reason = "Candid endpoint macros own decoded inputs"
)]
use crate::{ops, workflow};
use ic_blob_storage_contracts::dto::configuration::ServiceInstallationInput;

fn context() -> UploadContext {
    UploadContext {
        service: ic_cdk::api::canister_self(),
        actor: ic_cdk::api::msg_caller(),
    }
}
#[ic_cdk::init(hidden = true, decode_with = "ops::decode_configuration")]
// Custom decoding bounds raw Candid; explicitly retain its typed wire contract.
#[candid::candid_method(init)]
fn init(input: ServiceInstallationInput) {
    workflow::install(&input);
}
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    workflow::restore();
}
macro_rules! endpoint {
    (query, request, $($item:tt)*) => {
        #[ic_cdk::query(hidden = true, decode_with = "ops::decode")]
        #[candid::candid_method(query)]
        $($item)*
    };
    (update, request, $($item:tt)*) => {
        #[ic_cdk::update(hidden = true, decode_with = "ops::decode")]
        #[candid::candid_method(update)]
        $($item)*
    };
    (update, manifest, $($item:tt)*) => {
        #[ic_cdk::update(hidden = true, decode_with = "ops::decode_manifest")]
        #[candid::candid_method(update)]
        $($item)*
    };
    (certificate, request, $wire_name:literal, $($item:tt)*) => {
        #[ic_cdk::update(name = $wire_name, hidden = true, decode_with = "ops::decode")]
        #[candid::candid_method(update, rename = $wire_name)]
        $($item)*
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

ic_cdk::export_candid!();

pub(crate) fn interface() -> String {
    __export_service()
}
