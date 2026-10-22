use crate::operator::{
    Failure,
    model::{Binding, Target, decimal, flags},
};
use blob_test_protocol::funding::{FundingRequest, lookup::FundingLookupRequest};

pub(super) struct Selection {
    pub target: Target,
    pub request: FundingLookupRequest,
}

impl Selection {
    pub fn parse(args: &[String]) -> Result<Self, Failure> {
        if args.first().map(String::as_str) != Some("lookup") {
            return Err(Failure::Arguments);
        }
        let mut flags = flags(&args[1..])?;
        let target = Target::parse(&mut flags)?;
        let Binding::Funding(peer) = target.binding else {
            return Err(Failure::Arguments);
        };
        let mut take = |key| flags.remove(key).ok_or(Failure::Arguments);
        let attempt = FundingRequest {
            id: decimal(take("--id")?)?,
            offered: decimal(take("--amount")?)?,
            accept: decimal(take("--accept")?)?,
            reply: serde_json::from_value(serde_json::Value::from(take("--reply")?))
                .map_err(|_| Failure::Arguments)?,
            trap_callback: take("--trap-callback")?
                .parse()
                .map_err(|_| Failure::Arguments)?,
        };
        if !flags.is_empty()
            || peer == target.transport.canister
            || attempt.offered == 0
            || attempt.accept > attempt.offered
        {
            return Err(Failure::Arguments);
        }
        Ok(Self {
            request: FundingLookupRequest {
                service: target.transport.canister.into(),
                peer: peer.into(),
                attempt,
            },
            target,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_testkit::fake_principal;

    fn args() -> Vec<String> {
        [
            "lookup".into(),
            "--server".into(),
            "127.0.0.1:1".into(),
            "--instance".into(),
            "0".into(),
            "--canister".into(),
            fake_principal(1).to_text(),
            "--caller".into(),
            fake_principal(3).to_text(),
            "--kind".into(),
            "funding".into(),
            "--peer".into(),
            fake_principal(2).to_text(),
            "--id".into(),
            u64::MAX.to_string(),
            "--amount".into(),
            u128::MAX.to_string(),
            "--accept".into(),
            "1".into(),
            "--reply".into(),
            "InternalError".into(),
            "--trap-callback".into(),
            "false".into(),
        ]
        .into()
    }

    #[test]
    fn exact_inputs_are_required_without_defaults_or_mutating_commands() {
        let original = args();
        let selected = Selection::parse(&original).unwrap();
        assert_eq!(selected.request.attempt.id, u64::MAX);
        assert_eq!(selected.request.attempt.offered, u128::MAX);
        for flag in ["--id", "--amount", "--accept", "--reply", "--trap-callback"] {
            let i = original.iter().position(|a| a == flag).unwrap();
            let mut absent = original.clone();
            absent.drain(i..=i + 1);
            assert_eq!(Selection::parse(&absent).err(), Some(Failure::Arguments));
            let mut duplicate = original.clone();
            duplicate.extend_from_slice(&original[i..=i + 1]);
            assert_eq!(Selection::parse(&duplicate).err(), Some(Failure::Arguments));
        }
        for (flag, value) in [
            ("--amount", "0"),
            ("--id", "01"),
            ("--accept", "-1"),
            ("--reply", "unknown"),
            ("--trap-callback", "1"),
            ("--server", "8.8.8.8:80"),
        ] {
            let mut changed = original.clone();
            let i = changed.iter().position(|a| a == flag).unwrap();
            changed[i + 1] = value.into();
            assert_eq!(Selection::parse(&changed).err(), Some(Failure::Arguments));
        }
        let mut changed = original;
        changed[0] = "fund".into();
        assert_eq!(Selection::parse(&changed).err(), Some(Failure::Arguments));
    }
}
