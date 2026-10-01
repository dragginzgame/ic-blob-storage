//! Native signed operator reads through the existing managed account workflow.
use super::Journey;
use crate::{
    account_native_cli::{NativeAccountCli, signer},
    canic_managed::cli::{live, local_subnet_key},
};
use std::time::Duration;

#[test]
fn managed_native_account_observations_keep_scope_reports_and_restore_fences() {
    let j = Journey::with_operator(signer());
    let root = local_subnet_key(&j.f);
    let (mut gateway, url) = live(&j.f);
    let cli = NativeAccountCli::new(j.scope, &url, root, "managed");
    let before = j.f.pic().get_stable_memory(j.f.app());
    cli.reports(|kind, bytes| j.account_reply(kind, bytes));
    cli.refusals(|kind, bytes| j.account_reply(kind, bytes));
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
    j.f.pic().stop_progress();
    j.f.upgrade_same_release(Duration::from_secs(5));
    let before = j.f.pic().get_stable_memory(j.f.app());
    let source = j.f.pic().get_stable_memory(j.scope.cashier);
    j.f.pic().auto_progress();
    cli.fenced();
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
    assert_eq!(j.f.pic().get_stable_memory(j.scope.cashier), source);
    gateway.stop_live();
}
