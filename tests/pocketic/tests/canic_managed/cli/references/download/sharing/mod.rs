//! Two references deliver the same bytes; uncertainty and final liabilities stay distinct.
use super::*;

const RETAIN_FILES: [&str; 4] = [
    "request.candid",
    "signed-request.cbor",
    "intent.json",
    "outcome.json",
];
pub(super) struct RetainedReference {
    request: ReferenceCommand,
    saved: Vec<Vec<u8>>,
}
fn saved(t: &Client<'_>) -> Vec<Vec<u8>> {
    RETAIN_FILES
        .iter()
        .map(|name| std::fs::read(t.report.join("shared-retain").join(name)).unwrap())
        .collect()
}
fn status(t: &Client<'_>, live: bool, fenced: bool) {
    let args = t.reference_args(
        "reference-status",
        "shared-inputs/reference-status.candid",
        None,
    );
    let result = t.call(&args, 0);
    assert_eq!(result["live"], live);
    assert_eq!(result["fenced"], fenced);
    assert_eq!(result["retry_authorized"], false);
}
pub(super) fn retain(t: &Client<'_>) -> RetainedReference {
    let request = ReferenceCommand {
        upload: t.permission.upload,
        reference: u128::MAX - 4,
        operation: u128::MAX - 1,
        action: ReferenceAction::Retain,
    };
    assert_ne!(request.reference, request.upload.first_reference);
    t.prepare_reference(request, "shared-inputs");
    let proxy = Proxy::start(
        t.args[t.args.iter().position(|s| s == "--url").unwrap() + 1].clone(),
        t.report.join("shared-retain"),
        Dispatch {
            service: t.f.app(),
            actor: request.upload.tenant,
            method: "blob_apply_reference",
            argument_file: "request.candid",
        },
        Reply::Drop,
    );
    let mut args = t.reference_args(
        "submit-reference",
        "shared-inputs/reference.candid",
        Some("shared-retain"),
    );
    change(&mut args, "--url", &proxy.url);
    assert_eq!(t.call(&args, 3)["error"], "transport");
    assert_eq!(proxy.calls(), 1);
    drop(proxy);
    let original = saved(t);
    let receipt = t.reference_args("reference-receipt", "shared-retain/request.candid", None);
    let result = t.call(&receipt, 0);
    assert_eq!(result["outcome"], "found");
    assert_eq!(result["result"]["state"], "success");
    assert_eq!(result["reference_liveness"], "not_observed");
    status(t, true, false);
    let retry = t.reference_args(
        "submit-reference",
        "shared-retain/request.candid",
        Some("shared-retain"),
    );
    assert_eq!(t.call(&retry, 3)["error"], "submission_already_claimed");
    assert_eq!(saved(t), original);
    RetainedReference {
        request,
        saved: original,
    }
}
fn accounting(t: &Client<'_>, logical: u128, fenced: bool, label: &str) {
    let c = blob_canic_probe::configuration::input();
    let status: LocalServiceStatus =
        t.f.pic()
            .query_candid_as::<Result<_, LocalStatusFailure>, _>(
                t.f.app(),
                c.operator,
                "blob_local_status",
                (OperatorScope {
                    service: t.f.app(),
                    namespace: c.namespace,
                    cashier: c.billing.cashier,
                    payment_account: c.payment_account,
                },),
            )
            .unwrap()
            .unwrap();
    assert_eq!(
        [
            status.uploads.reserved_bytes,
            status.uploads.logical_bytes,
            status.uploads.physical_bytes,
            status.uploads.liability_bytes
        ],
        [0, logical, 10, 10]
    );
    assert_eq!(status.uploads.fenced, fenced);
    save(
        &t.report,
        label,
        &json!({"reserved_bytes":status.uploads.reserved_bytes.to_string(),
        "logical_bytes":status.uploads.logical_bytes.to_string(), "physical_bytes":status.uploads.physical_bytes.to_string(),
        "liability_bytes":status.uploads.liability_bytes.to_string(), "fenced":status.uploads.fenced}),
    );
}
pub(super) fn release_last(t: &Client<'_>, listener: &TcpListener, retained: &RetainedReference) {
    status(t, true, false);
    accounting(t, 10, false, "shared-accounting.json");
    t.fetch(
        listener,
        "shared-inputs/download.candid",
        "shared-download",
        "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n",
        &[42; 10],
        None,
    );
    assert_eq!(
        std::fs::read(t.report.join("shared-download/body.bin")).unwrap(),
        std::fs::read(t.report.join("good/body.bin")).unwrap()
    );
    let request = ReferenceCommand {
        action: ReferenceAction::Release,
        operation: u128::MAX - 2,
        ..retained.request
    };
    t.prepare_reference(request, "last-release-inputs");
    let args = t.reference_args(
        "submit-reference",
        "last-release-inputs/reference.candid",
        Some("last-release"),
    );
    let result = t.call(&args, 0);
    assert_eq!(result["result"]["state"], "success");
    assert_eq!(result["provider_deletion"], "not_established");
    assert_eq!(result["billing_cessation"], "not_established");
    status(t, false, false);
    refuse_download(t, listener, "last-released", "download_unavailable");
    accounting(t, 0, false, "released-accounting.json");
    assert_eq!(saved(t), retained.saved);
}
fn refuse_download(t: &Client<'_>, listener: &TcpListener, label: &str, error: &str) {
    let mut args = t.args(label);
    change(
        &mut args,
        "--request",
        t.report
            .join("shared-inputs/download.candid")
            .to_str()
            .unwrap(),
    );
    assert_eq!(t.call(&args, 3)["error"], error);
    assert!(!t.report.join(label).join("body.bin").exists());
    no_get(listener);
}
pub(super) fn restored(t: &Client<'_>, listener: &TcpListener, retained: &RetainedReference) {
    status(t, false, true);
    let receipt = t.reference_args("reference-receipt", "shared-retain/request.candid", None);
    let result = t.call(&receipt, 0);
    assert_eq!(result["outcome"], "found");
    assert_eq!(result["result"]["state"], "success");
    assert_eq!(result["reference_liveness"], "not_observed");
    let release = t.reference_args("reference-receipt", "last-release/request.candid", None);
    assert_eq!(t.call(&release, 0)["result"]["state"], "success");
    refuse_download(t, listener, "shared-restored", "download_fenced");
    let args = t.reference_args(
        "submit-reference",
        "last-release/request.candid",
        Some("shared-fenced-release"),
    );
    assert_eq!(t.call(&args, 3)["error"], "reference_fenced");
    accounting(t, 0, true, "restored-accounting.json");
    assert_eq!(saved(t), retained.saved);
    assert_eq!(
        std::fs::read(t.report.join("shared-download/body.bin")).unwrap(),
        [42; 10]
    );
}
