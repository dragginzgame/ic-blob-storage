//! Shared actual signed operator decisions; provider gateway lists are local substitutes.
use crate::{
    account_native_cli::{OUTSIDER_PEM, signer},
    authenticated_cli::{PEM, arguments, run},
    submission_proxy::{Dispatch, Proxy, Reply},
};
use blob_test_protocol::SourceMode;
use candid::Principal;
use ic_agent::{Identity, identity::BasicIdentity};
use ic_blob_storage::dto::operator::OperatorScope;
use serde_json::{Value, json};
use std::{cell::Cell, path::PathBuf};

pub(super) struct GatewayCli {
    scope: OperatorScope,
    gateway: Principal,
    backend: String,
    base: Vec<String>,
    key: PathBuf,
    report: PathBuf,
    sequence: Cell<u32>,
    baseline: Value,
    _temporary: tempfile::TempDir,
}
impl GatewayCli {
    pub fn new(
        scope: OperatorScope,
        gateway: Principal,
        backend: String,
        trusted: &[u8],
        label: &str,
    ) -> Self {
        let temporary = tempfile::tempdir().unwrap();
        let key = temporary.path().join("identity.pem");
        let root = temporary.path().join("root.der");
        std::fs::write(&key, PEM).unwrap();
        std::fs::write(&root, trusted).unwrap();
        let report = std::env::var_os("BLOB_GATEWAY_CONTROL_REPORT").map_or_else(
            || temporary.path().join(label),
            |p| PathBuf::from(p).join(label),
        );
        std::fs::create_dir(&report).unwrap();
        save(&report.join("root.der"), trusted);
        save(&report.join("plan.json"),&serde_json::to_vec_pretty(&json!({
            "evidence":"local_query_only_substitute","operator":signer().to_text(),
            "service":scope.service.to_text(),"namespace":scope.namespace.to_string(),
            "cashier":scope.cashier.to_text(),"payer":scope.payment_account.to_text(),
            "gateway":gateway.to_text(),"max_cli_invocations":40,"max_local_provider_queries":20,
            "client_deadline_seconds":30,"http_reply_bytes":262_144,"service_reply_bytes":4096,
            "deployed_provider_requests":0,"attached_provider_cycles":"0"
        })).unwrap());
        let base = arguments("status", scope, signer(), &backend, &key, &root);
        let mut client = Self {
            scope,
            gateway,
            backend,
            base,
            key,
            report,
            sequence: Cell::new(0),
            baseline: Value::Null,
            _temporary: temporary,
        };
        client.baseline = client.status("initial", false);
        assert_ne!(client.baseline["uploads"]["reserved_bytes"], "0");
        client
    }
    fn execute(&self, args: &[String], exit: i32) -> Value {
        let n = self.sequence.get() + 1;
        assert!(n <= 40, "bounded CLI journey");
        self.sequence.set(n);
        save(
            &self.report.join(format!("{n:02}-command.json")),
            &serde_json::to_vec(args).unwrap(),
        );
        let value = run(args, exit);
        save(
            &self.report.join(format!("{n:02}-result.json")),
            &serde_json::to_vec(&value).unwrap(),
        );
        value
    }
    fn args(&self, name: &str, label: &str, sequence: Option<u64>) -> Vec<String> {
        let mut args = self.base.clone();
        args[0] = name.into();
        args.extend([
            "--run-dir".into(),
            self.report.join(label).to_str().unwrap().into(),
        ]);
        if let Some(sequence) = sequence {
            args.extend(["--sequence".into(), sequence.to_string()]);
        }
        if name == "revoke-gateway" {
            args.extend(["--gateway".into(), self.gateway.to_text()]);
        }
        args
    }
    fn send(
        &self,
        name: &str,
        label: &str,
        sequence: Option<u64>,
        reply: Reply,
        expected: &str,
    ) -> Value {
        let mut args = self.args(name, label, sequence);
        let method = match name {
            "sync-gateways" => "blob_sync_gateways",
            "cancel-gateway-sync" => "blob_cancel_gateway_sync",
            _ => "blob_revoke_gateway",
        };
        let proxy = Proxy::start(
            self.backend.clone(),
            self.report.join(label),
            Dispatch {
                service: self.scope.service,
                actor: signer(),
                method,
                argument_file: "request.candid",
            },
            reply,
        );
        change(&mut args, "--url", &proxy.url);
        let output = self.execute(
            &args,
            if expected == "acknowledged" || expected == "pending" {
                0
            } else {
                3
            },
        );
        assert_eq!(proxy.calls(), 1);
        let record: Value = serde_json::from_slice(
            &std::fs::read(self.report.join(label).join("outcome.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(record["outcome"], expected);
        assert_eq!(record["retry_authorized"], false);
        assert_eq!(record["provider_deletion"], "not_established");
        assert_eq!(record["billing_cessation"], "not_established");
        assert_eq!(record["historical_receipt"], "not_available");
        if matches!(reply, Reply::Drop | Reply::Pending) {
            assert_eq!(
                self.execute(&args, 3)["error"],
                "submission_already_claimed"
            );
            assert_eq!(proxy.calls(), 1);
        }
        if expected == "refused" {
            assert_eq!(output["error"], record["error"]);
        }
        record
    }
    pub fn status(&self, label: &str, fenced: bool) -> Value {
        let status = self.execute(&self.base, 0);
        for owner in ["uploads", "funding", "reads"] {
            assert_eq!(status[owner]["fenced"], fenced);
            if !self.baseline.is_null() {
                let mut current = status[owner].clone();
                current["fenced"] = false.into();
                assert_eq!(current, self.baseline[owner]);
            }
        }
        assert_eq!(status["gateways"]["fenced"], fenced);
        save(
            &self.report.join(format!("{label}-status.json")),
            &serde_json::to_vec(&status).unwrap(),
        );
        status
    }
    pub fn refusals(&self) {
        let outsider = BasicIdentity::from_raw_key(&[43; 32]).sender().unwrap();
        for (i, (name, sequence)) in [
            ("sync-gateways", None),
            ("cancel-gateway-sync", Some(1)),
            ("revoke-gateway", None),
        ]
        .into_iter()
        .enumerate()
        {
            let label = format!("foreign-{i}");
            let mut args = self.args(name, &label, sequence);
            change(&mut args, "--operator", &outsider.to_text());
            std::fs::write(&self.key, OUTSIDER_PEM).unwrap();
            assert_eq!(self.execute(&args, 3)["error"], "gateway_denied");
            std::fs::write(&self.key, PEM).unwrap();
            let outcome: Value = serde_json::from_slice(
                &std::fs::read(self.report.join(label).join("outcome.json")).unwrap(),
            )
            .unwrap();
            assert_eq!(outcome["outcome"], "refused");
        }
        for (flag, value, label) in [
            ("--namespace", "1".into(), "wrong-namespace"),
            ("--cashier", outsider.to_text(), "wrong-cashier"),
            ("--payer", outsider.to_text(), "wrong-payer"),
        ] {
            let mut args = self.args("sync-gateways", label, None);
            change(&mut args, flag, &value);
            assert_eq!(self.execute(&args, 3)["error"], "gateway_binding");
        }
        let mut args = self.args("sync-gateways", "wrong-identity", None);
        change(&mut args, "--operator", &outsider.to_text());
        assert_eq!(self.execute(&args, 3)["error"], "identity_binding");
        assert!(!self.report.join("wrong-identity").exists());
        let args = self.args("cancel-gateway-sync", "invalid", Some(0));
        assert_eq!(self.execute(&args, 2)["error"], "arguments");
        assert!(!self.report.join("invalid").exists());
        std::fs::create_dir(self.report.join("interrupted")).unwrap();
        assert_eq!(
            self.execute(&self.args("sync-gateways", "interrupted", None), 3)["error"],
            "submission_already_claimed"
        );
        assert_eq!(
            std::fs::read_dir(self.report.join("interrupted"))
                .unwrap()
                .count(),
            0
        );
    }
    fn mode(&self, label: &str, mode: SourceMode, configure: &impl Fn(SourceMode)) {
        save(
            &self.report.join(format!("{label}-source.json")),
            &serde_json::to_vec(&json!({"local_source_mode":format!("{mode:?}")})).unwrap(),
        );
        configure(mode);
    }
    pub fn decisions(&self, configure: impl Fn(SourceMode)) -> String {
        self.mode("valid", SourceMode::Valid, &configure);
        assert_eq!(
            self.send(
                "sync-gateways",
                "initial-sync",
                None,
                Reply::Pass,
                "acknowledged"
            )["acknowledgment"]["sequence"],
            "1"
        );
        self.send(
            "revoke-gateway",
            "lost-revocation",
            None,
            Reply::Drop,
            "uncertain",
        );
        assert_eq!(
            self.status("lost-revocation", false)["gateways"]["members"],
            json!([])
        );
        self.send(
            "sync-gateways",
            "pending-sync",
            None,
            Reply::Pending,
            "pending",
        );
        let status = self.status("pending-sync", false);
        assert_eq!(
            status["gateways"]["members"],
            json!([self.gateway.to_text()])
        );
        assert_eq!(status["gateways"]["last_sequence"], "2");
        self.failed_decisions(&configure)
    }
    fn failed_decisions(&self, configure: &impl Fn(SourceMode)) -> String {
        self.mode("malformed", SourceMode::Malformed, configure);
        let refused = self.send(
            "sync-gateways",
            "malformed-sync",
            None,
            Reply::Pass,
            "refused",
        );
        assert_eq!(refused["error"], "gateway_invalid_reply");
        assert_eq!(
            self.status("failed-sync", false)["gateways"]["pending_sequence"],
            "3"
        );
        assert_eq!(
            self.send("sync-gateways", "busy", None, Reply::Pass, "refused")["error"],
            "gateway_busy"
        );
        assert_eq!(
            self.send(
                "cancel-gateway-sync",
                "stale-cancel",
                Some(2),
                Reply::Pass,
                "refused"
            )["error"],
            "gateway_conflict"
        );
        self.send(
            "cancel-gateway-sync",
            "lost-cancel",
            Some(3),
            Reply::Drop,
            "uncertain",
        );
        assert_eq!(
            self.status("lost-cancel", false)["gateways"]["pending_sequence"],
            Value::Null
        );
        assert_eq!(
            self.send(
                "cancel-gateway-sync",
                "already-cancelled",
                Some(3),
                Reply::Pass,
                "refused"
            )["error"],
            "gateway_conflict"
        );
        self.oversized_decisions(configure)
    }
    fn oversized_decisions(&self, configure: &impl Fn(SourceMode)) -> String {
        self.mode("oversized", SourceMode::Oversized, configure);
        assert_eq!(
            self.send(
                "sync-gateways",
                "oversized-sync",
                None,
                Reply::Pass,
                "refused"
            )["error"],
            "gateway_reply_too_large"
        );
        assert_eq!(
            self.status("oversized", false)["gateways"]["pending_sequence"],
            "4"
        );
        assert_eq!(
            self.send(
                "cancel-gateway-sync",
                "acknowledged-cancel",
                Some(4),
                Reply::Pass,
                "acknowledged"
            )["acknowledgment"]["sequence"],
            "4"
        );
        assert_eq!(
            self.send(
                "sync-gateways",
                "another-oversized-sync",
                None,
                Reply::Pass,
                "refused"
            )["error"],
            "gateway_reply_too_large"
        );
        self.send(
            "revoke-gateway",
            "pending-revocation",
            None,
            Reply::Pending,
            "pending",
        );
        let status = self.status("pending-revocation", false);
        assert_eq!(status["gateways"]["members"], json!([]));
        assert_eq!(status["gateways"]["pending_sequence"], Value::Null);
        assert_eq!(
            self.send(
                "revoke-gateway",
                "acknowledged-absent-revocation",
                None,
                Reply::Pass,
                "acknowledged"
            )["acknowledgment"]["removed"],
            false
        );
        self.mode("reject", SourceMode::Reject, configure);
        assert_eq!(
            self.send(
                "sync-gateways",
                "rejected-sync",
                None,
                Reply::Pass,
                "refused"
            )["error"],
            "gateway_rejected"
        );
        self.status("pre-restore", false)["gateways"]["pending_sequence"]
            .as_str()
            .unwrap()
            .into()
    }
    pub fn fenced(&self, pending: &str) {
        for (name, sequence, label) in [
            ("sync-gateways", None, "fenced-sync"),
            (
                "cancel-gateway-sync",
                Some(pending.parse().unwrap()),
                "fenced-cancel",
            ),
            ("revoke-gateway", None, "fenced-revoke"),
        ] {
            assert_eq!(
                self.send(name, label, sequence, Reply::Pass, "refused")["error"],
                "gateway_fenced"
            );
        }
        let status = self.status("restored", true);
        assert_eq!(status["gateways"]["pending_sequence"], pending);
        assert_eq!(status["gateways"]["members"], json!([]));
    }
}
fn change(args: &mut [String], flag: &str, value: &str) {
    let i = args.iter().position(|s| s == flag).unwrap();
    args[i + 1] = value.into();
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
