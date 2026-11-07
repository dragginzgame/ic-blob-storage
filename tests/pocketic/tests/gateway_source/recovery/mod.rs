//! Source-only retention and refusal; production service recovery is tested by storage/standalone.
use super::*;
use blob_test_protocol::{SourceMode, source::SourceRecoveryView};
use ic_testkit::pocket_ic::common::rest::BlobCompression;

impl Fixture {
    async fn journal(&self, actor: Principal) -> Option<SourceRecoveryView> {
        fixture_calls::query_fixture(
            &self.harness.pic,
            self.gateway,
            actor,
            "recovery_observation",
            (),
        )
        .await
        .map(|(value,)| value)
        .unwrap()
    }
    async fn delete(&self, roots: Vec<Vec<u8>>) -> Result<(), u32> {
        fixture_calls::update_fixture(
            &self.harness.pic,
            self.gateway,
            self.driver,
            "run_deletion",
            (roots,),
        )
        .await
        .map(|(value,)| value)
        .unwrap()
    }
    async fn assert_fenced(&self) {
        let before = self.journal(self.driver).await.unwrap();
        assert!(before.fenced);
        for actor in [self.service, self.gateway, Principal::anonymous()] {
            assert_eq!(self.journal(actor).await, None);
        }
        let (configured,): (bool,) = fixture_calls::update_fixture(
            &self.harness.pic,
            self.gateway,
            self.driver,
            "configure",
            (SourceMode::Reject,),
        )
        .await
        .unwrap();
        assert!(!configured);
        self.harness
            .pic
            .update_call(
                self.gateway,
                self.driver,
                "run_deletion",
                blob_test_protocol::fixture_wire::encode_one("run_deletion", vec![vec![99_u8; 32]])
                    .unwrap(),
            )
            .await
            .expect_err("restored source cannot send");
        assert_eq!(self.journal(self.driver).await, Some(before));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn full_source_history_survives_restore_without_releasing_lifetime_capacity() {
    let f = Fixture::new().await;
    let roots = vec![vec![99_u8; 32]; 8];
    for _ in 0..64 {
        assert_eq!(f.delete(roots.clone()).await, Ok(()));
    }
    let before = f.journal(f.driver).await.unwrap();
    assert_eq!(before.effects.len(), 64);
    assert!(
        before
            .effects
            .iter()
            .all(|effect| effect.succeeded == Some(true))
    );
    f.harness
        .pic
        .update_call(
            f.gateway,
            f.driver,
            "run_deletion",
            blob_test_protocol::fixture_wire::encode_one("run_deletion", roots).unwrap(),
        )
        .await
        .expect_err("bounded history is full");
    assert_eq!(f.journal(f.driver).await, Some(before.clone()));
    f.restore_source().await;
    assert_eq!(f.journal(f.driver).await.unwrap().effects, before.effects);
    f.assert_fenced().await;
    f.restore_source().await;
    f.assert_fenced().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn missing_source_journal_rejects_restore_atomically_and_keeps_the_live_history() {
    let f = Fixture::new().await;
    assert_eq!(f.delete(vec![]).await, Ok(()));
    let before = f.journal(f.driver).await;
    let saved = f.harness.pic.get_stable_memory(f.gateway).await.unwrap();
    f.harness
        .pic
        .set_stable_memory(f.gateway, vec![], BlobCompression::NoCompression)
        .await
        .unwrap();
    f.harness
        .pic
        .upgrade_canister(
            f.gateway,
            std::fs::read(fixture_path("BLOB_GATEWAY_SOURCE_WASM")).unwrap(),
            blob_test_protocol::fixture_wire::encode_args("fixture_post_upgrade", ()).unwrap(),
            Some(f.driver),
        )
        .await
        .expect_err("missing source journal cannot seed empty history");
    assert_eq!(f.journal(f.driver).await, before);
    f.harness
        .pic
        .set_stable_memory(f.gateway, saved, BlobCompression::NoCompression)
        .await
        .unwrap();
    f.restore_source().await;
    f.assert_fenced().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn older_source_history_never_reactivates_the_source() {
    let f = Fixture::new().await;
    let old = f.harness.pic.get_stable_memory(f.gateway).await.unwrap();
    assert_eq!(f.delete(vec![]).await, Ok(()));
    assert_eq!(f.journal(f.driver).await.unwrap().effects.len(), 1);
    f.harness
        .pic
        .set_stable_memory(f.gateway, old, BlobCompression::NoCompression)
        .await
        .unwrap();
    f.restore_source().await;
    assert_eq!(f.journal(f.driver).await.unwrap().effects, []);
    f.assert_fenced().await;
}
