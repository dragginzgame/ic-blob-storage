use super::*;
use clap::Parser;
use ic_platform::native::{Ed25519Identity, UpdateArgs};
use sha2::Digest;

const PEM: &str = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIAICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIC\n-----END PRIVATE KEY-----\n";

fn identity() -> Ed25519Identity {
    Ed25519Identity::from_pem(PEM, 4096).unwrap()
}

#[test]
fn offline_preparation_keeps_one_root_claim() {
    let directory = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/certificate-prepare-test")
        .join(format!(
            "{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    std::fs::create_dir_all(&directory).unwrap();
    let directory = directory.canonicalize().unwrap();
    let key = directory.join("uploader.pem");
    // Public deterministic test seed; no wallet key.
    std::fs::write(&key, PEM).unwrap();
    let identity = identity();
    let uploader = identity.principal().to_text();
    let args = [
        "blob",
        "certificate",
        "prepare",
        "--url",
        "https://invalid.example",
        "--canister",
        "rrkah-fqaaa-aaaaa-aaaaq-cai",
        "--uploader",
        &uploader,
        "--root",
        &format!("sha256:{}", "ab".repeat(32)),
        "--identity",
        key.to_str().unwrap(),
        "--custody",
        directory.to_str().unwrap(),
    ];
    let crate::Command::Certificate(crate::CertificateCommand::Prepare(input)) =
        crate::Cli::try_parse_from(args).unwrap().command
    else {
        unreachable!()
    };
    custody::prepare(&input).unwrap();
    assert!(custody::prepare(&input).is_err());
    let network =
        crate::overview::hex(&sha2::Sha256::digest(include_bytes!("../mainnet-root.der")));
    let claim = directory
        .join(network)
        .join("rrkah-fqaaa-aaaaa-aaaaq-cai")
        .join("ab".repeat(32));
    let packet = std::fs::read(claim.join("request.cbor")).unwrap();
    assert!(!claim.join("dispatch.claim").exists());
    // Simulate interrupted preparation without repairing its immutable claim.
    std::fs::rename(claim.join("request.cbor"), claim.join("preserved.cbor")).unwrap();
    assert!(custody::prepare(&input).is_err());
    assert!(!claim.join("request.cbor").exists());
    assert_eq!(std::fs::read(claim.join("preserved.cbor")).unwrap(), packet);
}

#[test]
fn original_request_binding_and_expired_history() {
    let identity = identity();
    let sender = identity.principal();
    let target = Principal::from_slice(&[1, 1]);
    let other = Principal::from_slice(&[3, 1]);
    let root = format!("sha256:{}", "ab".repeat(32));
    // Expiry is intentionally historical: recovery must not renew the request.
    let signed = identity
        .prepare_update(
            UpdateArgs {
                recipient: target,
                effective: target,
                method: METHOD,
                arg: &c::encode_root(&root, 1024).unwrap(),
                expiry: 0,
                nonce: None,
            },
            PACKET_LIMIT,
        )
        .unwrap();
    assert_eq!(
        original(signed.envelope_bytes(), sender, target, &root).unwrap(),
        *signed.request_id()
    );
    // Admission covers the original authentication envelope too. Never repair
    // malformed custody merely because its content still has the expected ID.
    let mut unknown_authentication = signed.envelope_bytes().to_vec();
    let key = b"sender_sig";
    let offset = unknown_authentication
        .windows(key.len())
        .position(|bytes| bytes == key)
        .unwrap();
    unknown_authentication[offset] = b'x';
    let mut trailing = signed.envelope_bytes().to_vec();
    trailing.push(0);
    for packet in [
        unknown_authentication,
        trailing,
        signed.envelope_bytes()[..signed.envelope_bytes().len() - 1].to_vec(),
    ] {
        assert_eq!(
            original(&packet, sender, target, &root).unwrap_err(),
            "invalid retained ingress content"
        );
    }
    assert_eq!(
        original(signed.envelope_bytes(), sender, target, &root).unwrap(),
        *signed.request_id()
    );
    for (uploader, issuer, expected) in [
        (other, target, root.clone()),
        (sender, other, root.clone()),
        (sender, target, format!("sha256:{}", "cd".repeat(32))),
    ] {
        assert_eq!(
            original(signed.envelope_bytes(), uploader, issuer, &expected).unwrap_err(),
            "retained request does not match uploader, issuer, method and root"
        );
    }
    let wrong_method = identity
        .prepare_update(
            UpdateArgs {
                recipient: target,
                effective: target,
                method: "unrelated",
                arg: &c::encode_root(&root, 1024).unwrap(),
                expiry: 0,
                nonce: None,
            },
            PACKET_LIMIT,
        )
        .unwrap();
    assert_eq!(
        original(wrong_method.envelope_bytes(), sender, target, &root).unwrap_err(),
        "retained request does not match uploader, issuer, method and root"
    );
    for packet in [vec![], vec![0; PACKET_LIMIT + 1]] {
        assert_eq!(
            original(&packet, sender, target, &root).unwrap_err(),
            "invalid retained ingress content"
        );
    }
}

#[test]
fn exact_upload_reply_and_root_admission() {
    let expected = format!("sha256:{}", "ab".repeat(32));
    assert_eq!(root(&expected).unwrap(), expected);
    assert_eq!(
        root(&expected.to_uppercase()).unwrap_err(),
        "expected canonical sha256 root"
    );
    for value in ["sha256:00".into(), format!("sha256:{}", "gg".repeat(32))] {
        assert_eq!(
            root(&value).unwrap_err(),
            "expected 64 lowercase hexadecimal root digits"
        );
    }
    let reply = |method: &str, hash: &str| {
        c::encode_certificate(
            &c::CertificateReply {
                method: method.into(),
                blob_hash: hash.into(),
            },
            1024,
        )
        .unwrap()
    };
    let good = reply("upload", &expected);
    admit_reply(&good, &expected).unwrap();
    let mut trailing = good;
    trailing.push(0);
    for bytes in [reply("delete", &expected), reply("upload", "wrong-root")] {
        assert_eq!(
            admit_reply(&bytes, &expected).unwrap_err(),
            "certified reply does not match upload and original root"
        );
    }
    for bytes in [trailing, vec![0; 1025]] {
        assert_eq!(
            admit_reply(&bytes, &expected).unwrap_err(),
            "invalid certified upload reply"
        );
    }
}
