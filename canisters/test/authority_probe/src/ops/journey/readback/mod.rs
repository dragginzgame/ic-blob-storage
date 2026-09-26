//! Single-step local source transport and bounded manifest verification.
use blob_test_protocol::journey::{JourneyFailure, readback::JourneyReadChunk};
use candid::{Principal, de::DecoderConfig, decode_one_with_config};
use ic_blob_storage::model::{
    catalog::admission::UploadRequest, identity::caffeine::CAFFEINE_CHUNK_BYTES,
};
use ic_cdk::call::Call;

pub(crate) fn begin(
    request: UploadRequest,
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
            tenant: request.object.first.object().tenant(),
            root: *request.object.root.as_bytes(),
            index,
            gateway,
        });
        if state.armed_read_trap == Some(request.object.root) {
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
            || !state.requests.iter().any(|r| r.request.object.root == root)
        {
            return false;
        }
        state.armed_read_trap = Some(root);
        true
    })
}

pub(crate) async fn fetch(
    gateway: Principal,
    request: UploadRequest,
    index: u64,
) -> Result<Vec<u8>, JourneyFailure> {
    Call::bounded_wait(gateway, "fixture_chunk")
        .with_args(&(request.object.root.as_bytes().to_vec(), index))
        .await
        .map(ic_cdk::call::Response::into_bytes)
        .map_err(|_| JourneyFailure::Transport)
}

pub(crate) fn verify(
    request: UploadRequest,
    index: u64,
    encoded: &[u8],
) -> Result<JourneyReadChunk, JourneyFailure> {
    // These bound application decoding, not the IC's prior reply buffering.
    if encoded.len() > CAFFEINE_CHUNK_BYTES + 64 {
        return Err(JourneyFailure::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(10_000_000)
        .set_skipping_quota(64)
        .set_max_type_len(8)
        .set_full_error_message(false);
    let bytes: Vec<u8> =
        decode_one_with_config(encoded, &config).map_err(|_| JourneyFailure::InvalidReply)?;
    crate::ops::read(|state| {
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
    })
}
