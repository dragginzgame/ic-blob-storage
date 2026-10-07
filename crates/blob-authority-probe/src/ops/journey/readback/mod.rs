//! Single-step local source transport and bounded manifest verification.
pub(crate) mod resources;

use crate::model::content::ContentRequest;
use blob_test_protocol::journey::{JourneyFailure, readback::JourneyReadChunk};
use candid::{Principal, de::DecoderConfig, decode_one_with_config};
use ic_blob_storage::model::identity::caffeine::CAFFEINE_CHUNK_BYTES;
use ic_cdk::call::Call;

pub(crate) fn begin(
    request: ContentRequest,
    index: u64,
    gateway: Principal,
) -> Result<u64, JourneyFailure> {
    super::mutate(|state| {
        let entry = state
            .requests
            .iter()
            .find(|r| r.request == request)
            .ok_or(JourneyFailure::Unknown)?;
        entry
            .content
            .manifest()
            .chunk_range(index)
            .map_err(|_| JourneyFailure::InvalidInput)?;
        if state.reads.busy() {
            return Err(JourneyFailure::ReadInProgress);
        }
        let token = state.reads.begin().ok_or(JourneyFailure::Limit)?;
        state.read_intent = Some(crate::model::archive::ReadRecord {
            token,
            valid: true,
            tenant: request.upload.object.first.object().tenant(),
            root: *request.upload.object.root.as_bytes(),
            index,
            gateway,
        });
        if state.armed_read_trap == Some(request.upload.object.root) {
            state.armed_read_trap = None;
            state.trap_read_token = Some(token);
        }
        Ok(token)
    })
}

pub(crate) fn finish(token: u64) -> Result<(), JourneyFailure> {
    super::mutate(|state| {
        let valid = state.reads.finish(token);
        if !state.reads.busy() {
            state.read_intent = None;
        }
        if state.trap_read_token == Some(token) {
            state.trap_read_token = None;
            // The IC must roll back the preceding slot release and preserve the
            // admission made before the source call. Never production cfg(test).
            ic_cdk::trap("deliberate fixture read callback failure");
        }
        valid.then_some(()).ok_or(JourneyFailure::StaleRead)
    })
}

pub(crate) fn arm_callback_trap(root: ic_blob_storage::model::identity::ProviderRootHash) -> bool {
    super::mutate(|state| {
        if state.reads.busy()
            || state.armed_read_trap.is_some()
            || !state
                .requests
                .iter()
                .any(|r| r.request.upload.object.root == root)
        {
            return false;
        }
        state.armed_read_trap = Some(root);
        true
    })
}

pub(crate) async fn fetch(
    gateway: Principal,
    request: ContentRequest,
    index: u64,
) -> Result<Vec<u8>, JourneyFailure> {
    Call::bounded_wait(gateway, "fixture_chunk")
        .with_args(&(request.upload.object.root.as_bytes().to_vec(), index))
        .await
        .map(ic_cdk::call::Response::into_bytes)
        .map_err(|_| JourneyFailure::Transport)
}

pub(crate) fn verify(
    request: ContentRequest,
    index: u64,
    encoded: &[u8],
    measurement: &mut resources::Measurement,
) -> Result<JourneyReadChunk, JourneyFailure> {
    // These bound application decoding, not the IC's prior reply buffering.
    if encoded.len() > CAFFEINE_CHUNK_BYTES + 64 {
        return Err(JourneyFailure::ReplyTooLarge);
    }
    measurement.decoding();
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(10_000_000)
        .set_skipping_quota(64)
        .set_max_type_len(8)
        .set_full_error_message(false);
    // Candid's blob path validates vec nat8 and charges the same bounded decoder
    // before one bulk copy, avoiding a Serde visitor call for every byte.
    let bytes = decode_one_with_config::<serde_bytes::ByteBuf>(encoded, &config)
        .map_err(|_| JourneyFailure::InvalidReply)?
        .into_vec();
    measurement.decoded();
    let result = crate::ops::read(|state| {
        let entry = state
            .journey
            .requests
            .iter()
            .find(|r| r.request == request)
            .ok_or(JourneyFailure::Unknown)?;
        let manifest = entry.content.manifest();
        manifest
            .verify_chunk(index, &bytes)
            .map_err(|_| JourneyFailure::ContentMismatch)?;
        let range = manifest
            .chunk_range(index)
            .map_err(|_| JourneyFailure::InvalidInput)?;
        Ok(JourneyReadChunk {
            index,
            offset: range.offset,
            bytes,
        })
    });
    measurement.verified();
    result
}
