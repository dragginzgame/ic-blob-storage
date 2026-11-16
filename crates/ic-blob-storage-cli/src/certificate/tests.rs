use super::*;
use clap::Parser;
use ic_platform_legacy::agent::{Agent, BasicIdentity};
use sha2::Digest;

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
    std::fs::write(&key, "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIAICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIC\n-----END PRIVATE KEY-----\n").unwrap();
    let agent = Agent::builder()
        .with_url("https://invalid.example")
        .with_identity(BasicIdentity::from_raw_key(&[2; 32]))
        .build()
        .unwrap();
    let uploader = agent.get_principal().unwrap().to_text();
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
    let network = crate::overview::hex(&sha2::Sha256::digest(agent.read_root_key()));
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
    let agent = Agent::builder()
        .with_url("http://127.0.0.1:1")
        .with_identity(BasicIdentity::from_raw_key(&[2; 32]))
        .build()
        .unwrap();
    let sender = Principal::from_slice(agent.get_principal().unwrap().as_slice());
    let target = Principal::from_slice(&[1, 1]);
    let other = Principal::from_slice(&[3, 1]);
    let root = format!("sha256:{}", "ab".repeat(32));
    // Expiry is intentionally historical: recovery must not renew the request.
    let signed = agent
        .update(&target.as_slice().try_into().unwrap(), METHOD)
        .with_arg(encode_args(&(root.clone(),), 1024).unwrap())
        .expire_at(std::time::SystemTime::UNIX_EPOCH)
        .sign()
        .unwrap();
    assert_eq!(
        original(&signed.signed_update, sender, target, &root).unwrap(),
        signed.request_id
    );
    // Admission covers the original authentication envelope too. Never repair
    // malformed custody merely because its content still has the expected ID.
    let mut unknown_authentication = signed.signed_update.clone();
    let key = b"sender_sig";
    let offset = unknown_authentication
        .windows(key.len())
        .position(|bytes| bytes == key)
        .unwrap();
    unknown_authentication[offset] = b'x';
    let mut trailing = signed.signed_update.clone();
    trailing.push(0);
    for packet in [
        unknown_authentication,
        trailing,
        signed.signed_update[..signed.signed_update.len() - 1].to_vec(),
    ] {
        assert!(original(&packet, sender, target, &root).is_err());
    }
    assert_eq!(
        original(&signed.signed_update, sender, target, &root).unwrap(),
        signed.request_id
    );
    for (uploader, issuer, expected) in [
        (other, target, root.clone()),
        (sender, other, root.clone()),
        (sender, target, format!("sha256:{}", "cd".repeat(32))),
    ] {
        assert!(original(&signed.signed_update, uploader, issuer, &expected).is_err());
    }
    let wrong_method = agent
        .update(&target.as_slice().try_into().unwrap(), "unrelated")
        .with_arg(encode_args(&(root.clone(),), 1024).unwrap())
        .sign()
        .unwrap();
    for packet in [
        wrong_method.signed_update,
        vec![],
        vec![0; PACKET_LIMIT + 1],
    ] {
        assert!(original(&packet, sender, target, &root).is_err());
    }
}

#[test]
fn exact_upload_reply_and_root_admission() {
    let expected = format!("sha256:{}", "ab".repeat(32));
    assert_eq!(root(&expected).unwrap(), expected);
    for value in [
        expected.to_uppercase(),
        "sha256:00".into(),
        format!("sha256:{}", "gg".repeat(32)),
    ] {
        assert!(root(&value).is_err());
    }
    let reply = |method: &str, hash: &str| {
        encode_args(
            &(Reply {
                method: method.into(),
                blob_hash: hash.into(),
            },),
            1024,
        )
        .unwrap()
    };
    let good = reply("upload", &expected);
    admit_reply(&good, &expected).unwrap();
    let mut trailing = good;
    trailing.push(0);
    for bytes in [
        reply("delete", &expected),
        reply("upload", "wrong-root"),
        trailing,
        vec![0; 1025],
    ] {
        assert!(admit_reply(&bytes, &expected).is_err());
    }
}
