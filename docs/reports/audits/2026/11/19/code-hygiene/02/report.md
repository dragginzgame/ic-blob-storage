# Release preparation and CLI principal checks

This follow-up to the [first review](../01/report.md) examined the pending
0.106.1 working tree on main at `dd1a5ad`. Both Cargo workspaces already
selected Toko Tooling 0.45.15, the latest published version at review time.
The review used the [revision-bound code hygiene rules](https://github.com/dragginzgame/toko-tooling/blob/a716b501274678a8ebf163ef576bbb549e974acd/audits/code-hygiene.md).

| Finding | Change and regression coverage |
| --- | --- |
| Installed release preparation could run after a plan parse failure. | A valid stage followed by truncated JSON made the old script invoke the fixture Cargo and Make commands. Preparation now reads the entire plan before selecting work. The regression expects a parse failure and an empty command log. Existing selected and full release fixtures exercise valid plans. |
| Several CLI checks converted already-typed principals into bytes or text to compare identities. | The CLI compares Platform principals directly. Installation preparation and CLI text input share the check for anonymous and management principals. Decoded installation cases cover both reserved identities and a user matching the canister, and check that refusal leaves no custody record. |

All 40 release-selection assertions passed in disposable checkouts. Local canister
tests passed for application installation (2), management (4) and callbacks (7).
The callback suite skipped its opt-in Chromium case. These suites started before
the lockfiles picked up IC Host FS and Artifacts 0.80.5. The input guard detected
the change during the aggregate gate. A fresh run passed all seven CI stages and
the release metadata check passed with the updated locks. Local logs are under
`target/hygiene-04515-followup/`.
