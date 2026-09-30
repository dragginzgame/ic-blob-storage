//! Pure installed-configuration read authority, independent of deployment roles.
use candid::Principal;

/// Only the exact installed service and operator may disclose configuration.
/// Managed activation, controller status and Fleet membership supply no delegation.
#[must_use]
pub fn may_inspect(
    installed_service: Principal,
    installed_operator: Principal,
    actual_service: Principal,
    actor: Principal,
) -> bool {
    actual_service == installed_service && actor == installed_operator
}
