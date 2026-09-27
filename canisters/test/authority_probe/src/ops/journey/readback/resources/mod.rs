//! One bounded, non-durable diagnostic; never read authority or recovery evidence.
use blob_test_protocol::journey::{
    JourneyFailure,
    readback::{JourneyReadChunk, ReadExecutionProfile},
};
use candid::Principal;
use ic_cdk::api::call_context_instruction_counter as counter;
use std::cell::Cell;

thread_local! {
    static LAST: Cell<Option<ReadExecutionProfile>> = const { Cell::new(None) };
}

pub(crate) fn latest() -> Option<ReadExecutionProfile> {
    LAST.get()
}

pub(crate) struct Measurement(ReadExecutionProfile);

impl Measurement {
    pub(crate) fn begin(caller: Principal) -> Self {
        Self(ReadExecutionProfile {
            caller,
            started: counter(),
            sending: None,
            replied: None,
            decoding: None,
            decoded: None,
            verified: None,
            finished: 0,
            received_bytes: 0,
            disclosed_bytes: 0,
            outcome: Ok(()),
        })
    }

    pub(crate) fn sending(&mut self) {
        self.0.sending = Some(counter());
    }

    pub(crate) fn replied(&mut self, response: &Result<Vec<u8>, JourneyFailure>) {
        self.0.replied = Some(counter());
        self.0.received_bytes = response.as_ref().map_or(0, |bytes| bytes.len() as u64);
    }

    pub(crate) fn decoding(&mut self) {
        self.0.decoding = Some(counter());
    }

    pub(crate) fn decoded(&mut self) {
        self.0.decoded = Some(counter());
    }

    pub(crate) fn verified(&mut self) {
        self.0.verified = Some(counter());
    }

    pub(crate) fn complete(mut self, result: &Result<JourneyReadChunk, JourneyFailure>) {
        self.0.outcome = result.as_ref().map(|_| ()).map_err(|failure| *failure);
        self.0.disclosed_bytes = result.as_ref().map_or(0, |chunk| chunk.bytes.len() as u64);
        self.0.finished = counter();
        LAST.set(Some(self.0));
    }
}
