use crate::operator::{
    Failure,
    model::{Binding, Target, decimal, flags},
};
use blob_test_protocol::funding::preview::FundingPreviewRequest;

pub(super) struct Selection {
    pub target: Target,
    pub request: FundingPreviewRequest,
}
impl Selection {
    pub fn parse(args: &[String]) -> Result<Self, Failure> {
        if args.first().map(String::as_str) != Some("dry-run") {
            return Err(Failure::Arguments);
        }
        let mut flags = flags(&args[1..])?;
        let target = Target::parse(&mut flags)?;
        let Binding::Funding(peer) = target.binding else {
            return Err(Failure::Arguments);
        };
        let mut take = |key| flags.remove(key).ok_or(Failure::Arguments);
        let id = decimal(take("--id")?)?;
        let requested_cycles = decimal(take("--amount")?)?;
        let revision = decimal(take("--revision")?)?;
        if !flags.is_empty() || requested_cycles == 0 || peer == target.canister {
            return Err(Failure::Arguments);
        }
        Ok(Self {
            request: FundingPreviewRequest {
                service: target.canister,
                peer,
                id,
                requested_cycles,
                revision,
            },
            target,
        })
    }
}
