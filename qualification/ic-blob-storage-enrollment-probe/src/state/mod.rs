//! Stable enrollment state and the record used for inspection-budget checks.

#[cfg(test)]
mod tests;

use crate::protocol::{Installation, LIMITS, Outcome, View};
use ic_platform_protocol::caffeine::{Nat, TopUpArgs, encode_top_up};
use ic_platform_types::Principal;
use ic_storage::{
    AllocationGrants, DefaultMemoryImpl, GenericAllocationPolicy, Memory, MemoryAuthority,
    MemoryRequest, MemoryRuntime, RuntimeMemory, SealedDeclarationSnapshot, StableKey,
    StorageConfig, adapter::StableCell,
};
use ic_wire as wire;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Record {
    version: u32,
    view: View,
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

pub(crate) struct State {
    record: Record,
    cell: StableCell<Vec<u8>, DefaultMemoryImpl>,
}

impl State {
    pub fn create(owner: Principal, configuration: Installation) -> Self {
        validate(&configuration, owner);
        let record = Record {
            version: 2,
            view: View {
                owner,
                arguments: arguments(owner, configuration.target_balance),
                configuration,
                attempted_at: None,
                outcome: None,
            },
        };
        let cell = StableCell::create(
            memory(true),
            ic_blob_storage::protocol::frame::encode(&record, wire::Encoding::Payload, LIMITS)
                .unwrap(),
        );

        Self { record, cell }
    }

    pub fn open(owner: Principal) -> Self {
        let memory = memory(false);
        ic_storage::stable_cell_payload_length(&memory, LIMITS.bytes)
            .expect("bounded stable payload");
        let cell =
            StableCell::<Vec<u8>, _>::load(memory).expect("existing bounded enrollment record");
        let mut record: Record =
            ic_blob_storage::protocol::frame::decode_record(cell.get(), LIMITS)
                .expect("enrollment record format");
        assert!(matches!(record.version, 1 | 2));
        let view = &record.view;
        assert_eq!(view.owner, owner, "foreign owner state");
        validate(&view.configuration, owner);
        assert_eq!(
            view.arguments,
            arguments(owner, view.configuration.target_balance)
        );
        assert!(view.outcome.is_none() || view.attempted_at.is_some());

        record.version = 2;
        Self { record, cell }
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

    fn persist(&mut self) {
        // Encoding/storage failure traps this message; never return after partial mutation.
        self.cell.set(
            ic_blob_storage::protocol::frame::encode(&self.record, wire::Encoding::Payload, LIMITS)
                .unwrap(),
        );
    }
}

fn validate(config: &Installation, owner: Principal) {
    for principal in [config.operator, config.cashier, owner] {
        assert!(
            principal != Principal::anonymous() && principal != Principal::management_canister()
        );
    }
    assert_ne!(config.cashier, owner);
    assert!((1..=9_900_000_000).contains(&config.target_balance));
}

fn memory(fresh: bool) -> RuntimeMemory<DefaultMemoryImpl> {
    let backing = DefaultMemoryImpl::default();
    assert_eq!(
        backing.size() == 0,
        fresh,
        "fresh and existing state are distinct"
    );
    let mut runtime = MemoryRuntime::new_with_config(
        backing,
        StorageConfig {
            allocations: 1,
            extents: 1,
            pages: 16,
            directory_bytes: 4096,
        },
    )
    .unwrap();
    runtime
        .bootstrap(
            &SealedDeclarationSnapshot::new(vec![
                MemoryRequest::new("enrollment", "enrollment.control.v1").unwrap(),
            ])
            .unwrap(),
            &AllocationGrants::new(vec![
                MemoryAuthority::new("enrollment", "enrollment.").unwrap(),
            ])
            .unwrap(),
            &GenericAllocationPolicy,
        )
        .unwrap();

    runtime
        .open_memory(&StableKey::parse("enrollment.control.v1").unwrap())
        .unwrap()
}
