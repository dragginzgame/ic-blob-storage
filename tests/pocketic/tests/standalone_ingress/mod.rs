//! Provider Candid budgets and current CBOR admission at actual dispatch boundaries.
use super::*;
use crate::support::REPLY_LIMITS;
use candid::{
    CandidType, DecoderConfig, Deserialize, IDLArgs, IDLValue,
    types::{
        TypeEnv,
        internal::{Field, TypeInner},
    },
};
use ic_blob_storage_pocketic_tests::service_calls::ServiceCallExt;
use std::fmt::Debug;

/// Each payload round-trips when only its exhausted budget is relaxed.
pub(super) fn refusal_arguments<T>(input: &T, max_bytes: usize) -> [Vec<u8>; 2]
where
    T: CandidType + for<'de> Deserialize<'de> + Debug + PartialEq,
{
    let skipped = candid::encode_args((input, "x".repeat(2048))).unwrap();
    let mut config = limits(max_bytes);
    config.set_skipping_quota(10_000);
    assert_eq!(
        candid::decode_one_with_config::<T>(&skipped, &config).unwrap(),
        *input
    );
    assert!(candid::decode_one_with_config::<T>(&skipped, &limits(max_bytes)).is_err());

    // Distinct absent optional records exceed the table bound with little skipping.
    // The Candid encoder owns the wire format; no handcrafted type-table offsets.
    let mut arguments = IDLArgs::from_bytes(&candid::encode_one(input).unwrap()).unwrap();
    let mut argument_types = arguments.get_types();
    for id in 0..33 {
        let record = TypeInner::Record(vec![Field {
            id: candid::types::Label::Id(id).into(),
            ty: TypeInner::Null.into(),
        }])
        .into();
        argument_types.push(TypeInner::Opt(record).into());
        arguments.args.push(IDLValue::None);
    }
    let types = arguments
        .to_bytes_with_types(&TypeEnv::new(), &argument_types)
        .unwrap();
    let mut config = limits(max_bytes);
    config.set_max_type_len(128);
    assert_eq!(
        candid::decode_one_with_config::<T>(&types, &config).unwrap(),
        *input
    );
    assert!(candid::decode_one_with_config::<T>(&types, &limits(max_bytes)).is_err());
    for bytes in [&skipped, &types] {
        assert!(bytes.len() <= max_bytes);
    }
    [skipped, types]
}

fn limits(max_bytes: usize) -> DecoderConfig {
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(2_000_000)
        .set_skipping_quota(1024)
        .set_max_type_len(64)
        .set_max_header_len(max_bytes)
        .set_full_error_message(false);
    config
}

#[derive(serde::Serialize)]
struct ForeignInstallation<'a> {
    #[serde(flatten)]
    installation: &'a ServiceInstallationInput,
    retired_authority: bool,
}

#[test]
fn standalone_cbor_admission_refuses_before_mutation_and_failed_reinstall_preserves_owner() {
    let f = Fixture::new();
    let enrollment = f.enroll(f.operator).unwrap();
    let configuration = f.configuration(f.operator).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let update = TenantUpdateRequest {
        scope: f.scope(),
        expected: enrollment.enrollment,
        active: false,
    };
    let mut trailing = ic_blob_storage_contracts::wire::encode(update).unwrap();
    trailing.push(0xf6);
    for bytes in [
        candid::encode_one(update).unwrap(),
        trailing,
        vec![0; 4097],
        vec![0xff],
    ] {
        assert!(
            f.harness
                .pic
                .update_call(f.service, f.operator, "blob_update_tenant", bytes)
                .is_err()
        );
        assert_eq!(f.tenant(), enrollment);
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    }
    assert!(
        f.harness
            .pic
            .query_call(
                f.service,
                f.tenant,
                "blob_tenant",
                candid::encode_one(f.scope()).unwrap()
            )
            .is_err()
    );
    let input: ServiceInstallationInput =
        ic_blob_storage_contracts::wire::decode(&installation(&f.config), 16384).unwrap();
    for bytes in [
        candid::encode_one(&input).unwrap(),
        ic_blob_storage_contracts::wire::encode(ForeignInstallation {
            installation: &input,
            retired_authority: true,
        })
        .unwrap(),
        vec![0; 16385],
    ] {
        assert!(
            f.harness
                .pic
                .reinstall_canister(f.service, wasm(), bytes, Some(f.controller))
                .is_err()
        );
        assert_eq!(f.configuration(f.operator).unwrap(), configuration);
        assert_eq!(f.tenant(), enrollment);
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    }
    let response: Result<TenantEnrollmentResponse, TenantFailure> = f
        .harness
        .pic
        .update_blob_as(
            f.service,
            f.operator,
            "blob_update_tenant",
            update,
            REPLY_LIMITS.reply_bytes,
        )
        .unwrap();
    assert!(!response.unwrap().enrollment.unwrap().active);
}

#[test]
fn standalone_byte_bounded_cbor_manifest_reaches_policy_and_refusals_preserve_upload() {
    let f = Fixture::new();
    f.enroll(f.operator).unwrap();
    let mut manifest = f.small_manifest();
    f.harness
        .pic
        .update_blob_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            manifest.permission,
            REPLY_LIMITS.reply_bytes,
        )
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, &manifest).unwrap();
    let admission = f.admission(manifest.permission);
    let before = f.harness.pic.get_stable_memory(f.service);
    manifest.declaration.headers = vec![
        UploadManifestHeader {
            name: String::new(),
            value: String::new()
        };
        4096
    ];
    let bytes = ic_blob_storage_contracts::wire::encode(&manifest).unwrap();
    assert!(bytes.len() <= 131_072);
    let reply = f
        .harness
        .pic
        .update_call(f.service, f.uploader, "blob_prepare_upload", bytes)
        .unwrap();
    assert_eq!(
        ic_blob_storage_contracts::wire::decode::<
            Result<UploadManifestMutation, UploadManifestFailure>,
        >(&reply, 4096)
        .unwrap(),
        Err(UploadManifestFailure::Limit)
    );
    assert_eq!(f.admission(manifest.permission), admission);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    assert!(
        f.harness
            .pic
            .update_call(
                f.service,
                f.uploader,
                "blob_prepare_upload",
                vec![0; 131_073]
            )
            .is_err()
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}

#[test]
fn tenant_wire_refuses_wrong_method_direction_and_old_packets_before_mutation() {
    use ic_blob_storage_contracts::dto::tenant::TenantEnrollment;
    use ic_blob_storage_contracts::wire::tenant as wire;
    let f = Fixture::new();
    let command = TenantUpdateRequest {
        scope: f.scope(),
        expected: None,
        active: true,
    };
    let query = wire::inspection_request(f.scope()).unwrap();
    let original = f.harness.pic.get_stable_memory(f.service);
    let mut trailing = wire::update_request(command).unwrap();
    trailing.push(0);
    for bytes in [
        query.clone(),
        wire::inspection_response(Err(TenantFailure::Denied)).unwrap(),
        ic_blob_storage_contracts::wire::encode(command).unwrap(),
        candid::encode_one(command).unwrap(),
        trailing,
        vec![0; 513],
    ] {
        let rejected = f
            .harness
            .pic
            .update_call(f.service, f.operator, "blob_update_tenant", bytes)
            .unwrap_err();
        assert_eq!(rejected.reject_code, RejectCode::CanisterError);
        unchanged(&f.harness.pic.get_stable_memory(f.service), &original);
    }
    let bytes = f
        .harness
        .pic
        .query_call(f.service, f.tenant, "blob_tenant", query)
        .unwrap();
    assert_eq!(
        wire::decode_inspection_response(&bytes)
            .unwrap()
            .unwrap()
            .enrollment,
        None
    );
    let denied = f
        .harness
        .pic
        .update_call(
            f.service,
            f.controller,
            "blob_update_tenant",
            wire::update_request(command).unwrap(),
        )
        .unwrap();
    assert_eq!(
        wire::decode_update_response(&denied).unwrap(),
        Err(TenantFailure::Denied)
    );
    unchanged(&f.harness.pic.get_stable_memory(f.service), &original);
    let accepted = f
        .harness
        .pic
        .update_call(
            f.service,
            f.operator,
            "blob_update_tenant",
            wire::update_request(command).unwrap(),
        )
        .unwrap();
    let accepted = wire::decode_update_response(&accepted).unwrap().unwrap();
    assert_eq!(accepted.scope, f.scope());
    assert_eq!(
        accepted.enrollment,
        Some(TenantEnrollment {
            generation: 1,
            active: true
        })
    );
    assert_eq!(f.tenant(), accepted);
    let conflict = f
        .harness
        .pic
        .update_call(
            f.service,
            f.operator,
            "blob_update_tenant",
            wire::update_request(command).unwrap(),
        )
        .unwrap();
    assert_eq!(
        wire::decode_update_response(&conflict).unwrap(),
        Err(TenantFailure::Conflict)
    );
}
