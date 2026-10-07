use crate::operator::{
    Failure,
    model::{canister, decimal},
    ops::QueryTarget,
};
use ic_blob_storage::dto::tenant::TenantScope;
use std::{collections::BTreeMap, net::SocketAddr, path::PathBuf};

pub(super) struct Selection {
    pub target: QueryTarget,
    pub scope: TenantScope,
    pub path: PathBuf,
}

impl Selection {
    pub fn parse(args: &[String]) -> Result<Self, Failure> {
        if args.first().map(String::as_str) != Some("inspect") {
            return Err(Failure::Arguments);
        }
        let mut flags = BTreeMap::new();
        for pair in args[1..].chunks(2) {
            if pair.len() != 2 || flags.insert(pair[0].as_str(), pair[1].as_str()).is_some() {
                return Err(Failure::Arguments);
            }
        }
        let mut take = |name| flags.remove(name).ok_or(Failure::Arguments);
        let server: SocketAddr = take("--server")?.parse().map_err(|_| Failure::Arguments)?;
        if !server.ip().is_loopback() || server.port() == 0 {
            return Err(Failure::Arguments);
        }
        let instance = decimal(take("--instance")?)?;
        let service = canister(take("--canister")?)?;
        let caller = canister(take("--caller")?)?;
        let tenant = canister(take("--tenant")?)?;
        let namespace = decimal(take("--namespace")?)?;
        let path = PathBuf::from(take("--inventory")?);
        if caller != tenant || namespace == 0 || path.as_os_str().is_empty() || !flags.is_empty() {
            return Err(Failure::Arguments);
        }
        Ok(Self {
            target: QueryTarget {
                server,
                instance,
                canister: service,
                caller,
            },
            scope: TenantScope {
                service,
                tenant,
                namespace,
            },
            path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn targets_are_explicit_local_and_tenant_bound() {
        let service = Principal::from_slice(&[1, 1]).to_text();
        let tenant = Principal::from_slice(&[2, 1]).to_text();
        let base = [
            "inspect",
            "--server",
            "127.0.0.1:12345",
            "--instance",
            "0",
            "--canister",
            &service,
            "--caller",
            &tenant,
            "--tenant",
            &tenant,
            "--namespace",
            "1",
            "--inventory",
            "input.json",
        ]
        .map(str::to_owned)
        .to_vec();
        assert!(Selection::parse(&base).is_ok());
        for (index, value) in [
            (0, "upload"),
            (2, "example.com:12345"),
            (2, "192.0.2.1:12345"),
            (2, "127.0.0.1:0"),
            (4, "01"),
            (8, &service),
            (12, "0"),
            (14, ""),
        ] {
            let mut changed = base.clone();
            changed[index] = value.into();
            assert!(matches!(
                Selection::parse(&changed),
                Err(Failure::Arguments)
            ));
        }
        let mut extra = base.clone();
        extra.extend(["--namespace".into(), "1".into()]);
        assert!(matches!(Selection::parse(&extra), Err(Failure::Arguments)));
        let mut missing = base;
        missing.pop();
        assert!(matches!(
            Selection::parse(&missing),
            Err(Failure::Arguments)
        ));
    }
}
