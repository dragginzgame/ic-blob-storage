use super::*;
use crate::native::publish_check::tests::{batch, frozen};

#[test]
fn recovery_requires_original_identity_even_when_a_fence_blocks_setup() {
    let directory = frozen();
    let original = batch(&directory).files[0].input.permission.upload;
    let mut observation = PreflightObservation {
        recoverable_upload: Some(original),
        setup_allowed: false,
        report: Value::Null,
    };
    assert_eq!(preflight_allows(&observation, original, true), Ok(false));
    let mut changed = original;
    changed.upload ^= 1;
    assert_eq!(
        preflight_allows(&observation, changed, true),
        Err(Failure::Binding)
    );
    observation.setup_allowed = true;
    assert_eq!(preflight_allows(&observation, original, true), Ok(true));
    assert_eq!(preflight_allows(&observation, original, false), Ok(false));
}
