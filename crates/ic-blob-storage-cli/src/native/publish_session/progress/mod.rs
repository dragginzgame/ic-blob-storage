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

/// A derived continuation, never a persisted cursor or renewed effect authority.
struct PhaseOutcome {
    report: Value,
    finished: bool,
    next_frame: Option<Frame>,
}

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
    fn following_file(&self) -> Frame {
        if self.next == self.batch.files.len() {
            Frame::Map {}
        } else {
            Frame::Status { index: self.next }
        }
    }
    fn incomplete_file(&self, index: usize) -> Frame {
        if self.sources.has_observation(index) {
            Frame::Verify {
                index,
                source_observation: None,
            }
        } else if self.sources.transfers[index].is_some() {
            Frame::Transfer {
                index,
                source_transfer: None,
            }
        } else {
            Frame::Prepare {
                index,
                source_run: None,
            }
        }
    }
    async fn prepare(&mut self, index: usize, directory: PathBuf) -> Result<PhaseOutcome, Failure> {
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
        let next_frame =
            (outcome.prepared() && self.browser.is_some()).then_some(Frame::Transfer {
                index,
                source_transfer: None,
            });
        Ok(PhaseOutcome {
            report: outcome.report,
            finished: false,
            next_frame,
        })
    }
    fn transfer(
        &mut self,
        index: usize,
        action: &Run,
        directory: &std::path::Path,
        source: Option<&std::path::Path>,
    ) -> Result<PhaseOutcome, Failure> {
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
        Ok(PhaseOutcome {
            report,
            finished: false,
            next_frame: self.actors.verifier.as_ref().map(|_| Frame::Verify {
                index,
                source_observation: None,
            }),
        })
    }
    async fn status(&mut self, index: usize, action: &Run) -> Result<PhaseOutcome, Failure> {
        let observation = self
            .observation(publish_map::Selection::File(index), action)
            .await?;
        let next_frame = if observation.complete {
            self.next += 1;
            self.following_file()
        } else {
            self.incomplete_file(index)
        };
        Ok(PhaseOutcome {
            report: observation.report,
            finished: false,
            next_frame: Some(next_frame),
        })
    }
    async fn phase(
        &mut self,
        frame: Frame,
        action: &Run,
        directory: PathBuf,
    ) -> Result<PhaseOutcome, Failure> {
        let index = match &frame {
            Frame::Prepare { index, .. }
            | Frame::Transfer { index, .. }
            | Frame::Verify { index, .. }
            | Frame::Status { index } => Some(*index),
            Frame::Map {} => None,
        };
        if let Some(index) = index {
            self.batch.files.get(index).ok_or(Failure::Arguments)?;
            if index != self.next {
                if !matches!(&frame, Frame::Status { .. }) {
                    control::record_unattempted(action, index, self.next)?;
                }
                return Ok(PhaseOutcome {
                    report: self.blocked("previous_file_unconfirmed"),
                    finished: false,
                    next_frame: None,
                });
            }
        }
        match frame {
            Frame::Prepare { index, .. } => self.prepare(index, directory).await,
            Frame::Status { index } => self.status(index, action).await,
            Frame::Transfer {
                index,
                source_transfer,
            } => self.transfer(index, action, &directory, source_transfer.as_deref()),
            Frame::Verify {
                index,
                source_observation,
            } => {
                let selected = self.batch.files.get(index).ok_or(Failure::Arguments)?;
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
                Ok(PhaseOutcome {
                    report: json!({"schema":1,"operation":"publish_session_verification",
                    "file_index":index,"observation":observed,"attestation":submitted,
                    "publication":publication.report,"file_live":publication.complete,
                    "max_provider_requests_this_phase":u8::from(!recovering),"retry_authorized":false}),
                    finished: false,
                    next_frame: publication.complete.then(|| self.following_file()),
                })
            }
            Frame::Map {} => {
                if self.next != self.batch.files.len() {
                    return Ok(PhaseOutcome {
                        report: self.blocked("files_incomplete"),
                        finished: false,
                        next_frame: None,
                    });
                }
                let observation = self
                    .observation(publish_map::Selection::Batch, action)
                    .await?;
                if observation.complete {
                    self.run.json("media-map.json", &observation.report)?;
                }
                Ok(PhaseOutcome {
                    report: observation.report,
                    finished: true,
                    next_frame: None,
                })
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
            let PhaseOutcome {
                report,
                finished,
                next_frame,
            } = match result {
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
                &json!({"schema":1,"event":"phase","step":step,"next_index":self.next,
                    "report":report,"next_frame":next_frame}),
            )?;
            if finished {
                return Ok(report);
            }
        }
        Ok(self.blocked("step_budget_exhausted"))
    }
}
