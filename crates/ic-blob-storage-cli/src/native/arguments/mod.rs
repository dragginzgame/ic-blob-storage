pub(super) mod schema;
use super::Failure;
use clap::Parser;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_platform::Principal;
use schema::{Cli, Invocation};
use std::path::PathBuf;
use url::{Host, Url};

// The caller selects the deployed Engine independently of Client/Contracts.
// Capture once before file/Agent setup; fresh and retained replies use this pin.
fn host_release() -> Result<String, Failure> {
    std::env::var("BLOB_EXPECTED_HOST_RELEASE")
        .ok()
        .filter(|release| !release.is_empty())
        .ok_or(Failure::Arguments)
}

pub(super) struct Options {
    pub network: &'static str,
    pub url: Url,
    pub identity: PathBuf,
    pub actor: Principal,
    pub root_key: Option<PathBuf>,
}

pub(super) enum Command {
    PublishSession(super::publish_session::Input),
    PublishMap(super::publish_map::Input),
    PublishPrepareBatch(super::publish_prepare::Input),
    PublishPrepare(super::publish_prepare::Input),
    PublishCheck(super::publish_check::Input),
    UploadSetup(super::upload_setup::Input),
    Download(super::download::Input),
    GatewayControl(super::gateway_controls::Input),
    FundingAssessment(
        ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationRequest,
    ),
    InspectAccount(ic_blob_storage_contracts::dto::account::AccountInspectionRequest),
    Reference(super::references::Input),
    FundingOutcome(ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest),
    UploadHistory(super::upload_history::Input),
    CertificateAssessment(super::certificate_assessment::Input),
    SubmitReference(super::submit_reference::Input),
    Status {
        scope: OperatorScope,
    },
    FundingHistory {
        scope: OperatorScope,
        cursor: Option<PathBuf>,
    },
    VerifyUpload {
        service: Principal,
        namespace: u128,
        permission: PathBuf,
        body: PathBuf,
        max_bytes: std::num::NonZeroU64,
    },
}

impl Options {
    #[cfg(test)]
    pub fn parse(args: &[String]) -> Result<(Command, Self), Failure> {
        parse(args)?
            .command
            .ok_or(Failure::Arguments)?
            .into_online()
    }
}
impl schema::Connection {
    fn finish(self, actor: Principal, command: Command) -> Result<(Command, Options), Failure> {
        let network = self.network.as_str();
        validate_url(&self.url, network, self.root_key.is_some())?;
        Ok((
            command,
            Options {
                network,
                url: self.url,
                root_key: self.root_key,
                actor,
                identity: self.identity,
            },
        ))
    }
}
impl<T: clap::Args> schema::Tenant<T> {
    fn convert(
        self,
        build: impl FnOnce(Principal, u128, &'static str, T) -> Result<Command, Failure>,
    ) -> Result<(Command, Options), Failure> {
        let command = build(
            self.connection.service,
            self.connection.namespace,
            self.connection.network.as_str(),
            self.input,
        )?;
        self.connection.finish(self.actor, command)
    }
}
impl<T: clap::Args> schema::Operator<T> {
    fn convert(
        self,
        build: impl FnOnce(Principal, u128, T) -> Command,
    ) -> Result<(Command, Options), Failure> {
        let command = build(
            self.connection.service,
            self.connection.namespace,
            self.input,
        );
        self.connection.finish(self.authority, command)
    }
}
impl<T: clap::Args> schema::Operator<schema::Payer<T>> {
    fn paid(
        self,
        build: impl FnOnce(OperatorScope, T) -> Command,
    ) -> Result<(Command, Options), Failure> {
        self.convert(|service, namespace, input| {
            build(
                OperatorScope {
                    service,
                    namespace,
                    cashier: input.cashier,
                    payment_account: input.payment_account,
                },
                input.input,
            )
        })
    }
}
impl Invocation {
    #[expect(
        clippy::too_many_lines,
        reason = "One exhaustive typed command projection keeps the grammar-to-workflow mapping auditable; trust policy remains in the shared scoped conversions"
    )]
    pub fn into_online(self) -> Result<(Command, Options), Failure> {
        use schema::Invocation as I;
        match self {
            I::AdmitUpload(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::UploadSetup(super::upload_setup::Input {
                    kind: super::upload_setup::Kind::Admit,
                    service,
                    namespace,
                    request: input.request,
                    directory: Some(input.run_dir),
                }))
            }),
            I::PrepareUpload(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::UploadSetup(super::upload_setup::Input {
                    kind: super::upload_setup::Kind::Prepare,
                    service,
                    namespace,
                    request: input.request,
                    directory: Some(input.run_dir),
                }))
            }),
            I::RevokeUpload(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::UploadSetup(super::upload_setup::Input {
                    kind: super::upload_setup::Kind::Revoke,
                    service,
                    namespace,
                    request: input.request,
                    directory: Some(input.run_dir),
                }))
            }),
            I::UploadPermission(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::UploadSetup(super::upload_setup::Input {
                    kind: super::upload_setup::Kind::Permission,
                    service,
                    namespace,
                    request: input.request,
                    directory: None,
                }))
            }),
            I::UploadManifest(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::UploadSetup(super::upload_setup::Input {
                    kind: super::upload_setup::Kind::Manifest,
                    service,
                    namespace,
                    request: input.request,
                    directory: None,
                }))
            }),
            I::ReferenceReceipt(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::Reference(super::references::Input {
                    service,
                    namespace,
                    kind: super::references::Kind::Receipt,
                    request: input.request,
                }))
            }),
            I::ReferenceStatus(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::Reference(super::references::Input {
                    service,
                    namespace,
                    kind: super::references::Kind::Status,
                    request: input.request,
                }))
            }),
            I::SubmitReference(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::SubmitReference(super::submit_reference::Input {
                    service,
                    namespace,
                    request: input.request,
                    directory: input.run_dir,
                }))
            }),
            I::Download(args) => args.convert(|service, namespace, network, input| {
                validate_url(&input.gateway, network, network == "local")?;
                Ok(Command::Download(super::download::Input {
                    service,
                    namespace,
                    request: input.request,
                    project: input.project,
                    gateway: input.gateway,
                    directory: input.run_dir,
                    max_bytes: input.max_bytes,
                }))
            }),
            I::VerifyUpload(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::VerifyUpload {
                    service,
                    namespace,
                    permission: input.permission,
                    body: input.body,
                    max_bytes: input.max_bytes,
                })
            }),
            I::CertificateAssessment(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::CertificateAssessment(super::certificate_assessment::Input {
                    service,
                    namespace,
                    permission: input.permission,
                }))
            }),
            I::UploadHistory(args) => args.convert(|service, namespace, input| {
                Command::UploadHistory(super::upload_history::Input {
                    service,
                    namespace,
                    filter: input.filter,
                    cursor: input.cursor,
                })
            }),
            I::Status(args) => args.paid(|scope, _| Command::Status { scope }),
            I::FundingHistory(args) => args.paid(|scope, input| {
                Command::FundingHistory { scope, cursor: input.cursor }
            }),
            I::InspectAccount(args) => args.paid(|scope, input| {
                Command::InspectAccount(ic_blob_storage_contracts::dto::account::AccountInspectionRequest {
                    scope,
                    kind: input.kind,
                })
            }),
            I::FundingAssessment(args) => args.paid(|scope, input| {
                Command::FundingAssessment(ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationRequest {
                    scope,
                    operation: input.operation,
                    offered: input.offered,
                    target_balance: input.target_balance,
                })
            }),
            I::FundingOutcome(args) => args.paid(|scope, input| {
                Command::FundingOutcome(ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest {
                    scope,
                    operation: input.operation,
                    offered: input.offered,
                    target_balance: input.target_balance,
                })
            }),
            I::SyncGateways(args) => args.paid(|scope, input| {
                Command::GatewayControl(super::gateway_controls::Input {
                    scope,
                    action: super::gateway_controls::Action::Sync,
                    directory: input.run_dir,
                })
            }),
            I::CancelGatewaySync(args) => args.paid(|scope, input| {
                Command::GatewayControl(super::gateway_controls::Input {
                    scope,
                    action: super::gateway_controls::Action::Cancel(input.sequence),
                    directory: input.run_dir,
                })
            }),
            I::RevokeGateway(args) => args.paid(|scope, input| {
                Command::GatewayControl(super::gateway_controls::Input {
                    scope,
                    action: super::gateway_controls::Action::Revoke(input.gateway),
                    directory: input.run_dir,
                })
            }),
            I::PublishCheck(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::PublishCheck(check_input(
                    service, namespace, input.files, input.max_queries,
                )))
            }),
            I::PublishMap(args) => args.convert(|service, namespace, network, input| {
                map_input(service, namespace, network, input, super::publish_map::Selection::Batch)
            }),
            I::PublishFileStatus(args) => args.convert(|service, namespace, network, input| {
                map_input(service, namespace, network, input.map, super::publish_map::Selection::File(input.file_index))
            }),
            I::PublishPrepare(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::PublishPrepare(prepare_input(
                    service, namespace, input.file_index, input.prepare, None,
                )))
            }),
            I::PublishPrepareResume(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::PublishPrepare(prepare_input(
                    service, namespace, input.indexed.file_index, input.indexed.prepare, Some(input.source_run),
                )))
            }),
            I::PublishPrepareBatch(args) => args.convert(|service, namespace, _, input| {
                Ok(Command::PublishPrepareBatch(prepare_input(
                    service, namespace, 0, input, None,
                )))
            }),
            I::PublishSession(args) => args.convert(|service, namespace, network, input| {
                validate_url(&input.gateway, network, network == "local")?;
                Ok(Command::PublishSession(super::publish_session::Input {
                    host_release: host_release()?,
                    source_session: input.source,
                    browser_selection: input.browser_selection,
                    operator_identity: input.operator_identity,
                    gateway: input.gateway,
                    max_steps: input.max_steps,
                    prepare: prepare_input(service, namespace, 0, input.prepare, None),
                }))
            }),
            I::AccountLinkInputs(_)
            | I::InstallationCheck(_)
            | I::UploadInputs(_)
            | I::PublishInputs(_)
            | I::ReferenceInputs(_) => Err(Failure::Arguments),
        }
    }
}
fn check_input(
    service: Principal,
    namespace: u128,
    files: schema::Files,
    max_queries: u64,
) -> super::publish_check::Input {
    super::publish_check::Input {
        service,
        namespace,
        inputs: files.inputs,
        directory: files.run_dir,
        max_bytes: files.max_bytes,
        max_total_bytes: files.max_total_bytes,
        max_queries,
        timeout_seconds: files.timeout_seconds,
    }
}
fn prepare_input(
    service: Principal,
    namespace: u128,
    index: usize,
    input: schema::Prepare,
    source: Option<PathBuf>,
) -> super::publish_prepare::Input {
    super::publish_prepare::Input {
        service,
        namespace,
        index,
        inputs: input.files.inputs,
        directory: input.files.run_dir,
        max_bytes: input.files.max_bytes,
        max_total_bytes: input.files.max_total_bytes,
        timeout_seconds: input.files.timeout_seconds,
        uploader_identity: input.uploader_identity,
        source,
    }
}
fn map_input<const QUERIES: u64>(
    service: Principal,
    namespace: u128,
    network: &str,
    input: schema::Map<QUERIES>,
    selection: super::publish_map::Selection,
) -> Result<Command, Failure> {
    validate_url(&input.gateway, network, network == "local")?;
    Ok(Command::PublishMap(super::publish_map::Input {
        host_release: host_release()?,
        selection,
        operator_identity: input.operator_identity,
        gateway: input.gateway,
        checks: check_input(
            service,
            namespace,
            input.checks.files,
            input.checks.max_queries,
        ),
    }))
}
pub(super) fn validate_url(url: &Url, network: &str, explicit_root: bool) -> Result<(), Failure> {
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path() != "/"
        || url.port() == Some(0)
    {
        return Err(Failure::Arguments);
    }
    let loopback = match url.host() {
        Some(Host::Ipv4(ip)) => ip.is_loopback(),
        Some(Host::Ipv6(ip)) => ip.is_loopback(),
        _ => false,
    };
    let valid_network = match network {
        "local" => loopback && matches!(url.scheme(), "http" | "https") && explicit_root,
        _ => url.scheme() == "https" && url.host().is_some() && !explicit_root,
    };
    if !valid_network {
        return Err(Failure::Arguments);
    }
    Ok(())
}
/// Parse the same typed grammar before any file or network effect.
#[cfg(test)]
pub(super) fn parse(args: &[String]) -> Result<Cli, Failure> {
    Cli::try_parse_from(std::iter::once("blob-storage").chain(args.iter().map(String::as_str)))
        .map_err(|_| Failure::Arguments)
}
/// Clap owns help and syntax errors; refusal never echoes sensitive arguments.
pub(super) fn invocation(args: &[std::ffi::OsString]) -> Result<Option<Cli>, Failure> {
    match Cli::try_parse_from(
        std::iter::once(std::ffi::OsStr::new("blob-storage"))
            .chain(args.iter().map(std::ffi::OsString::as_os_str)),
    ) {
        Ok(cli) => Ok(Some(cli)),
        Err(error) if error.kind() == clap::error::ErrorKind::DisplayHelp => {
            error.print().map_err(|_| Failure::File)?;
            Ok(None)
        }
        Err(_) => Err(Failure::Arguments),
    }
}
