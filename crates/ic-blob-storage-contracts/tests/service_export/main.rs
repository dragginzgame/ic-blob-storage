//! Qualify catalogue expansion before a consumer exports its Candid service.

mod endpoints {
    // Host-side constants are emitted through the public mount, not a copied roster.
    macro_rules! describe {
        ($codec:ident, $name:ident, $spec:expr) => {
            mod $name {
                const SPEC: ic_blob_storage_contracts::endpoints::MethodSpec = $spec;
                static WIRE: Result<
                    ic_blob_storage_contracts::endpoints::WireContract,
                    ic_blob_storage_contracts::wire::Error,
                > = SPEC.wire();
                #[test]
                fn static_descriptor_matches_runtime_and_classification() {
                    use ic_blob_storage_contracts::{
                        endpoints::{MethodCodec, SERVICE_METHODS},
                        wire,
                    };
                    assert!(SERVICE_METHODS.contains(&SPEC));
                    assert_eq!(SPEC.codec, MethodCodec::$codec);
                    match &WIRE {
                        Ok(contract) => {
                            assert_eq!(SPEC.codec, MethodCodec::IcWire);
                            assert_eq!(contract.id, wire::identity(SPEC.name).unwrap());
                            for (actual, expected) in [
                                (contract.request, wire::request_limits(SPEC.name).unwrap()),
                                (contract.response, wire::response_limits(SPEC.name).unwrap()),
                            ] {
                                assert_eq!(
                                    (actual.bytes, actual.allocation, actual.nodes, actual.depth),
                                    (
                                        expected.bytes,
                                        expected.allocation,
                                        expected.nodes,
                                        expected.depth
                                    )
                                );
                            }
                        }
                        Err(error) => {
                            assert_eq!(SPEC.codec, MethodCodec::Candid);
                            assert_eq!(*error, wire::Error::Identity);
                        }
                    }
                }
            }
        };
    }

    macro_rules! endpoint {
        (provider, update, $category:ident, $wire:literal, async fn $name:ident($($arg:ident: $ty:ty),* $(,)?) $body:block) => {
            #[ic_platform::candid::contract::candid_method(update, rename = $wire)]
            #[expect(dead_code, reason = "declaration-only host fixture")]
            fn $name($($arg: $ty),*) {
                $(let _ = $arg;)*
                panic!("declaration-only host fixture")
            }
        };
        (provider, $mode:ident, $category:ident, $wire:literal, async fn $name:ident($($arg:ident: $ty:ty),* $(,)?) -> $reply:ty $body:block) => {
            endpoint!(provider, $mode, $category, $wire, fn $name($($arg:$ty),*) -> $reply $body);
        };
        (provider, $mode:ident, $category:ident, $wire:literal, fn $name:ident($($arg:ident: $ty:ty),* $(,)?) -> $reply:ty $body:block) => {
            #[ic_platform::candid::contract::candid_method($mode, rename = $wire)]
            #[expect(dead_code, reason = "declaration-only host fixture")]
            fn $name($($arg: $ty),*) -> $reply {
                $(let _ = $arg;)*
                panic!("declaration-only host fixture")
            }
        };
        (certificate, request, $wire:literal, async fn $name:ident($($arg:ident: $ty:ty),* $(,)?) -> $reply:ty $body:block) => {
            #[ic_platform::candid::contract::candid_method(update, rename = $wire)]
            #[expect(dead_code, reason = "declaration-only host fixture")]
            fn $name($($arg: $ty),*) -> $reply {
                $(let _ = $arg;)*
                panic!("declaration-only host fixture")
            }
        };
        ($($raw:tt)*) => {};
    }
    ic_blob_storage_contracts::mount_service_endpoints!(
        emit = endpoint,
        describe = describe,
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
            "service:{_immutableObjectStorageBlobsAreLive:(vecblob)->(vecbool)query;",
            "_immutableObjectStorageBlobsToDelete:()->(vecblob)query;",
            "_immutableObjectStorageConfirmBlobDeletion:(vecblob)->();",
            "_immutableObjectStorageCreateCertificate:(text)->(CaffeineUploadCertificateResponse,);}",
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
