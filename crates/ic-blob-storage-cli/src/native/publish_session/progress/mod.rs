//! Sequential receipt gating; in-process cursor/history never grants replay authority.
use super::{
    Actors, Failure, Input, Options, PreparedBatch, Run, Value,
    control::{self, Frame},
    json, publish_map, publish_prepare, verification,
};
use std::path::PathBuf;
use tokio::sync::mpsc;

pub(super) struct Session<'a> {
    options: &'a Options,
    input: &'a Input,
    batch: &'a PreparedBatch,
    actors: &'a Actors,
    run: &'a Run,
    next: usize,
    origins: Vec<Option<PathBuf>>,
}
impl<'a> Session<'a> {
    pub fn new(
        options: &'a Options,
        input: &'a Input,
        batch: &'a PreparedBatch,
        actors: &'a Actors,
        run: &'a Run,
    ) -> Self {
        Self {
            options,
            input,
            batch,
            actors,
            run,
            next: 0,
            origins: vec![None; batch.files.len()],
        }
    }
    async fn observation(
        &self,
        selection: publish_map::Selection,
        run: &Run,
    ) -> Result<publish_map::PublicationObservation, Failure> {
        publish_map::inspect(
            self.batch,
            self.options.actor,
            &self.input.gateway,
            selection,
            run,
            |method, args| self.actors.query(self.input.prepare.service, method, args),
        )
        .await
    }
    fn blocked(&self, code: &str) -> Value {
        json!({"schema":1,"operation":"publish_session","state":"blocked","code":code,
            "next_index":self.next,"provider_requests":0,"retry_authorized":false,"batch_complete":false})
    }
    async fn prepare(
        &mut self,
        index: usize,
        source: Option<PathBuf>,
        directory: PathBuf,
    ) -> Result<Value, Failure> {
        let selected = self.batch.files.get(index).ok_or(Failure::Arguments)?;
        if index != self.next {
            return Ok(self.blocked("previous_file_unconfirmed"));
        }
        if let Some(source) = source {
            let metadata = std::fs::symlink_metadata(&source).map_err(|_| Failure::File)?;
            if !metadata.is_dir() || metadata.file_type().is_symlink() {
                return Err(Failure::Binding);
            }
            let source = source.canonicalize().map_err(|_| Failure::File)?;
            if self.origins[index]
                .as_ref()
                .is_some_and(|original| original != &source)
            {
                return Err(Failure::Binding);
            }
            self.origins[index] = Some(source);
        }
        // Startup verification is reused; only this file is checked again before
        // setup. Later files cannot cause O(files²) body I/O or substitute bytes.
        selected.input.verify_body(&selected.body_sha256)?;
        let setup = publish_prepare::Input {
            index,
            directory: directory.join("setup"),
            source: self.origins[index].clone(),
            ..self.input.prepare.clone()
        };
        let mut outcome = publish_prepare::run_selected(self.options, &setup, self.batch).await?;
        if self.origins[index].is_none() {
            self.origins[index] = Some(setup.directory.canonicalize().map_err(|_| Failure::File)?);
        }
        if outcome.prepared() {
            outcome.report["transfer"] = selected.input.transfer_input(&selected.body_sha256)?;
        }
        Ok(outcome.report)
    }
    async fn phase(
        &mut self,
        frame: Frame,
        action: &Run,
        directory: PathBuf,
    ) -> Result<(Value, bool), Failure> {
        match frame {
            Frame::Prepare { index, source_run } => {
                Ok((self.prepare(index, source_run, directory).await?, false))
            }
            Frame::Status { index } => {
                self.batch.files.get(index).ok_or(Failure::Arguments)?;
                if index != self.next {
                    return Ok((self.blocked("previous_file_unconfirmed"), false));
                }
                let observation = self
                    .observation(publish_map::Selection::File(index), action)
                    .await?;
                if observation.complete {
                    self.next += 1;
                }
                Ok((observation.report, false))
            }
            Frame::Verify {
                index,
                source_observation,
            } => {
                let selected = self.batch.files.get(index).ok_or(Failure::Arguments)?;
                if index != self.next {
                    return Ok((self.blocked("previous_file_unconfirmed"), false));
                }
                let verifier = self.actors.verifier.as_ref().ok_or(Failure::Denied)?;
                // One create-new owner per original file, independent of step
                // numbering. Partial observations and pending submissions never
                // permit another GET or another attestation in this session.
                let directory = self
                    .input
                    .prepare
                    .directory
                    .join(format!("verification-{index:04}"));
                let recovering = source_observation.is_some();
                let (observed, submitted) = verification::run(
                    verifier,
                    self.input,
                    selected,
                    directory,
                    source_observation,
                )
                .await?;
                let publication = self
                    .observation(publish_map::Selection::File(index), action)
                    .await?;
                if publication.complete {
                    self.next += 1;
                }
                Ok((
                    json!({"schema":1,"operation":"publish_session_verification",
                    "file_index":index,"observation":observed,"attestation":submitted,
                    "publication":publication.report,"file_live":publication.complete,
                    "max_provider_requests_this_phase":u8::from(!recovering),"retry_authorized":false}),
                    false,
                ))
            }
            Frame::Map {} => {
                if self.next != self.batch.files.len() {
                    return Ok((self.blocked("files_incomplete"), false));
                }
                let observation = self
                    .observation(publish_map::Selection::Batch, action)
                    .await?;
                if observation.complete {
                    self.run.json("media-map.json", &observation.report)?;
                }
                Ok((observation.report, true))
            }
        }
    }
    pub async fn drive(
        mut self,
        mut input: mpsc::Receiver<Result<Frame, Failure>>,
    ) -> Result<Value, Failure> {
        for step in 0..self.input.max_steps {
            let frame = input.recv().await.ok_or(Failure::Transport)??;
            let directory = self.input.prepare.directory.join(format!("step-{step:04}"));
            let action = Run::create(&directory)?;
            action.json("request.json", &frame)?;
            let result = self.phase(frame, &action, directory).await;
            let (report, finished) = match result {
                Ok(result) => result,
                Err(error) => {
                    action.json(
                        "failure.json",
                        &json!({"error":error.code(),"redispatch_authorized":false}),
                    )?;
                    return Err(error);
                }
            };
            action.json("summary.json", &report)?;
            control::output(
                &json!({"schema":1,"event":"phase","step":step,"next_index":self.next,"report":report}),
            )?;
            if finished {
                return Ok(report);
            }
        }
        Ok(self.blocked("step_budget_exhausted"))
    }
}
