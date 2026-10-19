//! Qualify catalogue expansion before a consumer exports its Candid service.

#[allow(
    unused_imports,
    reason = "the catalogue imports types for raw methods too"
)]
mod endpoints {
    macro_rules! endpoint {
        (certificate, request, $wire:literal, async fn $name:ident($($arg:ident: $ty:ty),* $(,)?) -> $reply:ty $body:block) => {
            #[ic_platform::candid::contract::candid_method(update, rename = $wire)]
            #[allow(dead_code, reason = "declaration-only host fixture")]
            fn $name($($arg: $ty),*) -> $reply {
                $(let _ = $arg;)*
                panic!("declaration-only host fixture")
            }
        };
        ($($raw:tt)*) => {};
    }
    ic_blob_storage_contracts::mount_service_endpoints!(
        emit = endpoint,
        workflow = unused,
        context = unused,
        contracts = ic_blob_storage_contracts,
        clock = unused,
        caller = unused,
        trap = unused,
    );
}
ic_platform::candid::contract::export_service!(ic_platform::candid::contract);

#[test]
fn provider_declaration_precedes_service_export() {
    let service = __export_service();
    let contract = service
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .flat_map(str::split_whitespace)
        .collect::<String>();
    assert_eq!(
        contract,
        concat!(
            "typeCaffeineUploadCertificateResponse=record{method:text;blob_hash:text;};",
            "service:{_immutableObjectStorageCreateCertificate:(text)->(CaffeineUploadCertificateResponse,);}",
        )
    );
    for method in ic_blob_storage_contracts::endpoints::SERVICE_METHODS {
        assert_eq!(
            service.contains(method.name),
            matches!(
                method.codec,
                ic_blob_storage_contracts::endpoints::MethodCodec::Candid
            )
        );
    }
}
