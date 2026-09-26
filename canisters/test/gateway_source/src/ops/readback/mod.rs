//! Bounded driver-supplied bytes and a local call chain for IC scheduling.
use crate::model::{CHUNK, LeafRecord};
use blob_test_protocol::journey::readback::{
    ReadSourceConfig, ReadSourceMode, ReadSourceObservation,
};
use ic_cdk::call::Call;

pub(super) fn config_view(leaf: &LeafRecord) -> ReadSourceConfig {
    ReadSourceConfig {
        root: leaf.root,
        index: leaf.index,
        bytes: leaf.bytes.clone(),
        mode: leaf.mode,
        hold: leaf.hold,
    }
}

pub(crate) fn configure(config: ReadSourceConfig) -> bool {
    let config = LeafRecord {
        root: config.root,
        index: config.index,
        bytes: config.bytes,
        mode: config.mode,
        hold: config.hold,
    };
    if !config.valid() {
        return false;
    }
    super::mutate(|journal| {
        let state = &mut journal.read;
        if state.pending {
            return false;
        }
        state.config = Some(config);
        true
    })
}

pub(crate) fn begin(root: &[u8], index: u64) -> Option<ReadSourceConfig> {
    super::mutate(|journal| {
        let state = &mut journal.read;
        if state.pending {
            return None;
        }
        let config = state.config.as_ref()?;
        if config.root.as_slice() != root || config.index != index {
            return None;
        }
        let config = config.clone();
        state.pending = true;
        state.ready = !config.hold;
        state.requests = state
            .requests
            .checked_add(1)
            .expect("fixture request bound");
        Some(config_view(&config))
    })
}

pub(crate) fn observation() -> ReadSourceObservation {
    super::read(|journal| ReadSourceObservation {
        requests: journal.read.requests,
        waiting: journal.read.pending && !journal.read.ready,
    })
}

pub(crate) fn ready() -> bool {
    super::read(|state| state.read.ready)
}

pub(crate) async fn wait() -> bool {
    // Retain the original IC call context while yielding to new rounds. Local
    // raw_rand callbacks provide scheduling only; their random bytes are unused.
    // Self-calls can all drain in one round before the driver gets to resume.
    for _ in 0..128 {
        if ready() {
            return true;
        }
        if Call::unbounded_wait(candid::Principal::management_canister(), "raw_rand")
            .await
            .is_err()
        {
            break;
        }
    }
    false
}

pub(crate) fn resume() -> bool {
    super::mutate(|journal| {
        let state = &mut journal.read;
        if !state.pending || state.ready {
            return false;
        }
        state.ready = true;
        true
    })
}

pub(crate) fn abandon() {
    super::mutate(|state| state.read.pending = false);
    super::reject();
}

pub(crate) fn reply(mut config: ReadSourceConfig) {
    super::mutate(|journal| {
        let state = &mut journal.read;
        state.pending = false;
    });
    match config.mode {
        ReadSourceMode::Reject => {
            super::reject();
            return;
        }
        ReadSourceMode::Malformed => {
            super::reply(vec![0]);
            return;
        }
        ReadSourceMode::Oversized => {
            super::reply(vec![0; CHUNK + 65]);
            return;
        }
        ReadSourceMode::Corrupt => config.bytes[0] ^= 1,
        ReadSourceMode::Truncated => {
            config.bytes.pop();
        }
        ReadSourceMode::Valid => {}
    }
    super::reply(candid::encode_one(config.bytes).expect("fixture bytes"));
}
