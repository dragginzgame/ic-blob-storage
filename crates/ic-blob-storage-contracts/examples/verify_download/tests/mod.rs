use super::*;

#[test]
fn claim_collection_preserves_json_bounds_and_stream_failures() {
    const JSON: &[u8] = br#"{"root":"sha256:example","bytes":3,"headers":[]}"#;
    let mut exact = JSON.to_vec();
    exact.resize(CLAIM_BYTES, b' ');
    assert_eq!(read_claim(exact.as_slice()).unwrap().bytes, 3);
    exact.push(b' ');
    assert_eq!(
        read_claim(exact.as_slice()).err().unwrap().to_string(),
        "claim exceeds example input bound"
    );
    for invalid in [
        &b""[..],
        &b"{"[..],
        &br#"{"root":"example","bytes":3,"headers":[],"extra":0}"#[..],
    ] {
        assert!(read_claim(invalid).err().unwrap().is::<serde_json::Error>());
    }
    assert_eq!(
        read_claim(InterruptedBody {
            body: JSON,
            interrupt: true,
            fail_at_end: false,
        })
        .unwrap()
        .bytes,
        3
    );
    let error = read_claim(InterruptedBody {
        body: JSON,
        interrupt: false,
        fail_at_end: true,
    })
    .err()
    .unwrap();
    assert_eq!(
        error.downcast_ref::<io::Error>().unwrap().kind(),
        io::ErrorKind::ConnectionReset
    );
}

pub(super) fn verifier() -> CaffeineRootVerifier {
    CaffeineRootVerifier::new(
        "sha256:b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7"
            .parse()
            .unwrap(),
        3,
        &[],
        CaffeineHashLimits {
            max_content_bytes: NonZeroU64::new(10).unwrap(),
            max_append_bytes: NonZeroUsize::new(FRAME).unwrap(),
            max_headers: NonZeroUsize::new(1).unwrap(),
            max_header_bytes: NonZeroUsize::new(128).unwrap(),
        },
    )
    .unwrap()
}

#[test]
fn interrupted_short_writes_are_completed_but_write_and_flush_failures_reject() {
    struct Staging {
        bytes: Vec<u8>,
        interrupt: bool,
        zero_write: bool,
        fail_flush: bool,
    }
    impl Write for Staging {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if std::mem::take(&mut self.interrupt) {
                return Err(io::ErrorKind::Interrupted.into());
            }
            if self.zero_write {
                return Ok(0);
            }
            self.bytes.push(bytes[0]);
            Ok(1)
        }
        fn flush(&mut self) -> io::Result<()> {
            if self.fail_flush {
                Err(io::ErrorKind::StorageFull.into())
            } else {
                Ok(())
            }
        }
    }
    let mut stage = Staging {
        bytes: Vec::new(),
        interrupt: true,
        zero_write: false,
        fail_flush: false,
    };
    verify_body(&b"abc"[..], verifier(), &mut stage).unwrap();
    assert_eq!(stage.bytes, b"abc");
    stage.zero_write = true;
    assert!(
        matches!(verify_body(&b"abc"[..], verifier(), &mut stage), Err(BodyError::Stage(error)) if error.kind() == io::ErrorKind::WriteZero)
    );
    stage.zero_write = false;
    stage.fail_flush = true;
    assert!(
        matches!(verify_body(&b"abc"[..], verifier(), &mut stage), Err(BodyError::Stage(error)) if error.kind() == io::ErrorKind::StorageFull)
    );
}

struct InterruptedBody {
    body: &'static [u8],
    interrupt: bool,
    fail_at_end: bool,
}

impl Read for InterruptedBody {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if std::mem::take(&mut self.interrupt) {
            return Err(io::ErrorKind::Interrupted.into());
        }
        if self.body.is_empty() && self.fail_at_end {
            return Err(io::ErrorKind::ConnectionReset.into());
        }
        // Force distinct calls even for the short vector and its trailing byte.
        self.body.read(&mut output[..1])
    }
}

#[test]
fn successful_eof_is_required_after_the_last_expected_byte() {
    let body = |bytes, fail| InterruptedBody {
        body: bytes,
        interrupt: true,
        fail_at_end: fail,
    };
    assert!(verify_body(body(b"abc", false), verifier(), io::sink()).is_ok());
    assert!(
        matches!(verify_body(body(b"abc", true), verifier(), io::sink()),
        Err(BodyError::Read(error)) if error.kind() == io::ErrorKind::ConnectionReset)
    );
    assert!(matches!(
        verify_body(body(b"abcd", false), verifier(), io::sink()),
        Err(BodyError::Integrity(CaffeineHashError::ExceedsLength))
    ));
    assert!(matches!(
        verify_body(body(b"ab", false), verifier(), io::sink()),
        Err(BodyError::Integrity(CaffeineHashError::Incomplete {
            remaining: 1
        }))
    ));
    assert!(matches!(
        verify_body(body(b"abd", false), verifier(), io::sink()),
        Err(BodyError::Integrity(
            CaffeineHashError::ProviderRootMismatch
        ))
    ));
}
