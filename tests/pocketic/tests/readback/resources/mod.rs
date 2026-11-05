//! Bounded read-slot and call-context sizing with the default Rust allocator.
use super::*;

async fn profile(f: &Fixture, outcome: Result<(), JourneyFailure>) -> ReadExecutionProfile {
    let p = f
        .read_resources(f.authority_operator)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(p.outcome, outcome);
    let stages = [
        Some(p.started),
        p.sending,
        p.replied,
        p.decoding,
        p.decoded,
        p.verified,
        Some(p.finished),
    ];
    let stages: Vec<_> = stages.into_iter().flatten().collect();
    assert!(stages.windows(2).all(|pair| pair[0] <= pair[1]));
    // Deliberately broad regression ceiling, not a production instruction budget.
    assert!(p.finished - p.started < 160_000_000);
    if outcome.is_err() {
        assert_eq!(p.disclosed_bytes, 0);
    }
    p
}

async fn heap(f: &Fixture) -> u64 {
    let status = f
        .harness
        .pic
        .canister_status(f.service, Some(f.operator))
        .await
        .unwrap();
    u64::try_from(status.memory_metrics.wasm_memory_size.0).unwrap()
}

fn sample(label: &str, p: ReadExecutionProfile, memory: u64) -> serde_json::Value {
    serde_json::json!({
        "case":label, "wasm_memory_bytes":memory,
        "workflow_instructions":p.finished - p.started,
        "before_call":p.sending.map(|v| v - p.started),
        "call_and_callback_entry":p.sending.zip(p.replied).map(|(a,b)| b-a),
        "callback_authorization":p.replied.zip(p.decoding).map(|(a,b)| b-a),
        "decode":p.decoding.zip(p.decoded).map(|(a,b)| b-a),
        "verify":p.decoded.zip(p.verified).map(|(a,b)| b-a),
        "received_bytes":p.received_bytes,"disclosed_bytes":p.disclosed_bytes,
        "outcome":format!("{:?}", p.outcome),
    })
}

fn wasm_hash(variable: &str) -> String {
    let bytes = std::fs::read(fixture_path(variable)).unwrap();
    hex::encode(Sha256::digest(bytes))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn read_resources_cover_full_chunks_faults_and_repeated_slot_reuse() {
    let f = Fixture::new().await;
    assert_eq!(f.read_resources(f.operator).await, Ok(None));
    let mut samples = Vec::new();
    for (name, id) in [("abc-text", 1), ("pattern-1048576", 2)] {
        let v = content_vectors::vector(name, id);
        f.confirm_bytes(&v).await;
        f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false)
            .await;
        let before = heap(&f).await;
        let usage = f.usage(f.first).await.unwrap();
        for attempt in 0..12 {
            let chunk = f.read_chunk(f.first, v.upload, 0).await.unwrap();
            assert_eq!(chunk.bytes, v.bytes);
            let p = profile(&f, Ok(())).await;
            assert_eq!(p.caller, f.first);
            assert_eq!(p.disclosed_bytes, v.bytes.len() as u64);
            assert!(p.verified.is_some());
            assert!(p.received_bytes > p.disclosed_bytes);
            assert!(heap(&f).await <= before + 8 * CHUNK as u64);
            samples.push(sample(&format!("{name}-{attempt}"), p, heap(&f).await));
        }
        for (mode, failure) in [
            (ReadSourceMode::Corrupt, JourneyFailure::ContentMismatch),
            (ReadSourceMode::Oversized, JourneyFailure::ReplyTooLarge),
            (ReadSourceMode::Malformed, JourneyFailure::InvalidReply),
            (ReadSourceMode::WrongType, JourneyFailure::InvalidReply),
            (
                ReadSourceMode::TruncatedEncoding,
                JourneyFailure::InvalidReply,
            ),
            (ReadSourceMode::Reject, JourneyFailure::Transport),
        ] {
            f.source_config(v.upload, 0, &v.bytes, mode, false).await;
            assert_eq!(f.read_chunk(f.first, v.upload, 0).await, Err(failure));
            let p = profile(&f, Err(failure)).await;
            match mode {
                ReadSourceMode::Corrupt => assert!(p.verified.is_some()),
                ReadSourceMode::Malformed
                | ReadSourceMode::WrongType
                | ReadSourceMode::TruncatedEncoding => {
                    assert!(p.decoding.is_some());
                    assert!(p.decoded.is_none());
                }
                ReadSourceMode::Oversized | ReadSourceMode::Reject => assert!(p.decoding.is_none()),
                _ => unreachable!(),
            }
            samples.push(sample(&format!("{name}-{mode:?}"), p, heap(&f).await));
        }
        assert_eq!(f.usage(f.first).await.unwrap(), usage);
    }
    let latest = f.read_resources(f.operator).await.unwrap();
    for caller in [f.first, f.second, f.gateway, Principal::anonymous()] {
        assert_eq!(f.read_resources(caller).await, Err(JourneyFailure::Denied));
    }
    assert_eq!(f.read_resources(f.operator).await.unwrap(), latest);
    let report = serde_json::json!({
        "scope":"Single-slot fixture read through a local source. Call-context counter 1 includes fixture journal saves and excludes ingress decode, source execution, final reply encoding and diagnostic storage. Allocated Wasm memory includes allocator headroom; neither peak live bytes nor production concurrency sizing. Default Rust allocator; no deployed provider calls.",
        "authority_wasm_sha256":wasm_hash("BLOB_AUTHORITY_PROBE_WASM"),
        "source_wasm_sha256":wasm_hash("BLOB_GATEWAY_SOURCE_WASM"),
        "samples":samples,
    });
    if let Some(path) = std::env::var_os("BLOB_READ_RESOURCE_REPORT") {
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
    println!("read resource profile: {report}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn held_read_keeps_one_slot_and_profiles_do_not_grant_authority() {
    let f = Fixture::new().await;
    let v = content_vectors::vector("pattern-1048576", 1);
    f.confirm_bytes(&v).await;
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, true)
        .await;
    let held = f.hold_read(v.upload).await;
    assert_eq!(f.read_resources(f.operator).await, Ok(None));
    let before = f.source_observation().await;
    for _ in 0..8 {
        for (caller, failure) in [
            (f.first, JourneyFailure::ReadInProgress),
            (f.second, JourneyFailure::Denied),
        ] {
            assert_eq!(f.read_chunk(caller, v.upload, 0).await, Err(failure));
            let p = profile(&f, Err(failure)).await;
            assert_eq!(p.caller, caller);
            assert!(p.sending.is_none());
            assert!(p.replied.is_none());
            assert_eq!(p.received_bytes, 0);
        }
    }
    assert_eq!(f.source_observation().await, before);
    assert_eq!(f.resume_read(held).await.unwrap().bytes, v.bytes);
    let p = profile(&f, Ok(())).await;
    assert_eq!(p.caller, f.first);
    assert_eq!(p.disclosed_bytes, CHUNK as u64);
    assert!(p.verified.is_some());
    f.source_config(v.upload, 0, &v.bytes, ReadSourceMode::Valid, false)
        .await;
    assert_eq!(
        f.read_chunk(f.first, v.upload, 0).await.unwrap().bytes,
        v.bytes
    );
}
