//! Actual IC caller/time composition of the shared owner, without provider effects.
#![cfg(not(target_family = "wasm"))]

use ic_blob_storage_contracts::dto::upload::{
    admission::UploadAdmissionFailure as AdmissionFailure,
    exposure::UploadExposureFailure as ExposureFailure,
};
use ic_blob_storage_pocketic_tests::fixture_calls;
mod admission_cases;
mod admission_content;
mod admission_resources;
mod admission_wire;
use ic_blob_storage_pocketic_tests::installation;
mod support;
use ic_blob_storage_pocketic_tests::service_calls::canister_rejection;
mod vectors;

use blob_test_protocol::admission::{
    Command, Enrollment, Failure, ManifestState, Observation, Outcome, Permission, Phase, Request,
    Workload,
};
use ic_platform::principal::ExternalPrincipal as Principal;
use ic_testkit::fake_principal;
use ic_testkit::pocket_ic::RejectCode;
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
    installation: blob_test_protocol::storage::resources::StorageProbeInstallation,
}

impl Fixture {
    async fn new() -> Self {
        Self::with_workload(Workload::Small).await
    }

    async fn with_workload(workload: Workload) -> Self {
        let harness = Harness::new().await;
        let operator = fake_principal(1);
        let controller = fake_principal(2);
        let service = installation::controlled_canister(&harness.pic, controller).await;
        let installation = blob_test_protocol::storage::resources::StorageProbeInstallation {
            operator,
            resources: blob_test_protocol::storage::resources::admission_resources(workload),
            max_funding_attempts: 4,
        };
        harness
            .pic
            .install_canister(
                service,
                Self::wasm(),
                blob_test_protocol::fixture_wire::encode_args("fixture_init", (installation,))
                    .unwrap(),
                Some(controller),
            )
            .await
            .unwrap();
        Self {
            harness,
            installation,
            service,
            operator,
            controller,
            project: fake_principal(3),
            other: fake_principal(4),
            uploader: fake_principal(5),
        }
    }

    fn wasm() -> Vec<u8> {
        std::fs::read(fixture_path("BLOB_STORAGE_PROBE_WASM")).unwrap()
    }

    async fn call(&self, caller: Principal, command: Command) -> Result<Outcome, Failure> {
        let (method, bytes) = admission_wire::encode(command);
        let reply = self
            .harness
            .pic
            .update_call(self.service, caller, method, bytes)
            .await
            .expect("typed update reply, not a trap");
        blob_test_protocol::fixture_wire::decode_one(
            method,
            &reply,
            blob_test_protocol::fixture_wire::MAX_BYTES,
        )
        .expect("typed update reply")
    }

    async fn inspect(&self, caller: Principal, request: Request) -> Result<Observation, Failure> {
        fixture_calls::query_fixture(
            &self.harness.pic,
            self.service,
            caller,
            "fixture_admission_observation",
            (request,),
        )
        .await
        .map(|(reply,)| reply)
        .expect("typed query reply")
    }

    fn enrollment(&self, expected: Option<Enrollment>, active: bool) -> Command {
        Command::Enroll {
            tenant: self.project.into(),
            expected,
            active,
        }
    }

    async fn enroll(&self) -> Enrollment {
        let expected = Enrollment {
            generation: 1,
            active: true,
        };
        assert_eq!(
            self.call(self.operator, self.enrollment(None, true)).await,
            Ok(Outcome::Enrolled(expected))
        );
        expected
    }

    async fn permission(&self, vector: &vectors::Vector) -> Permission {
        let nanos = self
            .harness
            .pic
            .get_time()
            .await
            .unwrap()
            .as_nanos_since_unix_epoch();
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

    async fn admit(&self, input: Permission) {
        assert_eq!(
            self.call(self.project, Command::Admit(input)).await,
            Ok(Outcome::Admitted)
        );
    }

    async fn prepare(&self, permission: Permission, vector: &vectors::Vector) {
        assert_eq!(
            self.call(
                self.uploader,
                Command::Prepare(permission.request, vector.manifest.clone())
            )
            .await,
            Ok(Outcome::Changed(true))
        );
    }

    async fn observe(&self, permission: Permission) -> Observation {
        self.inspect(self.project, permission.request)
            .await
            .unwrap()
    }

    async fn restart(&self) {
        self.harness
            .pic
            .stop_canister(self.service, Some(self.controller))
            .await
            .unwrap();
        self.harness
            .pic
            .start_canister(self.service, Some(self.controller))
            .await
            .unwrap();
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn enrollment_admission_bindings_and_exact_permission_are_enforced() {
    let f = Fixture::new().await;
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v).await;
    for caller in [f.controller, f.project, f.uploader, Principal::anonymous()] {
        assert_eq!(
            f.call(caller, f.enrollment(None, true)).await,
            Err(Failure::NotOperator)
        );
    }
    assert_eq!(
        f.call(f.project, Command::Admit(p)).await,
        Err(Failure::NotEnrolled)
    );
    f.enroll().await;
    assert_eq!(
        f.call(f.operator, f.enrollment(None, true)).await,
        Err(Failure::Conflict)
    );
    for caller in [
        f.controller,
        f.operator,
        f.other,
        f.uploader,
        Principal::anonymous(),
    ] {
        assert_eq!(
            f.call(caller, Command::Admit(p)).await,
            Err(Failure::NotProject)
        );
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
            f.call(f.project, Command::Admit(Permission { request, ..p }))
                .await,
            Err(error)
        );
    }
    f.admit(p).await;
    let original = f.observe(p).await;
    assert_eq!(original.manifest, ManifestState::Unprepared);
    assert_eq!(
        f.call(f.project, Command::Admit(p)).await,
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
            f.call(f.project, Command::Admit(changed)).await,
            Err(Failure::Conflict)
        );
    }
    assert_eq!(f.observe(p).await, original);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn actual_uploader_and_observer_roles_gate_manifest_and_exposure() {
    let f = Fixture::new().await;
    f.enroll().await;
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v).await;
    f.admit(p).await;
    let original = f.observe(p).await;
    for caller in [
        f.project,
        f.controller,
        f.operator,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(
            f.call(caller, Command::Prepare(p.request, v.manifest.clone()))
                .await,
            Err(Failure::NotUploader)
        );
        assert_eq!(
            f.call(caller, Command::Expose(p.request.root)).await,
            Err(Failure::Exposure(ExposureFailure::Permission(
                AdmissionFailure::Denied
            )))
        );
    }
    for caller in [f.controller, f.operator, f.other, Principal::anonymous()] {
        assert_eq!(
            f.inspect(caller, p.request).await,
            Err(Failure::NotObserver)
        );
        assert_eq!(
            f.call(caller, Command::Revoke(p.request)).await,
            Err(Failure::NotProject)
        );
    }
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)).await,
        Err(Failure::Exposure(ExposureFailure::Unprepared))
    );
    assert_eq!(f.observe(p).await, original);
    assert!(
        f.inspect(f.uploader, p.request)
            .await
            .unwrap()
            .usage
            .is_none()
    );
    f.prepare(p, &v).await;
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)).await,
        Ok(Outcome::Exposed)
    );
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)).await,
        Err(Failure::Exposure(ExposureFailure::Phase))
    );
    assert_eq!(
        f.call(f.project, Command::Revoke(p.request)).await,
        Ok(Outcome::Changed(true))
    );
    let exposed = f.observe(p).await;
    assert!(exposed.revoked);
    assert_eq!(exposed.phase, Phase::Registered);
    assert_eq!(exposed.usage, original.usage);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn suspension_reactivation_and_real_time_do_not_renew_authority() {
    let f = Fixture::new().await;
    let active = f.enroll().await;
    let v = vectors::vector("abc-text", 1);
    let p = f.permission(&v).await;
    f.admit(p).await;
    f.prepare(p, &v).await;
    let suspended = Enrollment {
        active: false,
        ..active
    };
    assert_eq!(
        f.call(f.operator, f.enrollment(Some(active), false)).await,
        Ok(Outcome::Enrolled(suspended))
    );
    assert_eq!(
        f.call(f.uploader, Command::Expose(p.request.root)).await,
        Err(Failure::Exposure(ExposureFailure::Permission(
            AdmissionFailure::Inactive
        )))
    );
    assert_eq!(
        f.call(f.project, Command::Admit(p)).await,
        Ok(Outcome::Existing(Phase::Reserved))
    );
    assert_eq!(
        f.call(f.operator, f.enrollment(Some(suspended), true))
            .await,
        Ok(Outcome::Enrolled(Enrollment {
            generation: 2,
            active: true
        }))
    );
    assert_eq!(
        f.call(f.uploader, Command::Prepare(p.request, v.manifest.clone()))
            .await,
        Err(Failure::StalePermission)
    );
    assert_eq!(
        f.call(f.project, Command::Revoke(p.request)).await,
        Ok(Outcome::Changed(true))
    );
    assert_eq!(f.observe(p).await.usage.unwrap().logical, 0);

    let second = vectors::vector("media-1048577", 2);
    let q = f.permission(&second).await;
    f.admit(q).await;
    f.prepare(q, &second).await;
    f.harness
        .pic
        .advance_time(Duration::from_secs(601))
        .await
        .unwrap();
    assert_eq!(
        f.call(f.uploader, Command::Expose(q.request.root)).await,
        Err(Failure::Exposure(ExposureFailure::Permission(
            AdmissionFailure::Expired
        )))
    );
    assert_eq!(
        f.call(f.project, Command::Admit(q)).await,
        Ok(Outcome::Existing(Phase::Reserved))
    );
    assert_eq!(f.observe(q).await.permission, q);
    assert_eq!(
        f.call(f.project, Command::Revoke(q.request)).await,
        Ok(Outcome::Changed(true))
    );
    assert_eq!(f.observe(q).await.usage.unwrap().logical, 0);
}
