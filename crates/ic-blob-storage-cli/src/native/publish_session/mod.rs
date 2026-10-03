//! One verified batch and existing phase owners; no certificate/provider dispatcher.
mod control;
mod progress;
mod verification;
use crate::native::{
    Failure, agent, agent_for, arguments::Options, artifacts::Run, candidate_candid, publish_check,
    publish_inputs::PreparedBatch, publish_map, publish_prepare, upload_inputs::digest,
};
use ic_agent::Agent;
use ic_blob_storage::dto::configuration::ServiceInstallationInput;
use serde::Serialize;
use serde_json::{Value, json};
use std::{path::PathBuf, time::Duration};
use url::Url;

pub(super) struct Input {
    pub prepare: publish_prepare::Input,
    pub operator_identity: PathBuf,
    pub verifier_identity: Option<PathBuf>,
    pub gateway: Url,
    pub max_steps: u64,
}

#[derive(Serialize)]
struct SessionIntentRecord<'a> {
    schema: u8,
    operation: &'static str,
    network: &'static str,
    service_url: &'a str,
    service: String,
    namespace: String,
    tenant: String,
    uploader: String,
    operator: String,
    verifier: Option<String>,
    gateway: &'a str,
    inventory_sha256: String,
    installation_sha256: String,
    root_key_sha256: String,
    files: usize,
    max_steps: u64,
    max_service_updates: usize,
    max_service_queries: u64,
    timeout_seconds: u64,
    max_provider_requests: usize,
    automatic_retries: u8,
    input_verification: &'static str,
}

struct Actors {
    tenant: Agent,
    operator: Agent,
    verifier: Option<Options>,
}
impl Actors {
    async fn query(
        &self,
        service: candid::Principal,
        method: &'static str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>, Failure> {
        let agent = if method == "blob_configuration" {
            &self.operator
        } else {
            &self.tenant
        };
        tokio::time::timeout(
            Duration::from_secs(30),
            agent.query(&service, method).with_arg(args).call(),
        )
        .await
        .map_err(|_| Failure::Timeout)?
        .map_err(|_| Failure::Transport)
    }
}

pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let batch = PreparedBatch::open_frozen(
        &input.prepare.inputs,
        input.prepare.max_bytes,
        input.prepare.max_total_bytes,
    )?;
    let scope = publish_check::observation::scope(
        &batch.files,
        input.prepare.service,
        input.prepare.namespace,
        options.actor,
    )?;
    if input.max_steps > 8 * batch.files.len() as u64 + 1 {
        return Err(Failure::Arguments);
    }
    let installation: ServiceInstallationInput = candidate_candid::decode(&batch.installation)?;
    let actors = Actors {
        tenant: agent(options)?,
        operator: agent_for(
            options,
            &input.operator_identity,
            installation.configuration.operator,
        )?,
        verifier: verification::options(options, input, &installation)?,
    };
    agent_for(
        options,
        &input.prepare.uploader_identity,
        installation.trusted_uploader,
    )?;
    let run = Run::create(&input.prepare.directory)?;
    run.bytes("inventory.json", &batch.inventory)?;
    run.bytes("installation.candid", &batch.installation)?;
    run.json(
        "intent.json",
        &SessionIntentRecord {
            schema: 1,
            operation: "publish_session",
            network: options.network,
            service_url: options.url.as_str(),
            service: scope.service.to_text(),
            namespace: scope.namespace.to_string(),
            tenant: scope.tenant.to_text(),
            uploader: installation.trusted_uploader.to_text(),
            operator: installation.configuration.operator.to_text(),
            verifier: actors
                .verifier
                .as_ref()
                .map(|options| options.actor.to_text()),
            gateway: input.gateway.as_str(),
            inventory_sha256: digest(&batch.inventory),
            installation_sha256: digest(&batch.installation),
            root_key_sha256: digest(&actors.tenant.read_root_key()),
            files: batch.files.len(),
            max_steps: input.max_steps,
            max_service_updates: (2 + usize::from(actors.verifier.is_some())) * batch.files.len(),
            max_service_queries: 5 * input.max_steps + 2 * batch.files.len() as u64 + 3,
            timeout_seconds: input.prepare.timeout_seconds,
            max_provider_requests: if actors.verifier.is_some() {
                batch.files.len()
            } else {
                0
            },
            automatic_retries: 0,
            input_verification: "one_complete_startup_pass_and_selected_body_before_setup",
        },
    )?;
    let result = tokio::time::timeout(Duration::from_secs(input.prepare.timeout_seconds), async {
        let preflight = Run::create(&input.prepare.directory.join("configuration"))?;
        let host = publish_map::configuration(&batch, &preflight, &mut |method,args| actors.query(scope.service,method,args)).await?;
        if host.fenced { return Err(Failure::Denied); }
        let ready = json!({"schema":1,"operation":"publish_session","event":"ready",
            "files":batch.files.len(),"inventory_sha256":digest(&batch.inventory),"installation_sha256":digest(&batch.installation),
            "max_steps":input.max_steps,"provider_requests":0,"input_verification":"session_start",
            "retry_authorized":false,"publication_lease":false});
        run.json("ready.json",&ready)?;
        control::output(&ready)?;
        progress::Session::new(options,input,&batch,&actors,&run).drive(control::input()?).await
    }).await.map_err(|_| Failure::Timeout).and_then(|r|r);
    match result {
        Ok(report) => {
            run.json("summary.json", &report)?;
            Ok(report)
        }
        Err(error) => {
            run.json(
                "failure.json",
                &json!({"error":error.code(),"complete":false,
                "original_journals_must_survive":true,"redispatch_authorized":false}),
            )?;
            Err(error)
        }
    }
}
