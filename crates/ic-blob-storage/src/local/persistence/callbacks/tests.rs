use super::*;
use crate::{
    caffeine::ManifestLimits,
    local::{Limits, UploadRequest},
    protocol::{Inspection, Installation},
};
use ic_storage::retained::{Bank, Geometry, Runtime};
use std::rc::Rc;

fn actor(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
fn input() -> Installation {
    Installation {
        application: actor(1),
        manager: actor(9),
        limits: Limits {
            manifest: ManifestLimits {
                max_content_bytes: 100.try_into().unwrap(),
                max_chunks: 1.try_into().unwrap(),
                max_headers: 1.try_into().unwrap(),
                max_header_bytes: 100.try_into().unwrap(),
            },
            uploads: MAX_RECORDS.try_into().unwrap(),
            references: MAX_RECORDS.try_into().unwrap(),
            logical_bytes: 100.try_into().unwrap(),
            exposure_bytes: 100.try_into().unwrap(),
        },
    }
}

fn runtime() -> Rc<Runtime> {
    Rc::new(
        Runtime::host(
            Geometry {
                bank: Bank::A,
                nodes: 512,
                bytes: 524288,
            },
            &[super::super::storage_root(1, 1), storage_root(2, 2)],
        )
        .unwrap(),
    )
}
fn memories(backing: &Rc<Runtime>) -> [Access; 2] {
    [1, 2].map(|id| Access::host(backing.clone(), backing.grant(id).unwrap(), id))
}
fn snapshot(callbacks: &Callbacks) -> [Vec<u8>; 2] {
    [&callbacks.service.access, &callbacks.access].map(|access| {
        access
            .read(|view, root| view.checkpoint(root, 1).unwrap())
            .unwrap()
    })
}
fn create(backing: &Rc<Runtime>) -> Callbacks {
    let [control, receipts] = memories(backing);
    Callbacks::create(
        Service::create(control, actor(8), input()).unwrap(),
        receipts,
        actor(7),
    )
    .unwrap()
}

fn reopen(backing: &Rc<Runtime>) -> Result<Callbacks, OpenError> {
    backing.close().unwrap();
    backing
        .reopen(&[super::super::storage_root(1, 1), storage_root(2, 2)])
        .unwrap();
    let [control, receipts] = memories(backing);
    Callbacks::open(Service::open(control, actor(8)).unwrap(), receipts)
}

fn register(callbacks: &mut Callbacks, n: u8) {
    let id = u128::from(n + 1).try_into().unwrap();
    callbacks
        .execute(
            actor(1),
            Command::Authorize {
                request: UploadRequest {
                    id,
                    first_reference: id,
                    uploader: actor(2),
                    root: [n; 32],
                    bytes: 1,
                    expires_at: 100,
                },
                chunks: vec![[n; 32]],
                headers: vec![],
            },
            10,
        )
        .unwrap();
    callbacks
        .execute(actor(2), Command::Register([n; 32]), 10)
        .unwrap();
}

#[test]
fn full_history_cleanup_retains_exposure_and_reopens_receipts() {
    let backing = runtime();
    let mut callbacks = create(&backing);
    for n in 0..64 {
        register(&mut callbacks, n);
    }
    let before = snapshot(&callbacks);
    assert_eq!(
        callbacks.acknowledge(actor(7), &[[0; 32]]),
        Err(CallbackError::NotClosing)
    );
    assert_eq!(callbacks.candidates(actor(9)), Err(CallbackError::Denied));
    assert_eq!(snapshot(&callbacks), before);
    for n in 0..64u128 {
        callbacks
            .execute(actor(1), Command::Release((n + 1).try_into().unwrap()), 10)
            .unwrap();
    }
    let usage = callbacks
        .service()
        .inspect(actor(1), Inspection::Usage)
        .unwrap();
    let roots: Vec<_> = (0..64).map(|n| [n; 32]).collect();
    assert_eq!(callbacks.candidates(actor(7)).unwrap(), roots);
    let before = snapshot(&callbacks);
    assert_eq!(
        callbacks.acknowledge(actor(7), &[[0; 32], [255; 32]]),
        Err(CallbackError::NotClosing)
    );
    assert_eq!(
        callbacks.liveness(&[[0; 32]; 65]),
        Err(CallbackError::Capacity)
    );
    assert_eq!(
        callbacks.acknowledge(actor(7), &[[0; 32]; 65]),
        Err(CallbackError::Capacity)
    );
    assert_eq!(snapshot(&callbacks), before);
    callbacks.execute(actor(9), Command::Drain(1), 10).unwrap();
    callbacks.acknowledge(actor(7), &roots).unwrap();
    callbacks
        .acknowledge(actor(7), &[[0; 32], [0; 32]])
        .unwrap();
    assert_eq!(
        callbacks
            .service()
            .inspect(actor(1), Inspection::Usage)
            .unwrap(),
        usage
    );
    drop(callbacks);
    let mut callbacks = reopen(&backing).unwrap();
    assert!(callbacks.candidates(actor(7)).unwrap().is_empty());
    assert_eq!(callbacks.liveness(&roots).unwrap(), vec![false; 64]);
    assert_eq!(
        callbacks.liveness(&[[255; 32], [0; 32], [0; 32]]).unwrap(),
        vec![true, false, false]
    );
    callbacks.acknowledge(actor(7), &roots).unwrap();
    assert_eq!(
        callbacks
            .service()
            .inspect(actor(1), Inspection::Usage)
            .unwrap(),
        usage
    );
}

#[test]
fn reopen_refuses_foreign_inconsistent_and_unordered_receipts_without_repair() {
    for case in 0..7 {
        let backing = runtime();
        let mut callbacks = create(&backing);
        register(&mut callbacks, 0);
        register(&mut callbacks, 1);
        callbacks
            .execute(actor(1), Command::Release(1.try_into().unwrap()), 10)
            .unwrap();
        callbacks.acknowledge(actor(7), &[[0; 32]]).unwrap();
        match case {
            0 => callbacks.record.installation = actor(6),
            1 => callbacks.record.gateway = Principal::anonymous(),
            2 => callbacks.record.acknowledged[0].1 = 2.try_into().unwrap(),
            3 => callbacks
                .record
                .acknowledged
                .push(([1; 32], 2.try_into().unwrap())), // live
            4 => callbacks
                .record
                .acknowledged
                .push(([255; 32], 3.try_into().unwrap())), // unknown
            5 => callbacks
                .record
                .acknowledged
                .push(callbacks.record.acknowledged[0]),
            6 => callbacks.record.version = 0,
            _ => unreachable!(),
        }
        let bytes = ic_wire::encode(&callbacks.record, ic_wire::Encoding::Payload, LIMITS).unwrap();
        callbacks
            .access
            .update(|mut view, root| view.insert(root, b"control", &bytes).unwrap())
            .unwrap();
        let before = snapshot(&callbacks);
        drop(callbacks);
        assert!(
            matches!(reopen(&backing), Err(OpenError::Record)),
            "case {case}"
        );
        backing.close().unwrap();
        backing
            .reopen(&[super::super::storage_root(1, 1), storage_root(2, 2)])
            .unwrap();
        let [a, b] = memories(&backing);
        assert_eq!(
            [a, b].map(|access| access
                .read(|view, root| view.checkpoint(root, 1).unwrap())
                .unwrap()),
            before
        );
    }
}
