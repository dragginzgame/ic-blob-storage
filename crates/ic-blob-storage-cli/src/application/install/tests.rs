use super::*;
use crate::{Cli, Command as RootCommand, application::Command};
use clap::Parser;
use ic_blob_storage::{caffeine::ManifestLimits, local::Limits};
use ic_platform::native::{Ed25519Identity, UpdateArgs};
use std::{fs, os::unix::fs::symlink};

// Public deterministic fixture seed; never read a wallet identity.
const PEM: &str = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIAICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIC\n-----END PRIVATE KEY-----\n";

fn frame<T: Serialize>(method: &str, value: &T) -> Vec<u8> {
    ic_blob_storage::protocol::frame::encode(
        value,
        ic_wire::Encoding::Frame {
            method: ic_wire::method_id(method),
            direction: ic_wire::Direction::Request,
        },
        RPC_LIMITS,
    )
    .unwrap()
}

pub(super) fn fixture(role: &str) -> Input {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/install-request-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            network::now().unwrap()
        ));
    fs::create_dir_all(&root).unwrap();
    let root = root.canonicalize().unwrap();
    let app = Principal::from_slice(&[1, 1]);
    let blob = Principal::from_slice(&[2, 1]);
    let configuration = BlobInstallation {
        application: app,
        manager: Principal::from_slice(&[3, 1]),
        limits: Limits {
            manifest: ManifestLimits {
                max_content_bytes: 100.try_into().unwrap(),
                max_chunks: 1.try_into().unwrap(),
                max_headers: 1.try_into().unwrap(),
                max_header_bytes: 100.try_into().unwrap(),
            },
            uploads: 64.try_into().unwrap(),
            references: 64.try_into().unwrap(),
            logical_bytes: 100.try_into().unwrap(),
            exposure_bytes: 100.try_into().unwrap(),
        },
    };
    fs::write(
        root.join("blob-init.wire"),
        frame("blob_init", &configuration),
    )
    .unwrap();
    fs::write(
        root.join("app-init.wire"),
        frame(
            "app_init",
            &AppInstallation {
                blob,
                users: vec![Principal::from_slice(&[4, 1])],
            },
        ),
    )
    .unwrap();
    fs::write(root.join("controller.pem"), PEM).unwrap();
    fs::write(root.join("module.wasm"), b"\0asm\x01\0\0\0").unwrap();
    let controller = Ed25519Identity::from_pem(PEM, 4096).unwrap().principal();
    let mut args = vec!["blob".to_owned(), "app".into(), "prepare-install".into()];
    for (flag, value) in [
        ("--url", "https://offline.invalid".into()),
        ("--role", role.into()),
        ("--canister", app.to_text()),
        ("--blob-canister", blob.to_text()),
        ("--controller", controller.to_text()),
        (
            "--identity",
            root.join("controller.pem").display().to_string(),
        ),
        ("--wasm", root.join("module.wasm").display().to_string()),
        ("--inputs", root.display().to_string()),
        ("--custody", root.join("custody").display().to_string()),
    ] {
        args.extend([flag.into(), value]);
    }
    let RootCommand::Application(Command::PrepareInstall(input)) =
        Cli::try_parse_from(args).unwrap().command
    else {
        panic!("expected prepare-install");
    };

    input
}

pub(super) fn replace_packet(input: &Input, packet: &[u8]) {
    let selected = input.selection().unwrap();
    let intent_path = selected.directory.join("intent.wire");
    let mut intent: Intent = ic_blob_storage::protocol::frame::decode(
        &fs::read(&intent_path).unwrap(),
        ic_wire::Encoding::Payload,
        INTENT_LIMITS,
    )
    .unwrap();
    intent.packet_hash = Sha256::digest(packet).into();
    fs::write(
        intent_path,
        ic_blob_storage::protocol::frame::encode(
            &intent,
            ic_wire::Encoding::Payload,
            INTENT_LIMITS,
        )
        .unwrap(),
    )
    .unwrap();
    fs::write(selected.directory.join("request.cbor"), packet).unwrap();
}

#[test]
fn both_roles_inspect_without_a_key_and_never_replace_completed_claims() {
    for role in ["app", "blob"] {
        let input = fixture(role);
        let report = prepare(&input).unwrap();
        let selected = input.selection().unwrap();
        let original = fs::read(selected.directory.join("request.cbor")).unwrap();
        assert_eq!(inspect(&input).unwrap(), report);
        assert!(prepare(&input).is_err());
        assert_eq!(
            fs::read(selected.directory.join("request.cbor")).unwrap(),
            original
        );
        fs::remove_file(input.inputs.join("controller.pem")).unwrap();
        assert_eq!(inspect(&input).unwrap(), report);
        assert!(!selected.directory.join("dispatch.claim").exists());
        assert_eq!(fs::read_dir(&selected.directory).unwrap().count(), 2);
    }
}

#[test]
fn changed_artifacts_targets_controller_and_network_refuse_without_mutation() {
    let mut input = fixture("blob");
    prepare(&input).unwrap();
    let record = input.selection().unwrap().directory;
    let packet = fs::read(record.join("request.cbor")).unwrap();
    let old = input.controller;
    input.controller = Principal::from_slice(&[9, 1]);
    assert!(inspect(&input).is_err());
    input.controller = old;
    let old = input.canister;
    input.canister = Principal::from_slice(&[8, 1]);
    assert!(inspect(&input).is_err());
    input.canister = old;
    input.network.url = "https://another.invalid".parse().unwrap();
    assert!(inspect(&input).is_err());
    assert!(prepare(&input).is_err()); // URL aliases cannot bypass the claim.
    input.network.url = "https://offline.invalid".parse().unwrap();
    fs::write(&input.wasm, b"\0asm\x01\0\0\0\0\x01\0").unwrap();
    assert!(inspect(&input).is_err());
    fs::write(&input.wasm, b"\0asm\x01\0\0\0").unwrap();
    let app_path = input.inputs.join("app-init.wire");
    fs::write(
        app_path,
        frame(
            "app_init",
            &AppInstallation {
                blob: input.blob_canister,
                users: vec![Principal::from_slice(&[5, 1])],
            },
        ),
    )
    .unwrap();
    assert!(inspect(&input).is_err()); // Even the other role's config is bound.
    assert_eq!(fs::read(record.join("request.cbor")).unwrap(), packet);
    assert_eq!(fs::read_dir(record).unwrap().count(), 2);
}

#[test]
fn partial_claims_are_not_repaired_and_symlink_inputs_are_refused() {
    let input = fixture("app");
    let record = input.selection().unwrap().directory;
    fs::create_dir_all(&record).unwrap();
    fs::write(record.join(".intent.wire.pending"), b"partial").unwrap();
    assert!(prepare(&input).is_err());
    assert!(inspect(&input).is_err());
    assert_eq!(fs::read_dir(record).unwrap().count(), 1);
    let input = fixture("blob");
    let path = input.inputs.join("app-init.wire");
    fs::rename(&path, input.inputs.join("retained.wire")).unwrap();
    symlink(input.inputs.join("retained.wire"), path).unwrap();
    assert!(prepare(&input).is_err());
    assert!(!input.custody.exists());
}

#[test]
fn invalid_pair_and_over_budget_inputs_leave_no_claim() {
    let input = fixture("app");
    for user in [
        input.canister,
        Principal::anonymous(),
        Principal::management_canister(),
    ] {
        fs::write(
            input.inputs.join("app-init.wire"),
            frame(
                "app_init",
                &AppInstallation {
                    blob: input.blob_canister,
                    users: vec![user],
                },
            ),
        )
        .unwrap();
        assert_eq!(
            prepare(&input).unwrap_err(),
            "canisters, manager and one to eight users must be distinct"
        );
        assert!(!input.custody.exists());
    }
    let input = fixture("blob");
    fs::write(&input.wasm, vec![0; 1024 * 1024 + 1]).unwrap();
    assert!(prepare(&input).is_err());
    assert!(!input.custody.exists());
}

#[test]
fn signature_is_checked_even_if_the_saved_packet_hash_matches() {
    let input = fixture("blob");
    prepare(&input).unwrap();
    let selected = input.selection().unwrap();
    let packet = fs::read(selected.directory.join("request.cbor")).unwrap();
    let signature = decode_envelope(&packet, PACKET_BYTES)
        .unwrap()
        .value
        .sender_sig
        .unwrap();
    let offsets: Vec<_> = packet
        .windows(signature.len())
        .enumerate()
        .filter(|(_, bytes)| *bytes == signature)
        .map(|(index, _)| index)
        .collect();
    assert_eq!(offsets.len(), 1);
    let mut tampered = packet;
    tampered[offsets[0]] ^= 1;
    replace_packet(&input, &tampered);
    assert_eq!(
        inspect(&input).unwrap_err(),
        "invalid original installation signature or envelope"
    );
    assert_eq!(
        fs::read(selected.directory.join("request.cbor")).unwrap(),
        tampered
    );
}

#[test]
fn valid_signatures_cannot_substitute_another_operation_and_expiry_never_renews() {
    let input = fixture("blob");
    prepare(&input).unwrap();
    let selected = input.selection().unwrap();
    let identity = Ed25519Identity::from_pem(PEM, 4096).unwrap();
    let exact = UpdateArgs {
        recipient: Principal::management_canister(),
        effective: selected.target,
        method: "install_code",
        arg: &selected.argument,
        expiry: 1,
        nonce: None,
    };
    let changed_argument = InstallCodeArgs {
        mode: InstallMode::Reinstall,
        canister_id: selected.target,
        wasm_module: fs::read(&input.wasm).unwrap(),
        arg: fs::read(input.inputs.join("blob-init.wire")).unwrap(),
        sender_canister_version: None,
    }
    .encode(PACKET_BYTES)
    .unwrap();
    for altered in [
        UpdateArgs {
            method: "stop_canister",
            ..exact
        },
        UpdateArgs {
            recipient: selected.target,
            ..exact
        },
        UpdateArgs {
            arg: &changed_argument,
            ..exact
        },
        UpdateArgs {
            nonce: Some(&[]),
            ..exact
        },
    ] {
        let packet = identity.prepare_update(altered, PACKET_BYTES).unwrap();
        replace_packet(&input, packet.envelope_bytes());
        assert_eq!(
            inspect(&input).unwrap_err(),
            "signed request differs from the exact fresh installation"
        );
        assert_eq!(
            fs::read(selected.directory.join("request.cbor")).unwrap(),
            packet.envelope_bytes()
        );
    }
    let expired = identity.prepare_update(exact, PACKET_BYTES).unwrap();
    replace_packet(&input, expired.envelope_bytes());
    assert!(inspect(&input).unwrap().contains("Expired: true"));
    assert!(prepare(&input).is_err());
    assert_eq!(
        fs::read(selected.directory.join("request.cbor")).unwrap(),
        expired.envelope_bytes()
    );
}
