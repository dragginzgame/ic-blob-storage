//! Real local source calls; no deployed provider durability or HTTP claims.
use ic_blob_storage_pocketic_tests::fixture_calls;

mod resources;

use super::*;
use blob_test_protocol::{
    SourceMode, SyncFailure,
    journey::readback::{
        JourneyReadChunk, ReadExecutionProfile, ReadSourceConfig, ReadSourceMode,
        ReadSourceObservation,
    },
};
use ic_testkit::pocket_ic::common::rest::RawMessageId;

const CHUNK: usize = 1024 * 1024;

impl Fixture {
    pub(super) async fn read_resources(
        &self,
        caller: Principal,
    ) -> Result<Option<ReadExecutionProfile>, JourneyFailure> {
        fixture_calls::query_fixture(
            &self.harness.pic,
            self.service,
            caller,
            "journey_read_resources",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .expect("read resource observation")
    }

    pub(super) async fn source_config(
        &self,
        input: JourneyUpload,
        index: u64,
        bytes: &[u8],
        mode: ReadSourceMode,
        hold: bool,
    ) {
        let accepted: bool = fixture_calls::update_fixture(
            &self.harness.pic,
            self.gateway,
            self.operator,
            "configure_read",
            (ReadSourceConfig {
                root: input.root,
                index,
                bytes: bytes.to_vec().into(),
                mode,
                hold,
            },),
        )
        .await
        .map(|(reply,)| reply)
        .expect("configure local source");
        assert!(accepted);
    }

    pub(super) async fn source_observation(&self) -> ReadSourceObservation {
        let observed: Option<ReadSourceObservation> = fixture_calls::query_fixture(
            &self.harness.pic,
            self.gateway,
            self.operator,
            "read_observation",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .expect("source observation");
        observed.expect("driver authority")
    }

    pub(super) async fn read_chunk(
        &self,
        caller: Principal,
        input: JourneyUpload,
        index: u64,
    ) -> Result<JourneyReadChunk, JourneyFailure> {
        fixture_calls::update_fixture(
            &self.harness.pic,
            self.service,
            caller,
            "journey_read",
            (root(input.root), index),
        )
        .await
        .map(|(reply,)| reply)
        .expect("read reply")
    }

    pub(super) async fn hold_read(&self, input: JourneyUpload) -> RawMessageId {
        let id = self
            .harness
            .pic
            .submit_call(
                self.service,
                self.first,
                "journey_read",
                blob_test_protocol::fixture_wire::encode_args(
                    "journey_read",
                    (root(input.root), 0_u64),
                )
                .expect("read args"),
            )
            .await
            .expect("submit read");
        for _ in 0..30 {
            self.harness.pic.tick().await.unwrap();
            if self.source_observation().await.waiting {
                return id;
            }
        }
        panic!("source did not hold the read within the scheduling bound");
    }

    pub(super) async fn resume_read(
        &self,
        id: RawMessageId,
    ) -> Result<JourneyReadChunk, JourneyFailure> {
        let resumed: bool = fixture_calls::update_fixture(
            &self.harness.pic,
            self.gateway,
            self.operator,
            "resume_read",
            (),
        )
        .await
        .map(|(reply,)| reply)
        .expect("resume source");
        assert!(resumed);
        blob_test_protocol::fixture_wire::decode_one(
            "journey_read",
            &self
                .harness
                .pic
                .await_call(id)
                .await
                .expect("read callback"),
            blob_test_protocol::fixture_wire::MAX_BYTES,
        )
        .expect("typed reply")
    }

    pub(super) async fn confirm_bytes(&self, v: &content_vectors::Vector) {
        assert_eq!(
            self.reserve_manifest(self.first, v.upload, v.manifest.clone())
                .await,
            Ok(())
        );
        for (index, bytes) in v.bytes.chunks(CHUNK).enumerate() {
            assert_eq!(
                self.append(
                    self.first,
                    v.upload,
                    u64::try_from(index).expect("index"),
                    bytes
                )
                .await,
                Ok(())
            );
        }
        self.certificate(self.first, v.upload.root)
            .await
            .expect("verified certificate");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn reads_verify_each_manifest_leaf_and_deny_unconfirmed_or_other_tenant_access() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("uneven-tree-with-metadata", 1);
    f.source_config(v.upload, 0, &v.bytes[..CHUNK], ReadSourceMode::Valid, false)
        .await;
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest.clone())
            .await,
        Ok(())
    );
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0).await,
        Err(JourneyFailure::InvalidPhase)
    );
    let before = f.source_observation().await;
    for caller in [f.second, f.gateway, f.operator, Principal::anonymous()] {
        assert_eq!(
            f.read_chunk(caller, v.upload, 0).await,
            Err(JourneyFailure::Denied)
        );
    }
    assert_eq!(f.source_observation().await, before);
    f.confirm_bytes(&v).await;
    for caller in [f.second, f.gateway, f.operator, Principal::anonymous()] {
        assert_eq!(
            f.read_chunk(caller, v.upload, 0).await,
            Err(JourneyFailure::Denied)
        );
    }
    assert_eq!(
        f.read_chunk(f.first, v.upload, 6).await,
        Err(JourneyFailure::InvalidInput)
    );
    assert_eq!(f.source_observation().await, before);
    let upload_progress = f.progress(f.first, v.upload).await;
    // Arbitrary leaf order is allowed for reads, each checked independently.
    for index in [5, 0, 3, 1, 4, 2, 5] {
        let offset = index * CHUNK;
        let bytes = &v.bytes[offset..(offset + CHUNK).min(v.bytes.len())];
        let index = u64::try_from(index).expect("index");
        f.source_config(v.upload, index, bytes, ReadSourceMode::Valid, false)
            .await;
        assert_eq!(
            f.read_chunk(f.first, v.upload, index).await,
            Ok(JourneyReadChunk {
                index,
                offset: u64::try_from(offset).expect("offset"),
                bytes: bytes.to_vec().into()
            })
        );
    }
    assert_eq!(f.progress(f.first, v.upload).await, upload_progress);
    let amount = u128::from(v.upload.bytes);
    assert_eq!(f.usage(f.first).await, Ok(usage(amount, amount, amount)));
    assert_eq!(f.source_observation().await.requests, before.requests + 7);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rejected_source_replies_return_no_bytes_and_release_the_exact_read_slot() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("abc-text", 1);
    f.confirm_bytes(&v).await;
    for (mode, error) in [
        (ReadSourceMode::Corrupt, JourneyFailure::ContentMismatch),
        (ReadSourceMode::Truncated, JourneyFailure::ContentMismatch),
        (ReadSourceMode::Oversized, JourneyFailure::ReplyTooLarge),
        (ReadSourceMode::Malformed, JourneyFailure::InvalidReply),
        (ReadSourceMode::WrongType, JourneyFailure::InvalidReply),
        (
            ReadSourceMode::TruncatedEncoding,
            JourneyFailure::InvalidReply,
        ),
        (ReadSourceMode::Reject, JourneyFailure::Transport),
    ] {
        f.source_config(v.upload, 0, &v.bytes, mode, false).await;
        let before = f.source_observation().await.requests;
        assert_eq!(f.read_chunk(f.first, v.upload, 0).await, Err(error));
        assert_eq!(f.source_observation().await.requests, before + 1); // no automatic retry
        assert_eq!(f.usage(f.first).await, Ok(usage(3, 3, 3)));
        f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false)
            .await;
        assert_eq!(
            f.read_chunk(f.first, v.upload, 0)
                .await
                .expect("new explicit read")
                .bytes,
            v.bytes
        );
    }
    // A source can claim the requested root while supplying another file's bytes.
    f.source_config(v.upload, 0, b"xyz", ReadSourceMode::Valid, false)
        .await;
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0).await,
        Err(JourneyFailure::ContentMismatch)
    );
    let rejected = f
        .harness
        .pic
        .update_call(
            f.gateway,
            f.first,
            "fixture_chunk",
            blob_test_protocol::fixture_wire::encode_args(
                "fixture_chunk",
                (root(v.upload.root), 0_u64),
            )
            .expect("args"),
        )
        .await
        .expect_err("source is service-only");
    assert_eq!(
        crate::canister_rejection(&rejected).reject_code,
        RejectCode::CanisterReject
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn release_during_a_held_read_prevents_reply_disclosure_and_later_reads() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("abc-text", 1);
    f.confirm_bytes(&v).await;
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, true)
        .await;
    let id = f.hold_read(v.upload).await;
    let resumed: bool =
        fixture_calls::update_fixture(&f.harness.pic, f.gateway, f.first, "resume_read", ())
            .await
            .map(|(reply,)| reply)
            .expect("denied resume");
    assert!(!resumed);
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0).await,
        Err(JourneyFailure::ReadInProgress)
    );
    assert_eq!(f.source_observation().await.requests, 1);
    assert_eq!(
        f.root_control(f.first, "journey_release", v.upload.root)
            .await,
        Ok(())
    );
    assert_eq!(f.resume_read(id).await, Err(JourneyFailure::InvalidPhase));
    assert_eq!(f.usage(f.first).await, Ok(usage(0, 3, 3)));
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0).await,
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(f.source_observation().await.requests, 1);
    assert_eq!(f.delete(vec![root(v.upload.root)]).await, Ok(()));
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0).await,
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(
        f.root_control(f.operator, "journey_settle", v.upload.root)
            .await,
        Ok(())
    );
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0).await,
        Err(JourneyFailure::InvalidPhase)
    );
    let next = content_vectors::vector("abc-binary", 2);
    f.confirm_bytes(&next).await;
    f.source_config(next.upload, 0, &next.bytes, ReadSourceMode::Valid, false)
        .await;
    assert_eq!(
        f.read_chunk(f.first, next.upload, 0)
            .await
            .expect("slot reusable")
            .bytes,
        next.bytes
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn revoked_and_readded_gateway_cannot_validate_its_old_held_reply() {
    let f = Fixture::with_source_operator(true).await;
    let v = content_vectors::vector("abc-text", 1);
    f.confirm_bytes(&v).await;
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, true)
        .await;
    let id = f.hold_read(v.upload).await;
    for (mode, expected) in [
        (SourceMode::Revoke, Err(SyncFailure::Stale)),
        (SourceMode::Valid, Ok(())),
    ] {
        let configured: bool = fixture_calls::update_fixture(
            &f.harness.pic,
            f.gateway,
            f.operator,
            "configure",
            (mode,),
        )
        .await
        .map(|(reply,)| reply)
        .expect("sync source mode");
        assert!(configured);
        let result: Result<(), SyncFailure> = fixture_calls::update_fixture(
            &f.harness.pic,
            f.gateway,
            f.operator,
            "run_sync",
            (f.sync_request().await,),
        )
        .await
        .map(|(reply,)| reply)
        .expect("sync response");
        assert_eq!(result, expected);
    }
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0).await,
        Err(JourneyFailure::ReadInProgress)
    );
    assert_eq!(f.source_observation().await.requests, 1);
    assert_eq!(f.resume_read(id).await, Err(JourneyFailure::StaleRead));
    assert_eq!(f.usage(f.first).await, Ok(usage(3, 3, 3)));
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false)
        .await;
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0)
            .await
            .expect("current gateway read")
            .bytes,
        v.bytes
    );
}
