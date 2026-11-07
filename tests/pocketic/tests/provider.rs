//! Provider decoder and source-journal evidence without a parallel service implementation.
#![cfg(not(target_family = "wasm"))]
mod gateway_source;
mod support;
use ic_blob_storage_pocketic_tests::{fixture_calls, installation};
use ic_platform::principal::ExternalPrincipal as Principal;
use ic_testkit::{
    fake_principal,
    pic::fixtures::{RawReplyLimits, RawReplyMethod, ReplyMode, raw_reply_canister},
};
use support::{Harness, fixture_path};

struct Fixture {
    harness: Harness,
    service: Principal,
    gateway: Principal,
    driver: Principal,
}
impl Fixture {
    async fn new() -> Self {
        let harness = Harness::new().await;
        let pic = &harness.pic;
        let driver = fake_principal(11);
        let service = installation::controlled_canister(pic, driver).await;
        let gateway = installation::controlled_canister(pic, driver).await;
        let unit = ic_platform::candid::contract::encode_one(()).unwrap();
        let name = "_immutableObjectStorageConfirmBlobDeletion";
        let provider = raw_reply_canister(
            &[RawReplyMethod {
                name,
                mode: ReplyMode::Update,
                reply: &unit,
            }],
            RawReplyLimits {
                methods: 1,
                reply_bytes: 128,
                method_name_bytes: name.len(),
            },
        )
        .unwrap();
        pic.install_canister(service, provider, vec![], Some(driver))
            .await
            .unwrap();
        installation::install_fixture(
            pic,
            gateway,
            "BLOB_GATEWAY_SOURCE_WASM",
            blob_test_protocol::fixture_wire::encode_args(
                "fixture_init",
                (service, gateway, driver),
            )
            .unwrap(),
            Some(driver),
        )
        .await;
        Self {
            harness,
            service,
            gateway,
            driver,
        }
    }
    async fn journals(&self) -> Vec<Vec<u8>> {
        vec![
            self.harness
                .pic
                .get_stable_memory(self.gateway)
                .await
                .unwrap(),
        ]
    }
    async fn restore_source(&self) {
        self.harness
            .pic
            .upgrade_canister(
                self.gateway,
                std::fs::read(fixture_path("BLOB_GATEWAY_SOURCE_WASM")).unwrap(),
                blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
                Some(self.driver),
            )
            .await
            .unwrap();
    }
}
