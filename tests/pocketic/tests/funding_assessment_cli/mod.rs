//! Signed passive funding diagnosis through both adapters; no external provider effects.
use crate::{
    account_native_cli::{OUTSIDER_PEM, signer},
    authenticated_cli::{PEM, arguments, run},
};
use candid::Principal;
use ic_agent::{Identity, identity::BasicIdentity};
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use serde_json::{Value, json};
use std::{cell::Cell, collections::BTreeSet, path::PathBuf};

pub(super) struct AssessmentCli {
    base: Vec<String>,
    key: PathBuf,
    report: PathBuf,
    sequence: Cell<u32>,
    baseline: Value,
    _temporary: tempfile::TempDir,
}
impl AssessmentCli {
    pub fn new(scope: OperatorScope, url: &str, trusted: &[u8], label: &str) -> Self {
        let temporary = tempfile::tempdir().unwrap();
        let key = temporary.path().join("identity.pem");
        let root = temporary.path().join("root.der");
        std::fs::write(&key, PEM).unwrap();
        std::fs::write(&root, trusted).unwrap();
        let report = std::env::var_os("BLOB_FUNDING_ASSESSMENT_REPORT").map_or_else(
            || temporary.path().join(label),
            |p| PathBuf::from(p).join(label),
        );
        std::fs::create_dir(&report).unwrap();
        save(&report.join("root.der"), trusted);
        save(
            &report.join("plan.json"),
            &serde_json::to_vec_pretty(&json!({
                "evidence":"local_passive_funding_assessment","operator":signer().to_text(),
                "service":scope.service.to_text(),"namespace":scope.namespace.to_string(),
                "cashier":scope.cashier.to_text(),"payer":scope.payment_account.to_text(),
                "max_cli_invocations":24,"client_deadline_seconds":30,
                "http_reply_bytes":262_144,"assessment_reply_bytes":4096,
                "cashier_requests":0,"deployed_provider_requests":0,"attached_provider_cycles":"0"
            }))
            .unwrap(),
        );
        let base = arguments("funding-assessment", scope, signer(), url, &key, &root);
        let mut client = Self {
            base,
            key,
            report,
            sequence: Cell::new(0),
            baseline: Value::Null,
            _temporary: temporary,
        };
        let mut status = client.base.clone();
        status[0] = "status".into();
        client.baseline = client.execute(&status, 0);
        assert_ne!(client.baseline["uploads"]["reserved_bytes"], "0");
        client
    }
    fn execute(&self, args: &[String], exit: i32) -> Value {
        let n = self.sequence.get() + 1;
        assert!(n <= 24, "bounded passive query journey");
        self.sequence.set(n);
        save(
            &self.report.join(format!("{n:02}-command.json")),
            &serde_json::to_vec(args).unwrap(),
        );
        let result = run(args, exit);
        save(
            &self.report.join(format!("{n:02}-result.json")),
            &serde_json::to_vec(&result).unwrap(),
        );
        result
    }
    fn request(&self, maximum: bool) -> Vec<String> {
        let mut args = self.base.clone();
        args.extend([
            "--operation".into(),
            u128::MAX.to_string(),
            "--offered".into(),
            if maximum {
                u128::MAX.to_string()
            } else {
                "1".into()
            },
        ]);
        if maximum {
            args.extend(["--target-balance".into(), u128::MAX.to_string()]);
        }
        args
    }
    pub fn inspect(&self, fenced: bool) {
        for maximum in [false, true] {
            let value = self.execute(&self.request(maximum), 0);
            assert_eq!(value["observation"], "funding_preparation_assessment");
            assert_eq!(value["verification"], "query_signatures");
            for authority in [
                "preparation_authorized",
                "dispatch_authorized",
                "retry_authorized",
            ] {
                assert_eq!(value[authority], false);
            }
            assert_eq!(value["provider_credit"], "not_established");
            assert_eq!(value["spendability"], "not_established");
            assert_eq!(value["request"]["operation"], u128::MAX.to_string());
            assert_eq!(
                value["request"]["offered"],
                if maximum {
                    u128::MAX.to_string()
                } else {
                    "1".into()
                }
            );
            assert_eq!(
                value["request"]["target_balance"],
                if maximum {
                    json!(u128::MAX.to_string())
                } else {
                    Value::Null
                }
            );
            let mut expected = self.baseline["funding"].clone();
            expected["fenced"] = json!(fenced);
            assert_eq!(value["journal"], expected);
            let mut kinds = BTreeSet::from([
                "provider_unqualified",
                "recovery_unknown",
                "funding_unknown",
                "spendability_unknown",
            ]);
            if fenced {
                kinds.insert("journal_fenced");
            }
            if maximum {
                kinds.insert("allocation_reserve");
            }
            let blockers = value["blockers"].as_array().unwrap();
            assert_eq!(
                blockers
                    .iter()
                    .map(|b| b["kind"].as_str().unwrap())
                    .collect::<BTreeSet<_>>(),
                kinds
            );
            if maximum {
                let reserve = blockers
                    .iter()
                    .find(|b| b["kind"] == "allocation_reserve")
                    .unwrap();
                assert_eq!(
                    reserve["attachment_allowance"],
                    expected["attachment_allowance"]
                );
            }
        }
    }
    pub fn refusals(&self) {
        let outsider = BasicIdentity::from_raw_key(&[43; 32]).sender().unwrap();
        let original = self.request(true);
        let mut args = original.clone();
        replace(&mut args, "--operator", outsider.to_text());
        assert_eq!(self.execute(&args, 3)["error"], "identity_binding");
        std::fs::write(&self.key, OUTSIDER_PEM).unwrap();
        assert_eq!(self.execute(&args, 3)["error"], "funding_denied");
        std::fs::write(&self.key, PEM).unwrap();
        for (flag, value) in [
            ("--namespace", "1".into()),
            ("--cashier", Principal::from_slice(&[99, 1]).to_text()),
            ("--payer", outsider.to_text()),
        ] {
            let mut args = original.clone();
            replace(&mut args, flag, value);
            assert_eq!(self.execute(&args, 3)["error"], "funding_binding");
        }
        let mut args = original.clone();
        args.extend(["--provider-qualified".into(), "true".into()]);
        assert_eq!(self.execute(&args, 2)["error"], "arguments");
        let root = self.key.with_file_name("root.der");
        let trusted = std::fs::read(&root).unwrap();
        let mut untrusted = trusted.clone();
        *untrusted.last_mut().unwrap() ^= 1;
        std::fs::write(&root, untrusted).unwrap();
        assert_eq!(self.execute(&self.request(true), 3)["error"], "transport");
        std::fs::write(&root, trusted).unwrap();
        let mut args = original;
        replace(&mut args, "--operation", "0".into());
        assert_eq!(self.execute(&args, 2)["error"], "arguments");
    }
}
fn replace(args: &mut [String], flag: &str, value: String) {
    let index = args.iter().position(|a| a == flag).unwrap();
    args[index + 1] = value;
}
fn save(path: &std::path::Path, bytes: &[u8]) {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    file.write_all(bytes).unwrap();
    file.sync_all().unwrap();
}
