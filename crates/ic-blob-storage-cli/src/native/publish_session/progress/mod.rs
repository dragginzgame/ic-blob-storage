//! Sequential receipt gating; in-process cursor/history never grants replay authority.
use super::{
    Actors, Failure, Input, Options, PreparedBatch, Run, Value,
    control::{self, Frame},
    json, publish_map, publish_prepare,
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
            origins: vec![None; batch.inputs.len()],
        }
    }
    async fn observation(
        &self,
        selection: publish_map::Selection,
        run: &Run,
    ) -> Result<Value, Failure> {
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
        let selected = self.batch.inputs.get(index).ok_or(Failure::Arguments)?;
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
        selected.verify_body(&self.batch.body_digests[index])?;
        let setup = publish_prepare::Input {
            index,
            directory: directory.join("setup"),
            source: self.origins[index].clone(),
            ..self.input.prepare.clone()
        };
        let mut report = publish_prepare::run_selected(self.options, &setup, self.batch).await?;
        if self.origins[index].is_none() {
            self.origins[index] = Some(setup.directory.canonicalize().map_err(|_| Failure::File)?);
        }
        if report["prepared"] == true {
            report["transfer"] = selected.transfer_input(&self.batch.body_digests[index])?;
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
            Frame::Prepare { index, source_run } => {
                Ok((self.prepare(index, source_run, directory).await?, false))
            }
            Frame::Status { index } => {
                self.batch.inputs.get(index).ok_or(Failure::Arguments)?;
                if index != self.next {
                    return Ok((self.blocked("previous_file_unconfirmed"), false));
                }
                let report = self
                    .observation(publish_map::Selection::File(index), action)
                    .await?;
                if report["file_live"] == true {
                    self.next += 1;
                }
                Ok((report, false))
            }
            Frame::Map {} => {
                if self.next != self.batch.inputs.len() {
                    return Ok((self.blocked("files_incomplete"), false));
                }
                let report = self
                    .observation(publish_map::Selection::Batch, action)
                    .await?;
                if report["all_references_live"] == true {
                    self.run.json("media-map.json", &report)?;
                }
                Ok((report, true))
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
