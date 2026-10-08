use super::*;
use crate::CaffeineHashError;
use crate::tests::verifier;
use std::fs;
use std::path::PathBuf;

struct ObservedBody<'a> {
    bytes: &'a [u8],
    destination: &'a Path,
    fail_at_end: bool,
    racing_output: Option<&'a [u8]>,
}

impl Read for ObservedBody<'_> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        // Includes the final EOF/error call after all declared bytes arrived.
        assert!(!self.destination.try_exists()?);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for entry in fs::read_dir(self.destination.parent().unwrap())? {
                let metadata = entry?.metadata()?;
                if metadata.is_dir() {
                    assert_eq!(metadata.permissions().mode() & 0o077, 0);
                }
            }
        }
        if self.bytes.is_empty() {
            if self.fail_at_end {
                return Err(io::ErrorKind::ConnectionReset.into());
            }
            if let Some(bytes) = self.racing_output.take() {
                fs::write(self.destination, bytes)?;
            }
        }
        self.bytes.read(&mut output[..1])
    }
}

fn contents(directory: &Path) -> Vec<PathBuf> {
    fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect()
}

#[test]
fn publishes_exact_bytes_only_after_successful_eof_and_removes_staging_area() {
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("verified.bin");
    let body = ObservedBody {
        bytes: b"abc",
        destination: &output,
        fail_at_end: false,
        racing_output: None,
    };
    let hashes = save_verified(body, verifier(), &output).unwrap();
    assert_eq!(hashes.provider_root, verifier().expected_root());
    assert_eq!(fs::read(&output).unwrap(), b"abc");
    assert_eq!(contents(dir.path()), std::slice::from_ref(&output));
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            fs::metadata(output).unwrap().permissions().mode() & 0o077,
            0
        );
    }
}

#[test]
fn bad_or_incomplete_bodies_never_publish_and_normal_errors_remove_temporary_bytes() {
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("verified.bin");
    for (bytes, fail_at_end) in [
        (b"abd".as_slice(), false),
        (b"ab", false),
        (b"abcd", false),
        (b"abc", true),
    ] {
        let body = ObservedBody {
            bytes,
            destination: &output,
            fail_at_end,
            racing_output: None,
        };
        let error = save_verified(body, verifier(), &output).unwrap_err();
        match (bytes, fail_at_end) {
            (b"abd", false) => assert!(matches!(
                error,
                OutputError::Body(BodyError::Integrity(
                    CaffeineHashError::ProviderRootMismatch
                ))
            )),
            (b"ab", false) => assert!(matches!(
                error,
                OutputError::Body(BodyError::Integrity(CaffeineHashError::Incomplete {
                    remaining: 1
                }))
            )),
            (b"abcd", false) => assert!(matches!(
                error,
                OutputError::Body(BodyError::Integrity(CaffeineHashError::ExceedsLength))
            )),
            (_, true) => assert!(
                matches!(error, OutputError::Body(BodyError::Read(error)) if error.kind() == io::ErrorKind::ConnectionReset)
            ),
            _ => unreachable!(),
        }
        assert_eq!(contents(dir.path()), Vec::<PathBuf>::new());
    }
}

#[test]
fn existing_and_concurrently_created_destinations_are_preserved() {
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("verified.bin");
    let body = ObservedBody {
        bytes: b"abc",
        destination: &output,
        fail_at_end: false,
        racing_output: Some(b"other writer"),
    };
    assert!(
        matches!(save_verified(body, verifier(), &output), Err(OutputError::Publish(error)) if error.kind() == io::ErrorKind::AlreadyExists)
    );
    for _ in 0..2 {
        assert!(
            matches!(save_verified(&b"abc"[..], verifier(), &output), Err(OutputError::Publish(error)) if error.kind() == io::ErrorKind::AlreadyExists)
        );
        assert_eq!(fs::read(&output).unwrap(), b"other writer");
        assert_eq!(contents(dir.path()), std::slice::from_ref(&output));
    }
}

#[cfg(unix)]
#[test]
fn existing_and_dangling_symlinks_are_not_followed_or_replaced() {
    use std::os::unix::fs::symlink;
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("verified.bin");
    let target = dir.path().join("target.bin");
    symlink(&target, &output).unwrap();
    for exists in [false, true] {
        if exists {
            fs::write(&target, b"protected").unwrap();
        }
        assert!(
            matches!(save_verified(&b"abc"[..], verifier(), &output), Err(OutputError::Publish(error)) if error.kind() == io::ErrorKind::AlreadyExists)
        );
        assert_eq!(fs::read_link(&output).unwrap(), target);
        if exists {
            assert_eq!(fs::read(&target).unwrap(), b"protected");
        } else {
            assert!(!target.exists());
        }
    }
}

#[test]
fn invalid_destination_rejects_before_reading_the_body() {
    struct Unread;
    impl Read for Unread {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            panic!("invalid destination must reject first")
        }
    }
    let dir = tempfile::tempdir().unwrap();
    assert!(matches!(
        save_verified(Unread, verifier(), Path::new("..")),
        Err(OutputError::InvalidDestination)
    ));
    assert!(
        matches!(save_verified(Unread, verifier(), &dir.path().join("missing/file")), Err(OutputError::Create(error)) if error.kind() == io::ErrorKind::NotFound)
    );
    assert_eq!(contents(dir.path()), Vec::<PathBuf>::new());
}
