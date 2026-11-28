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
        state_version: 2,
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

#[test]
fn oversized_cell_is_refused_before_domain_decode_without_touching_storage() {
    let backing = VectorMemory::default();
    let cell = StableCell::create(memory(backing.clone()), vec![0_u8; STATE_LIMITS.bytes + 1]);
    drop(cell);
    let before = backing.borrow().clone();
    assert!(matches!(
        Service::open(memory(backing.clone()), actor(8)),
        Err(OpenError::Storage(
            ic_storage::StableCellPayloadError::TooLarge { .. }
        ))
    ));
    assert_eq!(*backing.borrow(), before);
}

#[cfg(feature = "metrics")]
#[test]
fn shared_metrics_follow_committed_commands_and_read_without_recording() {
    use crate::metrics::CommandKind::{Authorize, Register};
    use std::rc::Rc;

    let backing = VectorMemory::default();
    let host = Rc::new(MetricsStorage::default());
    let mut config = input();
    config.limits.uploads = 2.try_into().unwrap();
    let mut service =
        Service::create_with_metrics(memory(backing.clone()), actor(8), config, Rc::clone(&host))
            .unwrap();
    let empty = host.snapshot();
    assert_eq!(service.metrics().unwrap(), empty);

    service.execute(actor(1), authorize(1), 10).unwrap();
    service
        .execute(actor(2), Command::Register(root()), 10)
        .unwrap();
    assert_eq!(
        service.execute(actor(2), Command::Register(root()), 10),
        Err(Error::NotReserved)
    );
    service.execute(actor(1), authorize(2), 10).unwrap();
    assert_eq!(
        service.execute(actor(2), authorize(3), 10),
        Err(Error::Denied)
    );
    assert_eq!(
        service.execute(actor(1), authorize(3), 10),
        Err(Error::Capacity)
    );
    let expected = host.snapshot();
    let registrations = expected.commands(Register);
    assert_eq!(registrations.succeeded.exact_total(), Ok(1));
    assert_eq!(registrations.other_refusals.exact_total(), Ok(1));
    assert_eq!(expected.commands(Authorize).succeeded.exact_total(), Ok(2));
    assert_eq!(expected.commands(Authorize).denied.exact_total(), Ok(1));
    assert_eq!(expected.commands(Authorize).capacity.exact_total(), Ok(1));
    assert_eq!(expected.registered_bytes.bucket_counts(), &[1, 0, 0, 0]);
    assert_eq!(expected.registered_bytes.aggregate().samples(), 1);
    assert_eq!(expected.registered_bytes.aggregate().total(), 3);

    let stable = backing.borrow().clone();
    for _ in 0..2 {
        assert_eq!(service.metrics().unwrap(), expected);
        assert_eq!(host.snapshot(), expected);
        assert_eq!(
            service.inspect(actor(2), Inspection::Usage),
            Err(Error::Denied)
        );
        assert!(matches!(service.inspect(actor(1), Inspection::Usage),
            Ok(Observation::Usage(usage)) if usage.live_bytes == 3 && usage.unresolved_bytes == 3));
    }
    assert_eq!(*backing.borrow(), stable);
    drop(service);
    assert!(matches!(
        Service::create_with_metrics(memory(backing.clone()), actor(8), config, Rc::clone(&host)),
        Err(OpenError::Occupied)
    ));
    assert_eq!(host.snapshot(), expected);
    let plain = Service::open(memory(backing.clone()), actor(8)).unwrap();
    assert_eq!(plain.metrics(), None);
    drop(plain);
    let reopened = Service::open_with_metrics(memory(backing), actor(8), Rc::default()).unwrap();
    assert_eq!(reopened.metrics().unwrap(), empty);
    assert_eq!(host.snapshot(), expected);
}

#[cfg(feature = "logging")]
#[test]
fn blob_and_application_share_one_bounded_host_log() {
    use ic_logging::{EntryInput, Filter, Level, Log, PageRequest};
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };

    let backing = VectorMemory::default();
    let mut config = input();
    config.limits.uploads = 2.try_into().unwrap();
    let limit = 8.try_into().unwrap();
    let host = Rc::new(RefCell::new(Log::new(ic_logging::Limits {
        max_entries: limit,
        max_label_bytes: 16,
        max_message_bytes: 64,
        max_text_bytes: 512,
    })));
    host.borrow_mut()
        .append(EntryInput {
            created_at_secs: 8,
            level: Level::Info,
            source: "app",
            topic: None,
            message: "ready",
        })
        .unwrap();
    let time = Rc::new(Cell::new(9));
    let sink = Rc::clone(&host);
    let clock = Rc::clone(&time);
    let mut service = Service::create(memory(backing.clone()), actor(8), config)
        .unwrap()
        .with_logging(move |level, source, message| {
            let _ = sink.borrow_mut().append(EntryInput {
                created_at_secs: clock.get(),
                level,
                source,
                topic: None,
                message,
            });
        });
    service.execute(actor(1), authorize(1), 10).unwrap();
    service
        .execute(actor(2), Command::Register(root()), 10)
        .unwrap();
    assert_eq!(
        service.execute(actor(2), Command::Register(root()), 10),
        Err(Error::NotReserved)
    );
    service.execute(actor(1), authorize(2), 10).unwrap();
    assert_eq!(
        service.execute(actor(1), authorize(3), 10),
        Err(Error::Capacity)
    );
    for n in [1, 2, 2] {
        service
            .execute(actor(1), Command::Release(id(n)), 10)
            .unwrap();
    }
    for _ in 0..2 {
        service.execute(actor(9), Command::Drain(1), 10).unwrap();
    }
    for _ in 0..2 {
        service.execute(actor(9), Command::Activate(1), 10).unwrap();
    }
    for _ in 0..2 {
        service.inspect(actor(1), Inspection::Usage).unwrap();
        let log = host.borrow();
        let rows = log.page(
            Filter::default(),
            PageRequest {
                before: None,
                max_results: limit,
                max_scanned: limit,
            },
        );
        assert_eq!(
            rows.entries
                .iter()
                .map(|e| (e.sequence, e.level, e.source.as_str(), e.message.as_str()))
                .collect::<Vec<_>>(),
            [
                (5, Level::Info, "blob", "admission reopened"),
                (4, Level::Info, "blob", "admission drained"),
                (3, Level::Info, "blob", "final reference released"),
                (2, Level::Warn, "blob", "capacity limit reached"),
                (1, Level::Info, "blob", "object registered"),
                (0, Level::Info, "app", "ready"),
            ]
        );
    }
    // The host log refuses a backwards timestamp. The durable drain still succeeds.
    time.set(7);
    service.execute(actor(9), Command::Drain(2), 11).unwrap();
    assert!(service.is_drained());
    assert_eq!(host.borrow().len(), 6);
    drop(service);
    let mut service = Service::open(memory(backing), actor(8)).unwrap();
    time.set(10);
    service.execute(actor(9), Command::Activate(2), 12).unwrap();
    assert_eq!(host.borrow().len(), 6);
}
