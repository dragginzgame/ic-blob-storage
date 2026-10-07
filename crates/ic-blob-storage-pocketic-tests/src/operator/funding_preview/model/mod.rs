use crate::operator::{
    Failure,
    model::{Binding, Command, Target, decimal},
};
use blob_test_protocol::funding::preview::FundingPreviewRequest;
use std::collections::BTreeMap;

pub(super) struct Selection {
    pub target: Target,
    pub request: FundingPreviewRequest,
}
impl Selection {
    pub fn parse(args: &[String]) -> Result<Self, Failure> {
        if args.first().map(String::as_str) != Some("dry-run") {
            return Err(Failure::Arguments);
        }
        let mut target = vec!["status".to_owned()];
        let mut extra = BTreeMap::new();
        for pair in args[1..].chunks(2) {
            if pair.len() != 2 {
                return Err(Failure::Arguments);
            }
            if matches!(pair[0].as_str(), "--id" | "--amount" | "--revision") {
                if extra.insert(pair[0].as_str(), pair[1].as_str()).is_some() {
                    return Err(Failure::Arguments);
                }
            } else {
                target.extend_from_slice(pair);
            }
        }
        let target = Command::parse(&target)?.target;
        let Binding::Funding(peer) = target.binding else {
            return Err(Failure::Arguments);
        };
        let id = decimal(extra.get("--id").ok_or(Failure::Arguments)?)?;
        let requested_cycles = decimal(extra.get("--amount").ok_or(Failure::Arguments)?)?;
        let revision = decimal(extra.get("--revision").ok_or(Failure::Arguments)?)?;
        if requested_cycles == 0 || peer == target.canister {
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
