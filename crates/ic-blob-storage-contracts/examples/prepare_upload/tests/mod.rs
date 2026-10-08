use super::*;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHashLimit;

#[test]
fn declaration_collection_preserves_json_bounds_and_stream_failures() {
    const JSON: &[u8] = br#"{"bytes":3,"headers":[]}"#;
    let mut exact = JSON.to_vec();
    exact.resize(DECLARATION_BYTES, b' ');
    assert_eq!(read_declaration(exact.as_slice()).unwrap().bytes, 3);
    exact.push(b' ');
    assert_eq!(
        read_declaration(exact.as_slice())
            .err()
            .unwrap()
            .to_string(),
        "declaration exceeds example input bound"
    );
    for invalid in [
        &b""[..],
        &b"{"[..],
        &br#"{"bytes":3,"headers":[],"extra":0}"#[..],
    ] {
        assert!(
            read_declaration(invalid)
                .err()
                .unwrap()
                .is::<serde_json::Error>()
        );
    }
    assert_eq!(
        read_declaration(Body {
            bytes: JSON,
            interrupted: true,
            fail_at_end: false,
        })
        .unwrap()
        .bytes,
        3
    );
    let error = read_declaration(Body {
        bytes: JSON,
        interrupted: false,
        fail_at_end: true,
    })
    .err()
    .unwrap();
    assert_eq!(
        error.downcast_ref::<io::Error>().unwrap().kind(),
        io::ErrorKind::ConnectionReset
    );
}

fn declaration() -> Declaration {
    Declaration {
        bytes: 3,
        headers: vec![
            Header {
                name: "Content-Length".into(),
                value: "3".into(),
            },
            Header {
                name: "Content-Type".into(),
                value: "text/plain".into(),
            },
        ],
    }
}
fn limits() -> PreparationLimits {
    PreparationLimits {
        bytes: NonZeroU64::new(10).unwrap(),
        chunks: NonZeroUsize::new(1).unwrap(),
    }
}

pub(super) struct Body {
    pub(super) bytes: &'static [u8],
    pub(super) interrupted: bool,
    pub(super) fail_at_end: bool,
}
impl Read for Body {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if std::mem::take(&mut self.interrupted) {
            return Err(io::ErrorKind::Interrupted.into());
        }
        if self.bytes.is_empty() && self.fail_at_end {
            return Err(io::ErrorKind::ConnectionReset.into());
        }
        self.bytes.read(&mut output[..1])
    }
}

#[test]
fn computes_independent_vector_and_preserves_original_metadata_only_after_clean_eof() {
    let expected = declaration().headers;
    let output = prepare(
        Body {
            bytes: b"abc",
            interrupted: true,
            fail_at_end: false,
        },
        declaration(),
        limits(),
    )
    .unwrap();
    assert_eq!(output.claim.headers, expected);
    assert_eq!(output.claim.bytes, 3);
    assert_eq!(
        output.claim.root,
        "sha256:0e9afaf413b048e40834d5b0e737d80fbf304af2045c7564d96ad8aebaf74dfd"
    );
    assert_eq!(
        output.chunk_hashes,
        ["sha256:b5b435d47a4cce7dfec493b1e020c5308d9c7fe90add1aff510f9c2a9c4ea8e7"]
    );
    assert_eq!(
        output.computed_content_digest,
        "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert!(
        matches!(prepare(Body { bytes: b"abc", interrupted: false, fail_at_end: true }, declaration(), limits()), Err(PreparationError::Io(error)) if error.kind() == io::ErrorKind::ConnectionReset)
    );
    assert!(matches!(
        prepare(&b"ab"[..], declaration(), limits()),
        Err(PreparationError::Hash(CaffeineHashError::Incomplete {
            remaining: 1
        }))
    ));
    assert!(matches!(
        prepare(&b"abcd"[..], declaration(), limits()),
        Err(PreparationError::Hash(CaffeineHashError::ExceedsLength))
    ));
}

#[test]
fn invalid_metadata_and_resource_limits_reject_before_consuming_source_bytes() {
    struct Unread;
    impl Read for Unread {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            panic!("rejected declaration must not consume body")
        }
    }
    let mut changed = declaration();
    changed.headers[0].value = "4".into();
    assert!(matches!(
        prepare(Unread, changed, limits()),
        Err(PreparationError::Metadata(
            UploadMetadataError::LengthMismatch
        ))
    ));
    let mut changed = declaration();
    changed.headers.push(Header {
        name: "content-type".into(),
        value: "text/plain".into(),
    });
    assert!(matches!(
        prepare(Unread, changed, limits()),
        Err(PreparationError::Metadata(
            UploadMetadataError::DuplicateHeader
        ))
    ));
    let mut changed = declaration();
    changed.headers[1].value = "text/plain\r\nX: value".into();
    assert!(matches!(
        prepare(Unread, changed, limits()),
        Err(PreparationError::Metadata(UploadMetadataError::HeaderValue))
    ));
    let mut budget = limits();
    budget.bytes = NonZeroU64::new(2).unwrap();
    assert!(matches!(
        prepare(Unread, declaration(), budget),
        Err(PreparationError::Builder(
            CaffeineManifestBuilderError::Hash(CaffeineHashError::Limit(
                CaffeineHashLimit::ContentBytes
            ))
        ))
    ));
    let mut large = declaration();
    large.bytes = 1_048_577;
    large.headers[0].value = large.bytes.to_string();
    let mut budget = limits();
    budget.bytes = NonZeroU64::new(large.bytes).unwrap();
    assert!(matches!(
        prepare(Unread, large, budget),
        Err(PreparationError::Builder(
            CaffeineManifestBuilderError::TooManyChunks
        ))
    ));
}

#[test]
fn staged_buffers_handle_short_writes_and_reject_write_or_flush_failure() {
    struct Output {
        bytes: Vec<u8>,
        interrupted: bool,
        fail_write: bool,
        fail_flush: bool,
    }
    impl Write for Output {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if std::mem::take(&mut self.interrupted) {
                return Err(io::ErrorKind::Interrupted.into());
            }
            if self.fail_write && self.bytes.len() == 1 {
                return Err(io::ErrorKind::StorageFull.into());
            }
            self.bytes.push(bytes[0]);
            Ok(1)
        }
        fn flush(&mut self) -> io::Result<()> {
            if self.fail_flush {
                return Err(io::ErrorKind::Other.into());
            }
            Ok(())
        }
    }
    let mut output = Output {
        bytes: vec![],
        interrupted: true,
        fail_write: false,
        fail_flush: false,
    };
    let prepared = prepare_to(&b"abc"[..], declaration(), limits(), &mut output).unwrap();
    assert_eq!(output.bytes, b"abc");
    assert_eq!(
        prepared.claim.root,
        "sha256:0e9afaf413b048e40834d5b0e737d80fbf304af2045c7564d96ad8aebaf74dfd"
    );
    output.bytes.clear();
    output.fail_write = true;
    assert!(
        matches!(prepare_to(&b"abc"[..], declaration(), limits(), &mut output), Err(PreparationError::Io(error)) if error.kind() == io::ErrorKind::StorageFull)
    );
    output.fail_write = false;
    output.fail_flush = true;
    assert!(
        matches!(prepare_to(&b"abc"[..], declaration(), limits(), &mut output), Err(PreparationError::Io(error)) if error.kind() == io::ErrorKind::Other)
    );
}
