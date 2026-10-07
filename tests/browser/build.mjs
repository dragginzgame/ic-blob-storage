import '../../scripts/ci/check-browser-tools.mjs';
import { build } from 'esbuild';
import { fileURLToPath } from 'node:url';
import { readFile, cp, mkdir } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import assert from 'node:assert/strict';

const repo = fileURLToPath(new URL('../..', import.meta.url));
const json = async path => JSON.parse(await readFile(new URL(path, import.meta.url), 'utf8'));
const client = await json('../../clients/browser/package.json');
const fixture = await json('package.json');
const sdk = await json('node_modules/@icp-sdk/core/package.json');
assert.equal(client.peerDependencies['@icp-sdk/core'], fixture.devDependencies['@icp-sdk/core']);
assert.equal(client.peerDependencies['@caffeineai/object-storage'], fixture.devDependencies['@caffeineai/object-storage']);
assert.equal(sdk.version, client.peerDependencies['@icp-sdk/core']);
const evidence = await json('../../docs/evidence/caffeine-browser-reuse.json');
const upstream = fileURLToPath(new URL('node_modules/@caffeineai/object-storage', import.meta.url));
const pkg = await json('node_modules/@caffeineai/object-storage/package.json');
assert.equal(pkg.version, fixture.devDependencies['@caffeineai/object-storage']);
assert.equal(pkg.version, evidence.version);
for (const [path, hash] of Object.entries(evidence.files)) {
  assert.equal(createHash('sha256').update(await readFile(`${upstream}/${path}`)).digest('hex'), hash);
}
const adapted = `${repo}/.tmp/browser/caffeine`;
await mkdir(adapted, { recursive: true });
await cp(`${upstream}/dist`, `${adapted}/dist`, { recursive: true });
await cp(`${upstream}/LICENSE`, `${adapted}/LICENSE`);
const patch = 'clients/browser/patches/caffeine-1.1.2.patch';
await promisify(execFile)('git', ['apply', '--check', '--directory=.tmp/browser/caffeine', patch], { cwd: repo });
await promisify(execFile)('git', ['apply', '--directory=.tmp/browser/caffeine', patch], { cwd: repo });
await build({ absWorkingDir: repo, entryPoints: ['tests/browser/client.js'], bundle: true,
  format: 'esm', outfile: '.tmp/browser/client.js',
  alias: { '@caffeineai/object-storage': `${adapted}/dist/index.js` },
  nodePaths: [fileURLToPath(new URL('node_modules', import.meta.url))] });
await build({ absWorkingDir: repo, entryPoints: ['tests/browser/store.js'], bundle: true,
  format: 'esm', outfile: '.tmp/browser/store.js',
  nodePaths: [fileURLToPath(new URL('node_modules', import.meta.url))] });
await build({ absWorkingDir: repo, entryPoints: ['tests/browser/standalone.js'], bundle: true,
  format: 'esm', outfile: '.tmp/browser/standalone.js',
  alias: { '@caffeineai/object-storage': `${adapted}/dist/index.js` },
  nodePaths: [fileURLToPath(new URL('node_modules', import.meta.url))] });
await build({ absWorkingDir: repo, entryPoints: ['clients/browser/worker-entry.js'], bundle: true,
  format: 'esm', outfile: '.tmp/browser/publication-worker.js',
  alias: { '@caffeineai/object-storage': `${adapted}/dist/index.js` },
  nodePaths: [fileURLToPath(new URL('node_modules', import.meta.url))] });
await build({ absWorkingDir: repo, entryPoints: ['clients/browser/bootstrap.js'], bundle: true,
  format: 'esm', outfile: '.tmp/browser/publication-host.js',
  alias: { '@caffeineai/object-storage': `${adapted}/dist/index.js` },
  nodePaths: [fileURLToPath(new URL('node_modules', import.meta.url))] });
await build({ absWorkingDir: repo, entryPoints: ['tests/browser/bootstrap.js'], bundle: true,
  format: 'esm', outfile: '.tmp/browser/bootstrap.js',
  alias: { '@caffeineai/object-storage': `${adapted}/dist/index.js` },
  nodePaths: [fileURLToPath(new URL('node_modules', import.meta.url))] });
for (const name of ['sdk-probe', 'native-inputs', 'publication', 'worker', 'launcher-serial']) await build({ absWorkingDir: repo,
  entryPoints: [`tests/browser/${name}.mjs`], bundle: true,
  format: 'esm', platform: 'node', outfile: `.tmp/browser/${name}.mjs`,
  external: ['/host.js'], // Executed inside Chromium, not a native module import.
  banner: { js: "import { createRequire } from 'node:module'; const require = createRequire(import.meta.url);" },
  alias: { '@caffeineai/object-storage': `${adapted}/dist/index.js` },
  nodePaths: [fileURLToPath(new URL('node_modules', import.meta.url))] });
