//! Actual IC caller/time composition of the shared owner, without provider effects.
#![cfg(not(target_family = "wasm"))]

mod admission_cases;
mod admission_content;
mod admission_resources;
mod support;
mod vectors;

use blob_test_protocol::admission::{
    Command, Enrollment, Failure, Installation, ManifestState, Observation, Outcome, Permission,
    Phase, Request, Workload,
};
use candid::Principal;
use ic_testkit::{
    Fake,
    pic::CandidCallExt,
    pocket_ic::{CanisterSettings, RejectCode},
};
use std::time::Duration;
use support::{Harness, fixture_path};

struct Fixture {
    harness: Harness,
    service: Principal,
    operator: Principal,
    controller: Principal,
    project: Principal,
    other: Principal,
    uploader: Principal,
}

impl Fixture {
    fn new() -> Self {
        Self::with_workload(Workload::Small)
    }

    fn with_workload(workload: Workload) -> Self {
        let harness = Harness::new();
        let operator = Fake::principal(1);
        let controller = Fake::principal(2);
        let service = harness.pic.create_canister_with_settings(
            Some(controller),
            Some(CanisterSettings {
                controllers: Some(vec![controller]),
                ..CanisterSettings::default()
            }),
        );
        harness.pic.install_canister(
            service,
            Self::wasm(),
            candid::encode_args((Installation { operator, workload },)).unwrap(),
            Some(controller),
        );
        Self {
            harness,
            service,
            operator,
            controller,
            project: Fake::principal(3),
            other: Fake::principal(4),
            uploader: Fake::principal(5),
        }
    }

    fn wasm() -> Vec<u8> {
        std::fs::read(fixture_path("BLOB_ADMISSION_PROBE_WASM")).unwrap()
    }

    fn call(&self, caller: Principal, command: Command) -> Result<Outcome, Failure> {
        self.harness
            .pic
            .update_candid_as(self.service, caller, "execute", (command,))
            .expect("typed update reply, not a trap")
    }

    fn inspect(&self, caller: Principal, request: Request) -> Result<Observation, Failure> {
        self.harness
            .pic
            .query_candid_as(self.service, caller, "inspect", (request,))
            .expect("typed query reply")
    }

    fn enrollment(&self, expected: Option<Enrollment>, active: bool) -> Command {
        Command::Enroll {
            tenant: self.project,
            expected,
            active,
        }
    }

    fn enroll(&self) -> Enrollment {
        let expected = Enrollment {
            generation: 1,
            active: true,
        };
        assert_eq!(
            self.call(self.operator, self.enrollment(None, true)),
            Ok(Outcome::Enrolled(expected))
        );
        expected
    }

    fn permission(&self, vector: &vectors::Vector) -> Permission {
        let nanos = self.harness.pic.get_time().as_nanos_since_unix_epoch();
        Permission {
            request: Request {
                service: self.service,
                tenant: self.project,
                namespace: 1,
                id: u128::from(vector.upload.id),
                root: vector.upload.root,
                bytes: vector.upload.bytes,
            },
            uploader: self.uploader,
            expires_at_ns: nanos + 600_000_000_000,
        }
    }

    fn admit(&self, input: Permission) {
        assert_eq!(
            self.call(self.project, Command::Admit(input)),
            Ok(Outcome::Admitted)
        );
    }

    fn prepare(&self, permission: Permission, vector: &vectors::Vector) {
        assert_eq!(
            self.call(
                self.uploader,
                Command::Prepare(permission.request, vector.manifest.clone())
            ),
            Ok(Outcome::Changed(true))
        );
    }

    fn observe(&self, permission: Permission) -> Observation {
        self.inspect(self.project, permission.request).unwrap()
    }

    fn restart(&self) {
        self.harness
            .pic
            .stop_canister(self.service, Some(self.controller))
            .unwrap();
        self.harness
            .pic
            .start_canister(self.service, Some(self.controller))
            .unwrap();
    }
}

#[test]
fn enrollment_admission_bindings_and_exact_permission_are_enforced() {
    let f = Fixture::new();
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v);
    for caller in [f.controller, f.project, f.uploader, Principal::anonymous()] {
        assert_eq!(
            f.call(caller, f.enrollment(None, true)),
            Err(Failure::NotOperator)
        );
    }
    assert_eq!(
        f.call(f.project, Command::Admit(p)),
        Err(Failure::NotEnrolled)
    );
    f.enroll();
    assert_eq!(
        f.call(f.operator, f.enrollment(None, true)),
        Err(Failure::Conflict)
    );
    for caller in [
        f.controller,
        f.operator,
        f.other,
        f.uploader,
        Principal::anonymous(),
    ] {
        assert_eq!(f.call(caller, Command::Admit(p)), Err(Failure::NotProject));
    }
    for (request, error) in [
        (
            Request {
                service: f.other,
                ..p.request
            },
            Failure::WrongService,
        ),
        (
            Request {
                namespace: 2,
                ..p.request
            },
            Failure::WrongNamespace,
        ),
        (Request { id: 0, ..p.request }, Failure::InvalidInput),
        (
            Request {
                bytes: 10 * 1024 * 1024 + 1,
                ..p.request
            },
            Failure::Capacity,
        ),
    ] {
        assert_eq!(
            f.call(f.project, Command::Admit(Permission { request, ..p })),
            Err(error)
        );
    }
    f.admit(p);
    let original = f.observe(p);
    assert_eq!(original.manifest, ManifestState::Unprepared);
    assert_eq!(
        f.call(f.project, Command::Admit(p)),
        Ok(Outcome::Existing(Phase::Reserved))
    );
    for changed in [
        Permission {
            uploader: f.other,
            ..p
        },
        Permission {
            expires_at_ns: p.expires_at_ns + 1,
            ..p
        },
    ] {
        assert_eq!(
            f.call(f.project, Command::Admit(changed)),
            Err(Failure::Conflict)
        );
    }
    assert_eq!(f.observe(p), original);
}

#[test]
fn actual_uploader_and_observer_roles_gate_manifest_and_exposure() {
    let f = Fixture::new();
    f.enroll();
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v);
    f.admit(p);
    let original = f.observe(p);
    for caller in [
        f.project,
        f.controller,
        f.operator,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(
            f.call(caller, Command::Prepare(p.request, v.manifest.clone())),
            Err(Failure::NotUploader)
        );
        assert_eq!(
            f.call(caller, Command::Expose(p.request.root)),
            Err(Failure::NotUploader)
        );
    }
    for caller in [f.controller, f.operator, f.other, Principal::anonymous()] {
        assert_eq!(f.inspect(caller, p.request), Err(Failure::NotObserver));
        assert_eq!(
            f.call(caller, Command::Revoke(p.request)),
            Err(Failure::NotProject)
        );
    }
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)),
        Err(Failure::Unprepared)
    );
    assert_eq!(f.observe(p), original);
    assert!(f.inspect(f.uploader, p.request).unwrap().usage.is_none());
    f.prepare(p, &v);
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)),
        Ok(Outcome::Exposed)
    );
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)),
        Err(Failure::Phase)
    );
    assert_eq!(
        f.call(f.project, Command::Revoke(p.request)),
        Ok(Outcome::Changed(true))
    );
    let exposed = f.observe(p);
    assert!(exposed.revoked);
    assert_eq!(exposed.phase, Phase::ExposurePossible);
    assert_eq!(exposed.usage, original.usage);
}

#[test]
fn suspension_reactivation_and_real_time_do_not_renew_authority() {
    let f = Fixture::new();
    let active = f.enroll();
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v);
    f.admit(p);
    f.prepare(p, &v);
    let suspended = Enrollment {
        active: false,
        ..active
    };
    assert_eq!(
        f.call(f.operator, f.enrollment(Some(active), false)),
        Ok(Outcome::Enrolled(suspended))
    );
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)),
        Err(Failure::Suspended)
    );
    assert_eq!(
        f.call(f.project, Command::Admit(p)),
        Ok(Outcome::Existing(Phase::Reserved))
    );
    assert_eq!(
        f.call(f.operator, f.enrollment(Some(suspended), true)),
        Ok(Outcome::Enrolled(Enrollment {
            generation: 2,
            active: true
        }))
    );
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, v.manifest.clone())),
        Err(Failure::StalePermission)
    );
    assert_eq!(
        f.call(f.project, Command::Revoke(p.request)),
        Ok(Outcome::Changed(true))
    );
    assert_eq!(f.observe(p).usage.unwrap().logical, 0);

    let second = vectors::vector("media-1048577", 2);
    let q = f.permission(&second);
    f.admit(q);
    f.prepare(q, &second);
    f.harness.pic.advance_time(Duration::from_secs(601));
    assert_eq!(
        f.call(f.uploader, Command::Expose(q.request.root)),
        Err(Failure::Expired)
    );
    assert_eq!(
        f.call(f.project, Command::Admit(q)),
        Ok(Outcome::Existing(Phase::Reserved))
    );
    assert_eq!(f.observe(q).permission, q);
    assert_eq!(
        f.call(f.project, Command::Revoke(q.request)),
        Ok(Outcome::Changed(true))
    );
    assert_eq!(f.observe(q).usage.unwrap().logical, 0);
}
