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

impl schema::Connection {
    // The five CLI commands share request admission/execution, but each selects
    // its own operation and supplies a journal directory only for mutations.
    fn upload_setup(
        self,
        actor: Principal,
        kind: super::upload_setup::Kind,
        request: PathBuf,
        directory: Option<PathBuf>,
    ) -> Result<serde_json::Value, Failure> {
        let input = super::upload_setup::Input {
            kind,
            service: self.service,
            namespace: self.namespace,
            request,
            directory,
        };
        self.run(actor, &input, super::upload_setup::run)
    }

    fn run<T>(
        self,
        actor: Principal,
        input: &T,
        run: impl AsyncFnOnce(&Options, &T) -> Result<serde_json::Value, Failure>,
    ) -> Result<serde_json::Value, Failure> {
        let network = self.network.as_str();
        validate_url(&self.url, network, self.root_key.is_some())?;
        let options = Options {
            network,
            url: self.url,
            root_key: self.root_key,
            actor,
            identity: self.identity,
        };
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|_| Failure::Transport)?;
        runtime.block_on(run(&options, input))
    }
}
impl<T: clap::Args> schema::Payer<T> {
    fn scope(&self, connection: &schema::Connection) -> OperatorScope {
        OperatorScope {
            service: connection.service,
            namespace: connection.namespace,
            cashier: self.cashier,
            payment_account: self.payment_account,
        }
    }
}
impl Invocation {
    #[expect(
        clippy::too_many_lines,
        reason = "One exhaustive invocation match validates scope and dispatches directly without a second command catalog"
    )]
    pub(super) fn execute(self) -> Result<serde_json::Value, Failure> {
        use schema::Invocation as I;
        match self {
            I::PublishInputs(input) => super::publish_inputs::run(input),
            I::InstallationCheck(input) => super::installation_check::run(input),
            I::AccountLinkInputs(input) => super::account_link_inputs::run(input),
            I::ReferenceInputs(input) => super::reference_inputs::run(input),
            I::UploadInputs(input) => super::upload_inputs::run(input),
            I::AdmitUpload(args) => args.connection.upload_setup(
                args.actor,
                super::upload_setup::Kind::Admit,
                args.input.request,
                Some(args.input.run_dir),
            ),
            I::PrepareUpload(args) => args.connection.upload_setup(
                args.actor,
                super::upload_setup::Kind::Prepare,
                args.input.request,
                Some(args.input.run_dir),
            ),
            I::RevokeUpload(args) => args.connection.upload_setup(
                args.actor,
                super::upload_setup::Kind::Revoke,
                args.input.request,
                Some(args.input.run_dir),
            ),
            I::UploadPermission(args) => args.connection.upload_setup(
                args.actor,
                super::upload_setup::Kind::Permission,
                args.input.request,
                None,
            ),
            I::UploadManifest(args) => args.connection.upload_setup(
                args.actor,
                super::upload_setup::Kind::Manifest,
                args.input.request,
                None,
            ),
            I::ReferenceReceipt(args) => {
                let input = args.input;
                let input = super::references::Input {
                    service: args.connection.service,
                    namespace: args.connection.namespace,
                    kind: super::references::Kind::Receipt,
                    request: input.request,
                };
                args.connection
                    .run(args.actor, &input, super::references::run)
            }
            I::ReferenceStatus(args) => {
                let input = args.input;
                let input = super::references::Input {
                    service: args.connection.service,
                    namespace: args.connection.namespace,
                    kind: super::references::Kind::Status,
                    request: input.request,
                };
                args.connection
                    .run(args.actor, &input, super::references::run)
            }
            I::SubmitReference(args) => {
                let input = args.input;
                let input = super::submit_reference::Input {
                    service: args.connection.service,
                    namespace: args.connection.namespace,
                    request: input.request,
                    directory: input.run_dir,
                };
                args.connection
                    .run(args.actor, &input, super::submit_reference::run)
            }
            I::Download(args) => {
                let input = args.input;
                validate_url(
                    &input.gateway,
                    args.connection.network.as_str(),
                    args.connection.network.as_str() == "local",
                )?;
                let input = super::download::Input {
                    service: args.connection.service,
                    namespace: args.connection.namespace,
                    request: input.request,
                    project: input.project,
                    gateway: input.gateway,
                    directory: input.run_dir,
                    max_bytes: input.max_bytes,
                };
                args.connection
                    .run(args.actor, &input, super::download::run)
            }
            I::VerifyUpload(args) => {
                let input = args.input;
                let input = (args.connection.service, args.connection.namespace, input);
                args.connection
                    .run(args.actor, &input, async |options, input| {
                        super::observe_upload(options, input.0, input.1, &input.2).await
                    })
            }
            I::CertificateAssessment(args) => {
                let input = args.input;
                let input = super::certificate_assessment::Input {
                    service: args.connection.service,
                    namespace: args.connection.namespace,
                    permission: input.permission,
                };
                args.connection
                    .run(args.actor, &input, super::certificate_assessment::run)
            }
            I::UploadHistory(args) => {
                let input = args.input;
                let input = super::upload_history::Input {
                    service: args.connection.service,
                    namespace: args.connection.namespace,
                    filter: input.filter,
                    cursor: input.cursor,
                };
                args.connection
                    .run(args.authority, &input, super::upload_history::run)
            }
            I::Status(args) => {
                let input = args.input.scope(&args.connection);
                args.connection
                    .run(args.authority, &input, async |options, input| {
                        super::observe_status(options, *input).await
                    })
            }
            I::FundingHistory(args) => {
                let scope = args.input.scope(&args.connection);
                let input = args.input.input;
                let input = (scope, input.cursor);
                args.connection
                    .run(args.authority, &input, async |options, input| {
                        super::observe_funding_history(options, input.0, input.1.as_deref()).await
                    })
            }
            I::InspectAccount(args) => {
                let scope = args.input.scope(&args.connection);
                let input = args.input.input;
                let input = ic_blob_storage_contracts::dto::account::AccountInspectionRequest {
                    scope,
                    kind: input.kind,
                };
                args.connection
                    .run(args.authority, &input, async |options, input| {
                        super::account::run(options, *input).await
                    })
            }
            I::FundingAssessment(args) => {
                let scope = args.input.scope(&args.connection);
                let input = args.input.input;
                let input = ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationRequest {
                        scope,
                        operation: input.operation,
                        offered: input.offered,
                        target_balance: input.target_balance,
                    };
                args.connection
                    .run(args.authority, &input, async |options, input| {
                        super::funding_assessment::run(options, *input).await
                    })
            }
            I::FundingOutcome(args) => {
                let scope = args.input.scope(&args.connection);
                let input = args.input.input;
                let input =
                    ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest {
                        scope,
                        operation: input.operation,
                        offered: input.offered,
                        target_balance: input.target_balance,
                    };
                args.connection
                    .run(args.authority, &input, async |options, input| {
                        super::funding_outcome::run(options, *input).await
                    })
            }
            I::SyncGateways(args) => {
                let scope = args.input.scope(&args.connection);
                let input = args.input.input;
                let input = super::gateway_controls::Input {
                    scope,
                    action: super::gateway_controls::Action::Sync,
                    directory: input.run_dir,
                };
                args.connection
                    .run(args.authority, &input, super::gateway_controls::run)
            }
            I::CancelGatewaySync(args) => {
                let scope = args.input.scope(&args.connection);
                let input = args.input.input;
                let input = super::gateway_controls::Input {
                    scope,
                    action: super::gateway_controls::Action::Cancel(input.sequence),
                    directory: input.run_dir,
                };
                args.connection
                    .run(args.authority, &input, super::gateway_controls::run)
            }
            I::RevokeGateway(args) => {
                let scope = args.input.scope(&args.connection);
                let input = args.input.input;
                let input = super::gateway_controls::Input {
                    scope,
                    action: super::gateway_controls::Action::Revoke(input.gateway),
                    directory: input.run_dir,
                };
                args.connection
                    .run(args.authority, &input, super::gateway_controls::run)
            }
            I::PublishCheck(args) => {
                let input = args.input;
                let input = check_input(
                    args.connection.service,
                    args.connection.namespace,
                    input.files,
                    input.max_queries,
                );
                args.connection
                    .run(args.actor, &input, super::publish_check::run)
            }
            I::PublishMap(args) => {
                let input = args.input;
                let input = map_input(
                    args.connection.service,
                    args.connection.namespace,
                    args.connection.network.as_str(),
                    input,
                    super::publish_map::Selection::Batch,
                )?;
                args.connection
                    .run(args.actor, &input, super::publish_map::run)
            }
            I::PublishFileStatus(args) => {
                let input = args.input;
                let input = map_input(
                    args.connection.service,
                    args.connection.namespace,
                    args.connection.network.as_str(),
                    input.map,
                    super::publish_map::Selection::File(input.file_index),
                )?;
                args.connection
                    .run(args.actor, &input, super::publish_map::run)
            }
            I::PublishPrepare(args) => {
                let input = args.input;
                let input = prepare_input(
                    args.connection.service,
                    args.connection.namespace,
                    input.file_index,
                    input.prepare,
                    None,
                );
                args.connection
                    .run(args.actor, &input, super::publish_prepare::run)
            }
            I::PublishPrepareResume(args) => {
                let input = args.input;
                let input = prepare_input(
                    args.connection.service,
                    args.connection.namespace,
                    input.indexed.file_index,
                    input.indexed.prepare,
                    Some(input.source_run),
                );
                args.connection
                    .run(args.actor, &input, super::publish_prepare::run)
            }
            I::PublishPrepareBatch(args) => {
                let input = args.input;
                let input = prepare_input(
                    args.connection.service,
                    args.connection.namespace,
                    0,
                    input,
                    None,
                );
                args.connection
                    .run(args.actor, &input, super::publish_prepare::batch::run)
            }
            I::PublishSession(args) => {
                let input = args.input;
                validate_url(
                    &input.gateway,
                    args.connection.network.as_str(),
                    args.connection.network.as_str() == "local",
                )?;
                let input = super::publish_session::Input {
                    host_release: host_release()?,
                    source_session: input.source,
                    browser_selection: input.browser_selection,
                    operator_identity: input.operator_identity,
                    gateway: input.gateway,
                    max_steps: input.max_steps,
                    prepare: prepare_input(
                        args.connection.service,
                        args.connection.namespace,
                        0,
                        input.prepare,
                        None,
                    ),
                };
                args.connection
                    .run(args.actor, &input, super::publish_session::run)
            }
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
) -> Result<super::publish_map::Input, Failure> {
    validate_url(&input.gateway, network, network == "local")?;
    Ok(super::publish_map::Input {
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
    })
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
