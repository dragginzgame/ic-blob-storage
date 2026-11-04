# Release notes

All thirteen packages remain in one workspace and lockfile. Each release group
owns its versions and ledger; there is no aggregate workspace version.

| Group | Packages | Notes |
| --- | --- | --- |
| Contracts | Published protocol and passive contracts | [Contracts](releases/contracts/CHANGELOG.md) |
| Client | Published client and native CLI | [Client](releases/client/CHANGELOG.md) |
| Engine | Service core, standalone host and private probes | [Engine](releases/engine/CHANGELOG.md) |

Pending **0.70.0** notes are maintained in the three group ledgers above.

[Coordinated history through 0.65.0](releases/history.md) is retained once.
Select `RELEASE_GROUP=contracts`, `client` or `engine` explicitly; see
[the release guide](docs/releasing.md). Splitting groups does not create tags,
republish existing versions or qualify a deployed service.
