//! Managed operator queries over the existing labelled local source, never deployed Caffeine.
use super::{Fixture, endpoints::manifest, installation::enroll, wasm};
use blob_test_protocol::{SourceMode, balance::BalanceSourceConfig};
use candid::Principal;
use canic::dto::abi::v1::CanisterInitPayload;
use ic_blob_storage::dto::{
    account::{
        AccountInspectionFailure as AccountError, AccountInspectionKind, AccountInspectionRequest,
        AccountInspectionResponse, AccountObservation,
    },
    funding::outcome::FundingReportedBalance,
    gateway::{
        GatewayRevocationFailure as RevokeError, GatewayRevocationRequest,
        GatewayRevocationResponse,
        sync::{GatewaySyncCancellation, GatewaySyncFailure as SyncError, GatewaySyncResponse},
    },
    operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope},
    upload::{
        admission::{UploadAdmissionFailure, UploadAdmissionMutation},
        manifest::{UploadManifestFailure, UploadManifestMutation},
    },
};
use ic_testkit::pic::{CandidCallExt, CanisterInstallExt};
use std::time::Duration;

struct Journey {
    f: Fixture,
    scope: OperatorScope,
    operator: Principal,
    driver: Principal,
    gateway: Principal,
}
impl Journey {
    fn new() -> Self {
        let f = Fixture::new();
        let cashier = f.pic().create_canister();
        let (payload, _): (CanisterInitPayload, Option<Vec<u8>>) =
            candid::decode_args(&f.arguments()).unwrap();
        let mut input = blob_canic_probe::configuration::input();
        input.billing.cashier = cashier;
        // Provision only this fresh empty local installation: no tenants, objects,
        // provider effects or obligations exist. Preserve the protected envelope.
        f.pic()
            .wait_out_install_code_rate_limit(Duration::from_secs(5));
        f.pic()
            .reinstall_canister(
                f.app(),
                wasm(),
                candid::encode_args((payload, Some(candid::encode_one(&input).unwrap()))).unwrap(),
                Some(f.root()),
            )
            .unwrap();
        f.configure_and_wait_until_active(16);
        let driver = Principal::from_slice(&[9, 1]);
        let gateway = Principal::from_slice(&[10, 1]);
        let source = std::fs::read(
            std::env::var_os("BLOB_GATEWAY_SOURCE_WASM").expect("explicit local source artifact"),
        )
        .unwrap();
        f.pic().install_canister(
            cashier,
            source,
            candid::encode_args((f.app(), gateway, driver)).unwrap(),
            None,
        );
        let scope = OperatorScope {
            service: f.app(),
            namespace: input.namespace,
            cashier,
            payment_account: input.payment_account,
        };
        let (tenant, _) = enroll(&f);
        let declaration = manifest(&f, tenant.tenant, Principal::from_slice(&[6, 1]));
        f.pic()
            .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
                f.app(),
                tenant.tenant,
                "blob_admit_upload",
                (declaration.permission,),
            )
            .unwrap()
            .unwrap();
        f.pic()
            .update_candid_as::<Result<UploadManifestMutation, UploadManifestFailure>, _>(
                f.app(),
                declaration.permission.uploader,
                "blob_prepare_upload",
                (&declaration,),
            )
            .unwrap()
            .unwrap();
        Self {
            f,
            scope,
            operator: input.operator,
            driver,
            gateway,
        }
    }
    fn status(&self) -> LocalServiceStatus {
        self.f
            .pic()
            .query_candid_as::<Result<_, LocalStatusFailure>, _>(
                self.f.app(),
                self.operator,
                "blob_local_status",
                (self.scope,),
            )
            .unwrap()
            .unwrap()
    }
    fn sync(
        &self,
        actor: Principal,
        scope: OperatorScope,
    ) -> Result<GatewaySyncResponse, SyncError> {
        self.f
            .pic()
            .update_candid_as(self.f.app(), actor, "blob_sync_gateways", (scope,))
            .unwrap()
    }
    fn cancel(
        &self,
        actor: Principal,
        scope: OperatorScope,
        sequence: u64,
    ) -> Result<(), SyncError> {
        self.f
            .pic()
            .update_candid_as(
                self.f.app(),
                actor,
                "blob_cancel_gateway_sync",
                (GatewaySyncCancellation { scope, sequence },),
            )
            .unwrap()
    }
    fn revoke(
        &self,
        actor: Principal,
        scope: OperatorScope,
    ) -> Result<GatewayRevocationResponse, RevokeError> {
        self.f
            .pic()
            .update_candid_as(
                self.f.app(),
                actor,
                "blob_revoke_gateway",
                (GatewayRevocationRequest {
                    scope,
                    gateway: self.gateway,
                },),
            )
            .unwrap()
    }
    fn account(
        &self,
        actor: Principal,
        scope: OperatorScope,
        kind: AccountInspectionKind,
    ) -> Result<AccountInspectionResponse, AccountError> {
        self.f
            .pic()
            .update_candid_as(
                self.f.app(),
                actor,
                "blob_inspect_account",
                (AccountInspectionRequest { scope, kind },),
            )
            .unwrap()
    }
    fn mode(&self, selected: SourceMode) {
        assert!(
            self.f
                .pic()
                .update_candid_as::<bool, _>(
                    self.scope.cashier,
                    self.driver,
                    "configure",
                    (selected,)
                )
                .unwrap()
        );
    }
    fn account_reply(&self, kind: AccountInspectionKind, bytes: Vec<u8>) {
        let account = match kind {
            AccountInspectionKind::Balance => self.scope.payment_account,
            AccountInspectionKind::PaymentRelationship => self.scope.service,
        };
        assert!(
            self.f
                .pic()
                .update_candid_as::<bool, _>(
                    self.scope.cashier,
                    self.driver,
                    "configure_balance",
                    (BalanceSourceConfig {
                        account,
                        bytes,
                        reject: false,
                        hold: false
                    },)
                )
                .unwrap()
        );
    }
}

fn authority(j: &Journey) {
    let before = j.f.pic().get_stable_memory(j.f.app());
    let source = j.f.pic().get_stable_memory(j.scope.cashier);
    for actor in [
        j.f.root(),
        j.scope.payment_account,
        Principal::from_slice(&[5, 1]),
        Principal::from_slice(&[6, 1]),
        Principal::anonymous(),
    ] {
        assert_eq!(j.sync(actor, j.scope), Err(SyncError::Denied));
        assert_eq!(j.cancel(actor, j.scope, 1), Err(SyncError::Denied));
        assert_eq!(j.revoke(actor, j.scope), Err(RevokeError::Denied));
        assert_eq!(
            j.account(actor, j.scope, AccountInspectionKind::Balance),
            Err(AccountError::Denied)
        );
    }
    for scope in [
        OperatorScope {
            service: j.driver,
            ..j.scope
        },
        OperatorScope {
            namespace: 1,
            ..j.scope
        },
        OperatorScope {
            cashier: j.driver,
            ..j.scope
        },
        OperatorScope {
            payment_account: j.driver,
            ..j.scope
        },
    ] {
        assert_eq!(j.sync(j.operator, scope), Err(SyncError::Binding));
        assert_eq!(j.cancel(j.operator, scope, 1), Err(SyncError::Binding));
        assert_eq!(j.revoke(j.operator, scope), Err(RevokeError::Binding));
        assert_eq!(
            j.account(j.operator, scope, AccountInspectionKind::Balance),
            Err(AccountError::Binding)
        );
    }
    assert_eq!(j.cancel(j.operator, j.scope, 0), Err(SyncError::Invalid));
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
    assert_eq!(j.f.pic().get_stable_memory(j.scope.cashier), source);
}

#[derive(candid::CandidType)]
struct PaddedScope {
    service: Principal,
    namespace: u128,
    cashier: Principal,
    payment_account: Principal,
    padding: Vec<u8>,
}
fn bounded_sync(j: &Journey) {
    let before = j.f.pic().get_stable_memory(j.f.app());
    let source = j.f.pic().get_stable_memory(j.scope.cashier);
    let mut input = PaddedScope {
        service: j.scope.service,
        namespace: 1,
        cashier: j.scope.cashier,
        payment_account: j.scope.payment_account,
        padding: vec![0; 4096],
    };
    let overhead = candid::encode_one(&input).unwrap().len() - 4096;
    input.padding.truncate(4096 - overhead);
    let bytes = candid::encode_one(&input).unwrap();
    assert_eq!(bytes.len(), 4096);
    // Valid Candid with a skippable field must reach scope validation at the bound.
    let reply =
        j.f.pic()
            .update_call(j.f.app(), j.operator, "blob_sync_gateways", bytes)
            .unwrap();
    assert_eq!(
        candid::decode_one::<Result<GatewaySyncResponse, SyncError>>(&reply).unwrap(),
        Err(SyncError::Binding)
    );
    input.padding.push(0);
    let bytes = candid::encode_one(input).unwrap();
    assert_eq!(bytes.len(), 4097);
    let error =
        j.f.pic()
            .update_call(j.f.app(), j.operator, "blob_sync_gateways", bytes)
            .unwrap_err();
    assert_eq!(
        error.reject_code,
        ic_testkit::pocket_ic::RejectCode::CanisterReject
    );
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
    assert_eq!(j.f.pic().get_stable_memory(j.scope.cashier), source);
}

fn gateways(j: &Journey) {
    let baseline = j.status();
    let source = j.f.pic().get_stable_memory(j.scope.cashier);
    assert_eq!(
        j.sync(j.operator, j.scope),
        Ok(GatewaySyncResponse {
            scope: j.scope,
            sequence: 1
        })
    );
    assert_eq!(j.status().gateways.members, vec![j.gateway]);
    assert_eq!(j.status().gateways.pending_sequence, None);
    assert_eq!(j.f.pic().get_stable_memory(j.scope.cashier), source);
    assert!(j.revoke(j.operator, j.scope).unwrap().removed);
    assert!(!j.revoke(j.operator, j.scope).unwrap().removed);
    assert!(j.status().gateways.members.is_empty());
    assert_eq!(j.sync(j.operator, j.scope).unwrap().sequence, 2);
    for (mode, expected) in [
        (SourceMode::Malformed, SyncError::InvalidReply),
        (SourceMode::Oversized, SyncError::ReplyTooLarge),
    ] {
        j.mode(mode);
        assert_eq!(j.sync(j.operator, j.scope), Err(expected));
        let pending = j.status();
        let sequence = pending.gateways.pending_sequence.unwrap();
        assert_eq!(pending.gateways.members, vec![j.gateway]);
        assert_eq!(pending.uploads, baseline.uploads);
        assert_eq!(pending.funding, baseline.funding);
        assert_eq!(pending.reads, baseline.reads);
        let before = j.f.pic().get_stable_memory(j.f.app());
        assert_eq!(j.sync(j.operator, j.scope), Err(SyncError::Busy));
        assert_eq!(
            j.cancel(j.operator, j.scope, sequence - 1),
            Err(SyncError::Conflict)
        );
        assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
        assert_eq!(j.cancel(j.operator, j.scope, sequence), Ok(()));
        assert_eq!(
            j.cancel(j.operator, j.scope, sequence),
            Err(SyncError::Conflict)
        );
    }
}

fn passive_accounts(j: &Journey) {
    use AccountInspectionKind::{Balance, PaymentRelationship};
    let bytes = candid_parser::parse_idl_args(&format!(r#"(variant {{ Ok = record {{ account = principal "{}"; account_cycle_balances = record {{ total = 340282366920938463463374607431768211455 : int; cycles_prepaid = 2 : int; cycles_promo = 3 : int; cycles_ledger = 4 : int; debt_target = variant {{ Prepaid }} }} }} }})"#, j.scope.payment_account)).unwrap().to_bytes().unwrap();
    j.account_reply(Balance, bytes);
    let before = j.f.pic().get_stable_memory(j.f.app());
    let source = j.f.pic().get_stable_memory(j.scope.cashier);
    assert_eq!(
        j.account(j.operator, j.scope, Balance),
        Ok(AccountInspectionResponse {
            request: AccountInspectionRequest {
                scope: j.scope,
                kind: Balance
            },
            observation: AccountObservation::Balance(FundingReportedBalance {
                total: u128::MAX,
                prepaid: 2,
                promotional: 3,
                ledger: 4
            })
        })
    );
    assert_eq!(j.f.pic().get_stable_memory(j.scope.cashier), source);
    let bytes = candid_parser::parse_idl_args("(variant { Ok = record { relationship = null } })")
        .unwrap()
        .to_bytes()
        .unwrap();
    j.account_reply(PaymentRelationship, bytes);
    assert_eq!(
        j.account(j.operator, j.scope, PaymentRelationship)
            .unwrap()
            .observation,
        AccountObservation::NoRelationshipReported
    );
    let bytes = candid_parser::parse_idl_args("(variant { Err = variant { AccountNotFound } })")
        .unwrap()
        .to_bytes()
        .unwrap();
    j.account_reply(Balance, bytes);
    assert_eq!(
        j.account(j.operator, j.scope, Balance).unwrap().observation,
        AccountObservation::AccountNotFound
    );
    for (bytes, expected) in [
        (vec![0], AccountError::InvalidReply),
        (vec![0; 4097], AccountError::ReplyTooLarge),
    ] {
        j.account_reply(Balance, bytes);
        assert_eq!(j.account(j.operator, j.scope, Balance), Err(expected));
    }
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
}

fn restored(j: &Journey) {
    j.mode(SourceMode::Reject);
    assert_eq!(j.sync(j.operator, j.scope), Err(SyncError::Rejected(4)));
    let status = j.status();
    let pending = status.gateways.pending_sequence.unwrap();
    j.f.upgrade_same_release(Duration::from_secs(5));
    let before = j.f.pic().get_stable_memory(j.f.app());
    let source = j.f.pic().get_stable_memory(j.scope.cashier);
    assert_eq!(j.sync(j.operator, j.scope), Err(SyncError::Fenced));
    assert_eq!(
        j.cancel(j.operator, j.scope, pending),
        Err(SyncError::Fenced)
    );
    assert_eq!(j.revoke(j.operator, j.scope), Err(RevokeError::Fenced));
    for kind in [
        AccountInspectionKind::Balance,
        AccountInspectionKind::PaymentRelationship,
    ] {
        assert_eq!(
            j.account(j.operator, j.scope, kind),
            Err(AccountError::Fenced)
        );
    }
    let restored = j.status();
    assert_eq!(restored.gateways.pending_sequence, Some(pending));
    assert_eq!(
        restored.gateways.last_sequence,
        status.gateways.last_sequence
    );
    assert_eq!(restored.gateways.members, status.gateways.members);
    assert_eq!(restored.uploads.reserved_bytes, 10);
    assert_eq!(
        [
            restored.uploads.fenced,
            restored.funding.fenced,
            restored.gateways.fenced,
            restored.reads.fenced
        ],
        [true; 4]
    );
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
    assert_eq!(j.f.pic().get_stable_memory(j.scope.cashier), source);
}

#[test]
fn managed_operator_sync_account_and_revocation_preserve_scope_pending_identity_and_fenced_owners()
{
    let j = Journey::new();
    authority(&j);
    bounded_sync(&j);
    gateways(&j);
    passive_accounts(&j);
    restored(&j);
}
