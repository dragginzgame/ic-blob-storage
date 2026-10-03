//! Bounded private process control; messages confer no service or retry authority.
use crate::native::Failure;
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
};
use tokio::sync::mpsc;

const MAX_FRAME_BYTES: u64 = 8192;

/// Durable fact written only when ordering rejects a phase before its owner
/// is called. Absence of its claim directory alone never proves an unattempted phase.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UnattemptedPhaseRecord {
    pub format: String,
    pub index: usize,
    pub next_index: usize,
}
impl UnattemptedPhaseRecord {
    pub const FORMAT: &'static str = "ic-blob-storage/unattempted-publication-phase";
}

pub(super) fn record_unattempted(
    run: &crate::native::artifacts::Run,
    index: usize,
    next_index: usize,
) -> Result<(), Failure> {
    run.json(
        "unattempted.json",
        &UnattemptedPhaseRecord {
            format: UnattemptedPhaseRecord::FORMAT.into(),
            index,
            next_index,
        },
    )
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "phase", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum Frame {
    Prepare {
        index: usize,
        source_run: Option<PathBuf>,
    },
    Status {
        index: usize,
    },
    Transfer {
        index: usize,
        source_transfer: Option<PathBuf>,
    },
    Verify {
        index: usize,
        source_observation: Option<PathBuf>,
    },
    Map {},
}

fn read_frame(reader: &mut impl BufRead) -> Result<Frame, Failure> {
    let mut bytes = Vec::new();
    reader
        .take(MAX_FRAME_BYTES + 1)
        .read_until(b'\n', &mut bytes)
        .map_err(|_| Failure::Transport)?;
    if bytes.is_empty() {
        return Err(Failure::Transport);
    }
    if bytes.len() as u64 > MAX_FRAME_BYTES || !bytes.ends_with(b"\n") {
        return Err(Failure::ReplyLimit);
    }
    serde_json::from_slice(&bytes).map_err(|_| Failure::Arguments)
}

pub(super) fn input() -> Result<mpsc::Receiver<Result<Frame, Failure>>, Failure> {
    let (sender, receiver) = mpsc::channel(1);
    // Tokio's blocking stdin pool can keep runtime shutdown waiting after a
    // deadline. An ordinary detached process thread does not own that runtime.
    // The bounded queue wakes on receiver loss; process exit ends an idle read.
    std::thread::Builder::new()
        .name("publication-control".into())
        .spawn(move || {
            let mut reader = BufReader::new(std::io::stdin().lock());
            loop {
                let frame = read_frame(&mut reader);
                let failed = frame.is_err();
                if sender.blocking_send(frame).is_err() || failed {
                    break;
                }
            }
        })
        .map_err(|_| Failure::Transport)?;
    Ok(receiver)
}

pub(super) fn output(value: &serde_json::Value) -> Result<(), Failure> {
    let mut writer = std::io::stdout().lock();
    serde_json::to_writer(&mut writer, value).map_err(|_| Failure::Transport)?;
    writer
        .write_all(b"\n")
        .and_then(|()| writer.flush())
        .map_err(|_| Failure::Transport)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn control_requires_bounded_exact_frames_and_preserves_distinct_messages() {
        let mut input = &b"{\"phase\":\"prepare\",\"index\":0}\n{\"phase\":\"map\"}\n"[..];
        assert!(matches!(
            read_frame(&mut input),
            Ok(Frame::Prepare {
                index: 0,
                source_run: None
            })
        ));
        assert!(matches!(read_frame(&mut input), Ok(Frame::Map {})));
        assert!(matches!(read_frame(&mut input), Err(Failure::Transport)));
        for bytes in [
            b"{\"phase\":\"map\",\"retry\":true}\n".as_slice(),
            b"{\"phase\":\"unknown\"}\n",
            b"[]\n",
        ] {
            assert!(matches!(read_frame(&mut &*bytes), Err(Failure::Arguments)));
        }
        assert!(matches!(
            read_frame(&mut &b"{\"phase\":\"map\"}"[..]),
            Err(Failure::ReplyLimit)
        ));
        assert!(matches!(
            read_frame(&mut vec![b'x'; 8193].as_slice()),
            Err(Failure::ReplyLimit)
        ));
    }
}
