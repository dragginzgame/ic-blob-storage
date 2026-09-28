//! Real local awaits validate durable authority, not blob transport or sessions.
use super::*;
use blob_test_protocol::{
    admission::{
        ContentLookup,
        input::{ReferenceInput, RetainedDescriptorInput},
    },
    storage::{ProviderFact, gateways::ReadAuthorityInput},
};
impl Fixture {
    fn read_input(&self) -> ReadAuthorityInput {
        self.enroll(None, true).unwrap();
        let (permission, preparation) = self.permission(1, 1);
        self.admit(self.tenant, permission).unwrap();
        self.prepare(&preparation).unwrap();
        self.expose(permission.request).unwrap();
        self.fact(permission.request, ProviderFact::Uploaded)
            .unwrap();
        self.gateways(self.operator, Action::Add(self.other))
            .unwrap();
        ReadAuthorityInput {
            cashier: self.operator,
            gateway: self.other,
            target: RetainedDescriptorInput {
                content: ContentLookup {
                    service: self.service,
                    tenant: self.tenant,
                    namespace: 1,
                    root: permission.request.root,
                },
                object: 1,
                incarnation: 1,
                reference: 1,
            },
        }
    }
    fn check_read_authority(
        &self,
        actor: Principal,
        input: ReadAuthorityInput,
    ) -> Result<(), Failure> {
        self.harness
            .pic
            .update_candid_as(self.service, actor, "fixture_read_authority", (input,))
            .unwrap()
    }
}
#[test]
fn read_authority_rejects_restored_instances_and_wrong_callers_before_sending() {
    let f = Fixture::with_gateway_source();
    let input = f.read_input();
    for actor in [
        f.operator,
        f.controller,
        f.uploader,
        f.other,
        Principal::anonymous(),
    ] {
        assert_eq!(f.check_read_authority(actor, input), Err(Failure::Denied));
    }
    let mut wrong = input;
    wrong.target.incarnation = 2;
    assert_eq!(
        f.check_read_authority(f.tenant, wrong),
        Err(Failure::Unknown)
    );
    assert_eq!(f.source_requests(), 0);
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(f.check_read_authority(f.tenant, input), Ok(()));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(
        f.check_read_authority(f.tenant, input),
        Err(Failure::Fenced)
    );
    assert_eq!(f.source_requests(), 1);
}
#[test]
fn held_read_authority_rejects_remove_readd_and_tenant_reactivation() {
    let f = Fixture::with_gateway_source();
    let input = f.read_input();
    for reactivate_tenant in [false, true] {
        f.source_mode(SourceMode::Hold);
        let call = f
            .harness
            .pic
            .submit_call(
                f.service,
                f.tenant,
                "fixture_read_authority",
                candid::encode_one(input).unwrap(),
            )
            .unwrap();
        f.wait_source();
        assert_eq!(f.check_read_authority(f.tenant, input), Err(Failure::Phase));
        if reactivate_tenant {
            f.enroll(Some(f.tenant().unwrap()), false).unwrap();
            f.enroll(Some(f.tenant().unwrap()), true).unwrap();
        } else {
            f.gateways(f.operator, Action::Remove(f.other)).unwrap();
            f.gateways(f.operator, Action::Add(f.other)).unwrap();
        }
        f.resume_source();
        let result: Result<(), Failure> =
            candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
        assert_eq!(result, Err(Failure::Conflict));
        f.source_mode(SourceMode::Valid);
        assert_eq!(f.check_read_authority(f.tenant, input), Ok(()));
    }
}
#[test]
fn held_read_authority_rejects_exact_reference_release_despite_another_live_reference() {
    let f = Fixture::with_gateway_source();
    let input = f.read_input();
    let request = f.permission(1, 1).0.request;
    f.reference(ReferenceInput {
        object: request,
        reference: 2,
        operation: 1,
        retain: true,
    })
    .unwrap();
    f.source_mode(SourceMode::Hold);
    let call = f
        .harness
        .pic
        .submit_call(
            f.service,
            f.tenant,
            "fixture_read_authority",
            candid::encode_one(input).unwrap(),
        )
        .unwrap();
    f.wait_source();
    f.reference(ReferenceInput {
        object: request,
        reference: 1,
        operation: 2,
        retain: false,
    })
    .unwrap();
    f.resume_source();
    let result: Result<(), Failure> =
        candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
    assert_eq!(result, Err(Failure::Unknown));
    f.source_mode(SourceMode::Valid);
    let mut other = input;
    other.target.reference = 2;
    assert_eq!(f.check_read_authority(f.tenant, other), Ok(()));
}
#[test]
fn gateway_write_rollback_preserves_read_authority_but_same_list_sync_invalidates_it() {
    let f = Fixture::with_gateway_source();
    let input = f.read_input();
    for commit in [false, true] {
        f.source_mode(SourceMode::Hold);
        let call = f
            .harness
            .pic
            .submit_call(
                f.service,
                f.tenant,
                "fixture_read_authority",
                candid::encode_one(input).unwrap(),
            )
            .unwrap();
        f.wait_source();
        if commit {
            let token = f.gateway_begin();
            f.gateways(
                f.operator,
                Action::Apply {
                    token,
                    source: f.gateway_scope(),
                    reply: gateway_reply(&[f.other]),
                },
            )
            .unwrap();
        } else {
            f.gateway_trap(Action::Remove(f.other));
        }
        f.resume_source();
        let result: Result<(), Failure> =
            candid::decode_one(&f.harness.pic.await_call(call).unwrap()).unwrap();
        assert_eq!(
            result,
            if commit {
                Err(Failure::Conflict)
            } else {
                Ok(())
            }
        );
    }
}
