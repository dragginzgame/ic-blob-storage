//! Native client subprocess with fixed test credentials and explicit local trust.
use candid::Principal;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use serde_json::Value;
use std::path::Path;
use std::process::Command;

// Fixed test-only Ed25519 seed [42; 32], never a deployment identity.
pub(super) const PEM: &str = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEICoqKioqKioqKioqKioqKioqKioqKioqKioqKioqKioq\n-----END PRIVATE KEY-----\n";

pub(super) fn arguments(
    command: &str,
    scope: OperatorScope,
    operator: Principal,
    url: &str,
    key: &Path,
    root: &Path,
) -> Vec<String> {
    [
        command,
        "--network",
        "local",
        "--url",
        url,
        "--identity",
        key.to_str().unwrap(),
        "--operator",
        &operator.to_text(),
        "--service",
        &scope.service.to_text(),
        "--namespace",
        &scope.namespace.to_string(),
        "--cashier",
        &scope.cashier.to_text(),
        "--payer",
        &scope.payment_account.to_text(),
        "--root-key",
        root.to_str().unwrap(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

pub(super) fn run(args: &[String], code: i32) -> Value {
    run_with_tls_roots(args, code, None)
}

/// Override only the child process's TLS trust for an owned HTTPS fixture.
pub(super) fn run_with_tls_roots(args: &[String], code: i32, roots: Option<&Path>) -> Value {
    let executable = std::env::var_os("BLOB_CLI_BIN").expect("explicit signed CLI artifact");
    assert!(
        Path::new(&executable).is_file(),
        "signed CLI artifact exists"
    );
    let mut command = Command::new(executable);
    command
        .args(args)
        // Local mode must bypass ambient proxies, even without NO_PROXY exclusions.
        .env("HTTP_PROXY", "http://127.0.0.1:9")
        .env("http_proxy", "http://127.0.0.1:9")
        .env("HTTPS_PROXY", "http://127.0.0.1:9")
        .env("https_proxy", "http://127.0.0.1:9")
        .env("ALL_PROXY", "http://127.0.0.1:9")
        .env("all_proxy", "http://127.0.0.1:9")
        .env("NO_PROXY", "")
        .env("no_proxy", "");
    if let Some(roots) = roots {
        assert!(roots.is_file(), "explicit fixture TLS root file exists");
        command
            .env("SSL_CERT_FILE", roots)
            .env_remove("SSL_CERT_DIR");
    }
    let result = command.output().unwrap();
    assert_eq!(
        result.status.code(),
        Some(code),
        "command {} stdout: {} stderr: {}",
        args[0],
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&result.stdout).unwrap()
}
