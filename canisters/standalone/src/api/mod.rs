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
#[ic_platform::cdk::init]
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
        #[ic_platform::cdk::query(decode_with = "ops::decode_tenant_inspection", encode_with = "ops::encode_tenant_inspection")]
        $($item)*
    };
    (update, tenant_update, $($item:tt)*) => {
        #[ic_platform::cdk::update(decode_with = "ops::decode_tenant_update", encode_with = "ops::encode_tenant_update")]
        $($item)*
    };
    (query, request, fn $name:ident($($arg:ident : $input:ty)? $(,)?) -> $output:ty $body:block) => {
        mod $name {
            use super::*;
            fn admit() -> Result<(), String> { ops::admit_owned(stringify!($name)) }
            #[ic_platform::cdk::query(manual_reply = true, guard = "admit", decode_with = "std::convert::identity")]
            fn $name(bytes: Vec<u8>) {
                endpoint!(@input stringify!($name), &bytes; $($arg : $input)?);
                drop(bytes);
                let value: $output = $body;
                ops::reply_owned(stringify!($name), value);
            }
        }
    };
    (update, $category:ident, async fn $name:ident($($arg:ident : $input:ty)? $(,)?) -> $output:ty $body:block) => {
        mod $name {
            use super::*;
            fn admit() -> Result<(), String> { ops::admit_owned(stringify!($name)) }
            #[ic_platform::cdk::update(manual_reply = true, guard = "admit", decode_with = "std::convert::identity")]
            async fn $name(bytes: Vec<u8>) {
                endpoint!(@input stringify!($name), &bytes; $($arg : $input)?);
                drop(bytes);
                let value: $output = (async $body).await;
                ops::reply_owned(stringify!($name), value);
            }
        }
    };
    (@input $method:expr, $bytes:expr; $arg:ident : $input:ty) => {
        let $arg: $input = ops::decode_request($method, $bytes);
    };
    (@input $method:expr, $bytes:expr;) => { let _: () = ops::decode_request($method, $bytes); };
    (certificate, request, $wire_name:literal, $($item:tt)*) => {
        #[ic_platform::cdk::update(name = $wire_name, decode_with = "ops::decode_certificate", encode_with = "ops::encode_certificate")]
        #[cfg_attr(not(target_arch = "wasm32"), ic_platform::candid::contract::candid_method(update, rename = $wire_name))]
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

// The catalogue registers its provider method before this native export is expanded.
#[cfg(not(target_arch = "wasm32"))]
ic_platform::candid::contract::export_service!(ic_platform::candid::contract);

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn interface() -> String {
    __export_service()
}
