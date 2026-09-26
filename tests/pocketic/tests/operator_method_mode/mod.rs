use super::*;

// Tiny real Wasm: the update named operator_status grows stable memory and replies.
// Hand-encoded standard sections avoid a new WAT toolchain/dependency for this one
// transport witness. PocketIC validates and executes it, including the control call.
fn update_only_wasm() -> Vec<u8> {
    fn section(wasm: &mut Vec<u8>, id: u8, payload: &[u8]) {
        wasm.push(id);
        wasm.push(u8::try_from(payload.len()).expect("small section"));
        wasm.extend_from_slice(payload);
    }
    let mut wasm = b"\0asm\x01\0\0\0".to_vec();
    // Types: (i64)->i64, ()->(). Imports: stable64_grow, msg_reply.
    section(&mut wasm, 1, &[2, 0x60, 1, 0x7e, 1, 0x7e, 0x60, 0, 0]);
    let mut imports = vec![2, 3];
    imports.extend_from_slice(b"ic0");
    imports.push(13);
    imports.extend_from_slice(b"stable64_grow");
    imports.extend_from_slice(&[0, 0, 3]);
    imports.extend_from_slice(b"ic0");
    imports.push(9);
    imports.extend_from_slice(b"msg_reply");
    imports.extend_from_slice(&[0, 1]);
    section(&mut wasm, 2, &imports);
    section(&mut wasm, 3, &[1, 1]);
    let name = b"canister_update operator_status";
    let mut exports = vec![1, u8::try_from(name.len()).unwrap()];
    exports.extend_from_slice(name);
    exports.extend_from_slice(&[0, 2]);
    section(&mut wasm, 7, &exports);
    section(
        &mut wasm,
        10,
        &[1, 9, 0, 0x42, 1, 0x10, 0, 0x1a, 0x10, 1, 0x0b],
    );
    wasm
}

#[test]
fn update_only_status_is_rejected_without_executing_it() {
    let f = Fixture::new();
    let pic = &f.harness.pic;
    let canister = pic.create_canister();
    pic.install_canister(canister, update_only_wasm(), vec![], None);
    let args = f.args(canister, f.driver, "authority", "--namespace", "1");
    assert!(pic.get_stable_memory(canister).is_empty());
    let (code, result) = command(&args);
    assert_eq!(code, 3);
    assert_eq!(result["error"], "query_rejected");
    assert!(pic.get_stable_memory(canister).is_empty());
    // Positive control proves this is executable mutation, not an inert/missing export.
    pic.update_call(canister, f.driver, "operator_status", vec![])
        .unwrap();
    assert_eq!(pic.get_stable_memory(canister).len(), 65_536);
}
