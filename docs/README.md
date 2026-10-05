<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Documentation

Start with the [project overview](../README.md) for a plain-language explanation
of what IC Blob Storage does and when it may be useful. The documents below are
organized by the job a reader is trying to do.

## Understand the service

| Document | Use it for |
| --- | --- |
| [Service contract](service-contract.md) | Current responsibilities, authority boundaries, lifecycle and recovery rules |
| [Service qualification gaps](service-gaps.md) | What has been demonstrated and what remains open |
| [Acceptance plan](acceptance-plan.md) | Evidence required before broader service acceptance |
| [Development plan](roadmap.md) | Current direction, milestones and consumer work |
| [Provider review](provider-review.md) | Reviewed Caffeine behavior and unresolved provider questions |

## Integrate and operate it

| Document | Use it for |
| --- | --- |
| [Dependency setup](dependencies.md) | Rust, browser, PocketIC and memory-composition setup |
| [Local tools](local-tools.md) | Preparing files and using local fixtures |
| [Operator and verifier guide](operator-guide.md) | Installation, publication, inspection, references, verification and recovery |
| [Standalone trial](standalone-trial.md) | Historical live-trial scope, retained obligations and current constraints |
| [Supported hosts](supported-hosts.md) | Host-platform expectations and prerequisites |

## Contribute and release

| Document | Use it for |
| --- | --- |
| [Development governance](governance/development.md) | Repository authority, validation and delivery rules |
| [Releasing](releasing.md) | Maintainer release workflow |
| [Shared tooling snapshots](consuming-snapshots.md) | Reviewing and refreshing vendored tooling |
| [Engineering principles](principles/README.md) | Shared design and review guidance |

## Current status and retained evidence

Start with [current status](status/current.md). The remaining documents preserve
source-bound evidence and historical decisions; they are not current operating
instructions unless the current status explicitly points to them.

| Document | Scope |
| --- | --- |
| [Current status](status/current.md) | Released baseline, active work and remaining product gaps |
| [Implementation history](status/history.md) | Superseded handoffs and their original evidence |
| [Caffeine probe ledger](evidence/caffeine-probes/README.md) | Local, substitute and deployed provider observations |
| [Core primitive evidence](evidence/core-primitives.md) | Source-bound implementation and validation history |
| [Shared tooling adoption](evidence/shared-tooling-adoption.md) | Reviewed tooling source and snapshot identity |

The [forum introduction](forum-introduction.md) is a historical announcement
from the `0.1.7` period and does not describe the current implementation.
