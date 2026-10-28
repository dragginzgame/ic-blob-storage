//! History freshness requires a continuous observed execution chain in one heap.
use super::*;

#[test]
fn observed_updates_preserve_a_pending_history_observation_without_writes() {
    let memory = memory();
    let mut installation = ServiceInstallation::install(memories(&memory), validated()).unwrap();
    installation.last_platform_version = Some(7);
    let before = bytes(&memory);
    let observation = installation.begin_history_observation(7);
    for version in [8, 8, 9] {
        installation.observe_platform_version(version);
    }
    assert!(installation.check_history_observation(observation, 7, 10));
    assert_eq!(bytes(&memory), before);
}

#[test]
fn a_missed_execution_invalidates_history_even_after_later_contiguous_observations() {
    for missed in [6, 9] {
        let memory = memory();
        let mut installation =
            ServiceInstallation::install(memories(&memory), validated()).unwrap();
        installation.last_platform_version = Some(7);
        let before = bytes(&memory);
        let observation = installation.begin_history_observation(7);
        installation.observe_platform_version(missed);
        installation.observe_platform_version(missed + 1);
        assert!(!installation.check_history_observation(observation, 7, missed + 2));
        assert!(installation.stores().uploads.is_fenced());
        assert_eq!(bytes(&memory), before);
    }
}

#[test]
fn an_explicit_new_fence_invalidates_a_pending_observation_without_writes() {
    let memory = memory();
    let mut installation = ServiceInstallation::install(memories(&memory), validated()).unwrap();
    installation.last_platform_version = Some(7);
    let before = bytes(&memory);
    let observation = installation.begin_history_observation(7);
    installation.fence();
    assert!(!installation.check_history_observation(observation, 7, 8));
    assert!(installation.stores().uploads.is_fenced());
    assert_eq!(bytes(&memory), before);
}

#[test]
fn reopening_the_same_stable_installation_cannot_reuse_an_old_heap_observation() {
    let memory = memory();
    let mut installation = ServiceInstallation::install(memories(&memory), validated()).unwrap();
    installation.last_platform_version = Some(7);
    let before = bytes(&memory);
    let observation = installation.begin_history_observation(7);
    drop(installation);
    let mut restored =
        ServiceInstallation::open(memories(&memory), candidate().service, RELEASE).unwrap();
    assert!(!restored.check_history_observation(observation, 7, 8));
    assert!(restored.stores().uploads.is_fenced());
    assert_eq!(bytes(&memory), before);
}
