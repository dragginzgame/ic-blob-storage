//! Sequential receipt gating; in-process cursor/history never grants replay authority.
use super::{
    Actors, Failure, Input, Options, PreparedBatch, Run, Value,
    control::{self, Frame},
    json, publish_map, publish_prepare,
    sources::Sources,
    verification,
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
    sources: Sources,
    browser: Option<&'a super::browser::BrowserSelectionRecord>,
}
impl<'a> Session<'a> {
    pub fn new(
        options: &'a Options,
        input: &'a Input,
        batch: &'a PreparedBatch,
        actors: &'a Actors,
        run: &'a Run,
        sources: Sources,
        browser: Option<&'a super::browser::BrowserSelectionRecord>,
    ) -> Self {
        Self {
            options,
            input,
            batch,
            actors,
            run,
            next: 0,
            sources,
            browser,
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
    async fn prepare(&mut self, index: usize, directory: PathBuf) -> Result<Value, Failure> {
        let selected = self.batch.files.get(index).ok_or(Failure::Arguments)?;
        // Startup verification is reused; only this file is checked again before
        // setup. Later files cannot cause O(files²) body I/O or substitute bytes.
        selected.input.verify_body(&selected.body_sha256)?;
        let setup = publish_prepare::Input {
            index,
            directory: directory.join("setup"),
            source: self.sources.setup[index].clone(),
            ..self.input.prepare.clone()
        };
        let mut outcome = publish_prepare::run_selected(self.options, &setup, self.batch).await?;
        if self.sources.setup[index].is_none() {
            self.sources.setup[index] =
                Some(setup.directory.canonicalize().map_err(|_| Failure::File)?);
        }
        if outcome.prepared() && self.input.browser_selection.is_none() {
            outcome.report["transfer"] = selected.input.transfer_input(&selected.body_sha256)?;
        }
        Ok(outcome.report)
    }
    fn transfer(
        &mut self,
        index: usize,
        action: &Run,
        directory: &std::path::Path,
        source: Option<&std::path::Path>,
    ) -> Result<Value, Failure> {
        let report = super::transfer::Context {
            options: self.options,
            input: self.input,
            batch: self.batch,
            browser: self.browser.ok_or(Failure::Denied)?,
        }
        .run(
            index,
            action,
            directory,
            self.sources.setup[index].as_deref(),
            source,
        )?;
        if self.sources.transfers[index].is_none() {
            self.sources.transfers[index] = Some(super::sources::directory(directory)?);
        }
        Ok(report)
    }
    async fn phase(
        &mut self,
        frame: Frame,
        action: &Run,
        directory: PathBuf,
    ) -> Result<(Value, bool), Failure> {
        match frame {
            Frame::Prepare { index, .. } => {
                self.batch.files.get(index).ok_or(Failure::Arguments)?;
                if index != self.next {
                    control::record_unattempted(action, index, self.next)?;
                    return Ok((self.blocked("previous_file_unconfirmed"), false));
                }
                Ok((self.prepare(index, directory).await?, false))
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
            Frame::Transfer {
                index,
                source_transfer,
            } => {
                self.batch.files.get(index).ok_or(Failure::Arguments)?;
                if index != self.next {
                    control::record_unattempted(action, index, self.next)?;
                    return Ok((self.blocked("previous_file_unconfirmed"), false));
                }
                let report =
                    self.transfer(index, action, &directory, source_transfer.as_deref())?;
                Ok((report, false))
            }
            Frame::Verify {
                index,
                source_observation,
            } => {
                let selected = self.batch.files.get(index).ok_or(Failure::Arguments)?;
                if index != self.next {
                    control::record_unattempted(action, index, self.next)?;
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
            // Persist the resolved original source before execution, so the next
            // parent can recover this run without following session chains.
            let frame = self.sources.resolve(frame)?;
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
