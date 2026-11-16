use super::*;
use crate::application::install::{
    prepare,
    tests::{fixture, replace_packet},
};
use ic_platform::native::{Ed25519Identity, UpdateArgs};
use ic_platform_types::Principal;
use std::fs;

#[tokio::test]
async fn partial_and_mismatched_dispatch_claims_never_send_or_repair() {
    for marker in [vec![], vec![0; 31], vec![0; 32]] {
        let input = fixture("blob");
        prepare(&input).unwrap();
        let directory = input.selection().unwrap().directory;
        let packet = fs::read(directory.join("request.cbor")).unwrap();
        fs::write(directory.join(CLAIM), &marker).unwrap();
        assert!(observe(&input).await.is_err());
        assert!(
            dispatch(DispatchInput {
                input,
                checkout: PathBuf::from("not-opened"),
                confirm_fresh_target: true,
            })
            .await
            .is_err()
        );
        assert_eq!(fs::read(directory.join(CLAIM)).unwrap(), marker);
        assert_eq!(fs::read(directory.join("request.cbor")).unwrap(), packet);
        assert!(!directory.join(OUTCOME).exists());
    }
}

#[tokio::test]
async fn expired_original_is_never_dispatched_or_renewed() {
    let input = fixture("app");
    prepare(&input).unwrap();
    let selected = input.selection().unwrap();
    let identity = Ed25519Identity::from_pem(
        &fs::read_to_string(input.inputs.join("controller.pem")).unwrap(),
        4096,
    )
    .unwrap();
    let expired = identity
        .prepare_update(
            UpdateArgs {
                recipient: Principal::management_canister(),
                effective: selected.target,
                method: "install_code",
                arg: &selected.argument,
                expiry: 1,
                nonce: None,
            },
            super::super::PACKET_BYTES,
        )
        .unwrap();
    replace_packet(&input, expired.envelope_bytes());
    assert!(
        dispatch(DispatchInput {
            input,
            checkout: PathBuf::from("not-opened"),
            confirm_fresh_target: true,
        })
        .await
        .is_err()
    );
    assert_eq!(
        fs::read(selected.directory.join("request.cbor")).unwrap(),
        expired.envelope_bytes()
    );
    assert!(!selected.directory.join(CLAIM).exists());
    assert!(!selected.directory.join(OUTCOME).exists());
}
