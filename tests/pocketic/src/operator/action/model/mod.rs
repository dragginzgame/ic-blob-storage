use crate::operator::{
    Failure,
    model::{Binding, Target, canister, decimal, flags},
};
use blob_test_protocol::GatewaySyncRequest;
use blob_test_protocol::balance::{BalanceRefreshRequest, BalanceScope};

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
        let mut flags = flags(&args[1..])?;
        let target = Target::parse(&mut flags)?;
        let Binding::Authority(namespace) = target.binding else {
            return Err(Failure::Arguments);
        };
        let mut take = |key| flags.remove(key).ok_or(Failure::Arguments);
        let source = canister(take("--source")?)?;
        let revision = decimal(take("--revision")?)?;
        let sequence = decimal(take("--sequence")?)?;
        if namespace == 0 || source == target.transport.canister || sequence == 0 {
            return Err(Failure::Arguments);
        }
        let request = match action {
            Action::Balance => {
                if revision == 0 {
                    return Err(Failure::Arguments);
                }
                Request::Balance(BalanceRefreshRequest {
                    scope: BalanceScope {
                        service: target.transport.canister.into(),
                        namespace,
                        source: source.into(),
                        account: canister(take("--account")?)?.into(),
                    },
                    revision,
                    sequence,
                })
            }
            Action::Gateway => Request::Gateway(GatewaySyncRequest {
                service: target.transport.canister.into(),
                namespace,
                source: source.into(),
                revision,
                sequence,
            }),
        };
        if !flags.is_empty() {
            return Err(Failure::Arguments);
        }
        Ok(Self {
            target,
            request,
            dry_run,
        })
    }
}
