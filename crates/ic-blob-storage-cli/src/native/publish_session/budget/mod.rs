//! Session-owned arithmetic; each input and recovery boundary still admits its budgets.

/// Upper step allowance for the selected file count; no effect is authorized.
pub(in crate::native) fn step_limit(files: usize) -> Option<u64> {
    u64::try_from(files).ok()?.checked_mul(8)?.checked_add(1)
}

/// Query allowance written and checked against the same session contract.
pub(super) fn query_limit(steps: u64, files: usize) -> Option<u64> {
    steps
        .checked_mul(5)?
        .checked_add(u64::try_from(files).ok()?.checked_mul(2)?)?
        .checked_add(3)
}
