//! Existing application substitute and Rust-owned Candid for the browser handshake.
use super::{BrowserDriver, BrowserPreparation, Fixture};
use blob_test_protocol::consumer::{
    AssetView, Failure, Fault, Registration, RegistrationSource, Revocation, Run,
};
use candid::Principal;
use ic_agent::{Identity, identity::BasicIdentity};
use ic_blob_storage::{
    dto::{
        operator::{LocalServiceStatus, LocalStatusFailure, OperatorScope},
        tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
        upload::{
            UploadState,
            admission::{
                UploadAdmissionFailure as A, UploadAdmissionMutation, UploadAdmissionResponse,
            },
            manifest::{UploadManifestFailure, UploadManifestHeader, UploadManifestRequest},
        },
    },
    model::identity::caffeine::{
        CaffeineHashLimits, CaffeineHeader,
        manifest::{CaffeineManifestLimits, builder::CaffeineManifestBuilder},
    },
    ops::{
        caffeine::preparation::{PreparedManifestLimits, decode_prepared_manifest},
        service::uploads::manifests::reply::{self, UploadManifestReplyLimits},
    },
};
use ic_testkit::{pic::CandidCallExt, pocket_ic::CanisterSettings};
use serde_json::{Value, json};
use std::num::{NonZeroU64, NonZeroUsize};

pub(super) struct Application {
    pub(super) f: Fixture,
    pub(super) tenant: Principal,
    pub(super) request: UploadManifestRequest,
    pub(super) root: String,
}
impl Application {
    pub(super) fn new() -> Self {
        let f = Fixture::new();
        let uploader = BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap();
        let tenant = f.pic().create_canister_with_settings(
            Some(f.root()),
            Some(CanisterSettings {
                controllers: Some(vec![f.root()]),
                ..CanisterSettings::default()
            }),
        );
        f.pic().install_canister(
            tenant,
            std::fs::read(
                std::env::var_os("BLOB_CONSUMER_PROBE_WASM").expect("explicit consumer artifact"),
            )
            .unwrap(),
            candid::encode_args((uploader, f.app())).unwrap(),
            Some(f.root()),
        );
        let config = blob_canic_probe::configuration::input();
        f.pic()
            .update_candid_as::<Result<TenantEnrollmentResponse, TenantFailure>, _>(
                f.app(),
                config.operator,
                "blob_update_tenant",
                (TenantUpdateRequest {
                    scope: TenantScope {
                        service: f.app(),
                        namespace: config.namespace,
                        tenant,
                    },
                    expected: None,
                    active: true,
                },),
            )
            .unwrap()
            .unwrap();
        let mut request = super::super::endpoints::manifest(&f, tenant, uploader);
        let mut builder = CaffeineManifestBuilder::new(
            10,
            &[
                CaffeineHeader {
                    name: "Content-Length",
                    value: "10",
                },
                CaffeineHeader {
                    name: "Content-Type",
                    value: "image/png",
                },
            ],
            CaffeineHashLimits {
                max_content_bytes: NonZeroU64::new(10).unwrap(),
                max_append_bytes: NonZeroUsize::new(10).unwrap(),
                max_headers: NonZeroUsize::new(8).unwrap(),
                max_header_bytes: NonZeroUsize::new(1024).unwrap(),
            },
            NonZeroUsize::new(2).unwrap(),
        )
        .unwrap();
        builder.append(0, &[42; 10]).unwrap();
        let prepared = builder.finish().unwrap();
        request.permission.upload.root = *prepared.hashes().provider_root.as_bytes();
        request.declaration.headers.push(UploadManifestHeader {
            name: "Content-Type".into(),
            value: "image/png".into(),
        });
        Self {
            f,
            tenant,
            request,
            root: prepared.hashes().provider_root.to_string(),
        }
    }
    pub(super) fn handshake(
        &self,
        prepared: &BrowserPreparation,
        driver: &mut BrowserDriver,
    ) -> Run {
        assert_eq!(prepared.hash, self.root);
        assert_eq!(prepared.byte_length, self.request.permission.upload.bytes);
        let declaration = decode_prepared_manifest(
            self.root.parse().unwrap(),
            prepared.byte_length,
            prepared.manifest_json.as_bytes(),
            PreparedManifestLimits {
                max_json_bytes: NonZeroUsize::new(4096).unwrap(),
                manifest: limits(),
            },
        )
        .unwrap();
        assert_eq!(declaration, self.request.declaration);
        let run = Run {
            registration: Registration {
                asset: u128::MAX,
                payload: vec![1; 8],
                source: RegistrationSource::Fresh(self.request.permission),
                release_operation: u128::MAX - 7,
            },
            fault: Fault::None,
            hold: false,
            max_reply_bytes: 4096,
        };
        driver.send(&json!({ "verifyAuthority": true,
            "admission": candid::encode_one(&run).unwrap(),
            "permission": candid::encode_one(self.request.permission).unwrap(),
            "preparation": candid::encode_one(&self.request).unwrap(),
        }));
        let replies: SetupReplies = driver.read(32768);
        let authority = replies.authority;
        assert_eq!(
            decode::<Result<AssetView, Failure>>(&authority.foreign_admission),
            Err(Failure::Denied)
        );
        assert_eq!(
            decode::<Result<UploadAdmissionMutation, A>>(&authority.direct_admission),
            Err(A::Denied)
        );
        assert_eq!(
            reply::mutation(
                &self.request,
                &authority.foreign_preparation,
                reply_limits()
            ),
            Err(reply::UploadManifestReplyError::Remote(
                UploadManifestFailure::Permission(A::Denied)
            ))
        );
        let admitted: Result<AssetView, Failure> = decode(&replies.admission);
        let admitted = admitted.unwrap();
        assert_eq!(admitted.registration, run.registration);
        assert!(admitted.admission_started && !admitted.published);
        assert_eq!(admitted.admission_result, Some(Ok(())));
        let prepared =
            reply::mutation(&self.request, &replies.preparation, reply_limits()).unwrap();
        assert!(prepared.changed);
        self.verify_unexposed(&run);
        run
    }
    pub(super) fn consumer_commands(run: &Run) -> Value {
        json!({ "registration": candid::encode_one(run).unwrap(),
            "asset": candid::encode_one(run.registration.asset).unwrap(),
            "revocation": candid::encode_one(Revocation { asset: run.registration.asset,
                fault: Fault::None, max_reply_bytes: 4096 }).unwrap() })
    }
    pub(super) fn verify_unexposed(&self, run: &Run) {
        let view = self.asset(run);
        assert_eq!(view.registration, run.registration);
        assert!(!view.cancelled && !view.published && !view.retain_started);
        assert_eq!(view.upload_state, Some(UploadState::Reserved));
        assert_eq!(self.admission().state, UploadState::Reserved);
        assert!(!self.admission().revoked);
        let status = self.status();
        assert_eq!(status.uploads.reserved_bytes, 10);
        // Global capacity includes reservations before any certificate escapes.
        assert_eq!(status.uploads.logical_bytes, 10);
        assert_eq!(status.uploads.physical_bytes, 10);
        assert_eq!(status.uploads.liability_bytes, 10);
    }
    pub(super) fn verify_cleanup(&self, run: &Run, value: &Value) {
        let replies: ConsumerReplies = serde_json::from_value(value.clone()).unwrap();
        let pending = decode::<Result<AssetView, Failure>>(&replies.pending).unwrap();
        assert_eq!(pending.registration, run.registration);
        assert_eq!(pending.upload_state, Some(UploadState::Reserved));
        assert!(!pending.cancelled && !pending.published && !pending.retain_started);
        let cancelled = decode::<Result<AssetView, Failure>>(&replies.cancelled).unwrap();
        assert!(cancelled.cancelled && !cancelled.revocation_started);
        let withdrawn = decode::<Result<AssetView, Failure>>(&replies.withdrawn).unwrap();
        assert_eq!(withdrawn.registration, run.registration);
        assert!(withdrawn.cancelled && withdrawn.revocation_started && !withdrawn.published);
        assert_eq!(withdrawn.revocation_result, Some(Ok(())));
        assert_eq!(self.asset(run), withdrawn);
        let admission = self.admission();
        assert_eq!(admission.permission, self.request.permission);
        assert_eq!(admission.state, UploadState::Cancelled);
        assert!(admission.revoked);
        let status = self.status();
        assert_eq!(status.uploads.operations, 1);
        assert_eq!(status.uploads.active_reservations, 0);
        assert_eq!(status.uploads.reserved_bytes, 0);
        assert_eq!(status.uploads.logical_bytes, 0);
        assert_eq!(status.uploads.physical_bytes, 0);
        assert_eq!(status.uploads.liability_bytes, 0);
    }
    fn asset(&self, run: &Run) -> AssetView {
        self.f
            .pic()
            .query_candid_as::<Result<_, Failure>, _>(
                self.tenant,
                self.request.permission.uploader,
                "asset",
                (run.registration.asset,),
            )
            .unwrap()
            .unwrap()
    }
    pub(super) fn admission(&self) -> UploadAdmissionResponse {
        self.f
            .pic()
            .query_candid_as::<Result<_, A>, _>(
                self.f.app(),
                self.tenant,
                "blob_upload_admission",
                (self.request.permission,),
            )
            .unwrap()
            .unwrap()
    }
    pub(super) fn status(&self) -> LocalServiceStatus {
        let config = blob_canic_probe::configuration::input();
        self.f
            .pic()
            .query_candid_as::<Result<_, LocalStatusFailure>, _>(
                self.f.app(),
                config.operator,
                "blob_local_status",
                (OperatorScope {
                    service: self.f.app(),
                    namespace: config.namespace,
                    cashier: config.billing.cashier,
                    payment_account: config.payment_account,
                },),
            )
            .unwrap()
            .unwrap()
    }
}
fn limits() -> CaffeineManifestLimits {
    CaffeineManifestLimits {
        max_content_bytes: NonZeroU64::new(10).unwrap(),
        max_chunks: NonZeroUsize::MIN,
        max_headers: NonZeroUsize::new(8).unwrap(),
        max_header_bytes: NonZeroUsize::new(1024).unwrap(),
    }
}
fn reply_limits() -> UploadManifestReplyLimits {
    UploadManifestReplyLimits {
        max_reply_bytes: NonZeroUsize::new(4096).unwrap(),
        declaration: limits(),
    }
}
fn decode<T: for<'a> candid::Deserialize<'a> + candid::CandidType>(bytes: &[u8]) -> T {
    assert!(bytes.len() <= 4096);
    candid::decode_one(bytes).unwrap()
}
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AuthorityReplies {
    foreign_admission: Vec<u8>,
    direct_admission: Vec<u8>,
    foreign_preparation: Vec<u8>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SetupReplies {
    authority: AuthorityReplies,
    admission: Vec<u8>,
    preparation: Vec<u8>,
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ConsumerReplies {
    pending: Vec<u8>,
    cancelled: Vec<u8>,
    withdrawn: Vec<u8>,
}
