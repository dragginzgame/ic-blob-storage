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
        ("probe_enroll", 1, false),
        ("probe_tenant", 1, true),
        ("probe_fleet_caller", 0, true),
        ("probe_forward_enroll", 2, false),
    ] {
        let method = environment.get_method(&actor, name).unwrap();
        assert_eq!(method.args.len(), arguments);
        assert_eq!(method.rets.len(), 1);
        assert_eq!(method.is_query(), query);
    }
}
