//! Real local source calls; no deployed provider durability or HTTP claims.
use super::*;
use blob_test_protocol::{
    SourceMode, SyncFailure,
    journey::readback::{
        JourneyReadChunk, ReadSourceConfig, ReadSourceMode, ReadSourceObservation,
    },
};
use ic_testkit::pocket_ic::common::rest::RawMessageId;

const CHUNK: usize = 1024 * 1024;

impl Fixture {
    pub(super) fn source_config(
        &self,
        input: JourneyUpload,
        index: u64,
        bytes: &[u8],
        mode: ReadSourceMode,
        hold: bool,
    ) {
        let accepted: bool = self
            .harness
            .pic
            .update_candid_as(
                self.gateway,
                self.operator,
                "configure_read",
                (ReadSourceConfig {
                    root: input.root,
                    index,
                    bytes: bytes.to_vec(),
                    mode,
                    hold,
                },),
            )
            .expect("configure local source");
        assert!(accepted);
    }

    pub(super) fn source_observation(&self) -> ReadSourceObservation {
        let observed: Option<ReadSourceObservation> = self
            .harness
            .pic
            .query_candid_as(self.gateway, self.operator, "read_observation", ())
            .expect("source observation");
        observed.expect("driver authority")
    }

    pub(super) fn read_chunk(
        &self,
        caller: Principal,
        input: JourneyUpload,
        index: u64,
    ) -> Result<JourneyReadChunk, JourneyFailure> {
        self.harness
            .pic
            .update_candid_as(
                self.service,
                caller,
                "journey_read",
                (root(input.root), index),
            )
            .expect("read reply")
    }

    pub(super) fn hold_read(&self, input: JourneyUpload) -> RawMessageId {
        let id = self
            .harness
            .pic
            .submit_call(
                self.service,
                self.first,
                "journey_read",
                candid::encode_args((root(input.root), 0_u64)).expect("read args"),
            )
            .expect("submit read");
        for _ in 0..30 {
            self.harness.pic.tick();
            if self.source_observation().waiting {
                return id;
            }
        }
        panic!("source did not hold the read within the scheduling bound");
    }

    pub(super) fn resume_read(&self, id: RawMessageId) -> Result<JourneyReadChunk, JourneyFailure> {
        let resumed: bool = self
            .harness
            .pic
            .update_candid_as(self.gateway, self.operator, "resume_read", ())
            .expect("resume source");
        assert!(resumed);
        candid::decode_one(&self.harness.pic.await_call(id).expect("read callback"))
            .expect("typed reply")
    }

    pub(super) fn confirm_bytes(&self, v: &chunks::Vector) {
        assert_eq!(
            self.reserve_manifest(self.first, v.upload, v.manifest.clone()),
            Ok(())
        );
        for (index, bytes) in v.bytes.chunks(CHUNK).enumerate() {
            assert_eq!(
                self.append(
                    self.first,
                    v.upload,
                    u64::try_from(index).expect("index"),
                    bytes
                ),
                Ok(())
            );
        }
        self.certificate(self.first, v.upload.root)
            .expect("verified certificate");
        assert_eq!(self.complete(self.first, v.upload), Ok(()));
    }
}

#[test]
fn reads_verify_each_manifest_leaf_and_deny_unconfirmed_or_other_tenant_access() {
    let f = Fixture::new();
    let v = chunks::vector("uneven-tree-with-metadata", 1);
    f.source_config(v.upload, 0, &v.bytes[..CHUNK], ReadSourceMode::Valid, false);
    assert_eq!(
        f.reserve_manifest(f.first, v.upload, v.manifest.clone()),
        Ok(())
    );
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0),
        Err(JourneyFailure::InvalidPhase)
    );
    let before = f.source_observation();
    for caller in [f.second, f.gateway, f.operator, Principal::anonymous()] {
        assert_eq!(
            f.read_chunk(caller, v.upload, 0),
            Err(JourneyFailure::Denied)
        );
    }
    assert_eq!(f.source_observation(), before);
    f.confirm_bytes(&v);
    for caller in [f.second, f.gateway, f.operator, Principal::anonymous()] {
        assert_eq!(
            f.read_chunk(caller, v.upload, 0),
            Err(JourneyFailure::Denied)
        );
    }
    assert_eq!(
        f.read_chunk(f.first, v.upload, 6),
        Err(JourneyFailure::InvalidInput)
    );
    assert_eq!(f.source_observation(), before);
    let upload_progress = f.progress(f.first, v.upload);
    // Arbitrary leaf order is allowed for reads, each checked independently.
    for index in [5, 0, 3, 1, 4, 2, 5] {
        let offset = index * CHUNK;
        let bytes = &v.bytes[offset..(offset + CHUNK).min(v.bytes.len())];
        let index = u64::try_from(index).expect("index");
        f.source_config(v.upload, index, bytes, ReadSourceMode::Valid, false);
        assert_eq!(
            f.read_chunk(f.first, v.upload, index),
            Ok(JourneyReadChunk {
                index,
                offset: u64::try_from(offset).expect("offset"),
                bytes: bytes.to_vec()
            })
        );
    }
    assert_eq!(f.progress(f.first, v.upload), upload_progress);
    let amount = u128::from(v.upload.bytes);
    assert_eq!(f.usage(f.first), Ok(usage(amount, amount, amount)));
    assert_eq!(f.source_observation().requests, before.requests + 7);
}

#[test]
fn rejected_source_replies_return_no_bytes_and_release_the_exact_read_slot() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    for (mode, error) in [
        (ReadSourceMode::Corrupt, JourneyFailure::ContentMismatch),
        (ReadSourceMode::Truncated, JourneyFailure::ContentMismatch),
        (ReadSourceMode::Oversized, JourneyFailure::ReplyTooLarge),
        (ReadSourceMode::Malformed, JourneyFailure::InvalidReply),
        (ReadSourceMode::Reject, JourneyFailure::Transport),
    ] {
        f.source_config(v.upload, 0, &v.bytes, mode, false);
        let before = f.source_observation().requests;
        assert_eq!(f.read_chunk(f.first, v.upload, 0), Err(error));
        assert_eq!(f.source_observation().requests, before + 1); // no automatic retry
        assert_eq!(f.usage(f.first), Ok(usage(3, 3, 3)));
        f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false);
        assert_eq!(
            f.read_chunk(f.first, v.upload, 0)
                .expect("new explicit read")
                .bytes,
            v.bytes
        );
    }
    // A source can claim the requested root while supplying another file's bytes.
    f.source_config(v.upload, 0, b"xyz", ReadSourceMode::Valid, false);
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0),
        Err(JourneyFailure::ContentMismatch)
    );
    let rejected = f
        .harness
        .pic
        .update_call(
            f.gateway,
            f.first,
            "fixture_chunk",
            candid::encode_args((root(v.upload.root), 0_u64)).expect("args"),
        )
        .expect_err("source is service-only");
    assert_eq!(rejected.reject_code, RejectCode::CanisterReject);
}

#[test]
fn release_during_a_held_read_prevents_reply_disclosure_and_later_reads() {
    let f = Fixture::new();
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, true);
    let id = f.hold_read(v.upload);
    let resumed: bool = f
        .harness
        .pic
        .update_candid_as(f.gateway, f.first, "resume_read", ())
        .expect("denied resume");
    assert!(!resumed);
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0),
        Err(JourneyFailure::ReadInProgress)
    );
    assert_eq!(f.source_observation().requests, 1);
    assert_eq!(
        f.root_control(f.first, "journey_release", v.upload.root),
        Ok(())
    );
    assert_eq!(f.resume_read(id), Err(JourneyFailure::InvalidPhase));
    assert_eq!(f.usage(f.first), Ok(usage(0, 3, 3)));
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0),
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(f.source_observation().requests, 1);
    assert_eq!(f.delete(vec![root(v.upload.root)]), Ok(()));
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0),
        Err(JourneyFailure::InvalidPhase)
    );
    assert_eq!(
        f.root_control(f.operator, "journey_settle", v.upload.root),
        Ok(())
    );
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0),
        Err(JourneyFailure::InvalidPhase)
    );
    let next = chunks::vector("abc-binary", 2);
    f.confirm_bytes(&next);
    f.source_config(next.upload, 0, &next.bytes, ReadSourceMode::Valid, false);
    assert_eq!(
        f.read_chunk(f.first, next.upload, 0)
            .expect("slot reusable")
            .bytes,
        next.bytes
    );
}

#[test]
fn revoked_and_readded_gateway_cannot_validate_its_old_held_reply() {
    let f = Fixture::with_source_operator(true);
    let v = chunks::vector("abc-text", 1);
    f.confirm_bytes(&v);
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, true);
    let id = f.hold_read(v.upload);
    for (mode, expected) in [
        (SourceMode::Revoke, Err(SyncFailure::Stale)),
        (SourceMode::Valid, Ok(())),
    ] {
        let configured: bool = f
            .harness
            .pic
            .update_candid_as(f.gateway, f.operator, "configure", (mode,))
            .expect("sync source mode");
        assert!(configured);
        let result: Result<(), SyncFailure> = f
            .harness
            .pic
            .update_candid_as(f.gateway, f.operator, "run_sync", ())
            .expect("sync response");
        assert_eq!(result, expected);
    }
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0),
        Err(JourneyFailure::ReadInProgress)
    );
    assert_eq!(f.source_observation().requests, 1);
    assert_eq!(f.resume_read(id), Err(JourneyFailure::StaleRead));
    assert_eq!(f.usage(f.first), Ok(usage(3, 3, 3)));
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false);
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0)
            .expect("current gateway read")
            .bytes,
        v.bytes
    );
}
