//! Valid Candid budget failures at the actual standalone dispatch boundaries.
use super::*;
use candid::{
    CandidType, DecoderConfig, Deserialize, IDLArgs, IDLValue,
    types::{
        TypeEnv,
        internal::{Field, TypeInner},
    },
};
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

#[test]
fn standalone_valid_candid_budgets_refuse_before_mutation_and_failed_reinstall_preserves_owner() {
    let f = Fixture::new();
    let enrollment = f.enroll(f.operator).unwrap();
    let configuration = f.configuration(f.operator).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let update = TenantUpdateRequest {
        scope: f.scope(),
        expected: enrollment.enrollment,
        active: false,
    };
    for input in refusal_arguments(&update, 4096) {
        let failure = f
            .harness
            .pic
            .update_call(f.service, f.operator, "blob_update_tenant", input)
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
        assert_eq!(f.tenant(), enrollment);
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    }
    for input in refusal_arguments(&f.scope(), 4096) {
        let failure = f
            .harness
            .pic
            .query_call(f.service, f.tenant, "blob_tenant", input)
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    }
    let installation: ServiceInstallationInput =
        candid::decode_one(&installation(&f.config)).unwrap();
    for input in refusal_arguments(&installation, 16_384)
        .into_iter()
        .chain([super::standalone_hard_cut::frozen_installation(&f)])
    {
        let failure = f
            .harness
            .pic
            .reinstall_canister(f.service, wasm(), input, Some(f.controller))
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
        assert_eq!(f.tenant(), enrollment);
        assert_eq!(f.configuration(f.operator).unwrap(), configuration);
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    }
    // Small extra values are legal Candid subtyping, not a blanket extra-argument refusal.
    let reply = f
        .harness
        .pic
        .query_call(
            f.service,
            f.tenant,
            "blob_tenant",
            candid::encode_args((f.scope(), "small")).unwrap(),
        )
        .unwrap();
    assert_eq!(
        candid::decode_one::<Result<TenantEnrollmentResponse, TenantFailure>>(&reply).unwrap(),
        Ok(enrollment)
    );
    let reply = f
        .harness
        .pic
        .update_call(
            f.service,
            f.operator,
            "blob_update_tenant",
            candid::encode_args((update, "small")).unwrap(),
        )
        .unwrap();
    let updated = candid::decode_one::<Result<TenantEnrollmentResponse, TenantFailure>>(&reply)
        .unwrap()
        .unwrap();
    assert!(!updated.enrollment.unwrap().active);
    assert_eq!(f.tenant(), updated);
    // Installation is exact: an extra argument cannot be silently discarded.
    let before = f.harness.pic.get_stable_memory(f.service);
    let failure = f
        .harness
        .pic
        .reinstall_canister(
            f.service,
            wasm(),
            candid::encode_args((installation, "small")).unwrap(),
            Some(f.controller),
        )
        .unwrap_err();
    assert_eq!(failure.reject_code, RejectCode::CanisterError);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    assert_eq!(f.configuration(f.operator).unwrap(), configuration);
    assert_eq!(f.tenant(), updated);
}

#[test]
fn standalone_byte_bounded_dense_manifest_reaches_policy_and_budget_refusals_preserve_upload() {
    let f = Fixture::new();
    f.enroll(f.operator).unwrap();
    let mut manifest = f.small_manifest();
    f.harness
        .pic
        .update_candid_as::<Result<UploadAdmissionMutation, UploadAdmissionFailure>, _>(
            f.service,
            f.tenant,
            "blob_admit_upload",
            (manifest.permission,),
        )
        .unwrap()
        .unwrap();
    f.prepare(f.uploader, &manifest).unwrap();
    let admission = f.admission(manifest.permission);
    let root = super::standalone_certificate::root(&manifest);
    let assessment = super::standalone_certificate::inspect(&f, f.uploader, &root).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    for input in refusal_arguments(&manifest, 131_072) {
        let failure = f
            .harness
            .pic
            .update_call(f.service, f.uploader, "blob_prepare_upload", input)
            .unwrap_err();
        assert_eq!(failure.reject_code, RejectCode::CanisterError);
        assert_eq!(f.admission(manifest.permission), admission);
        unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
    }
    manifest.declaration.headers = vec![
        UploadManifestHeader {
            name: String::new(),
            value: String::new()
        };
        65_379
    ];
    let bytes = candid::encode_one(&manifest).unwrap();
    assert!(bytes.len() <= 131_072);
    let config = limits(131_072);
    let mut decoder = candid::de::IDLDeserialize::new_with_config(&bytes, &config).unwrap();
    assert_eq!(
        decoder.get_value::<UploadManifestRequest>().unwrap(),
        manifest
    );
    decoder.done().unwrap();
    let cost = decoder.get_config().compute_cost(&config);
    // Dense maintained records fit the work budget; do not weaken it to manufacture
    // a work-quota failure. The independent domain count bound still refuses them.
    assert!(cost.decoding_quota.unwrap() < 2_000_000);
    assert!(cost.decoding_quota.unwrap() > 1_800_000);
    assert_eq!(cost.skipping_quota, Some(0));
    let reply = f
        .harness
        .pic
        .update_call(f.service, f.uploader, "blob_prepare_upload", bytes)
        .unwrap();
    assert_eq!(
        candid::decode_one::<Result<UploadManifestMutation, UploadManifestFailure>>(&reply)
            .unwrap(),
        Err(UploadManifestFailure::Limit)
    );
    assert_eq!(f.admission(manifest.permission), admission);
    let current = super::standalone_certificate::inspect(&f, f.uploader, &root).unwrap();
    assert_eq!(current.permission, assessment.permission);
    assert_eq!(current.blockers, assessment.blockers);
    unchanged(&f.harness.pic.get_stable_memory(f.service), &before);
}
