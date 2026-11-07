//! Actual gateway callback decoding and retained effect outcomes for external Candid replies.
use blob_test_protocol::source::{SourceAction, SourceEffectView, SourceRecoveryView};
use ic_blob_storage_pocketic_tests::{fixture_calls, installation};
use ic_platform::candid::contract;
use ic_testkit::pic::fixtures::{RawReplyLimits, RawReplyMethod, ReplyMode, raw_reply_canister};

const METHOD: &str = "_immutableObjectStorageConfirmBlobDeletion";

fn provider(reply: Option<&[u8]>) -> Vec<u8> {
    let methods = reply
        .map(|reply| RawReplyMethod {
            name: METHOD,
            mode: ReplyMode::Update,
            reply,
        })
        .into_iter()
        .collect::<Vec<_>>();
    raw_reply_canister(
        &methods,
        RawReplyLimits {
            methods: 1,
            reply_bytes: 128,
            method_name_bytes: METHOD.len(),
        },
    )
    .expect("bounded external reply fixture")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn candid_deletion_replies_preserve_arity_errors_and_journal_after_restore() {
    let harness = crate::support::Harness::new().await;
    let pic = &harness.pic;
    let driver = ic_testkit::fake_principal(91);
    let service = installation::controlled_canister(pic, driver).await;
    let source = installation::controlled_canister(pic, driver).await;
    let cases = [
        (Some(contract::encode_one(()).unwrap()), Ok(())),
        (Some(contract::encode_args(()).unwrap()), Ok(())),
        (Some(contract::encode_args(((), 17_u64)).unwrap()), Ok(())),
        (Some(contract::encode_one(false).unwrap()), Err(0)),
        (Some(vec![0xff]), Err(0)),
        (
            Some([contract::encode_one(()).unwrap(), vec![0]].concat()),
            Err(0),
        ),
        // A missing update method retains the original raw CanisterError code.
        (None, Err(5)),
    ];
    pic.install_canister(
        service,
        provider(cases[0].0.as_deref()),
        vec![],
        Some(driver),
    )
    .await
    .unwrap();
    installation::install_fixture(
        pic,
        source,
        "BLOB_GATEWAY_SOURCE_WASM",
        blob_test_protocol::fixture_wire::encode_args("fixture_init", (service, source, driver))
            .unwrap(),
        Some(driver),
    )
    .await;
    let mut effects = Vec::new();
    for (index, (reply, expected)) in cases.into_iter().enumerate() {
        if index != 0 {
            pic.upgrade_canister(service, provider(reply.as_deref()), vec![], Some(driver))
                .await
                .unwrap();
        }
        let roots = vec![vec![u8::try_from(index).unwrap(); 32]];
        let actual: Result<(), u32> =
            fixture_calls::update_fixture(pic, source, driver, "run_deletion", (roots.clone(),))
                .await
                .map(|(reply,)| reply)
                .unwrap();
        assert_eq!(actual, expected);
        effects.push(SourceEffectView {
            action: SourceAction::Delete(vec![roots[0].as_slice().try_into().unwrap()]),
            succeeded: Some(expected.is_ok()),
        });
        let (view,): (Option<SourceRecoveryView>,) =
            fixture_calls::query_fixture(pic, source, driver, "recovery_observation", ())
                .await
                .unwrap();
        assert_eq!(view.unwrap().effects, effects);
    }
    pic.upgrade_canister(
        source,
        std::fs::read(installation::fixture_path("BLOB_GATEWAY_SOURCE_WASM")).unwrap(),
        blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
        Some(driver),
    )
    .await
    .unwrap();
    let (view,): (Option<SourceRecoveryView>,) =
        fixture_calls::query_fixture(pic, source, driver, "recovery_observation", ())
            .await
            .unwrap();
    let view = view.unwrap();
    assert!(view.fenced);
    assert_eq!(view.effects, effects);
}
