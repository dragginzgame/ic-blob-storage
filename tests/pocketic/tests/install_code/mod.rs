//! Platform cooldown for the installation/restoration qualification journeys.

use ic_testkit::pocket_ic::PocketIc;

/// Drain install-code debt through real rounds without disabling rate limits.
/// The selected application's 300B per-install maximum and 2B per-round
/// allowance bound the cooldown to 150 rounds; advancing time alone cannot
/// drain this platform-owned instruction debt.
pub(super) fn settle_install_code_debt(pic: &PocketIc) {
    for _ in 0..150 {
        pic.tick();
    }
}
