# Node/npm prerequisite checks

Opt-in Node consumers can adopt `scripts/ci/check-node-tools.mjs` as a snapshot
entrypoint. Select the Node executable before invoking it:

```sh
/path/to/node scripts/ci/check-node-tools.mjs /path/to/package /path/to/npm
```

The package root owns `.nvmrc`, `package.json` and `package-lock.json`. The checker
requires an exact Node version and `npm@X.Y.Z` package-manager declaration, then
compares the lock root's production dependencies, development dependencies and
engine declarations. It executes only the selected npm's `--version`, with a
30-second timeout and 4 KiB output bound. It performs no installation, network
request or lockfile mutation and does not claim a resolved dependency audit.

The npm executable is optional and defaults to `npm` on the caller's PATH.
Paths are individual arguments, including paths with spaces. Package roots and
executables are trusted caller selections; this is not a sandbox.

Build scripts may import `checkNodeInputs(packageRoot)` for Node and declaration
admission without spawning npm, or `checkNodeTools(packageRoot, npmExecutable)`
for the complete check. Both return the admitted root and version selections.
Importing the module alone has no checking or execution side effect.

The standalone fixture runs with `node scripts/ci/test-node-tools.mjs`.
The portable suite includes it when Node is available and reports a skip otherwise;
Node remains an opt-in consumer prerequisite, not a portable-shell prerequisite.
Actual browser engines, SDK preparation, compiler inputs and product tests stay
with consumers. Adopt this checker from a reviewed committed snapshot before
removing a local equivalent.
