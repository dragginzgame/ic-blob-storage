use std::num::NonZeroU128;

use ic_storage::{
    AllocationGrants, GenericAllocationPolicy, MemoryAuthority, MemoryRequest, MemoryRuntime,
    SealedDeclarationSnapshot, StableKey, VectorMemory,
};

use super::*;
use crate::{
    caffeine::ManifestLimits,
    local::{Limits, UploadRequest},
    protocol::Installation,
};

fn actor(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
fn id(n: u128) -> NonZeroU128 {
    n.try_into().unwrap()
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
fn memory(backing: VectorMemory) -> RuntimeMemory<VectorMemory> {
    let mut runtime = MemoryRuntime::new(backing).unwrap();
    runtime
        .bootstrap(
            &SealedDeclarationSnapshot::new(vec![
                MemoryRequest::new(STORAGE_AUTHORITY, STORAGE_KEY).unwrap(),
            ])
            .unwrap(),
            &AllocationGrants::new(vec![
                MemoryAuthority::new(STORAGE_AUTHORITY, "blob.").unwrap(),
            ])
            .unwrap(),
            &GenericAllocationPolicy,
        )
        .unwrap();
    runtime
        .open_memory(&StableKey::parse(STORAGE_KEY).unwrap())
        .unwrap()
}
fn root() -> [u8; 32] {
    let hex = "b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7";
    std::array::from_fn(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap())
}
fn request(n: u128) -> UploadRequest {
    UploadRequest {
        id: id(n),
        first_reference: id(n),
        uploader: actor(2),
        root: root(),
        bytes: 3,
        expires_at: 100,
    }
}
fn authorize(n: u128) -> Command {
    Command::Authorize {
        request: request(n),
        chunks: vec![root()],
        headers: vec![],
    }
}

#[test]
fn management_inspection_is_authorized_read_only_and_survives_reopen() {
    let backing = VectorMemory::default();
    let mut service = Service::create(memory(backing.clone()), actor(8), input()).unwrap();
    let expected = ManagementView {
        installation: actor(8),
        configuration: input(),
        state_version: 1,
        maintenance: Maintenance {
            attempt: 0,
            active: true,
        },
    };
    let before = backing.borrow().clone();
    for caller in [actor(1), actor(2), actor(3), Principal::anonymous()] {
        assert_eq!(service.inspect_management(caller), Err(Error::Denied));
    }
    assert_eq!(service.inspect_management(actor(9)), Ok(expected));
    assert_eq!(*backing.borrow(), before);
    service.execute(actor(1), authorize(1), 10).unwrap();
    service
        .execute(actor(2), Command::Register(root()), 10)
        .unwrap();
    service.execute(actor(9), Command::Drain(1), 10).unwrap();
    service
        .execute(actor(1), Command::Release(id(1)), 10)
        .unwrap();
    let before = backing.borrow().clone();
    let drained = ManagementView {
        maintenance: Maintenance {
            attempt: 1,
            active: false,
        },
        ..expected
    };
    assert_eq!(service.inspect_management(actor(9)), Ok(drained));
    drop(service);
    let service = Service::open(memory(backing.clone()), actor(8)).unwrap();
    assert_eq!(service.inspect_management(actor(9)), Ok(drained));
    assert_eq!(*backing.borrow(), before);
}

#[test]
fn full_capacity_reopens_and_cleanup_needs_no_new_slot() {
    let backing = VectorMemory::default();
    let mut service = Service::create(memory(backing.clone()), actor(8), input()).unwrap();
    service.execute(actor(1), authorize(1), 10).unwrap();
    service
        .execute(actor(2), Command::Register(root()), 10)
        .unwrap();
    for n in 2..=64 {
        service.execute(actor(1), authorize(n), 10).unwrap();
    }
    assert_eq!(
        service.execute(actor(1), authorize(65), 10),
        Err(Error::Capacity)
    );
    let usage = service.inspect(actor(1), Inspection::Usage).unwrap();
    drop(service);
    let mut service = Service::open(memory(backing.clone()), actor(8)).unwrap();
    assert_eq!(service.inspect(actor(1), Inspection::Usage).unwrap(), usage);
    service.execute(actor(9), Command::Drain(1), 10).unwrap();
    for n in 1..=64 {
        service
            .execute(actor(1), Command::Release(id(n)), 10)
            .unwrap();
    }
    drop(service);
    let mut service = Service::open(memory(backing.clone()), actor(8)).unwrap();
    assert!(service.is_drained());
    for n in 1..=64 {
        service.execute(actor(1), authorize(n), 200).unwrap();
    }
    assert!(
        matches!(service.inspect(actor(1), Inspection::Usage).unwrap(), Observation::Usage(u) if u.live_bytes == 0 && u.unresolved_bytes == 3 && u.references == 64)
    );
    let before = backing.borrow().clone();
    drop(service);
    assert!(Service::open(memory(backing.clone()), actor(7)).is_err());
    assert_eq!(*backing.borrow(), before);
}

#[test]
fn retired_grants_and_denied_calls_survive_reopen_without_reset() {
    let backing = VectorMemory::default();
    let mut service = Service::create(memory(backing.clone()), actor(8), input()).unwrap();
    service.execute(actor(1), authorize(1), 10).unwrap();
    let before = backing.borrow().clone();
    assert_eq!(
        service.execute(actor(3), Command::Register(root()), 10),
        Err(Error::Denied)
    );
    assert_eq!(*backing.borrow(), before);
    service
        .execute(actor(1), Command::Expire(id(1)), 100)
        .unwrap();
    drop(service);
    let mut service = Service::open(memory(backing.clone()), actor(8)).unwrap();
    assert_eq!(
        service.execute(actor(1), authorize(2), 10),
        Err(Error::RootRetired)
    );
    assert_eq!(
        service.execute(actor(2), Command::Register(root()), 10),
        Err(Error::NotReserved)
    );
    drop(service);
    let before = backing.borrow().clone();
    assert!(Service::create(memory(backing.clone()), actor(8), input()).is_err());
    assert_eq!(*backing.borrow(), before);
    let empty = VectorMemory::default();
    let lease = memory(empty.clone());
    let before = empty.borrow().clone();
    assert!(Service::open(lease, actor(8)).is_err());
    assert_eq!(*empty.borrow(), before);
}
