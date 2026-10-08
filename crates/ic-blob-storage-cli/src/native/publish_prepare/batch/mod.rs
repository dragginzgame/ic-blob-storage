//! Validate one frozen batch once, retaining independent, non-repeatable file claims.
use super::{Failure, Input, Options, Run, Value, json, publish_inputs, run_selected};
use crate::native::{agent, publish_check, upload_inputs::digest};
use ic_blob_storage_contracts::identity::ContentDigest;
use std::time::Duration;

pub(in crate::native) async fn run(options: &Options, input: &Input) -> Result<Value, Failure> {
    let batch = publish_inputs::PreparedBatch::open_frozen(
        &input.inputs,
        input.max_bytes,
        input.max_total_bytes,
    )?;
    publish_check::observation::scope(&batch.files, input.service, input.namespace, options.actor)?;
    let tenant = agent(options)?;
    let selected = &batch.files.first().ok_or(Failure::Arguments)?.input;
    let uploader = super::setup_options(
        options,
        super::upload_setup::Input {
            kind: super::upload_setup::Kind::Prepare,
            service: input.service,
            namespace: input.namespace,
            request: input.inputs.clone(),
            directory: None,
        },
        input.uploader_identity.clone(),
        selected.permission.uploader,
    );
    agent(&uploader)?;
    let run = Run::create(&input.directory)?;
    run.bytes("inventory.json", &batch.inventory)?;
    run.bytes("installation.candid", &batch.installation)?;
    run.json("intent.json", &json!({"schema":1,"operation":"publish_prepare_batch",
        "network":options.network,"url":options.url.as_str(),"tenant":options.actor.to_text(),
        "uploader":selected.permission.uploader.to_text(),"service":input.service.to_text(),
        "namespace":input.namespace.to_string(),"inventory_sha256":digest(&batch.inventory),
        "installation_sha256":digest(&batch.installation),"root_key_sha256":ContentDigest::compute(&tenant.read_root_key()).to_string(),
        "files":batch.files.len(),"max_service_updates":2*batch.files.len(),
        "max_service_queries":4*batch.files.len(),"timeout_seconds":input.timeout_seconds,
        "provider_requests":0,"automatic_retries":0,"publication_authorized":false}))?;
    let result = tokio::time::timeout(Duration::from_secs(input.timeout_seconds), async {
        let mut results = Vec::with_capacity(batch.files.len());
        let mut all_prepared = true;
        for index in 0..batch.files.len() {
            let file = Input {
                index, directory: input.directory.join(format!("file-{index:04}")),
                ..input.clone()
            };
            let outcome = run_selected(options, &file, &batch).await?;
            let prepared = outcome.prepared();
            all_prepared &= prepared;
            results.push(outcome.report);
            if !prepared { break; }
        }
        Ok::<_, Failure>(json!({"schema":1,"operation":"publish_prepare_batch",
            "all_files_prepared":results.len()==batch.files.len() && all_prepared,
            "files":results,"provider_requests":0,"retry_authorized":false,"publication_authorized":false}))
    }).await.map_err(|_| Failure::Timeout).and_then(|r| r);
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
