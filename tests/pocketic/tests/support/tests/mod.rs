//! Final subprocess output must retain bytes prefetched by its control reader.
use super::*;
use std::{
    io::BufRead,
    process::{Command, Stdio},
};

#[test]
fn final_output_retains_buffered_response_and_stderr_after_control_frame() {
    let mut command = Command::new("/bin/sh");
    command
        .args([
            "-c",
            "printf 'control\\nfinal\\n'; printf 'diagnostic' >&2; exit 7",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = OwnedChild::spawn(&mut command).unwrap();
    let mut reader = std::io::BufReader::new(child.take_stdout().unwrap());
    // All output is available before the control reader prefetches the final frame.
    child.wait().unwrap();
    assert_eq!(reader.fill_buf().unwrap(), b"control\nfinal\n");
    let mut frame = String::new();
    reader.read_line(&mut frame).unwrap();
    assert_eq!(frame, "control\n");
    let output = wait_with_output(&mut child, reader);
    assert_eq!(output.status.code(), Some(7));
    assert_eq!(output.stdout, b"final\n");
    assert_eq!(output.stderr, b"diagnostic");
}
