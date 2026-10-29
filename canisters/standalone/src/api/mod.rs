//! Explicit standalone endpoint attributes and lifecycle ownership.
use crate::ops;
use crate::ops::Workflow as workflow;
use crate::workflow as observations;
use ic_blob_storage_contracts::dto::configuration::ServiceInstallationInput;
use ic_blob_storage_contracts::standalone::MetricOperation;

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
fn admit_metrics() -> Result<(), String> {
    if ic_platform::ic0::msg_arg_data_size()
        > ic_blob_storage_contracts::standalone::METRICS_WIRE
            .request
            .bytes
    {
        return Err("metrics ingress byte bound".into());
    }
    Ok(())
}
#[ic_platform::cdk::query(
    manual_reply = true,
    guard = "admit_metrics",
    decode_with = "std::convert::identity"
)]
fn blob_metrics(bytes: Vec<u8>) {
    let _: () = ops::decode_request("blob_metrics", &bytes);
    drop(bytes);
    let value = observations::metrics(ic_platform::cdk::api::msg_caller());
    ops::reply_owned("blob_metrics", value);
}
macro_rules! endpoint {
    (provider, query, roots, $wire:literal, $($item:tt)*) => {
        #[ic_platform::cdk::query(name = $wire, guard = "ops::admit_gc", decode_with = "ops::decode_gc_roots", encode_with = "ops::encode_gc")]
        #[cfg_attr(not(target_arch = "wasm32"), ic_platform::candid::contract::candid_method(query, rename = $wire))]
        $($item)*
    };
    (provider, query, empty, $wire:literal, $($item:tt)*) => {
        #[ic_platform::cdk::query(name = $wire, guard = "ops::admit_gc", decode_with = "ops::decode_gc_empty", encode_with = "ops::encode_gc")]
        #[cfg_attr(not(target_arch = "wasm32"), ic_platform::candid::contract::candid_method(query, rename = $wire))]
        $($item)*
    };
    (provider, update, roots, $wire:literal, async fn $name:ident($arg:ident : $input:ty) $body:block) => {
        #[ic_platform::cdk::update(name = $wire, guard = "ops::admit_gc", decode_with = "ops::decode_gc_roots", encode_with = "ops::encode_gc_empty")]
        #[cfg_attr(not(target_arch = "wasm32"), ic_platform::candid::contract::candid_method(update, rename = $wire))]
        async fn $name($arg: $input) {
            let started = observations::begin(MetricOperation::GatewayAcknowledgment, ic_platform::ic0::msg_arg_data_size());
            (async $body).await;
            observations::acknowledged(started);
        }
    };
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
    (update, $category:ident, async fn blob_admit_upload($($arg:ident : $input:ty)? $(,)?) -> $output:ty $body:block) => {
        endpoint!(@observed Admission, admission, blob_admit_upload, ($($arg : $input)?), $output, $body);
    };
    (update, $category:ident, async fn blob_prepare_upload($($arg:ident : $input:ty)? $(,)?) -> $output:ty $body:block) => {
        endpoint!(@observed Manifest, manifest, blob_prepare_upload, ($($arg : $input)?), $output, $body);
    };
    (update, $category:ident, async fn blob_apply_reference($($arg:ident : $input:ty)? $(,)?) -> $output:ty $body:block) => {
        endpoint!(@observed Reference, reference, blob_apply_reference, ($($arg : $input)?), $output, $body);
    };
    (update, $category:ident, async fn blob_sync_gateways($($arg:ident : $input:ty)? $(,)?) -> $output:ty $body:block) => {
        endpoint!(@observed GatewaySync, gateway_sync, blob_sync_gateways, ($($arg : $input)?), $output, $body);
    };
    (@observed $operation:ident, $observe:ident, $name:ident, ($($arg:ident : $input:ty)?), $output:ty, $body:block) => {
        mod $name {
            use super::*;
            fn admit() -> Result<(), String> { ops::admit_owned(stringify!($name)) }
            #[ic_platform::cdk::update(manual_reply = true, guard = "admit", decode_with = "std::convert::identity")]
            async fn $name(bytes: Vec<u8>) {
                endpoint!(@input stringify!($name), &bytes; $($arg : $input)?);
                let encoded_bytes = bytes.len();
                drop(bytes);
                let started = observations::begin(MetricOperation::$operation, encoded_bytes);
                let value: $output = (async $body).await;
                observations::$observe(started, &value);
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
    (certificate, request, $wire_name:literal, async fn $name:ident($arg:ident : $input:ty $(,)?) -> $output:ty $body:block) => {
        #[ic_platform::cdk::update(name = $wire_name, decode_with = "ops::decode_certificate", encode_with = "ops::encode_certificate")]
        #[cfg_attr(not(target_arch = "wasm32"), ic_platform::candid::contract::candid_method(update, rename = $wire_name))]
        async fn $name($arg: $input) -> $output {
            let started = observations::begin(MetricOperation::Registration, ic_platform::ic0::msg_arg_data_size());
            let value: $output = (async $body).await;
            observations::registered(started);
            value
        }
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
