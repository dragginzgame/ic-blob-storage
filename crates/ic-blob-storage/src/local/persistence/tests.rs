use super::*;
use crate::{
    caffeine::ManifestLimits,
    local::{Limits, UploadRequest},
};
use ic_storage::retained::{Bank, Geometry, Runtime};
use std::{num::NonZeroU128, rc::Rc};

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
fn runtime() -> Rc<Runtime> {
    Rc::new(
        Runtime::host(
            Geometry {
                bank: Bank::A,
                nodes: 512,
                bytes: 262_144,
            },
            &[storage_root(1, 1)],
        )
        .unwrap(),
    )
}
fn access(runtime: &Rc<Runtime>) -> Access {
    Access::host(runtime.clone(), runtime.grant(1).unwrap(), 1)
}
fn reopen(runtime: &Rc<Runtime>) -> Access {
    runtime.close().unwrap();
    runtime.reopen(&[storage_root(1, 1)]).unwrap();
    access(runtime)
}
fn snapshot(access: &Access) -> Vec<u8> {
    access
        .read(|view, root| view.checkpoint(root, 1).unwrap())
        .unwrap()
}
fn root() -> [u8; 32] {
    let hex = "b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7";
    std::array::from_fn(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap())
}
fn authorize(n: u128) -> Command {
    Command::Authorize {
        request: UploadRequest {
            id: id(n),
            first_reference: id(n),
            uploader: actor(2),
            root: root(),
            bytes: 3,
            expires_at: 100,
        },
        chunks: vec![root()],
        headers: vec![],
    }
}
#[test]
fn management_inspection_is_authorized_read_only_and_survives_reopen() {
    let runtime = runtime();
    let mut service = Service::create(access(&runtime), actor(8), input()).unwrap();
    let expected = ManagementView {
        installation: actor(8),
        configuration: input(),
        state_version: 1,
        maintenance: Maintenance {
            attempt: 0,
            active: true,
        },
    };
    let before = snapshot(&service.access);
    for caller in [actor(1), actor(2), actor(3), Principal::anonymous()] {
        assert_eq!(service.inspect_management(caller), Err(Error::Denied));
    }
    assert_eq!(service.inspect_management(actor(9)), Ok(expected));
    assert_eq!(snapshot(&service.access), before);
    service.execute(actor(1), authorize(1), 10).unwrap();
    service
        .execute(actor(2), Command::Register(root()), 10)
        .unwrap();
    service.execute(actor(9), Command::Drain(1), 10).unwrap();
    service
        .execute(actor(1), Command::Release(id(1)), 10)
        .unwrap();
    let before = snapshot(&service.access);
    let drained = ManagementView {
        maintenance: Maintenance {
            attempt: 1,
            active: false,
        },
        ..expected
    };
    drop(service);
    let service = Service::open(reopen(&runtime), actor(8)).unwrap();
    assert_eq!(service.inspect_management(actor(9)), Ok(drained));
    assert_eq!(snapshot(&service.access), before);
}
#[test]
fn full_capacity_reopens_and_cleanup_needs_no_new_slot() {
    let runtime = runtime();
    let mut service = Service::create(access(&runtime), actor(8), input()).unwrap();
    service.execute(actor(1), authorize(1), 10).unwrap();
    service
        .execute(actor(2), Command::Register(root()), 10)
        .unwrap();
    for n in 2..=64 {
        service.execute(actor(1), authorize(n), 10).unwrap();
    }
    let before = snapshot(&service.access);
    assert_eq!(
        service.execute(actor(1), authorize(65), 10),
        Err(Error::Capacity)
    );
    assert_eq!(snapshot(&service.access), before);
    let usage = service.inspect(actor(1), Inspection::Usage).unwrap();
    drop(service);
    let mut service = Service::open(reopen(&runtime), actor(8)).unwrap();
    assert_eq!(service.inspect(actor(1), Inspection::Usage).unwrap(), usage);
    service.execute(actor(9), Command::Drain(1), 10).unwrap();
    for n in 1..=64 {
        service
            .execute(actor(1), Command::Release(id(n)), 10)
            .unwrap();
    }
    drop(service);
    let mut service = Service::open(reopen(&runtime), actor(8)).unwrap();
    assert!(service.is_drained());
    for n in 1..=64 {
        service.execute(actor(1), authorize(n), 200).unwrap();
    }
    assert!(
        matches!(service.inspect(actor(1),Inspection::Usage).unwrap(),Observation::Usage(u) if u.live_bytes==0 && u.unresolved_bytes==3 && u.references==64)
    );
    let before = snapshot(&service.access);
    drop(service);
    assert!(Service::open(reopen(&runtime), actor(7)).is_err());
    let access = reopen(&runtime);
    assert_eq!(snapshot(&access), before);
}
#[test]
fn retired_grants_and_denied_calls_survive_reopen_without_reset() {
    let runtime = runtime();
    let mut service = Service::create(access(&runtime), actor(8), input()).unwrap();
    service.execute(actor(1), authorize(1), 10).unwrap();
    let before = snapshot(&service.access);
    assert_eq!(
        service.execute(actor(3), Command::Register(root()), 10),
        Err(Error::Denied)
    );
    assert_eq!(snapshot(&service.access), before);
    service
        .execute(actor(1), Command::Expire(id(1)), 100)
        .unwrap();
    drop(service);
    let mut service = Service::open(reopen(&runtime), actor(8)).unwrap();
    assert_eq!(
        service.execute(actor(1), authorize(2), 10),
        Err(Error::RootRetired)
    );
    assert_eq!(
        service.execute(actor(2), Command::Register(root()), 10),
        Err(Error::NotReserved)
    );
    let before = snapshot(&service.access);
    drop(service);
    assert!(Service::create(reopen(&runtime), actor(8), input()).is_err());
    assert_eq!(snapshot(&reopen(&runtime)), before);
    let empty = self::runtime();
    let access = access(&empty);
    let before = snapshot(&access);
    assert!(Service::open(access, actor(8)).is_err());
    assert_eq!(snapshot(&reopen(&empty)), before);
}
#[test]
fn native_rows_refuse_unknown_keys_and_stale_grants() {
    let runtime = runtime();
    let service = Service::create(access(&runtime), actor(8), input()).unwrap();
    runtime.close().unwrap();
    runtime.reopen(&[storage_root(1, 1)]).unwrap();
    assert!(matches!(
        service.access.read(|_, _| ()),
        Err(ic_storage::retained::Error::Authority)
    ));
    drop(service);
    let access = access(&runtime);
    access
        .update(|mut view, root| view.insert(root, b"unknown", b"data").unwrap())
        .unwrap();
    assert!(matches!(
        Service::open(access, actor(8)),
        Err(OpenError::Record)
    ));
}

#[test]
fn native_upload_key_cannot_rebind_a_saved_identity() {
    let runtime = runtime();
    let mut service = Service::create(access(&runtime), actor(8), input()).unwrap();
    service.execute(actor(1), authorize(1), 10).unwrap();
    let Observation::Upload(saved) = service
        .inspect(actor(1), Inspection::Upload(id(1)))
        .unwrap()
    else {
        panic!("expected upload");
    };
    drop(service);
    let lease = reopen(&runtime);
    let mut wrong_key = vec![1];
    wrong_key.extend_from_slice(&2u128.to_be_bytes());
    let bytes = ic_wire::encode(
        &(saved, 10u64),
        ic_wire::Encoding::Payload,
        record::STATE_LIMITS,
    )
    .unwrap();
    lease
        .update(|mut view, root| view.insert(root, &wrong_key, &bytes).unwrap())
        .unwrap();
    let before = snapshot(&lease);
    assert!(matches!(
        Service::open(reopen(&runtime), actor(8)),
        Err(OpenError::Record)
    ));
    assert_eq!(snapshot(&reopen(&runtime)), before);
}
