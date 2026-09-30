//! Real managed completion/reference/delivery boundaries with a labelled local exposure cut.
//! Fixture bytes and exposure are substitutes; no Caffeine request or certificate is issued.
use super::{Fixture, endpoints::manifest, installation::enroll};
use blob_canic_probe::ProbeExposureFailure;
use candid::Principal;
use ic_blob_storage::{
    dto::{
        download::{DownloadFailure, DownloadHeader, DownloadRequest, DownloadResponse},
        operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope},
        reference::{
            ReferenceAction, ReferenceChange, ReferenceCommand, ReferenceFailure,
            ReferenceMutationResponse, ReferenceReceiptLookup,
            capacity::{
                ReferenceCapacityFailure, ReferenceCapacityRequest, ReferenceCapacityResponse,
            },
            status::{ReferenceStatusRequest, ReferenceStatusResponse},
        },
        tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
        upload::{
            admission::{UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest},
            completion::{
                UploadAttestationFailure, UploadAttestationLookup, UploadAttestationMutation,
                UploadAttestationRequest, UploadAttestationResponse, UploadVerificationPlan,
            },
            history::{
                UploadContentState, UploadHistoryFailure, UploadHistoryFilter, UploadHistoryPage,
                UploadHistoryRequest, UploadHistoryScope,
            },
            manifest::{
                UploadManifestFailure, UploadManifestInspection, UploadManifestMutation,
                UploadManifestRequest, UploadManifestResponse,
            },
        },
    },
    model::identity::ContentDigest,
};
use ic_testkit::pic::CandidCallExt;
use std::time::Duration;

struct Journey {
    f: Fixture,
    scope: TenantScope,
    enrollment: TenantEnrollmentResponse,
    input: UploadManifestRequest,
    operator: Principal,
    verifier: Principal,
}
impl Journey {
    fn prepared() -> Self {
        let f = Fixture::new();
        let (scope, enrollment) = enroll(&f);
        let input = manifest(&f, scope.tenant, Principal::from_slice(&[6, 1]));
        let admission: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
            .pic()
            .update_candid_as(
                f.app(),
                scope.tenant,
                "blob_admit_upload",
                (input.permission,),
            )
            .unwrap();
        assert!(!admission.unwrap().replayed);
        let preparation: Result<UploadManifestMutation, UploadManifestFailure> = f
            .pic()
            .update_candid_as(
                f.app(),
                input.permission.uploader,
                "blob_prepare_upload",
                (&input,),
            )
            .unwrap();
        preparation.unwrap();
        Self {
            f,
            scope,
            enrollment,
            input,
            operator: blob_canic_probe::configuration::input().operator,
            verifier: blob_canic_probe::configuration::input().completion_verifier,
        }
    }
    fn operator(&self) -> Principal {
        self.operator
    }
    fn verifier(&self) -> Principal {
        self.verifier
    }
    fn statement(&self) -> UploadAttestationRequest {
        UploadAttestationRequest {
            permission: self.input.permission,
            content_digest: *ContentDigest::compute(&[42; 10]).as_bytes(),
            observed_at_ns: self.f.pic().get_time().as_nanos_since_unix_epoch(),
        }
    }
    fn attest(
        &self,
        actor: Principal,
        input: &UploadAttestationRequest,
    ) -> Result<UploadAttestationMutation, UploadAttestationFailure> {
        self.f
            .pic()
            .update_candid_as(self.f.app(), actor, "blob_attest_upload", (input,))
            .unwrap()
    }
    fn attestation(
        &self,
        actor: Principal,
    ) -> Result<UploadAttestationResponse, UploadAttestationFailure> {
        self.f
            .pic()
            .query_candid_as(
                self.f.app(),
                actor,
                "blob_upload_attestation",
                (self.input.permission,),
            )
            .unwrap()
    }
    fn plan(&self, actor: Principal) -> Result<UploadVerificationPlan, UploadAttestationFailure> {
        self.f
            .pic()
            .query_candid_as(
                self.f.app(),
                actor,
                "blob_verification_plan",
                (self.input.permission,),
            )
            .unwrap()
    }
    fn verification_manifest(
        &self,
        actor: Principal,
    ) -> Result<UploadManifestResponse, UploadManifestFailure> {
        self.f
            .pic()
            .query_candid_as(
                self.f.app(),
                actor,
                "blob_verification_manifest",
                (self.input.permission,),
            )
            .unwrap()
    }
    fn expose(
        &self,
        actor: Principal,
        input: UploadAdmissionRequest,
    ) -> Result<(), ProbeExposureFailure> {
        self.f
            .pic()
            .update_candid_as(self.f.app(), actor, "probe_expose_upload", (input,))
            .unwrap()
    }
    fn request(&self, reference: u128) -> DownloadRequest {
        let u = self.input.permission.upload;
        DownloadRequest {
            service: u.service,
            tenant: u.tenant,
            namespace: u.namespace,
            root: u.root,
            object: u.object,
            incarnation: u.incarnation,
            reference,
        }
    }
    fn download(
        &self,
        actor: Principal,
        input: DownloadRequest,
    ) -> Result<DownloadResponse, DownloadFailure> {
        self.f
            .pic()
            .update_candid_as(self.f.app(), actor, "blob_download_descriptor", (input,))
            .unwrap()
    }
    fn reference(
        &self,
        command: ReferenceCommand,
    ) -> Result<ReferenceMutationResponse, ReferenceFailure> {
        self.f
            .pic()
            .update_candid_as(
                self.f.app(),
                self.scope.tenant,
                "blob_apply_reference",
                (command,),
            )
            .unwrap()
    }
    fn reference_status(&self, reference: u128) -> ReferenceStatusResponse {
        let input = ReferenceStatusRequest {
            upload: self.input.permission.upload,
            reference,
        };
        self.f
            .pic()
            .query_candid_as::<Result<_, ReferenceFailure>, _>(
                self.f.app(),
                self.scope.tenant,
                "blob_reference_status",
                (input,),
            )
            .unwrap()
            .unwrap()
    }
    fn capacity(&self) -> ReferenceCapacityResponse {
        let input = ReferenceCapacityRequest {
            scope: self.scope,
            root: self.input.permission.upload.root,
        };
        self.f
            .pic()
            .query_candid_as::<Result<_, ReferenceCapacityFailure>, _>(
                self.f.app(),
                self.scope.tenant,
                "blob_reference_capacity",
                (input,),
            )
            .unwrap()
            .unwrap()
    }
    fn status(&self) -> LocalServiceStatus {
        let installed = blob_canic_probe::configuration::input();
        let scope = OperatorScope {
            service: self.f.app(),
            namespace: installed.namespace,
            cashier: installed.billing.cashier,
            payment_account: installed.payment_account,
        };
        self.f
            .pic()
            .query_candid_as::<Result<_, LocalStatusFailure>, _>(
                self.f.app(),
                self.operator(),
                "blob_local_status",
                (scope,),
            )
            .unwrap()
            .unwrap()
    }
    fn history(&self) -> UploadHistoryPage {
        let input = UploadHistoryRequest {
            service: self.f.app(),
            namespace: self.scope.namespace,
            scope: UploadHistoryScope::Tenant(self.scope.tenant),
            filter: UploadHistoryFilter::Outstanding,
            cursor: None,
        };
        self.f
            .pic()
            .query_candid_as::<Result<_, UploadHistoryFailure>, _>(
                self.f.app(),
                self.scope.tenant,
                "blob_upload_history",
                (input,),
            )
            .unwrap()
            .unwrap()
    }
    fn enrollment(
        &self,
        expected: TenantEnrollmentResponse,
        active: bool,
    ) -> TenantEnrollmentResponse {
        let input = TenantUpdateRequest {
            scope: self.scope,
            expected: expected.enrollment,
            active,
        };
        self.f
            .pic()
            .update_candid_as::<Result<_, TenantFailure>, _>(
                self.f.app(),
                self.operator(),
                "blob_update_tenant",
                (input,),
            )
            .unwrap()
            .unwrap()
    }
}

fn denied_completion(j: &Journey) {
    let before = j.f.pic().get_stable_memory(j.f.app());
    for actor in [
        j.operator(),
        j.scope.tenant,
        j.input.permission.uploader,
        j.f.root(),
        Principal::anonymous(),
    ] {
        assert_eq!(
            j.attest(actor, &j.statement()),
            Err(UploadAttestationFailure::Denied)
        );
        assert_eq!(j.plan(actor), Err(UploadAttestationFailure::Denied));
        assert_eq!(
            j.verification_manifest(actor),
            Err(UploadManifestFailure::Permission(
                UploadAdmissionFailure::Denied
            ))
        );
        if actor != j.scope.tenant && actor != j.input.permission.uploader {
            assert_eq!(j.attestation(actor), Err(UploadAttestationFailure::Denied));
        }
    }
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
}

fn refused_delivery(j: &Journey, expected: DownloadFailure) {
    let request = j.request(j.input.permission.upload.first_reference);
    let before = j.f.pic().get_stable_memory(j.f.app());
    assert_eq!(j.download(j.scope.tenant, request), Err(expected));
    for actor in [
        j.operator(),
        j.verifier(),
        j.input.permission.uploader,
        j.f.root(),
        Principal::anonymous(),
    ] {
        assert_eq!(j.download(actor, request), Err(DownloadFailure::Denied));
    }
    for invalid in [
        DownloadRequest {
            namespace: 1,
            ..request
        },
        DownloadRequest {
            service: j.operator(),
            ..request
        },
    ] {
        assert_eq!(
            j.download(j.scope.tenant, invalid),
            Err(DownloadFailure::Binding)
        );
    }
    let query =
        j.f.pic()
            .query_call(
                j.f.app(),
                j.scope.tenant,
                "blob_download_descriptor",
                candid::encode_one(request).unwrap(),
            )
            .unwrap_err();
    assert_eq!(
        query.reject_code,
        ic_testkit::pocket_ic::RejectCode::CanisterError
    );
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
}

#[test]
fn managed_verifier_boundaries_refuse_unexposed_uploads_and_preserve_fenced_inspection() {
    let j = Journey::prepared();
    denied_completion(&j);
    refused_delivery(&j, DownloadFailure::Unavailable);
    assert_eq!(
        j.verification_manifest(j.verifier()).unwrap().manifest,
        UploadManifestInspection::Prepared(j.input.declaration.clone())
    );
    assert_eq!(j.plan(j.verifier()), Err(UploadAttestationFailure::Phase));
    let before = j.f.pic().get_stable_memory(j.f.app());
    assert_eq!(
        j.attest(j.verifier(), &j.statement()),
        Err(UploadAttestationFailure::Phase)
    );
    assert_eq!(
        j.attestation(j.verifier()).unwrap().attestation,
        UploadAttestationLookup::Absent
    );
    for actor in [
        j.scope.tenant,
        j.input.permission.uploader,
        j.verifier(),
        j.f.root(),
    ] {
        assert_eq!(
            j.expose(actor, j.input.permission),
            Err(ProbeExposureFailure::Denied)
        );
    }
    assert_eq!(
        j.expose(
            j.operator(),
            UploadAdmissionRequest {
                expires_at_ns: j.input.permission.expires_at_ns - 1,
                ..j.input.permission
            }
        ),
        Err(ProbeExposureFailure::Conflict)
    );
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
    j.f.upgrade_same_release(Duration::from_secs(5));
    let before = j.f.pic().get_stable_memory(j.f.app());
    assert_eq!(
        j.verification_manifest(j.verifier()).unwrap().manifest,
        UploadManifestInspection::Prepared(j.input.declaration.clone())
    );
    assert!(j.attestation(j.verifier()).unwrap().fenced);
    let fenced = UploadAttestationFailure::Permission(UploadAdmissionFailure::Fenced);
    assert_eq!(j.plan(j.verifier()), Err(fenced));
    assert_eq!(j.attest(j.verifier(), &j.statement()), Err(fenced));
    assert_eq!(
        j.expose(j.operator(), j.input.permission),
        Err(ProbeExposureFailure::State)
    );
    refused_delivery(&j, DownloadFailure::Fenced);
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
}

fn accepted_completion(j: &Journey) -> UploadAttestationMutation {
    j.expose(j.operator(), j.input.permission).unwrap();
    let plan = j.plan(j.verifier()).unwrap();
    assert_eq!(plan.permission, j.input.permission);
    assert_eq!(plan.verifier, j.verifier());
    assert_eq!(plan.owner, j.f.app());
    assert_eq!(
        plan.project,
        blob_canic_probe::configuration::input().project
    );
    assert_eq!(plan.declaration, j.input.declaration);
    let statement = j.statement();
    let before = j.f.pic().get_stable_memory(j.f.app());
    for observed_at_ns in [plan.admitted_at_ns - 1, u64::MAX] {
        assert_eq!(
            j.attest(
                j.verifier(),
                &UploadAttestationRequest {
                    observed_at_ns,
                    ..statement
                }
            ),
            Err(UploadAttestationFailure::Observation)
        );
    }
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
    let accepted = j.attest(j.verifier(), &statement).unwrap();
    assert!(accepted.changed);
    assert_eq!(accepted.receipt.request, statement);
    assert_eq!(accepted.receipt.verifier, j.verifier());
    assert!(accepted.receipt.accepted_at_ns >= statement.observed_at_ns);
    let before = j.f.pic().get_stable_memory(j.f.app());
    assert_eq!(
        j.attest(j.verifier(), &statement),
        Ok(UploadAttestationMutation {
            changed: false,
            receipt: accepted.receipt
        })
    );
    assert_eq!(
        j.attest(
            j.verifier(),
            &UploadAttestationRequest {
                content_digest: [0; 32],
                ..statement
            }
        ),
        Err(UploadAttestationFailure::Conflict)
    );
    for actor in [j.verifier(), j.scope.tenant, j.input.permission.uploader] {
        assert_eq!(
            j.attestation(actor),
            Ok(UploadAttestationResponse {
                permission: j.input.permission,
                attestation: UploadAttestationLookup::Found(accepted.receipt),
                fenced: false
            })
        );
    }
    assert_eq!(j.plan(j.verifier()), Err(UploadAttestationFailure::Phase));
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
    accepted
}

fn delivered(j: &Journey, reference: u128) {
    let before = j.f.pic().get_stable_memory(j.f.app());
    let request = j.request(reference);
    assert_eq!(
        j.download(j.scope.tenant, request),
        Ok(DownloadResponse {
            request,
            owner: j.f.app(),
            project: blob_canic_probe::configuration::input().project,
            bytes: 10,
            headers: vec![DownloadHeader {
                name: "Content-Length".into(),
                value: "10".into()
            }]
        })
    );
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
}

fn releases(j: &Journey, accepted: &UploadAttestationMutation) -> ReferenceCommand {
    let first = j.input.permission.upload.first_reference;
    assert!(j.reference_status(first).live);
    delivered(j, first);
    let retain = ReferenceCommand {
        upload: j.input.permission.upload,
        reference: u128::MAX - 4,
        operation: u128::MAX - 5,
        action: ReferenceAction::Retain,
    };
    let retained = j.reference(retain).unwrap();
    assert_eq!(retained.receipt.result, Ok(ReferenceChange::Changed));
    assert!(!retained.replayed);
    assert!(j.reference_status(retain.reference).live);
    assert_eq!(j.capacity().headroom.unwrap().fresh_retains, 0);
    assert_eq!(j.history().entries[0].state, UploadContentState::Live);
    delivered(j, retain.reference);
    let first_release = ReferenceCommand {
        reference: first,
        operation: u128::MAX - 6,
        action: ReferenceAction::Release,
        ..retain
    };
    j.reference(first_release).unwrap();
    assert!(!j.reference_status(first).live);
    assert_eq!(
        j.download(j.scope.tenant, j.request(first)),
        Err(DownloadFailure::Unavailable)
    );
    delivered(j, retain.reference);
    let suspended = j.enrollment(j.enrollment, false);
    refused_delivery(j, DownloadFailure::Inactive);
    let release = ReferenceCommand {
        operation: u128::MAX - 7,
        action: ReferenceAction::Release,
        ..retain
    };
    let released = j.reference(release).unwrap();
    assert_eq!(released.receipt.result, Ok(ReferenceChange::Changed));
    assert!(!j.reference_status(retain.reference).live);
    assert!(j.reference(release).unwrap().replayed);
    assert_eq!(
        j.attest(j.verifier(), &accepted.receipt.request),
        Ok(UploadAttestationMutation {
            changed: false,
            receipt: accepted.receipt
        })
    );
    assert!(!j.reference_status(first).live);
    assert!(!j.reference_status(retain.reference).live);
    j.enrollment(suspended, true);
    refused_delivery(j, DownloadFailure::Unavailable);
    assert_eq!(
        j.history().entries[0].state,
        UploadContentState::DeletionPending
    );
    let status = j.status();
    assert_eq!(
        [
            status.uploads.reserved_bytes,
            status.uploads.logical_bytes,
            status.uploads.physical_bytes,
            status.uploads.liability_bytes
        ],
        [0, 0, 10, 10]
    );
    release
}

fn restored_confirmation(
    j: &Journey,
    accepted: &UploadAttestationMutation,
    release: ReferenceCommand,
) {
    j.f.upgrade_same_release(Duration::from_secs(5));
    let before = j.f.pic().get_stable_memory(j.f.app());
    for actor in [j.verifier(), j.scope.tenant, j.input.permission.uploader] {
        assert_eq!(
            j.attestation(actor),
            Ok(UploadAttestationResponse {
                permission: j.input.permission,
                attestation: UploadAttestationLookup::Found(accepted.receipt),
                fenced: true
            })
        );
    }
    assert_eq!(
        j.verification_manifest(j.verifier()).unwrap().manifest,
        UploadManifestInspection::Prepared(j.input.declaration.clone())
    );
    assert_eq!(
        j.attest(j.verifier(), &accepted.receipt.request),
        Err(UploadAttestationFailure::Permission(
            UploadAdmissionFailure::Fenced
        ))
    );
    assert_eq!(
        j.plan(j.verifier()),
        Err(UploadAttestationFailure::Permission(
            UploadAdmissionFailure::Fenced
        ))
    );
    assert_eq!(j.reference(release), Err(ReferenceFailure::Fenced));
    let receipt: Result<ReferenceReceiptLookup, ReferenceFailure> =
        j.f.pic()
            .query_candid_as(
                j.f.app(),
                j.scope.tenant,
                "blob_reference_receipt",
                (release,),
            )
            .unwrap();
    let ReferenceReceiptLookup::Found(receipt) = receipt.unwrap() else {
        panic!("retained release receipt");
    };
    assert_eq!(receipt.request, release);
    assert_eq!(receipt.result, Ok(ReferenceChange::Changed));
    assert_eq!(
        j.reference_status(release.reference),
        ReferenceStatusResponse {
            request: ReferenceStatusRequest {
                upload: release.upload,
                reference: release.reference
            },
            live: false,
            fenced: true
        }
    );
    assert_eq!(
        j.history().entries[0].state,
        UploadContentState::DeletionPending
    );
    assert!(j.history().fenced);
    assert!(j.capacity().fenced);
    let status = j.status();
    assert_eq!(
        [
            status.uploads.fenced,
            status.funding.fenced,
            status.gateways.fenced,
            status.reads.fenced
        ],
        [true; 4]
    );
    assert_eq!(
        [
            status.uploads.reserved_bytes,
            status.uploads.logical_bytes,
            status.uploads.physical_bytes,
            status.uploads.liability_bytes
        ],
        [0, 0, 10, 10]
    );
    refused_delivery(j, DownloadFailure::Fenced);
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
}

#[test]
fn managed_attestation_reference_delivery_and_cleanup_preserve_receipts_liabilities_and_restore_fences()
 {
    let j = Journey::prepared();
    denied_completion(&j);
    let accepted = accepted_completion(&j);
    denied_completion(&j);
    let release = releases(&j, &accepted);
    restored_confirmation(&j, &accepted, release);
}
