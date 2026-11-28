use super::*;
use crate::{
    caffeine::ManifestLimits,
    local::{Limits, UploadRequest},
    protocol::{Inspection, Installation},
};
use ic_storage::{
    AllocationGrants, GenericAllocationPolicy, MemoryAuthority, MemoryRequest, MemoryRuntime,
    SealedDeclarationSnapshot, StableKey, VectorMemory,
};

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

fn memories(backing: VectorMemory) -> [RuntimeMemory<VectorMemory>; 2] {
    let mut runtime = MemoryRuntime::new(backing).unwrap();
    let keys = [super::super::STORAGE_KEY, STORAGE_KEY];
    runtime
        .bootstrap(
            &SealedDeclarationSnapshot::new(
                keys.iter()
                    .map(|key| MemoryRequest::new("blob", key).unwrap())
                    .collect(),
            )
            .unwrap(),
            &AllocationGrants::new(vec![MemoryAuthority::new("blob", "blob.").unwrap()]).unwrap(),
            &GenericAllocationPolicy,
        )
        .unwrap();

    keys.map(|key| {
        runtime
            .open_memory(&StableKey::parse(key).unwrap())
            .unwrap()
    })
}

fn create(backing: VectorMemory) -> Callbacks<VectorMemory> {
    let [control, receipts] = memories(backing);

    Callbacks::create(
        Service::create(control, actor(8), input()).unwrap(),
        receipts,
        actor(7),
    )
    .unwrap()
}

fn reopen(backing: VectorMemory) -> Result<Callbacks<VectorMemory>, OpenError> {
    let [control, receipts] = memories(backing);

    Callbacks::open(Service::open(control, actor(8)).unwrap(), receipts)
}

fn register(callbacks: &mut Callbacks<VectorMemory>, n: u8) {
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
    let backing = VectorMemory::default();
    let mut callbacks = create(backing.clone());
    for n in 0..64 {
        register(&mut callbacks, n);
    }
    let before = backing.borrow().clone();
    assert_eq!(
        callbacks.acknowledge(actor(7), &[[0; 32]]),
        Err(CallbackError::NotClosing)
    );
    assert_eq!(callbacks.candidates(actor(9)), Err(CallbackError::Denied));
    assert_eq!(*backing.borrow(), before);
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
    let before = backing.borrow().clone();
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
    assert_eq!(*backing.borrow(), before);
    callbacks.execute(actor(9), Command::Drain(1), 10).unwrap();
    callbacks.acknowledge(actor(7), &roots[..1]).unwrap();
    {
        // VectorMemory needs a mutable borrow for every write, even identical bytes.
        let _read_only = backing.borrow();
        callbacks.acknowledge(actor(7), &[]).unwrap();
        callbacks.acknowledge(actor(7), &[roots[0]; 2]).unwrap();
        assert_eq!(
            callbacks.acknowledge(actor(9), &[]),
            Err(CallbackError::Denied)
        );
        assert_eq!(
            callbacks.acknowledge(actor(7), &[roots[0], roots[1], [255; 32]]),
            Err(CallbackError::NotClosing)
        );
        assert_eq!(callbacks.candidates(actor(7)).unwrap(), roots[1..]);
    }
    callbacks.acknowledge(actor(7), &roots).unwrap();
    assert_eq!(
        callbacks
            .service()
            .inspect(actor(1), Inspection::Usage)
            .unwrap(),
        usage
    );
    drop(callbacks);
    let mut callbacks = reopen(backing.clone()).unwrap();
    let _read_only = backing.borrow();
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
        let backing = VectorMemory::default();
        let mut callbacks = create(backing.clone());
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
        callbacks
            .cell
            .set(crate::storage_codec::encode(&callbacks.record, LIMITS).unwrap());
        drop(callbacks);
        let before = backing.borrow().clone();
        assert!(
            matches!(
                (case, reopen(backing.clone())),
                (
                    6,
                    Err(OpenError::Codec(crate::storage_codec::Error::Version))
                ) | (0..=5, Err(OpenError::Record))
            ),
            "case {case}"
        );
        assert_eq!(*backing.borrow(), before);
    }
}
