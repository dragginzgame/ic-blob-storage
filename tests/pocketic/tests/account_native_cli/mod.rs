//! Shared signed account client journey; all provider reports are local substitutes.
use crate::authenticated_cli::{PEM, arguments, run};
use candid::Principal;
use ic_agent::{Identity, identity::BasicIdentity};
use ic_blob_storage_contracts::dto::account::AccountInspectionKind as Kind;
use ic_blob_storage_contracts::dto::account::AccountInspectionRequest;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use serde_json::{Value, json};
use std::{cell::Cell, path::PathBuf};

// A distinct fixed test-only Ed25519 seed [43; 32].
pub(super) const OUTSIDER_PEM: &str = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEICsrKysrKysrKysrKysrKysrKysrKysrKysrKysrKysr\n-----END PRIVATE KEY-----\n";

pub(super) fn signer() -> Principal {
    BasicIdentity::from_raw_key(&[42; 32]).sender().unwrap()
}
pub(super) struct NativeAccountCli {
    scope: OperatorScope,
    args: Vec<String>,
    key: PathBuf,
    root: PathBuf,
    trusted: Vec<u8>,
    sequence: Cell<u32>,
    report: PathBuf,
    _temporary: tempfile::TempDir,
}
impl NativeAccountCli {
    pub fn new(scope: OperatorScope, url: &str, trusted: Vec<u8>, label: &str) -> Self {
        let temporary = tempfile::tempdir().unwrap();
        let key = temporary.path().join("identity.pem");
        let root = temporary.path().join("root.der");
        std::fs::write(&key, PEM).unwrap();
        std::fs::write(&root, &trusted).unwrap();
        let report = std::env::var_os("BLOB_ACCOUNT_REPORT").map_or_else(
            || temporary.path().join(label),
            |p| PathBuf::from(p).join(label),
        );
        std::fs::create_dir(&report).unwrap();
        save(&report.join("root.der"), &trusted);
        let plan = json!({"evidence":"local_query_only_substitute","service":scope.service.to_text(),
            "operator":signer().to_text(),"namespace":scope.namespace.to_string(),
            "cashier":scope.cashier.to_text(),"payer":scope.payment_account.to_text(),
            "max_cli_invocations":24,"max_local_provider_queries":24,"deadline_seconds":30,
            "transport_bytes":262_144,"account_reply_bytes":4096,"deployed_provider_requests":0,"attached_provider_cycles":"0"});
        save(
            &report.join("plan.json"),
            &serde_json::to_vec_pretty(&plan).unwrap(),
        );
        Self {
            scope,
            args: arguments("inspect-account", scope, signer(), url, &key, &root),
            key,
            root,
            trusted,
            sequence: Cell::new(0),
            report,
            _temporary: temporary,
        }
    }
    fn arguments(&self, kind: Kind) -> Vec<String> {
        let mut args = self.args.clone();
        args.extend([
            "--kind".into(),
            match kind {
                Kind::Balance => "balance",
                Kind::PaymentRelationship => "relationship",
            }
            .into(),
        ]);
        args
    }
    fn execute(&self, args: &[String], code: i32) -> Value {
        let sequence = self.sequence.get() + 1;
        assert!(sequence <= 24, "bounded local CLI journey");
        self.sequence.set(sequence);
        save(
            &self.report.join(format!("{sequence:02}-command.json")),
            &serde_json::to_vec(args).unwrap(),
        );
        let result = run(args, code);
        save(
            &self.report.join(format!("{sequence:02}-result.json")),
            &serde_json::to_vec(&result).unwrap(),
        );
        result
    }
    pub fn reports(&self, configure: impl Fn(Kind, Vec<u8>)) {
        for (index, (kind, bytes, expected)) in cases(self.scope).into_iter().enumerate() {
            save(
                &self.report.join(format!("source-{index:02}.candid")),
                &bytes,
            );
            save(
                &self.report.join(format!("request-{index:02}.candid")),
                &candid::encode_one(AccountInspectionRequest {
                    scope: self.scope,
                    kind,
                })
                .unwrap(),
            );
            configure(kind, bytes);
            let value = self.execute(&self.arguments(kind), 0);
            assert_eq!(value["observation"], "account_inspection");
            assert_eq!(value["verification"], "ic_update_certificate");
            assert_eq!(value["provider_credit"], "not_established");
            assert_eq!(value["spendability"], "not_established");
            assert_eq!(value["retry_authorized"], false);
            assert_eq!(value["scope"]["namespace"], u128::MAX.to_string());
            assert_eq!(value["provider_report"], expected);
        }
        for (index, bytes, error) in [
            (0, vec![0], "account_invalid_reply"),
            (1, vec![0; 4097], "account_reply_too_large"),
        ] {
            save(
                &self.report.join(format!("invalid-source-{index}.candid")),
                &bytes,
            );
            configure(Kind::Balance, bytes);
            assert_eq!(
                self.execute(&self.arguments(Kind::Balance), 3)["error"],
                error
            );
        }
    }
    pub fn refusals(&self, configure: impl Fn(Kind, Vec<u8>)) {
        configure(Kind::Balance, balance(self.scope.payment_account));
        let args = self.arguments(Kind::Balance);
        let other = BasicIdentity::from_raw_key(&[43; 32]);
        let principal = other.sender().unwrap().to_text();
        let mut wrong = args.clone();
        change(&mut wrong, "--operator", &principal);
        assert_eq!(self.execute(&wrong, 3)["error"], "identity_binding");
        std::fs::write(&self.key, OUTSIDER_PEM).unwrap();
        assert_eq!(self.execute(&wrong, 3)["error"], "account_denied");
        std::fs::write(&self.key, PEM).unwrap();
        for (flag, value) in [
            ("--namespace", "1"),
            ("--cashier", principal.as_str()),
            ("--payer", principal.as_str()),
        ] {
            let mut wrong = args.clone();
            change(&mut wrong, flag, value);
            assert_eq!(self.execute(&wrong, 3)["error"], "account_binding");
        }
        let mut untrusted = self.trusted.clone();
        *untrusted.last_mut().unwrap() ^= 1;
        std::fs::write(&self.root, untrusted).unwrap();
        // Update trust failure may follow execution. It is not evidence of an
        // unsent provider read and never causes this client to resubmit.
        assert_eq!(self.execute(&args, 3)["error"], "transport");
        std::fs::write(&self.root, &self.trusted).unwrap();
    }
    pub fn fenced(&self) {
        for kind in [Kind::Balance, Kind::PaymentRelationship] {
            assert_eq!(
                self.execute(&self.arguments(kind), 3)["error"],
                "account_fenced"
            );
        }
    }
}
fn save(path: &std::path::Path, bytes: &[u8]) {
    use std::io::Write;
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap()
        .write_all(bytes)
        .unwrap();
}
fn change(args: &mut [String], flag: &str, value: &str) {
    let i = args.iter().position(|s| s == flag).unwrap();
    args[i + 1] = value.into();
}
fn wire(value: &str) -> Vec<u8> {
    candid_parser::parse_idl_args(value)
        .unwrap()
        .to_bytes()
        .unwrap()
}
fn balance(account: Principal) -> Vec<u8> {
    wire(&format!(
        r#"(variant {{ Ok = record {{ account = principal "{account}"; account_cycle_balances = record {{ total = 340282366920938463463374607431768211455 : int; cycles_prepaid = 2 : int; cycles_promo = 3 : int; cycles_ledger = 4 : int; debt_target = variant {{ Prepaid }} }} }} }})"#
    ))
}
fn cases(scope: OperatorScope) -> Vec<(Kind, Vec<u8>, Value)> {
    vec![
        (
            Kind::Balance,
            balance(scope.payment_account),
            json!({"kind":"reported_balance","total":u128::MAX.to_string(),"prepaid":"2","promotional":"3","ledger":"4"}),
        ),
        (
            Kind::PaymentRelationship,
            wire(&format!(
                r#"(variant {{ Ok = record {{ relationship = opt record {{ paid_canister = principal "{}"; payment_account = principal "{}"; spending_limit_per_day = -340282366920938463463374607431768211456 : int; current_period_spent = 340282366920938463463374607431768211456 : int; current_period_start = 18446744073709551615 : nat64; added_timestamp = 1 : nat64; expiration_timestamp = null; bandwidth_baseline_uploaded = 340282366920938463463374607431768211456 : nat; bandwidth_baseline_downloaded = 2 : nat; bandwidth_baseline_ts_ns = 3 : nat64 }} }} }})"#,
                scope.service, scope.payment_account
            )),
            json!({"kind":"reported_relationship","paid_canister":scope.service.to_text(),"payment_account":scope.payment_account.to_text(),"spending_limit_per_day":"-340282366920938463463374607431768211456","current_period_spent":"340282366920938463463374607431768211456","current_period_start":u64::MAX.to_string(),"added_timestamp":"1","expiration_timestamp":null,"bandwidth_baseline_uploaded":"340282366920938463463374607431768211456","bandwidth_baseline_downloaded":"2","bandwidth_baseline_ts_ns":"3"}),
        ),
        (
            Kind::Balance,
            wire("(variant { Err = variant { AccountNotFound } })"),
            json!({"kind":"account_not_found"}),
        ),
        (
            Kind::PaymentRelationship,
            wire("(variant { Ok = record { relationship = null } })"),
            json!({"kind":"no_relationship_reported"}),
        ),
        (
            Kind::PaymentRelationship,
            wire("(variant { Err = variant { NotAuthorized = principal \"2vxsx-fae\" } })"),
            json!({"kind":"not_authorized","reported_principal":"2vxsx-fae"}),
        ),
    ]
}
