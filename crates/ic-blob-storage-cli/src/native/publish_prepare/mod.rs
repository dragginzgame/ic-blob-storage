//! One indexed setup at a time, with original signed-operation claims surviving loss.
pub(super) mod batch;
mod journal;
mod progress;
#[cfg(test)]
mod tests;
use super::{
    Failure,
    arguments::{Command, Options},
    artifacts::Run,
    publish_check, publish_inputs, upload_setup,
};
use candid::Principal;
use serde_json::{Value, json};
use std::{num::NonZeroU64, path::PathBuf, time::Duration};

#[derive(Clone)]
pub(super) struct Input {
    pub service: Principal,
    pub namespace: u128,
    pub inputs: PathBuf,
    pub directory: PathBuf,
    pub source: Option<PathBuf>,
    pub uploader_identity: PathBuf,
    pub index: usize,
    pub max_bytes: NonZeroU64,
    pub max_total_bytes: NonZeroU64,
    pub timeout_seconds: u64,
}

/// Each role authenticates independently, even when the principals happen to coincide.
fn setup_options(
    options: &Options,
    setup: upload_setup::Input,
    identity: PathBuf,
    actor: Principal,
) -> Options {
    Options {
        command: Command::UploadSetup(setup),
        network: options.network,
        url: options.url.clone(),
        identity,
        actor,
        root_key: options.root_key.clone(),
    }
}

pub(super) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let batch = publish_inputs::PreparedBatch::open_frozen(
        &input.inputs,
        input.max_bytes,
        input.max_total_bytes,
    )?;
    run_selected(options, input, &batch).await
}

pub(in crate::native) async fn run_selected(
    options: &Options,
    input: &Input,
    batch: &publish_inputs::PreparedBatch,
) -> Result<Value, Failure> {
    let selected = batch.inputs.get(input.index).ok_or(Failure::Arguments)?;
    let scope = publish_check::observation::scope(
        &batch.inputs,
        input.service,
        input.namespace,
        options.actor,
    )?;
    let origin = input.source.as_ref().unwrap_or(&input.directory);
    let packet = |kind, name: &str| upload_setup::Input {
        kind,
        service: input.service,
        namespace: input.namespace,
        request: origin.join(name),
        directory: None,
    };
    let uploader = setup_options(
        options,
        packet(upload_setup::Kind::Prepare, "manifest.candid"),
        input.uploader_identity.clone(),
        selected.permission.uploader,
    );
    let agent = super::agent(options)?;
    super::agent(&uploader)?;
    let binding = journal::binding(options, input, batch, &agent.read_root_key());
    if input.source.is_some() {
        journal::validate(origin, &binding, selected)?;
        journal::validate_attempt(origin, upload_setup::Kind::Admit, options, selected)?;
    }
    let run = Run::create(&input.directory)?;
    run.json("intent.json", &binding)?;
    if input.source.is_some() {
        run.json(
            "recovery.json",
            &json!({"source_run":origin,
            "max_service_updates_this_run":1,"max_service_queries":4,
            "timeout_seconds":input.timeout_seconds,"redispatch_authorized":false}),
        )?;
    }
    let (permission, manifest) = selected.setup_requests();
    run.bytes("permission.candid", permission)?;
    run.bytes("manifest.candid", manifest)?;
    let context = progress::Context {
        options,
        uploader: &uploader,
        input,
        selected,
        origin,
        run: &run,
    };
    let result = tokio::time::timeout(Duration::from_secs(input.timeout_seconds), async {
        let report = publish_check::observation::inspect(
            scope,
            std::slice::from_ref(selected),
            &run,
            |method, args| {
                let agent = &agent;
                async move {
                    tokio::time::timeout(
                        Duration::from_secs(30),
                        agent.query(&scope.service, method).with_arg(args).call(),
                    )
                    .await
                    .map_err(|_| Failure::Timeout)?
                    .map_err(|_| Failure::Transport)
                }
            },
        )
        .await?;
        run.json("preflight.json", &report)?;
        context.advance(report).await
    })
    .await
    .map_err(|_| Failure::Timeout)
    .and_then(|r| r);
    match result {
        Ok(report) => {
            run.json("summary.json", &report)?;
            Ok(report)
        }
        Err(error) => {
            run.json(
                "failure.json",
                &json!({"error":error.code(),"complete":false,
                "redispatch_authorized":false,"original_journal_must_survive":true}),
            )?;
            Err(error)
        }
    }
}
