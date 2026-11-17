//! One bounded durable intent/result using the existing Storage and Wire owners.

use crate::protocol::{Installation, LIMITS, Outcome, View};
use ic_platform_protocol::caffeine::{Nat, TopUpArgs, encode_top_up};
use ic_platform_types::Principal;
#[cfg(target_arch = "wasm32")]
use ic_storage::retained::{Bank, Canister, Geometry, Grant, Root};
use ic_wire as wire;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Record {
    version: u32,
    view: View,
}

#[cfg(target_arch = "wasm32")]
pub(crate) struct State {
    record: Record,
    grant: Grant,
}

#[cfg(target_arch = "wasm32")]
impl State {
    pub fn create(owner: Principal, configuration: Installation) -> Self {
        validate(&configuration, owner);
        let record = Record {
            version: 1,
            view: View {
                owner,
                arguments: arguments(owner, configuration.target_balance),
                configuration,
                attempted_at: None,
                outcome: None,
            },
        };
        let grant = memory(true);
        let state = Self { record, grant };
        state.persist();
        state
    }

    pub fn open(owner: Principal) -> Self {
        let grant = memory(false);
        let bytes = Canister::read(&[&grant], |view| {
            view.get(1, b"control")
                .unwrap()
                .unwrap()
                .as_slice()
                .to_vec()
        })
        .unwrap();
        let record: Record = wire::decode(&bytes, wire::Encoding::Payload, LIMITS)
            .expect("enrollment record format");
        assert_eq!(record.version, 1);
        let view = &record.view;
        assert_eq!(view.owner, owner, "foreign owner state");
        validate(&view.configuration, owner);
        assert_eq!(
            view.arguments,
            arguments(owner, view.configuration.target_balance)
        );
        assert!(view.outcome.is_none() || view.attempted_at.is_some());
        Self { record, grant }
    }

    pub fn view(&self) -> &View {
        &self.record.view
    }

    pub fn begin(&mut self, now: u64) {
        assert!(self.record.view.attempted_at.is_none());
        self.record.view.attempted_at = Some(now);
        self.persist();
    }

    pub fn finish(&mut self, outcome: Outcome) {
        assert!(self.record.view.attempted_at.is_some() && self.record.view.outcome.is_none());
        self.record.view.outcome = Some(outcome);
        self.persist();
    }

    fn persist(&self) {
        // Encoding/storage failure traps this message; never return after partial mutation.
        let bytes = wire::encode(&self.record, wire::Encoding::Payload, LIMITS).unwrap();
        Canister::update(&[&self.grant], |mut view| {
            view.insert(1, b"control", &bytes).unwrap()
        })
        .unwrap();
    }
}

fn arguments(owner: Principal, target: u64) -> ic_wire::Bytes<1024> {
    ic_wire::Bytes::new(
        encode_top_up(
            Some(&TopUpArgs {
                target_balance: Some(Nat::from(target)),
                account: Some(owner),
            }),
            1024,
        )
        .expect("fixed Cashier request"),
    )
    .unwrap()
}

#[cfg(target_arch = "wasm32")]
fn validate(config: &Installation, owner: Principal) {
    for principal in [config.operator, config.cashier, owner] {
        assert!(
            principal != Principal::anonymous() && principal != Principal::management_canister()
        );
    }
    assert_ne!(config.cashier, owner);
    assert!((1..=9_900_000_000).contains(&config.target_balance));
}

#[cfg(target_arch = "wasm32")]
fn memory(fresh: bool) -> Grant {
    let geometry = Geometry {
        bank: Bank::A,
        nodes: 16,
        bytes: 65536,
    };
    let roots = [Root {
        id: 1,
        participant: 1,
        schema: 0x454e524f4c4c3031,
        entries: 1,
        key_bytes: 7,
        value_bytes: 16384,
    }];
    if fresh {
        Canister::create(geometry, &roots)
    } else {
        Canister::reopen(geometry, &roots)
    }
    .expect("native enrollment admission");
    Canister::grant(1).unwrap()
}

#[cfg(test)]
mod tests;
