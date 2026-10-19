//! Explicit standalone endpoint attributes and lifecycle ownership.
use crate::ops;
use crate::ops::Workflow as workflow;
use ic_blob_storage_contracts::dto::configuration::ServiceInstallationInput;

fn context() -> UploadContext {
    UploadContext {
        service: ic_platform::cdk::api::canister_self(),
        actor: ic_platform::cdk::api::msg_caller(),
    }
}
#[ic_platform::cdk::init(hidden = true)]
fn init() {
    let input: ServiceInstallationInput = ops::decode_owned("blob_install");
    ops::install(&input);
}
#[ic_platform::cdk::post_upgrade]
fn post_upgrade() {
    let _: () = ops::decode_owned("blob_post_upgrade");
    ops::restore();
}
macro_rules! endpoint {
    (query, tenant_inspection, $($item:tt)*) => {
        #[ic_platform::cdk::query(hidden = true, decode_with = "ops::decode_tenant_inspection", encode_with = "ops::encode_tenant_inspection")]
        $($item)*
    };
    (update, tenant_update, $($item:tt)*) => {
        #[ic_platform::cdk::update(hidden = true, decode_with = "ops::decode_tenant_update", encode_with = "ops::encode_tenant_update")]
        $($item)*
    };
    (query, request, fn $name:ident($($arg:ident : $input:ty)? $(,)?) -> $output:ty $body:block) => {
        #[ic_platform::cdk::query(manual_reply = true, hidden = true)]
        fn $name() {
            endpoint!(@input stringify!($name); $($arg : $input)?);
            let value: $output = $body;
            ops::reply_owned(stringify!($name), value);
        }
    };
    (update, $category:ident, async fn $name:ident($($arg:ident : $input:ty)? $(,)?) -> $output:ty $body:block) => {
        #[ic_platform::cdk::update(manual_reply = true, hidden = true)]
        async fn $name() {
            endpoint!(@input stringify!($name); $($arg : $input)?);
            let value: $output = (async $body).await;
            ops::reply_owned(stringify!($name), value);
        }
    };
    (@input $method:expr; $arg:ident : $input:ty) => {
        let $arg: $input = ops::decode_owned($method);
    };
    (@input $method:expr;) => { let _: () = ops::decode_owned($method); };
    (certificate, request, $wire_name:literal, $($item:tt)*) => {
        #[ic_platform::cdk::update(name = $wire_name, hidden = true, decode_with = "ops::decode_certificate")]
        #[ic_platform::candid::contract::candid_method(update, rename = $wire_name)]
        $($item)*


    };
}
ic_blob_storage_contracts::mount_service_endpoints!(
    emit = endpoint,
    workflow = workflow,
    context = context,
    contracts = ic_blob_storage_contracts,
    clock = ic_platform::cdk::api::time,
    caller = ic_platform::cdk::api::msg_caller,
    trap = ic_platform::cdk::api::trap,
);

// The catalogue registers its provider method before this export is expanded.
#[cfg(all(target_arch = "wasm32", feature = "provider-candid-export"))]
ic_platform::cdk::export_candid!();

#[cfg(not(target_arch = "wasm32"))]
ic_platform::candid::contract::export_service!(ic_platform::candid::contract);

#[cfg(any(not(target_arch = "wasm32"), feature = "provider-candid-export"))]
pub(crate) fn interface() -> String {
    __export_service()
}
