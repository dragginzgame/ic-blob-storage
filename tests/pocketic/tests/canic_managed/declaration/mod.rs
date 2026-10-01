//! The supported build's adjacent Candid must describe every called fixture endpoint.
#[test]
fn built_candid_declares_application_methods_with_their_actual_modes_and_shapes() {
    let wasm = std::path::PathBuf::from(std::env::var_os("BLOB_CANIC_PROBE_WASM").unwrap());
    let source = std::fs::read_to_string(wasm.with_extension("did")).unwrap();
    let program: candid_parser::IDLProg = source.parse().unwrap();
    let mut environment = candid::TypeEnv::new();
    let actor = candid_parser::check_prog(&mut environment, &program)
        .unwrap()
        .unwrap();
    for (name, arguments, query) in [
        ("probe_snapshot", 0, true),
        ("probe_init_arguments", 0, true),
        ("probe_fleet_caller", 0, true),
        ("probe_forward_enroll", 1, false),
        ("probe_forward_prepare", 1, false),
        ("probe_expose_upload", 1, false),
    ] {
        let method = environment.get_method(&actor, name).unwrap();
        assert_eq!(method.args.len(), arguments);
        assert_eq!(method.rets.len(), 1);
        assert_eq!(method.is_query(), query);
    }
    // Compare deployment contracts without linking two endpoint-exporting artifacts
    // into one native executable. Standalone's own test checks its exported source.
    let standalone = include_str!("../../../../../canisters/standalone/service.did");
    let program: candid_parser::IDLProg = standalone.parse().unwrap();
    let mut standalone_environment = candid::TypeEnv::new();
    let standalone_actor = candid_parser::check_prog(&mut standalone_environment, &program)
        .unwrap()
        .unwrap();
    let standalone_actor = environment.merge_type(standalone_environment, standalone_actor);
    for name in [
        "blob_configuration",
        "blob_admit_upload",
        "blob_upload_admission",
        "blob_prepare_upload",
        "blob_upload_manifest",
        "blob_upload_capacity",
        "blob_update_tenant",
        "blob_tenant",
        "blob_upload_status",
        "blob_revoke_upload",
        "blob_upload_history",
        "blob_lookup_content",
        "blob_apply_reference",
        "blob_reference_receipt",
        "blob_reference_status",
        "blob_reference_capacity",
        "blob_local_status",
        "blob_funding_history",
        "blob_funding_outcome",
        "blob_funding_preparation_assessment",
        "blob_attest_upload",
        "blob_upload_attestation",
        "blob_verification_manifest",
        "blob_verification_plan",
        "blob_download_descriptor",
        "blob_upload_certificate_assessment",
        "blob_revoke_gateway",
        "blob_cancel_gateway_sync",
        "blob_sync_gateways",
        "blob_inspect_account",
    ] {
        let managed =
            candid::types::TypeInner::Func(environment.get_method(&actor, name).unwrap().clone())
                .into();
        let standalone = candid::types::TypeInner::Func(
            environment
                .get_method(&standalone_actor, name)
                .unwrap()
                .clone(),
        )
        .into();
        candid::types::subtype::equal(
            &mut std::collections::HashSet::new(),
            &environment,
            &managed,
            &standalone,
        )
        .unwrap();
    }
}
