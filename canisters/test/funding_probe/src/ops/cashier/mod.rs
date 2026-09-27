//! Configured local Cashier substitute; shares the existing receiver's accounting.
use blob_test_protocol::{
    funding::{FundingFailure, FundingReplyMode},
    storage::funding::transport::Substitute,
};
use candid::Principal;
use std::cell::RefCell;
thread_local! {
    // Deliberately ephemeral test controls. Restored receiver accounting is fenced.
    static CONTROL: RefCell<Option<Substitute>> = const { RefCell::new(None) };
}
pub(crate) fn configure(caller: Principal, input: Substitute) -> Result<(), FundingFailure> {
    if super::receipts(caller).is_none() {
        return Err(FundingFailure::Denied);
    }
    if input.expected_arguments.is_empty() || input.expected_arguments.len() > 1024 {
        return Err(FundingFailure::Limit);
    }
    CONTROL.set(Some(input));
    Ok(())
}
pub(crate) fn accept(caller: Principal, bytes: &[u8]) -> FundingReplyMode {
    CONTROL.with_borrow(|control| {
        let control = control.as_ref().expect("explicit local Cashier control");
        assert_eq!(
            bytes, control.expected_arguments,
            "exact canonical top-up arguments"
        );
        super::accept(caller, control.behavior);
        control.behavior.reply
    })
}
pub(crate) fn raw_arguments(bytes: Vec<u8>) -> Vec<u8> {
    assert!(bytes.len() <= 1024, "bounded top-up arguments");
    bytes
}
