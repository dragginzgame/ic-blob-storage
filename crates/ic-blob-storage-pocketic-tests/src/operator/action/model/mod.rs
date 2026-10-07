use crate::operator::{
    Failure,
    model::{Binding, Command, Target, canister, decimal},
};
use blob_test_protocol::GatewaySyncRequest;
use blob_test_protocol::balance::{BalanceRefreshRequest, BalanceScope};
use std::collections::BTreeMap;

pub(super) struct Selection {
    pub target: Target,
    pub request: Request,
    pub dry_run: bool,
}

#[derive(Clone, Copy)]
pub(super) enum Action {
    Balance,
    Gateway,
}

pub(super) enum Request {
    Balance(BalanceRefreshRequest),
    Gateway(GatewaySyncRequest),
}

impl Selection {
    pub fn parse(args: &[String], action: Action) -> Result<Self, Failure> {
        let dry_run = match args.first().map(String::as_str) {
            Some("dry-run") => true,
            Some("refresh") if matches!(action, Action::Balance) => false,
            Some("sync") if matches!(action, Action::Gateway) => false,
            _ => return Err(Failure::Arguments),
        };
        let mut target = vec!["status".to_owned()];
        let mut extra = BTreeMap::new();
        for pair in args[1..].chunks(2) {
            if pair.len() != 2 {
                return Err(Failure::Arguments);
            }
            if matches!(
                pair[0].as_str(),
                "--source" | "--account" | "--revision" | "--sequence"
            ) {
                if extra.insert(pair[0].as_str(), pair[1].as_str()).is_some() {
                    return Err(Failure::Arguments);
                }
            } else {
                target.extend_from_slice(pair);
            }
        }
        let target = Command::parse(&target)?.target;
        let Binding::Authority(namespace) = target.binding else {
            return Err(Failure::Arguments);
        };
        let get = |key| extra.get(key).copied().ok_or(Failure::Arguments);
        let source = canister(get("--source")?)?;
        let revision = decimal(get("--revision")?)?;
        let sequence = decimal(get("--sequence")?)?;
        if namespace == 0 || source == target.canister || sequence == 0 {
            return Err(Failure::Arguments);
        }
        let request = match action {
            Action::Balance => {
                if revision == 0 {
                    return Err(Failure::Arguments);
                }
                Request::Balance(BalanceRefreshRequest {
                    scope: BalanceScope {
                        service: target.canister,
                        namespace,
                        source,
                        account: canister(get("--account")?)?,
                    },
                    revision,
                    sequence,
                })
            }
            Action::Gateway => {
                if extra.contains_key("--account") {
                    return Err(Failure::Arguments);
                }
                Request::Gateway(GatewaySyncRequest {
                    service: target.canister,
                    namespace,
                    source,
                    revision,
                    sequence,
                })
            }
        };
        Ok(Self {
            target,
            request,
            dry_run,
        })
    }
}
