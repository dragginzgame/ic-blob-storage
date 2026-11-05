//! Typed command grammar; effect handlers retain authority and lifecycle policy.
use crate::native::publish_inputs::{
    MAX_FILE_BYTES, MAX_FILES, MAX_TIMEOUT_SECONDS, MAX_TOTAL_BYTES,
};
use clap::builder::{StringValueParser, TypedValueParser};
use clap::{Args, Parser, Subcommand, ValueEnum};
use ic_platform::Principal;
use std::{
    fmt::Display,
    num::{NonZeroU64, NonZeroU128},
    path::PathBuf,
    str::FromStr,
};
use url::Url;
// Canonical spelling is product policy; Clap owns invocation, storage and errors.
fn positive<T>(maximum: T) -> impl TypedValueParser<Value = T>
where
    T: FromStr + Display + Ord + Clone + Send + Sync + 'static,
{
    StringValueParser::new().try_map(move |text| {
        let value = crate::native::parsing::positive::<T>(&text)
            .map_err(|_| "invalid canonical positive integer")?;
        if value > maximum {
            Err("value exceeds command limit")
        } else {
            Ok(value)
        }
    })
}
fn principal(text: &str) -> Result<Principal, &'static str> {
    crate::native::parsing::principal(text).map_err(|_| "invalid canonical principal")
}
fn ordinary_principal(text: &str) -> Result<Principal, &'static str> {
    let value = principal(text)?;
    if value == Principal::anonymous() || value == Principal::management_canister() {
        Err("ordinary principal required")
    } else {
        Ok(value)
    }
}
fn file_index(text: &str) -> Result<usize, &'static str> {
    let index = text.parse::<usize>().map_err(|_| "invalid file index")?;
    if index.to_string() != text || index >= crate::native::publish_inputs::MAX_FILES {
        Err("invalid file index")
    } else {
        Ok(index)
    }
}
fn help(name: &str) -> &'static str {
    match name {
        "account-link-inputs" => {
            "Offline provider terms only; no signing, linking, funding or financial-cap proof."
        }
        "installation-check" => {
            "Offline validation and retained configuration packets; no deployment, memory grant or provider qualification."
        }
        "upload-inputs" | "publish-inputs" => {
            "Freeze verified bodies and caller-supplied identities offline. No allocation, installed-state proof or provider effect. Partial output is retained and cannot be overwritten."
        }
        "reference-inputs" => {
            "Encode exact reference/download requests from the saved permission. No identity allocation, mutation, liveness proof or retry authority."
        }
        "admit-upload" | "revoke-upload" => {
            "Tenant-authorized update. Persist exact signed intent before one dispatch; no polling or retry. Preserve partial and uncertain journals."
        }
        "prepare-upload" => {
            "Uploader-authorized manifest preparation. Persist exact signed intent before one dispatch; no certificate, provider upload or automatic retry."
        }
        "upload-permission" | "upload-manifest" => {
            "Inspect the exact saved permission. Historical state does not authorize resubmission or establish stored bytes."
        }
        "submit-reference" => {
            "Submit the exact retained reference command once. Lost replies require receipt inspection; never allocate a replacement operation to retry."
        }
        "reference-receipt" | "reference-status" => {
            "Read the exact reference identity without mutation. Observation is not a retention lease or retry permit."
        }
        "download" => {
            "One replicated descriptor and one bounded provider GET. Only root-verified bytes become body.bin; partial output remains body.part. No automatic retry."
        }
        "verify-upload" => {
            "Compare a local body with the saved declaration. This does not establish provider retention or issue a certificate."
        }
        "certificate-assessment" => {
            "Inspect current issuance eligibility without issuing a certificate. A successful assessment is not retry authority."
        }
        "upload-history" | "funding-history" => {
            "One bounded page. Retain a non-null cursor for explicit continuation, including empty filtered pages; no automatic pagination or provider calls."
        }
        "publish-check" => {
            "Retain bounded capacity/content observations. Existing roots require recovery, retain or retirement handling; no reservation, mutation or publication authority."
        }
        "publish-map" | "publish-file-status" => {
            "Authenticate current scope and exact reference liveness. Sequential observations are not a publication lease or provider availability proof; incomplete batches emit no map."
        }
        "publish-prepare" | "publish-prepare-resume" => {
            "At most two original setup updates and four queries per file. Resume binds --source-run and never resubmits a claimed update. Preserve original journals; no certificate or provider transfer."
        }
        "publish-prepare-batch" => {
            "Serial per-file preparation stops at the first blocker or uncertainty. Never resume the whole batch; use publish-prepare-resume with the original file journal."
        }
        "publish-session" => {
            "Bounded JSON-lines prepare/transfer/status/map session. Retain browser handoff journals for lost replies. Registration is not provider availability; no automatic retry or publication lease."
        }
        "sync-gateways" | "cancel-gateway-sync" | "revoke-gateway" => {
            "Operator-authorized control with retained intent. Uncertain results require original-state inspection, never automatic redispatch."
        }
        _ => {
            "Authenticated observation only. Local accounting and provider acknowledgments do not establish independent account credit or authorize a paid effect."
        }
    }
}

#[derive(Parser)]
#[command(
    name = "blob-storage",
    disable_version_flag = true,
    args_conflicts_with_subcommands = true,
    arg_required_else_help = true,
    after_long_help = "Use COMMAND --help for operation constraints. New run directories refuse existing or partial output and CR/LF paths. Preserve original journals after uncertainty; never infer retry authority from a missing reply. See docs/operator-guide.md for recovery procedures."
)]
pub(in crate::native) struct Cli {
    #[arg(long, exclusive = true)]
    pub version: bool,
    #[command(subcommand)]
    pub command: Option<Invocation>,
}
#[derive(Subcommand)]
pub(in crate::native) enum Invocation {
    #[command(after_long_help = help("account-link-inputs"))]
    AccountLinkInputs(AccountLink),
    #[command(after_long_help = help("installation-check"))]
    InstallationCheck(Installation),
    #[command(after_long_help = help("upload-inputs"))]
    UploadInputs(UploadInputs),
    #[command(after_long_help = help("publish-inputs"))]
    PublishInputs(PublishInputs),
    #[command(after_long_help = help("reference-inputs"))]
    ReferenceInputs(ReferenceInputs),
    #[command(after_long_help = help("admit-upload"))]
    AdmitUpload(Tenant<Mutation>),
    #[command(after_long_help = help("prepare-upload"))]
    PrepareUpload(Tenant<Mutation>),
    #[command(after_long_help = help("revoke-upload"))]
    RevokeUpload(Tenant<Mutation>),
    #[command(after_long_help = help("upload-permission"))]
    UploadPermission(Tenant<Request>),
    #[command(after_long_help = help("upload-manifest"))]
    UploadManifest(Tenant<Request>),
    #[command(after_long_help = help("submit-reference"))]
    SubmitReference(Tenant<Mutation>),
    #[command(after_long_help = help("reference-receipt"))]
    ReferenceReceipt(Tenant<Request>),
    #[command(after_long_help = help("reference-status"))]
    ReferenceStatus(Tenant<Request>),
    #[command(after_long_help = help("download"))]
    Download(Tenant<Download>),
    #[command(after_long_help = help("verify-upload"))]
    VerifyUpload(Tenant<VerifyUpload>),
    #[command(after_long_help = help("certificate-assessment"))]
    CertificateAssessment(Tenant<Permission>),
    #[command(after_long_help = help("upload-history"))]
    UploadHistory(Operator<UploadHistory>),
    #[command(after_long_help = help("status"))]
    Status(Operator<Payer<Empty>>),
    #[command(after_long_help = help("funding-history"))]
    FundingHistory(Operator<Payer<Cursor>>),
    #[command(after_long_help = help("inspect-account"))]
    InspectAccount(Operator<Payer<AccountKind>>),
    #[command(after_long_help = help("funding-assessment"))]
    FundingAssessment(Operator<Payer<Amount>>),
    #[command(after_long_help = help("funding-outcome"))]
    FundingOutcome(Operator<Payer<Amount>>),
    #[command(after_long_help = help("sync-gateways"))]
    SyncGateways(Operator<Payer<Directory>>),
    #[command(after_long_help = help("cancel-gateway-sync"))]
    CancelGatewaySync(Operator<Payer<Cancel>>),
    #[command(after_long_help = help("revoke-gateway"))]
    RevokeGateway(Operator<Payer<Revoke>>),
    #[command(after_long_help = help("publish-check"))]
    PublishCheck(Tenant<Checks<{ MAX_FILES as u64 + 1 }>>),
    #[command(after_long_help = help("publish-map"), after_help = "Requires BLOB_EXPECTED_HOST_RELEASE: the exact deployed Engine release, independent of Client and Contracts versions.")]
    PublishMap(Tenant<Map<{ 2 * MAX_FILES as u64 + 2 }>>),
    #[command(after_long_help = help("publish-file-status"), after_help = "Requires BLOB_EXPECTED_HOST_RELEASE: the exact deployed Engine release, independent of Client and Contracts versions.")]
    PublishFileStatus(Tenant<FileStatus>),
    #[command(after_long_help = help("publish-prepare"))]
    PublishPrepare(Tenant<IndexedPrepare>),
    #[command(after_long_help = help("publish-prepare-resume"))]
    PublishPrepareResume(Tenant<ResumePrepare>),
    #[command(after_long_help = help("publish-prepare-batch"))]
    PublishPrepareBatch(Tenant<Prepare>),
    #[command(after_long_help = help("publish-session"), after_help = "Requires BLOB_EXPECTED_HOST_RELEASE: the exact deployed Engine release, independent of Client and Contracts versions.")]
    PublishSession(Tenant<Session>),
}
#[derive(Clone, Copy, ValueEnum)]
pub(in crate::native) enum Network {
    Ic,
    Local,
}
impl Network {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ic => "ic",
            Self::Local => "local",
        }
    }
}
#[derive(Args)]
pub(in crate::native) struct Connection {
    #[arg(long)]
    pub network: Network,
    #[arg(long)]
    pub url: Url,
    #[arg(long)]
    pub identity: PathBuf,
    #[arg(long, value_parser = ordinary_principal)]
    pub service: Principal,
    #[arg(long, value_parser = positive(u128::MAX))]
    pub namespace: u128,
    #[arg(long)]
    pub root_key: Option<PathBuf>,
}
#[derive(Args)]
pub(in crate::native) struct Tenant<T: Args> {
    #[command(flatten)]
    pub connection: Connection,
    #[arg(long, value_parser = ordinary_principal)]
    pub actor: Principal,
    #[command(flatten)]
    pub input: T,
}
#[derive(Args)]
pub(in crate::native) struct Operator<T: Args> {
    #[command(flatten)]
    pub connection: Connection,
    #[arg(long = "operator", value_parser = ordinary_principal)]
    pub authority: Principal,
    #[command(flatten)]
    pub input: T,
}
#[derive(Args)]
pub(in crate::native) struct Payer<T: Args> {
    #[arg(long, value_parser = ordinary_principal)]
    pub cashier: Principal,
    #[arg(long = "payer", value_parser = ordinary_principal)]
    pub payment_account: Principal,
    #[command(flatten)]
    pub input: T,
}
#[derive(Args)]
pub(in crate::native) struct Empty {}
#[derive(Args)]
pub(in crate::native) struct Request {
    #[arg(long)]
    pub request: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Mutation {
    #[arg(long)]
    pub request: PathBuf,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Permission {
    #[arg(long)]
    pub permission: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Directory {
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Cursor {
    #[arg(long)]
    pub cursor: Option<PathBuf>,
}
#[derive(Args)]
pub(in crate::native) struct UploadHistory {
    #[arg(long, value_parser = history_filter)]
    pub filter: ic_blob_storage_contracts::dto::upload::history::UploadHistoryFilter,
    #[arg(long)]
    pub cursor: Option<PathBuf>,
}
#[derive(Args)]
pub(in crate::native) struct AccountKind {
    #[arg(long, value_parser = account_kind)]
    pub kind: ic_blob_storage_contracts::dto::account::AccountInspectionKind,
}
#[derive(Args)]
pub(in crate::native) struct Amount {
    #[arg(long, value_parser = positive(u128::MAX))]
    pub operation: u128,
    #[arg(long, value_parser = positive(u128::MAX))]
    pub offered: u128,
    #[arg(long, value_parser = positive(u128::MAX))]
    pub target_balance: Option<u128>,
}
#[derive(Args)]
pub(in crate::native) struct Cancel {
    #[arg(long)]
    pub run_dir: PathBuf,
    #[arg(long, value_parser = positive(u64::MAX))]
    pub sequence: u64,
}
#[derive(Args)]
pub(in crate::native) struct Revoke {
    #[arg(long)]
    pub run_dir: PathBuf,
    #[arg(long, value_parser = ordinary_principal)]
    pub gateway: Principal,
}
#[derive(Args)]
pub(in crate::native) struct Download {
    #[arg(long)]
    pub request: PathBuf,
    #[arg(long)]
    pub project: String,
    #[arg(long)]
    pub gateway: Url,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_FILE_BYTES).expect("positive ceiling")))]
    pub max_bytes: NonZeroU64,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct VerifyUpload {
    #[arg(long)]
    pub permission: PathBuf,
    #[arg(long)]
    pub body: PathBuf,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_FILE_BYTES).expect("positive ceiling")))]
    pub max_bytes: NonZeroU64,
}
#[derive(Args)]
pub(in crate::native) struct Files {
    #[arg(long)]
    pub inputs: PathBuf,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_FILE_BYTES).expect("positive ceiling")))]
    pub max_bytes: NonZeroU64,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_TOTAL_BYTES).expect("positive ceiling")))]
    pub max_total_bytes: NonZeroU64,
    #[arg(long, value_parser = positive(MAX_TIMEOUT_SECONDS))]
    pub timeout_seconds: u64,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Checks<const QUERIES: u64> {
    #[command(flatten)]
    pub files: Files,
    #[arg(long, value_parser = positive(QUERIES))]
    pub max_queries: u64,
}
#[derive(Args)]
pub(in crate::native) struct Map<const QUERIES: u64> {
    #[command(flatten)]
    pub checks: Checks<QUERIES>,
    #[arg(long)]
    pub gateway: Url,
    #[arg(long)]
    pub operator_identity: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct FileStatus {
    #[command(flatten)]
    pub map: Map<4>,
    #[arg(long, value_parser = file_index)]
    pub file_index: usize,
}
#[derive(Args)]
pub(in crate::native) struct Prepare {
    #[command(flatten)]
    pub files: Files,
    #[arg(long)]
    pub uploader_identity: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct IndexedPrepare {
    #[command(flatten)]
    pub prepare: Prepare,
    #[arg(long, value_parser = file_index)]
    pub file_index: usize,
}
#[derive(Args)]
pub(in crate::native) struct ResumePrepare {
    #[command(flatten)]
    pub indexed: IndexedPrepare,
    #[arg(long)]
    pub source_run: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Session {
    #[command(flatten)]
    pub prepare: Prepare,
    #[arg(long)]
    pub gateway: Url,
    #[arg(long)]
    pub operator_identity: PathBuf,
    #[arg(long, value_parser = positive(crate::native::publish_session::budget::step_limit(MAX_FILES).expect("bounded file ceiling")))]
    pub max_steps: u64,
    #[arg(long = "source-session")]
    pub source: Option<PathBuf>,
    #[arg(long)]
    pub browser_selection: Option<PathBuf>,
}
#[derive(Args)]
pub(in crate::native) struct AccountLink {
    #[arg(long, value_parser = principal)]
    pub cashier: Principal,
    #[arg(long, value_parser = principal)]
    pub caller: Principal,
    #[arg(long, value_parser = principal)]
    pub owner: Principal,
    #[arg(long, value_parser = principal)]
    pub payer: Principal,
    #[arg(long, value_parser = positive(NonZeroU128::MAX))]
    pub daily_limit: NonZeroU128,
    #[arg(long, value_parser = positive(NonZeroU64::MAX))]
    pub expiry: NonZeroU64,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct Installation {
    #[arg(long)]
    pub configuration: PathBuf,
    #[arg(long, value_parser = principal)]
    pub service: Principal,
    #[arg(long)]
    pub project: String,
    #[arg(long)]
    pub release: String,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct UploadInputs {
    #[arg(long)]
    pub installation: PathBuf,
    #[arg(long)]
    pub binding: PathBuf,
    #[arg(long)]
    pub manifest: PathBuf,
    #[arg(long)]
    pub body: PathBuf,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_FILE_BYTES).expect("positive ceiling")))]
    pub max_bytes: NonZeroU64,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct PublishInputs {
    #[arg(long)]
    pub inventory: PathBuf,
    #[arg(long)]
    pub root: PathBuf,
    #[arg(long)]
    pub installation: PathBuf,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_FILE_BYTES).expect("positive ceiling")))]
    pub max_bytes: NonZeroU64,
    #[arg(long, value_parser = positive(NonZeroU64::new(MAX_TOTAL_BYTES).expect("positive ceiling")))]
    pub max_total_bytes: NonZeroU64,
    #[arg(long)]
    pub run_dir: PathBuf,
}
#[derive(Args)]
pub(in crate::native) struct ReferenceInputs {
    #[arg(long)]
    pub permission: PathBuf,
    #[arg(long, value_parser = reference_action)]
    pub action: ic_blob_storage_contracts::dto::reference::ReferenceAction,
    #[arg(long, value_parser = positive(u128::MAX))]
    pub reference: u128,
    #[arg(long, value_parser = positive(u128::MAX))]
    pub operation: u128,
    #[arg(long)]
    pub run_dir: PathBuf,
}
fn account_kind(
    text: &str,
) -> Result<ic_blob_storage_contracts::dto::account::AccountInspectionKind, &'static str> {
    crate::native::account::kind(text).map_err(|_| "invalid account observation kind")
}
fn history_filter(
    text: &str,
) -> Result<ic_blob_storage_contracts::dto::upload::history::UploadHistoryFilter, &'static str> {
    crate::native::upload_history::filter(text).map_err(|_| "invalid upload filter")
}
fn reference_action(
    text: &str,
) -> Result<ic_blob_storage_contracts::dto::reference::ReferenceAction, &'static str> {
    use ic_blob_storage_contracts::dto::reference::ReferenceAction;
    match text {
        "retain" => Ok(ReferenceAction::Retain),
        "release" => Ok(ReferenceAction::Release),
        _ => Err("invalid reference action"),
    }
}
#[cfg(test)]
mod tests;
