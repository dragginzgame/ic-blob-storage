use super::*;
use crate::support::REPLY_LIMITS;
use blob_test_protocol::storage::reference::ReferenceFaultInput;
use ic_blob_storage_contracts::dto::reference::*;
use ic_blob_storage_contracts::protocol::REFERENCE_APPLY_METHOD;
use ic_blob_storage_contracts::protocol::REFERENCE_RECEIPT_METHOD;
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
mod receipts;
mod status;
use blob_test_protocol::{
    admission::input::ReferenceInput,
    storage::{FactInput, ProviderFact},
};
impl Fixture {
    fn exposed(&self) -> Permission {
        self.enroll(None, true).unwrap();
        let (permission, manifest) = self.permission(1, 1);
        self.admit(self.tenant, permission).unwrap();
        self.prepare(&manifest).unwrap();
        self.expose(permission.request).unwrap();
        permission
    }
    pub(super) fn fact(&self, request: Request, fact: ProviderFact) -> Result<bool, Failure> {
        self.harness
            .pic
            .update_control_as(
                self.service,
                self.operator,
                "fixture_provider_fact",
                (FactInput {
                    request,
                    fact,
                    fault: None,
                },),
                REPLY_LIMITS.reply_bytes,
            )
            .map(|(reply,)| reply)
            .unwrap()
    }
    pub(super) fn reference(
        &self,
        request: ReferenceInput,
    ) -> Result<ReferenceMutationResponse, ReferenceFailure> {
        self.harness
            .pic
            .update_blob_as(
                self.service,
                self.tenant,
                REFERENCE_APPLY_METHOD,
                receipt_request(request),
                REPLY_LIMITS.reply_bytes,
            )
            .unwrap()
    }
    pub(super) fn receipt(
        &self,
        request: ReferenceInput,
    ) -> Result<ReferenceReceiptLookup, ReferenceFailure> {
        self.harness
            .pic
            .query_blob_as(
                self.service,
                self.tenant,
                REFERENCE_RECEIPT_METHOD,
                receipt_request(request),
                REPLY_LIMITS.reply_bytes,
            )
            .unwrap()
    }
    pub(super) fn live(&self, input: ReferenceInput) -> Result<bool, ReferenceFailure> {
        use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest;
        use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusResponse;
        let command = receipt_request(input);
        let request = ReferenceStatusRequest {
            upload: command.upload,
            reference: command.reference,
        };
        let result: Result<ReferenceStatusResponse, ReferenceFailure> = self
            .harness
            .pic
            .query_blob_as(
                self.service,
                self.tenant,
                "blob_reference_status",
                request,
                REPLY_LIMITS.reply_bytes,
            )
            .unwrap();
        result.map(|response| {
            assert_eq!(response.request, request);
            assert_eq!(response.fenced, self.status().fenced);
            response.live
        })
    }
}
fn reference(object: Request, operation: u128, id: u128, retain: bool) -> ReferenceInput {
    ReferenceInput {
        object,
        reference: id,
        operation,
        retain,
    }
}
fn changed(request: ReferenceInput, replayed: bool) -> ReferenceMutationResponse {
    ReferenceMutationResponse {
        replayed,
        receipt: ReferenceReceiptResponse {
            request: receipt_request(request),
            result: Ok(ReferenceChange::Changed),
        },
    }
}
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered reference/receipt/settlement rollback journey shares the same constrained history"
)]
fn reference_receipt_and_last_release_faults_roll_back_all_obligations() {
    let f = Fixture::new();
    let permission = f.exposed();
    let retain = reference(permission.request, u128::MAX, 2, true);
    let before = f.status();
    for fault in [WriteFault::Receipts, WriteFault::Confirmed] {
        let error = f
            .harness
            .pic
            .update_call(
                f.service,
                f.tenant,
                "fixture_apply_reference_with_write_trap",
                ic_blob_storage_pocketic_tests::control_calls::encode_control_one(
                    ReferenceFaultInput {
                        request: receipt_request(retain),
                        fault,
                    },
                )
                .unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
        assert_eq!(f.status(), before);
        assert_eq!(f.receipt(retain), Ok(ReferenceReceiptLookup::Absent));
        assert_eq!(f.live(retain), Ok(false));
    }
    assert_eq!(f.reference(retain), Ok(changed(retain, false)));
    assert_eq!(f.reference(retain), Ok(changed(retain, true)));
    assert_eq!(
        f.reference(reference(permission.request, 4, 9, false)),
        Err(ReferenceFailure::Capacity)
    );
    let active = f.tenant().unwrap();
    f.enroll(Some(active), false).unwrap();
    assert_eq!(f.reference(retain), Ok(changed(retain, true)));
    assert_eq!(
        f.reference(reference(permission.request, 4, 3, true)),
        Err(ReferenceFailure::Inactive)
    );
    f.reference(reference(permission.request, 2, 1, false))
        .unwrap();
    let release = reference(permission.request, 3, 2, false);
    let error = f
        .harness
        .pic
        .update_call(
            f.service,
            f.tenant,
            "fixture_apply_reference_with_write_trap",
            ic_blob_storage_pocketic_tests::control_calls::encode_control_one(
                ReferenceFaultInput {
                    request: receipt_request(release),
                    fault: WriteFault::Usage,
                },
            )
            .unwrap(),
        )
        .unwrap_err();
    assert_eq!(error.reject_code, RejectCode::CanisterError);
    assert_eq!(f.receipt(release), Ok(ReferenceReceiptLookup::Absent));
    assert_eq!(f.live(release), Ok(true));
    assert_eq!(
        f.status().usage,
        JourneyUsage {
            logical: 10,
            physical: 10,
            liability: 10
        }
    );
    assert_eq!(f.reference(release), Ok(changed(release, false)));
    assert_eq!(
        f.status().usage,
        JourneyUsage {
            logical: 0,
            physical: 10,
            liability: 10
        }
    );
    assert_eq!(
        f.fact(permission.request, ProviderFact::Settled),
        Err(Failure::Phase)
    );
    for fact in [ProviderFact::Deleted, ProviderFact::Settled] {
        let before = f.status();
        let error = f
            .harness
            .pic
            .update_call(
                f.service,
                f.operator,
                "fixture_provider_fact",
                ic_blob_storage_pocketic_tests::control_calls::encode_control_one(FactInput {
                    request: permission.request,
                    fact,
                    fault: Some(WriteFault::Usage),
                })
                .unwrap(),
            )
            .unwrap_err();
        assert_eq!(error.reject_code, RejectCode::CanisterError);
        assert_eq!(f.status(), before);
        assert_eq!(f.fact(permission.request, fact), Ok(true));
        assert_eq!(f.fact(permission.request, fact), Ok(false));
    }
    assert_eq!(
        f.status().usage,
        JourneyUsage {
            logical: 0,
            physical: 0,
            liability: 0
        }
    );
    assert_eq!(f.reference(retain), Ok(changed(retain, true)));
    assert_eq!(f.live(retain), Ok(false));
    assert_eq!(f.live(retain), Ok(false));
}
#[test]
fn upgrade_retains_each_confirmed_phase_and_receipts_under_the_restore_fence() {
    for stage in 0..4 {
        let f = Fixture::new();
        let permission = f.exposed();
        let release = reference(permission.request, 1, 1, false);
        if stage > 0 {
            f.reference(release).unwrap();
        }
        if stage > 1 {
            f.fact(permission.request, ProviderFact::Deleted).unwrap();
        }
        if stage > 2 {
            f.fact(permission.request, ProviderFact::Settled).unwrap();
        }
        let before = f.status();
        let receipt = f.receipt(release).unwrap();
        f.harness
            .pic
            .upgrade_canister(
                f.service,
                Fixture::wasm(),
                Fixture::installation(f.operator),
                Some(f.controller),
            )
            .unwrap();
        assert_eq!(
            f.status(),
            Status {
                fenced: true,
                ..before
            }
        );
        assert_eq!(f.receipt(release), Ok(receipt));
        assert_eq!(f.live(release), Ok(stage == 0));
        assert_eq!(f.reference(release), Err(ReferenceFailure::Fenced));
        for fact in [ProviderFact::Deleted, ProviderFact::Settled] {
            assert_eq!(f.fact(permission.request, fact), Err(Failure::Fenced));
        }
        let denied: Result<ReferenceReceiptLookup, ReferenceFailure> = f
            .harness
            .pic
            .query_blob_as(
                f.service,
                f.controller,
                REFERENCE_RECEIPT_METHOD,
                receipt_request(release),
                REPLY_LIMITS.reply_bytes,
            )
            .unwrap();
        assert_eq!(denied, Err(ReferenceFailure::Denied));
        f.restart();
        assert_eq!(f.receipt(release), Ok(receipt));
    }
}

// This probe's admission fixture deliberately uses upload == object and first == 1.
// The maintained receipt boundary carries those identities independently.
fn receipt_request(input: ReferenceInput) -> ReferenceCommand {
    ReferenceCommand {
        upload: ReferenceUpload {
            service: input.object.service.into(),
            tenant: input.object.tenant.into(),
            namespace: input.object.namespace,
            upload: input.object.id,
            object: input.object.id,
            incarnation: 1,
            first_reference: 1,
            root: input.object.root,
            bytes: input.object.bytes,
        },
        reference: input.reference,
        operation: input.operation,
        action: if input.retain {
            ReferenceAction::Retain
        } else {
            ReferenceAction::Release
        },
    }
}
