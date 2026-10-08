//! Standalone Wasm installation, maintained endpoints and synchronous restore fencing.
#![cfg(not(target_family = "wasm"))]
mod account_native_cli;
mod authenticated_cli;
mod browser_driver;
mod funding_assessment_cli;
mod gateway_native_cli;
mod native_session;
mod reference_cli;
mod snapshots;
mod standalone_account;
mod standalone_browser;
mod standalone_capacity;
mod standalone_certificate;
mod standalone_certificate_cli;
mod standalone_cli;
mod standalone_completion;
mod standalone_discovery;
mod standalone_download;
mod standalone_funding;
mod standalone_funding_cli;
mod standalone_gateways;
mod standalone_hard_cut;
mod standalone_history;
mod standalone_history_cli;
mod standalone_ingress;
mod standalone_installation_cli;
mod standalone_lifecycle;
mod standalone_operator;
mod standalone_publish_check;
mod standalone_publish_map;
mod standalone_publish_prepare;
mod standalone_publish_session;
mod standalone_reference_capacity;
mod standalone_reference_native_cli;
mod standalone_reference_status;
mod standalone_snapshot;
mod standalone_upload_setup;
mod standalone_upload_status;
mod standalone_verify_upload;
mod submission_proxy;
mod support;
mod upload_setup_cli;
use candid::Principal;
use ic_blob_storage_contracts::dto::configuration::HostConfigurationView;
use ic_blob_storage_contracts::dto::configuration::HostFailure;
use ic_blob_storage_contracts::dto::configuration::ServiceBillingInput;
use ic_blob_storage_contracts::dto::configuration::ServiceConfigurationInput;
use ic_blob_storage_contracts::dto::configuration::ServiceFundingInput;
use ic_blob_storage_contracts::dto::configuration::ServiceInstallationInput;
use ic_blob_storage_contracts::dto::configuration::ServiceReadInput;
use ic_blob_storage_contracts::dto::configuration::ServiceResourceInput;
use ic_blob_storage_contracts::dto::reference::ReferenceAction;
use ic_blob_storage_contracts::dto::reference::ReferenceCommand;
use ic_blob_storage_contracts::dto::reference::ReferenceFailure;
use ic_blob_storage_contracts::dto::reference::ReferenceMutationResponse;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::tenant::TenantEnrollmentResponse;
use ic_blob_storage_contracts::dto::tenant::TenantFailure;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::tenant::TenantUpdateRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionMutation;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse;
use ic_blob_storage_contracts::dto::upload::admission::UploadRevocationResponse;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestDeclaration;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestHeader;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestMutation;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHashLimits;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHeader;
use ic_blob_storage_contracts::identity::caffeine::manifest::builder::CaffeineManifestBuilder;
use ic_testkit::{
    Fake,
    pic::CandidCallExt,
    pocket_ic::{CanisterSettings, RejectCode},
};
use support::{Harness, fixture_path};

struct Fixture {
    harness: Harness,
    service: Principal,
    controller: Principal,
    operator: Principal,
    tenant: Principal,
    uploader: Principal,
    config: ServiceConfigurationInput,
}
#[derive(Clone, Copy)]
enum Envelope {
    Regular,
    Single,
    Serial {
        max_object_bytes: u64,
        total_bytes: u64,
    },
}
fn wasm() -> Vec<u8> {
    std::fs::read(fixture_path("BLOB_STANDALONE_WASM")).unwrap()
}
fn unchanged(actual: &[u8], expected: &[u8]) {
    assert_eq!(actual.len(), expected.len(), "byte length");
    assert_eq!(
        actual.iter().zip(expected).position(|(a, b)| a != b),
        None,
        "first changed byte offset"
    );
}
impl Fixture {
    fn new() -> Self {
        Self::with_harness(Harness::new())
    }
    fn with_harness(harness: Harness) -> Self {
        Self::with_cashier(harness, Fake::principal(5))
    }
    fn with_cashier(harness: Harness, cashier: Principal) -> Self {
        Self::with_cashier_and_operator(harness, cashier, Fake::principal(2))
    }
    fn with_cashier_and_operator(
        harness: Harness,
        cashier: Principal,
        operator: Principal,
    ) -> Self {
        Self::with_profile(
            harness,
            cashier,
            operator,
            Fake::principal(4),
            Envelope::Regular,
            Fake::principal(90),
            PROJECT,
        )
    }
    fn small(harness: Harness, uploader: Principal) -> Self {
        Self::with_profile(
            harness,
            Fake::principal(5),
            Fake::principal(2),
            uploader,
            Envelope::Single,
            Fake::principal(90),
            PROJECT,
        )
    }
    #[expect(
        clippy::too_many_lines,
        reason = "One fixture constructor binds the envelope, principals and installed service"
    )]
    fn with_profile(
        harness: Harness,
        cashier: Principal,
        operator: Principal,
        uploader: Principal,
        envelope: Envelope,
        verifier: Principal,
        project: &str,
    ) -> Self {
        let controller = Fake::principal(1);
        let service = harness.pic.create_canister_with_settings(
            Some(controller),
            Some(CanisterSettings {
                controllers: Some(vec![controller]),
                ..CanisterSettings::default()
            }),
        );
        let mut config = ServiceConfigurationInput {
            service,
            operator,
            payment_account: service,
            namespace: u128::MAX,
            resources: ServiceResourceInput {
                max_tenants: 2,
                max_object_bytes: 10 * 1024 * 1024,
                max_headers: 8,
                max_header_bytes: 1024,
                max_chunks: 20,
                max_tenant_chunks: 20,
                max_objects: 2,
                max_tenant_objects: 2,
                max_physical_bytes: 20 * 1024 * 1024,
                max_liability_bytes: 20 * 1024 * 1024,
                max_tenant_logical_bytes: 20 * 1024 * 1024,
                max_references_per_object: 2,
                max_receipts_per_object: 3,
                max_active: 2,
                max_tenant_active: 2,
            },
            billing: ServiceBillingInput {
                cashier,
                reserve: 1,
                minimum_balance: 10,
                target_balance: 100,
                max_gateway_entries: 8,
                max_gateway_unique: 4,
            },
            funding: ServiceFundingInput {
                allocated: u128::from(u64::MAX) + 1,
                renewal_ceiling: u128::from(u64::MAX) + 1,
                reserve: 100,
                max_attempts: 4,
            },
            reads: ServiceReadInput {
                sessions: 1,
                tenant_sessions: 1,
                reply_bytes: 2048,
                bytes: 2048,
                tenant_bytes: 2048,
            },
        };
        if matches!(envelope, Envelope::Single | Envelope::Serial { .. }) {
            let r = &mut config.resources;
            r.max_tenants = 1;
            r.max_object_bytes = 1024;
            r.max_objects = 1;
            r.max_tenant_objects = 1;
            r.max_physical_bytes = 1024;
            r.max_liability_bytes = 1024;
            r.max_tenant_logical_bytes = 1024;
            r.max_references_per_object = 1;
            r.max_receipts_per_object = 2;
            r.max_active = 1;
            r.max_tenant_active = 1;
            if let Envelope::Serial {
                max_object_bytes,
                total_bytes,
            } = envelope
            {
                r.max_object_bytes = max_object_bytes;
                r.max_objects = 2;
                r.max_tenant_objects = 2;
                r.max_physical_bytes = u128::from(total_bytes);
                r.max_liability_bytes = u128::from(total_bytes);
                r.max_tenant_logical_bytes = u128::from(total_bytes);
                r.max_references_per_object = 2;
                r.max_receipts_per_object = 4;
                config.reads.bytes = total_bytes;
                config.reads.tenant_bytes = total_bytes;
            }
        }
        let mut input: ServiceInstallationInput =
            candid::decode_one(&installation(&config)).unwrap();
        input.trusted_uploader = uploader;
        input.completion_verifier = verifier;
        input.project = project.into();
        harness.pic.install_canister(
            service,
            wasm(),
            candid::encode_one(input).unwrap(),
            Some(controller),
        );
        Self {
            harness,
            service,
            controller,
            operator,
            tenant: Fake::principal(3),
            uploader,
            config,
        }
    }
    fn configuration(&self, actor: Principal) -> Result<HostConfigurationView, HostFailure> {
        self.harness
            .pic
            .query_candid_as(self.service, actor, "blob_configuration", ())
            .unwrap()
    }
    fn scope(&self) -> TenantScope {
        TenantScope {
            service: self.service,
            namespace: self.config.namespace,
            tenant: self.tenant,
        }
    }
    fn enroll(&self, actor: Principal) -> Result<TenantEnrollmentResponse, TenantFailure> {
        self.harness
            .pic
            .update_candid_as(
                self.service,
                actor,
                "blob_update_tenant",
                (TenantUpdateRequest {
                    scope: self.scope(),
                    expected: None,
                    active: true,
                },),
            )
            .unwrap()
    }
    fn tenant(&self) -> TenantEnrollmentResponse {
        self.harness
            .pic
            .query_candid_as::<Result<TenantEnrollmentResponse, TenantFailure>, _>(
                self.service,
                self.tenant,
                "blob_tenant",
                (self.scope(),),
            )
            .unwrap()
            .unwrap()
    }
    fn admission(&self, input: UploadAdmissionRequest) -> UploadAdmissionResponse {
        self.harness
            .pic
            .query_candid_as::<Result<UploadAdmissionResponse, UploadAdmissionFailure>, _>(
                self.service,
                self.tenant,
                "blob_upload_admission",
                (input,),
            )
            .unwrap()
            .unwrap()
    }
    fn prepare(
        &self,
        actor: Principal,
        input: &UploadManifestRequest,
    ) -> Result<UploadManifestMutation, UploadManifestFailure> {
        self.harness
            .pic
            .update_candid_as(self.service, actor, "blob_prepare_upload", (input,))
            .unwrap()
    }
    fn upgrade(&self, args: Vec<u8>) -> Result<(), ic_testkit::pocket_ic::RejectResponse> {
        self.harness
            .pic
            .upgrade_canister(self.service, wasm(), args, Some(self.controller))
    }
    fn resume(
        &self,
        actor: Principal,
    ) -> Result<(), ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure> {
        self.harness
            .pic
            .update_candid_as(self.service, actor, "blob_resume_current_instance", ())
            .unwrap()
    }
    fn manifest(&self) -> UploadManifestRequest {
        self.manifest_bytes(10 * 1024 * 1024)
    }
    fn small_manifest(&self) -> UploadManifestRequest {
        self.manifest_bytes(1024)
    }
    fn manifest_bytes(&self, bytes: u64) -> UploadManifestRequest {
        self.manifest_body(
            &vec![42; usize::try_from(bytes).unwrap()],
            "image/png",
            None,
        )
    }
    fn manifest_body(
        &self,
        body: &[u8],
        content_type: &str,
        cache_control: Option<&str>,
    ) -> UploadManifestRequest {
        let bytes = u64::try_from(body.len()).unwrap();
        let length = bytes.to_string();
        let mut headers = vec![
            CaffeineHeader {
                name: "Content-Length",
                value: &length,
            },
            CaffeineHeader {
                name: "Content-Type",
                value: content_type,
            },
        ];
        if let Some(value) = cache_control {
            headers.push(CaffeineHeader {
                name: "Cache-Control",
                value,
            });
        }
        headers.sort_unstable_by(|a, b| a.name.cmp(b.name));
        let mut builder = CaffeineManifestBuilder::new(
            bytes,
            &headers,
            CaffeineHashLimits {
                max_content_bytes: bytes.try_into().unwrap(),
                max_append_bytes: (1024 * 1024).try_into().unwrap(),
                max_headers: 8.try_into().unwrap(),
                max_header_bytes: 1024.try_into().unwrap(),
            },
            usize::try_from(bytes.div_ceil(1024 * 1024))
                .unwrap()
                .try_into()
                .unwrap(),
        )
        .unwrap();
        for (index, chunk) in body.chunks(1024 * 1024).enumerate() {
            builder
                .append(u64::try_from(index * 1024 * 1024).unwrap(), chunk)
                .unwrap();
        }
        let prepared = builder.finish().unwrap();
        UploadManifestRequest {
            permission: UploadAdmissionRequest {
                upload: ReferenceUpload {
                    service: self.service,
                    tenant: self.tenant,
                    namespace: self.config.namespace,
                    upload: 1,
                    object: 2,
                    incarnation: 1,
                    first_reference: 3,
                    root: *prepared.hashes().provider_root.as_bytes(),
                    bytes,
                },
                uploader: self.uploader,
                expires_at_ns: u64::MAX,
            },
            declaration: UploadManifestDeclaration {
                chunks: prepared
                    .manifest()
                    .chunks()
                    .iter()
                    .map(|v| *v.as_bytes())
                    .collect(),
                headers: headers
                    .iter()
                    .map(|h| UploadManifestHeader {
                        name: h.name.into(),
                        value: h.value.into(),
                    })
                    .collect(),
            },
        }
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One actual installation journey preserves the same permission through lifecycle checks"
)]
fn standalone_admission_manifest_and_restore_use_shared_authority() {
    let f = Fixture::new();
    let installed = f.configuration(f.operator).unwrap();
    assert_eq!(installed.configuration, f.config);
    assert_eq!(installed.project, PROJECT);
    assert!(!installed.fenced);
    for actor in [f.controller, f.tenant, Principal::anonymous()] {
        assert_eq!(f.configuration(actor), Err(HostFailure::Denied));
        assert_eq!(f.enroll(actor), Err(TenantFailure::Denied));
    }
    f.enroll(f.operator).unwrap();
    let request = f.manifest();
    for actor in [f.controller, f.operator, f.uploader] {
        let result: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
            .harness
            .pic
            .update_candid_as(f.service, actor, "blob_admit_upload", (request.permission,))
            .unwrap();
        assert_eq!(result, Err(UploadAdmissionFailure::Denied));
    }
    let admitted: Result<UploadAdmissionMutation, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (request.permission,),
        )
        .unwrap();
    assert!(!admitted.unwrap().replayed);
    assert_eq!(
        f.prepare(f.tenant, &request),
        Err(UploadManifestFailure::Permission(
            UploadAdmissionFailure::Denied
        ))
    );
    let prepared = f.prepare(f.uploader, &request).unwrap();
    assert!(prepared.changed);
    assert!(!f.prepare(f.uploader, &request).unwrap().changed);
    let reference: Result<ReferenceMutationResponse, ReferenceFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.tenant,
            "blob_apply_reference",
            (ReferenceCommand {
                upload: request.permission.upload,
                reference: 4,
                operation: 5,
                action: ReferenceAction::Retain,
            },),
        )
        .unwrap();
    assert_eq!(reference, Err(ReferenceFailure::Unconfirmed));
    let observed = f.admission(request.permission);
    let bytes = f.harness.pic.get_stable_memory(f.service);
    f.harness
        .pic
        .stop_canister(f.service, Some(f.controller))
        .unwrap();
    f.harness
        .pic
        .start_canister(f.service, Some(f.controller))
        .unwrap();
    assert!(f.configuration(f.operator).unwrap().fenced);
    f.resume(f.operator).unwrap();
    assert_eq!(f.configuration(f.operator).unwrap(), installed);
    assert!(!f.prepare(f.uploader, &request).unwrap().changed);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &bytes);
    // No upgrade-supplied input can replace the retained operator or limits.
    assert_eq!(
        f.upgrade(installation(&f.config)).unwrap_err().reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(f.configuration(f.operator).unwrap(), installed);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &bytes);
    f.upgrade(candid::encode_args(()).unwrap()).unwrap();
    // The memory runtime commits its allocation ledger during bootstrap.
    // Service observations must survive; fenced calls must not write afterward.
    let restored = f.harness.pic.get_stable_memory(f.service);
    assert!(f.configuration(f.operator).unwrap().fenced);
    assert_eq!(f.configuration(f.operator).unwrap().configuration, f.config);
    assert!(f.tenant().fenced);
    assert_eq!(f.admission(request.permission), observed);
    let retained: Result<UploadManifestResponse, UploadManifestFailure> = f
        .harness
        .pic
        .query_candid_as(
            f.service,
            f.uploader,
            "blob_upload_manifest",
            (request.permission,),
        )
        .unwrap();
    assert_eq!(retained.unwrap(), prepared.observation);
    assert_eq!(
        f.prepare(f.uploader, &request),
        Err(UploadManifestFailure::Permission(
            UploadAdmissionFailure::Fenced
        ))
    );
    let revoked: Result<UploadRevocationResponse, UploadAdmissionFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.service,
            f.tenant,
            "blob_revoke_upload",
            (request.permission,),
        )
        .unwrap();
    assert_eq!(revoked, Err(UploadAdmissionFailure::Fenced));
    unchanged(&f.harness.pic.get_stable_memory(f.service), &restored);
}

#[test]
fn standalone_invalid_installation_rolls_back_and_bounded_ingress_does_not_write() {
    let f = Fixture::new();
    let enrollment = f.enroll(f.operator).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let mut invalid_limits = f.config;
    invalid_limits.resources.max_tenant_active = 0;
    for input in [
        installation(&ServiceConfigurationInput {
            service: f.tenant,
            ..f.config
        }),
        installation(&invalid_limits),
        vec![0; 16_385],
    ] {
        // Failed management reinstallation rolls back the old module and all state.
        let failure = f
            .harness
            .pic
            .reinstall_canister(f.service, wasm(), input, Some(f.controller))
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
        assert_eq!(f.tenant(), enrollment);
        assert_eq!(f.configuration(f.operator).unwrap().configuration, f.config);
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    }
    for (method, input) in [
        ("blob_update_tenant", vec![0; 4097]),
        ("blob_prepare_upload", vec![0; 131_073]),
        ("blob_admit_upload", b"DIDL".to_vec()),
    ] {
        let failure = f
            .harness
            .pic
            .update_call(f.service, f.operator, method, input)
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    }
}

#[test]
fn standalone_restore_rejects_foreign_and_missing_installation_memory() {
    let original = Fixture::new();
    original.enroll(original.operator).unwrap();
    let foreign = original.harness.pic.get_stable_memory(original.service);
    let target_harness = Harness::new();
    target_harness.pic.create_canister();
    let target = Fixture::with_harness(target_harness);
    assert_ne!(target.service, original.service);
    target.harness.pic.set_stable_memory(
        target.service,
        foreign.clone(),
        ic_testkit::pocket_ic::common::rest::BlobCompression::NoCompression,
    );
    assert_eq!(
        target
            .upgrade(candid::encode_args(()).unwrap())
            .unwrap_err()
            .reject_code,
        RejectCode::CanisterError
    );
    unchanged(
        &target.harness.pic.get_stable_memory(target.service),
        &foreign,
    );
    target.harness.pic.set_stable_memory(
        target.service,
        Vec::new(),
        ic_testkit::pocket_ic::common::rest::BlobCompression::NoCompression,
    );
    assert_eq!(
        target
            .upgrade(candid::encode_args(()).unwrap())
            .unwrap_err()
            .reject_code,
        RejectCode::CanisterError
    );
    assert_eq!(
        target.harness.pic.get_stable_memory(target.service),
        Vec::<u8>::new()
    );

    // Change only the retained release in a same-service snapshot. This is
    // deliberate corruption in the emulator, never a supported restore tool.
    let release = original.configuration(original.operator).unwrap().release;
    let mut wrong_release = original.harness.pic.get_stable_memory(original.service);
    let offsets: Vec<_> = wrong_release
        .windows(release.len())
        .enumerate()
        .filter_map(|(offset, bytes)| (bytes == release.as_bytes()).then_some(offset))
        .collect();
    let [offset] = offsets.as_slice() else {
        panic!("expected one installation release field");
    };
    wrong_release[*offset] = b'x';
    original.harness.pic.set_stable_memory(
        original.service,
        wrong_release.clone(),
        ic_testkit::pocket_ic::common::rest::BlobCompression::NoCompression,
    );
    assert_eq!(
        original
            .upgrade(candid::encode_args(()).unwrap())
            .unwrap_err()
            .reject_code,
        RejectCode::CanisterError
    );
    unchanged(
        &original.harness.pic.get_stable_memory(original.service),
        &wrong_release,
    );
}

const PROJECT: &str = "standalone fixture project/β?&=";
fn installation(configuration: &ServiceConfigurationInput) -> Vec<u8> {
    candid::encode_one(ServiceInstallationInput {
        configuration: *configuration,
        project: PROJECT.into(),
        completion_verifier: Fake::principal(90),
        trusted_uploader: Fake::principal(4),
    })
    .unwrap()
}
