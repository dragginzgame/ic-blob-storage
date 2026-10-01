//! Existing bounded application/outbox probe uses real clients against managed storage.
//! Exposure/completion are labelled local cuts; no deployed Caffeine object exists.
use super::{Fixture, endpoints::manifest};
use blob_test_protocol::consumer::{
    AssetView, Failure, Fault, Recovery, Registration, RegistrationSource, Release, Run, Use,
};
use candid::{CandidType, Principal};
use ic_blob_storage::{
    dto::{
        operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope},
        reference::{
            ReferenceAction, ReferenceChange, ReferenceCommand, ReferenceFailure,
            ReferenceReceiptLookup,
            status::{ReferenceStatusRequest, ReferenceStatusResponse},
        },
        tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
        upload::{
            completion::{
                UploadAttestationFailure, UploadAttestationMutation, UploadAttestationRequest,
            },
            manifest::{UploadManifestFailure, UploadManifestMutation, UploadManifestRequest},
        },
    },
    model::identity::ContentDigest,
};
use ic_testkit::{
    pic::{CandidCallExt, CanisterInstallExt},
    pocket_ic::{CanisterSettings, RejectCode},
};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};
mod interruption;

fn wasm() -> Vec<u8> {
    let path = std::env::var_os("BLOB_CONSUMER_PROBE_WASM").expect("explicit consumer artifact");
    std::fs::read(path).unwrap()
}
fn capture(directory: &Path, name: &str, value: &impl CandidType) {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(directory.join(name))
        .unwrap();
    file.write_all(&candid::encode_one(value).unwrap()).unwrap();
}
struct Application {
    f: Fixture,
    tenant: Principal,
    operator: Principal,
    fresh: Run,
    reuse: Run,
    enrollment: TenantEnrollmentResponse,
    report: PathBuf,
    _temporary: tempfile::TempDir,
}
impl Application {
    fn new(label: &str) -> Self {
        let mut input = blob_canic_probe::configuration::input();
        // The existing probe deliberately checks this exact fixture project.
        "fixture project/β?&=".clone_into(&mut input.project);
        let f = Fixture::with_input(&input);
        let tenant = f.pic().create_canister_with_settings(
            Some(f.root()),
            Some(CanisterSettings {
                controllers: Some(vec![f.root()]),
                ..CanisterSettings::default()
            }),
        );
        f.pic().install_canister(
            tenant,
            wasm(),
            candid::encode_args((input.operator, f.app())).unwrap(),
            Some(f.root()),
        );
        let enrollment = f
            .pic()
            .update_candid_as::<Result<TenantEnrollmentResponse, TenantFailure>, _>(
                f.app(),
                input.operator,
                "blob_update_tenant",
                (TenantUpdateRequest {
                    scope: TenantScope {
                        service: f.app(),
                        namespace: input.namespace,
                        tenant,
                    },
                    expected: None,
                    active: true,
                },),
            )
            .unwrap()
            .unwrap();
        let declaration = manifest(&f, tenant, Principal::from_slice(&[6, 1]));
        let fresh = Run {
            registration: Registration {
                asset: 1,
                payload: vec![1; 8],
                release_operation: u128::MAX - 7,
                source: RegistrationSource::Fresh(declaration.permission),
            },
            fault: Fault::None,
            hold: false,
            max_reply_bytes: 4096,
        };
        let reuse = Run {
            registration: Registration {
                asset: 2,
                payload: vec![2; 8],
                release_operation: u128::MAX - 6,
                source: RegistrationSource::Existing(ReferenceCommand {
                    upload: declaration.permission.upload,
                    reference: u128::MAX - 4,
                    operation: u128::MAX - 5,
                    action: ReferenceAction::Retain,
                }),
            },
            ..fresh.clone()
        };
        let temporary = tempfile::tempdir().unwrap();
        let report = std::env::var_os("BLOB_MANAGED_CONSUMER_REPORT").map_or_else(
            || temporary.path().join(label),
            |root| PathBuf::from(root).join(label),
        );
        std::fs::create_dir(&report).unwrap();
        std::fs::write(report.join("fixture-plan.json"), serde_json::to_vec_pretty(&serde_json::json!({
            "evidence":"local_application_substitute","service":f.app().to_text(),"tenant":tenant.to_text(),
            "operator":input.operator.to_text(),"uploader":declaration.permission.uploader.to_text(),
            "verifier":input.completion_verifier.to_text(),"namespace":input.namespace.to_string(),"project":input.project,
            "max_application_service_updates":64,"max_inspection_queries":64,"max_hold_management_calls":128,
            "object_bytes":"10","provider_gets":0,
            "deployed_provider_requests":0,"attached_provider_cycles":"0","production_consumer":false,
        })).unwrap()).unwrap();
        capture(&report, "fresh-intent.candid", &fresh.registration);
        capture(&report, "reuse-intent.candid", &reuse.registration);
        let app = Self {
            f,
            tenant,
            operator: input.operator,
            fresh,
            reuse,
            enrollment,
            report,
            _temporary: temporary,
        };
        complete(&app, &declaration, input.completion_verifier);
        app
    }
    fn mutate(&self, method: &str, argument: &impl CandidType) -> Result<AssetView, Failure> {
        self.f
            .pic()
            .update_candid_as(self.tenant, self.operator, method, (argument,))
            .unwrap()
    }
    fn trap(&self, method: &str, argument: &impl CandidType) {
        let error = self
            .f
            .pic()
            .update_call(
                self.tenant,
                self.operator,
                method,
                candid::encode_one(argument).unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
    }
    fn asset(&self, id: u128) -> AssetView {
        self.f
            .pic()
            .query_candid_as::<Result<_, Failure>, _>(self.tenant, self.operator, "asset", (id,))
            .unwrap()
            .unwrap()
    }
    fn command(&self, id: u128, release: bool) -> ReferenceCommand {
        let intent = if id == 1 {
            &self.fresh.registration
        } else {
            &self.reuse.registration
        };
        let original = match intent.source {
            RegistrationSource::Fresh(permission) => ReferenceCommand {
                upload: permission.upload,
                reference: permission.upload.first_reference,
                operation: intent.release_operation,
                action: ReferenceAction::Release,
            },
            RegistrationSource::Existing(command) => command,
        };
        if release {
            ReferenceCommand {
                operation: intent.release_operation,
                action: ReferenceAction::Release,
                ..original
            }
        } else {
            original
        }
    }
    fn receipt(&self, id: u128, release: bool) -> ReferenceReceiptLookup {
        self.f
            .pic()
            .query_candid_as::<Result<_, ReferenceFailure>, _>(
                self.f.app(),
                self.tenant,
                "blob_reference_receipt",
                (self.command(id, release),),
            )
            .unwrap()
            .unwrap()
    }
    fn reference(&self, id: u128, live: bool, fenced: bool) {
        let result = self.reference_state(id);
        assert_eq!((result.live, result.fenced), (live, fenced));
    }
    fn reference_state(&self, id: u128) -> ReferenceStatusResponse {
        let command = self.command(id, true);
        self.f
            .pic()
            .query_candid_as::<Result<_, ReferenceFailure>, _>(
                self.f.app(),
                self.tenant,
                "blob_reference_status",
                (ReferenceStatusRequest {
                    upload: command.upload,
                    reference: command.reference,
                },),
            )
            .unwrap()
            .unwrap()
    }
    fn release(&self, id: u128) -> AssetView {
        self.mutate(
            "release",
            &Release {
                asset: id,
                fault: Fault::None,
            },
        )
        .unwrap()
    }
    fn recover(&self, id: u128, release: bool) -> AssetView {
        self.mutate("recover", &Recovery { asset: id, release })
            .unwrap()
    }
    fn status(&self) -> LocalServiceStatus {
        let input = blob_canic_probe::configuration::input();
        self.f
            .pic()
            .query_candid_as::<Result<_, LocalStatusFailure>, _>(
                self.f.app(),
                self.operator,
                "blob_local_status",
                (OperatorScope {
                    service: self.f.app(),
                    namespace: input.namespace,
                    cashier: input.billing.cashier,
                    payment_account: input.payment_account,
                },),
            )
            .unwrap()
            .unwrap()
    }
    fn accounting(&self, fenced: bool) {
        let value = self.status();
        assert_eq!(
            [
                value.uploads.reserved_bytes,
                value.uploads.logical_bytes,
                value.uploads.physical_bytes,
                value.uploads.liability_bytes
            ],
            [0, 0, 10, 10]
        );
        assert_eq!(
            [
                value.uploads.fenced,
                value.funding.fenced,
                value.gateways.fenced,
                value.reads.fenced
            ],
            [fenced; 4]
        );
        capture(
            &self.report,
            if fenced {
                "fenced-accounting.candid"
            } else {
                "released-accounting.candid"
            },
            &value,
        );
    }
}

fn complete(app: &Application, declaration: &UploadManifestRequest, verifier: Principal) {
    let admission = app.mutate("admit", &app.fresh).unwrap();
    capture(&app.report, "admission-ack.candid", &admission);
    capture(&app.report, "prepared-manifest.candid", declaration);
    app.f
        .pic()
        .update_candid_as::<Result<UploadManifestMutation, UploadManifestFailure>, _>(
            app.f.app(),
            declaration.permission.uploader,
            "blob_prepare_upload",
            (declaration,),
        )
        .unwrap()
        .unwrap();
    app.f
        .pic()
        .update_candid_as::<Result<(), blob_canic_probe::ProbeExposureFailure>, _>(
            app.f.app(),
            app.operator,
            "probe_expose_upload",
            (declaration.permission,),
        )
        .unwrap()
        .unwrap();
    let statement = UploadAttestationRequest {
        permission: declaration.permission,
        content_digest: *ContentDigest::compute(&[42; 10]).as_bytes(),
        observed_at_ns: app.f.pic().get_time().as_nanos_since_unix_epoch(),
    };
    let completion = app
        .f
        .pic()
        .update_candid_as::<Result<UploadAttestationMutation, UploadAttestationFailure>, _>(
            app.f.app(),
            verifier,
            "blob_attest_upload",
            (statement,),
        )
        .unwrap()
        .unwrap();
    capture(&app.report, "local-completion.candid", &completion);
}
fn registration(app: &Application) {
    let before = app.f.pic().get_stable_memory(app.f.app());
    for actor in [app.operator, app.f.root()] {
        let refusal: Result<
            ic_blob_storage::dto::reference::ReferenceMutationResponse,
            ReferenceFailure,
        > = app
            .f
            .pic()
            .update_candid_as(
                app.f.app(),
                actor,
                "blob_apply_reference",
                (app.command(2, false),),
            )
            .unwrap();
        assert_eq!(refusal, Err(ReferenceFailure::Denied));
    }
    assert_eq!(app.f.pic().get_stable_memory(app.f.app()), before);
    let first = app.mutate("register", &app.fresh).unwrap();
    assert!(first.published && first.published_once);
    assert!(!first.retain_started);
    assert_eq!(first.registration, app.fresh.registration);
    capture(&app.report, "fresh-published.candid", &first);
    app.reference(1, true, false);
    let attempt = Run {
        fault: Fault::AfterRetain,
        ..app.reuse.clone()
    };
    app.trap("register", &attempt);
    let pending = app.asset(2);
    assert!(pending.retain_started);
    assert!(!pending.published);
    assert_eq!(pending.retain_result, None);
    capture(&app.report, "reuse-pending.candid", &pending);
    assert!(
        matches!(app.receipt(2, false), ReferenceReceiptLookup::Found(r) if r.result == Ok(ReferenceChange::Changed))
    );
    assert!(app.mutate("cancel", &2u128).unwrap().cancelled);
    assert_eq!(
        app.mutate(
            "release",
            &Release {
                asset: 2,
                fault: Fault::None
            }
        ),
        Err(Failure::State)
    );
    let before = app.f.pic().get_stable_memory(app.f.app());
    let recovered = app.recover(2, false);
    assert_eq!(recovered.retain_result, Some(Ok(ReferenceChange::Changed)));
    assert!(recovered.cancelled && !recovered.published_once);
    assert_eq!(app.f.pic().get_stable_memory(app.f.app()), before);
    capture(&app.report, "reuse-recovered.candid", &recovered);
    assert!(!app.mutate("register", &app.reuse).unwrap().published);
    app.reference(2, true, false);
    let capacity: Result<
        ic_blob_storage::dto::reference::capacity::ReferenceCapacityResponse,
        ic_blob_storage::dto::reference::capacity::ReferenceCapacityFailure,
    > = app
        .f
        .pic()
        .query_candid_as(
            app.f.app(),
            app.tenant,
            "blob_reference_capacity",
            (
                ic_blob_storage::dto::reference::capacity::ReferenceCapacityRequest {
                    scope: app.enrollment.scope,
                    root: app.command(2, false).upload.root,
                },
            ),
        )
        .unwrap();
    assert_eq!(capacity.unwrap().headroom.unwrap().fresh_retains, 0);
}
fn cleanup(app: &Application) {
    app.trap(
        "release",
        &Release {
            asset: 2,
            fault: Fault::AfterRelease,
        },
    );
    let pending = app.asset(2);
    assert!(pending.release_started);
    assert_eq!(pending.release_result, None);
    capture(&app.report, "release-pending.candid", &pending);
    app.reference(2, false, false);
    app.mutate(
        "use_asset",
        &Use {
            asset: 1,
            attach: true,
        },
    )
    .unwrap();
    let consumer = app.f.pic().get_stable_memory(app.tenant);
    let service = app.f.pic().get_stable_memory(app.f.app());
    assert_eq!(app.mutate("cancel", &1u128), Err(Failure::State));
    assert_eq!(app.f.pic().get_stable_memory(app.tenant), consumer);
    assert_eq!(app.f.pic().get_stable_memory(app.f.app()), service);
    app.mutate(
        "use_asset",
        &Use {
            asset: 1,
            attach: false,
        },
    )
    .unwrap();
    let cancelled = app.mutate("cancel", &1u128).unwrap();
    assert!(cancelled.cancelled && !cancelled.published && cancelled.published_once);
    app.f
        .pic()
        .update_candid_as::<Result<TenantEnrollmentResponse, TenantFailure>, _>(
            app.f.app(),
            app.operator,
            "blob_update_tenant",
            (TenantUpdateRequest {
                scope: app.enrollment.scope,
                expected: app.enrollment.enrollment,
                active: false,
            },),
        )
        .unwrap()
        .unwrap();
    assert_eq!(
        app.release(1).release_result,
        Some(Ok(ReferenceChange::Changed))
    );
    assert!(!app.mutate("register", &app.fresh).unwrap().published);
    app.reference(1, false, false);
    app.accounting(false);
}
fn restored(app: &Application) {
    let retained = app.receipt(2, false);
    let released = app.receipt(2, true);
    app.f.upgrade_same_release(Duration::from_secs(5));
    let service = app.f.pic().get_stable_memory(app.f.app());
    // A service fence rejects every reference update. Successful recovery through
    // the actual tenant therefore proves passive receipt inspection, not redispatch.
    let recovered = app.recover(2, true);
    assert_eq!(recovered.release_result, Some(Ok(ReferenceChange::Changed)));
    assert!(recovered.cancelled && !recovered.published_once);
    assert_eq!(app.release(2), recovered);
    assert_eq!(app.f.pic().get_stable_memory(app.f.app()), service);
    assert_eq!(app.receipt(2, false), retained);
    assert_eq!(app.receipt(2, true), released);
    capture(
        &app.report,
        "fenced-service-release-recovery.candid",
        &recovered,
    );
    app.reference(1, false, true);
    app.reference(2, false, true);
    app.accounting(true);
    let first = app.asset(1);
    app.f
        .pic()
        .wait_out_install_code_rate_limit(Duration::from_secs(5));
    app.f
        .pic()
        .upgrade_canister(
            app.tenant,
            wasm(),
            candid::encode_args(()).unwrap(),
            Some(app.f.root()),
        )
        .unwrap();
    let consumer = app.f.pic().get_stable_memory(app.tenant);
    assert_eq!(app.asset(1), first);
    assert_eq!(app.asset(2), recovered);
    let fenced: Result<bool, Failure> = app
        .f
        .pic()
        .query_candid_as(app.tenant, app.operator, "fenced", ())
        .unwrap();
    assert_eq!(fenced, Ok(true));
    for (method, argument) in [
        ("register", candid::encode_one(&app.fresh).unwrap()),
        (
            "release",
            candid::encode_one(Release {
                asset: 1,
                fault: Fault::None,
            })
            .unwrap(),
        ),
        (
            "recover",
            candid::encode_one(Recovery {
                asset: 2,
                release: true,
            })
            .unwrap(),
        ),
    ] {
        let result: Result<AssetView, Failure> = candid::decode_one(
            &app.f
                .pic()
                .update_call(app.tenant, app.operator, method, argument)
                .unwrap(),
        )
        .unwrap();
        assert_eq!(result, Err(Failure::Fenced));
    }
    assert_eq!(app.f.pic().get_stable_memory(app.tenant), consumer);
    assert_eq!(app.f.pic().get_stable_memory(app.f.app()), service);
    capture(&app.report, "restored-fresh.candid", &app.asset(1));
    capture(&app.report, "restored-reuse.candid", &app.asset(2));
}

#[test]
fn managed_application_outbox_recovers_callbacks_and_preserves_cleanup_under_fences() {
    let app = Application::new("outbox");
    registration(&app);
    cleanup(&app);
    restored(&app);
}

#[test]
fn managed_application_cancel_race_blocks_late_publication_and_new_uses() {
    let app = Application::new("cancel-race");
    assert!(app.mutate("register", &app.fresh).unwrap().published);
    let waiting = Run {
        hold: true,
        ..app.reuse.clone()
    };
    let call = app
        .f
        .pic()
        .submit_call(
            app.tenant,
            app.operator,
            "register",
            candid::encode_one(waiting).unwrap(),
        )
        .unwrap();
    let mut held = false;
    for _ in 0..40 {
        app.f.pic().tick();
        let result: Result<bool, Failure> = app
            .f
            .pic()
            .query_candid_as(app.tenant, app.operator, "waiting", ())
            .unwrap();
        if result.unwrap() {
            held = true;
            break;
        }
    }
    assert!(
        held,
        "application reaches the post-descriptor publication hold"
    );
    let acquired = app.asset(2);
    assert!(!acquired.published);
    assert_eq!(acquired.retain_result, Some(Ok(ReferenceChange::Changed)));
    capture(&app.report, "held-registration.candid", &acquired);
    assert!(app.mutate("cancel", &2u128).unwrap().cancelled);
    assert_eq!(
        app.mutate(
            "use_asset",
            &Use {
                asset: 2,
                attach: true
            }
        ),
        Err(Failure::State)
    );
    assert_eq!(
        app.release(2).release_result,
        Some(Ok(ReferenceChange::Changed))
    );
    app.reference(2, false, false);
    let resumed: Result<(), Failure> = app
        .f
        .pic()
        .update_candid_as(app.tenant, app.operator, "resume", ())
        .unwrap();
    resumed.unwrap();
    let result: Result<AssetView, Failure> =
        candid::decode_one(&app.f.pic().await_call(call).unwrap()).unwrap();
    assert_eq!(result, Err(Failure::State));
    capture(&app.report, "late-registration-result.candid", &result);
    let cancelled = app.mutate("register", &app.reuse).unwrap();
    assert!(cancelled.cancelled && !cancelled.published && !cancelled.published_once);
    capture(&app.report, "race-tombstone.candid", &cancelled);
    app.reference(1, true, false);
    assert!(app.asset(1).published);
    app.mutate("cancel", &1u128).unwrap();
    app.release(1);
    app.accounting(false);
    restored(&app);
}
