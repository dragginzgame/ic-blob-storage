use super::*;

// Tiny real Wasm: the update named operator_status grows stable memory and replies.
// Hand-encoded standard sections avoid a new WAT toolchain/dependency for this one
// transport witness. PocketIC validates and executes it, including the control call.
pub(super) fn update_wasm(method: &str, reply: &[u8]) -> Vec<u8> {
    fn leb(bytes: &mut Vec<u8>, mut value: usize) {
        loop {
            let mut byte = u8::try_from(value & 127).unwrap();
            value >>= 7;
            if value != 0 {
                byte |= 128;
            }
            bytes.push(byte);
            if value == 0 {
                break;
            }
        }
    }
    fn section(wasm: &mut Vec<u8>, id: u8, payload: &[u8]) {
        wasm.push(id);
        leb(wasm, payload.len());
        wasm.extend_from_slice(payload);
    }
    let mut wasm = b"\0asm\x01\0\0\0".to_vec();
    // Types: (i64)->i64, ()->(), (i32,i32)->().
    section(
        &mut wasm,
        1,
        &[
            3, 0x60, 1, 0x7e, 1, 0x7e, 0x60, 0, 0, 0x60, 2, 0x7f, 0x7f, 0,
        ],
    );
    let mut imports = vec![3, 3];
    imports.extend_from_slice(b"ic0");
    imports.push(13);
    imports.extend_from_slice(b"stable64_grow");
    imports.extend_from_slice(&[0, 0, 3]);
    imports.extend_from_slice(b"ic0");
    imports.push(9);
    imports.extend_from_slice(b"msg_reply");
    imports.extend_from_slice(&[0, 1]);
    imports.push(3);
    imports.extend_from_slice(b"ic0");
    imports.push(21);
    imports.extend_from_slice(b"msg_reply_data_append");
    imports.extend_from_slice(&[0, 2]);
    section(&mut wasm, 2, &imports);
    section(&mut wasm, 3, &[1, 1]);
    section(&mut wasm, 5, &[1, 0, 1]); // One page of linear memory.
    let name = format!("canister_update {method}");
    let mut exports = vec![2, u8::try_from(name.len()).unwrap()];
    exports.extend_from_slice(name.as_bytes());
    exports.extend_from_slice(&[0, 3, 6]);
    exports.extend_from_slice(b"memory");
    exports.extend_from_slice(&[2, 0]);
    section(&mut wasm, 7, &exports);
    // Positive signed LEB32 encoded with fixed five bytes, avoiding sign-bit ambiguity.
    let len = u32::try_from(reply.len()).unwrap();
    let mut body = vec![0, 0x42, 1, 0x10, 0, 0x1a, 0x41, 0, 0x41];
    for shift in [0, 7, 14, 21] {
        body.push(((len >> shift) & 127) as u8 | 128);
    }
    body.push((len >> 28) as u8);
    body.extend_from_slice(&[0x10, 2, 0x10, 1, 0x0b]);
    let mut code = vec![1];
    leb(&mut code, body.len());
    code.extend(body);
    section(&mut wasm, 10, &code);
    let mut data = vec![1, 0, 0x41, 0, 0x0b];
    leb(&mut data, reply.len());
    data.extend_from_slice(reply);
    section(&mut wasm, 11, &data);
    wasm
}

#[test]
fn update_only_status_is_rejected_without_executing_it() {
    let f = Fixture::new();
    let pic = &f.harness.pic;
    let canister = pic.create_canister();
    pic.install_canister(canister, update_wasm("operator_status", &[]), vec![], None);
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
