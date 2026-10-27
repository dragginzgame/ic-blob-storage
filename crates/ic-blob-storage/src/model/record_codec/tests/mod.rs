use super::*;
use ic_storage::adapter::{Storable, storable::Bound};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, panic::catch_unwind};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Record {
    counter: u128,
    bytes: ic_wire::ByteBuf,
}
#[derive(Serialize, Deserialize)]
struct ForeignRecord {
    counter: u128,
    bytes: ic_wire::ByteBuf,
}
storable!(Record, id: b"BLTEST01", bytes: 128, layout: Bound::Unbounded);
storable!(ForeignRecord, id: b"BLTEST02", bytes: 128, layout: Bound::Unbounded);

fn value() -> Record {
    Record {
        counter: u128::MAX,
        bytes: vec![7; 80].into(),
    }
}

#[test]
fn full_width_records_round_trip_and_refuse_foreign_or_retired_bytes() {
    let record = value();
    let original = record.to_bytes().into_owned();
    assert_eq!(Record::from_bytes(Cow::Borrowed(&original)), record);
    assert_eq!(value().into_bytes(), original);
    let mut trailing = original.clone();
    trailing.push(0);
    let mut changed_id = original.clone();
    changed_id[3] ^= 1;
    let mut response = original.clone();
    response[10] = 1;
    for bytes in [
        ic_platform::candid::contract::encode_one(42_u64).unwrap(),
        ForeignRecord {
            counter: u128::MAX,
            bytes: vec![7; 80].into(),
        }
        .into_bytes(),
        original[..original.len() - 1].to_vec(),
        trailing,
        changed_id,
        response,
        Vec::new(),
        vec![0; 129],
    ] {
        assert!(catch_unwind(|| Record::from_bytes(Cow::Borrowed(&bytes))).is_err());
    }
    assert_eq!(Record::from_bytes(Cow::Borrowed(&original)), record);
}

#[test]
fn writes_and_reads_obey_wire_byte_allocation_node_and_depth_policy() {
    let oversized = Record {
        counter: u128::MAX,
        bytes: vec![0; 128].into(),
    };
    assert_eq!(oversized.encode_record(), Err(ic_wire::Error::Limit));
    assert!(catch_unwind(|| oversized.into_bytes()).is_err());
    let bytes = value().into_bytes();
    for policy in [
        ic_wire::Limits {
            bytes: bytes.len() - 1,
            ..limits(128)
        },
        ic_wire::Limits {
            allocation: 128,
            ..limits(128)
        },
        ic_wire::Limits {
            nodes: 1,
            ..limits(128)
        },
        ic_wire::Limits {
            depth: 1,
            ..limits(128)
        },
    ] {
        assert_eq!(
            ic_wire::native::decode_request::<Record>(
                u64::from_le_bytes(*b"BLTEST01"),
                &bytes,
                policy
            ),
            Err(ic_wire::Error::Limit)
        );
    }
}
