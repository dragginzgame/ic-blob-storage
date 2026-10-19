//! Native nanosecond measurements; creates a fresh report and refuses overwrite.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .nth(1)
        .ok_or("usage: wire-benchmark OUTPUT.json")?;
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    let epoch = std::time::Instant::now();
    let rows = blob_test_protocol::wire_benchmark::run(|| {
        u64::try_from(epoch.elapsed().as_nanos()).unwrap()
    });
    serde_json::to_writer_pretty(
        file,
        &serde_json::json!({"unit": "native_ns", "measurements": rows}),
    )?;
    Ok(())
}
