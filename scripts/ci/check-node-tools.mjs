import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { resolve, join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { execFileSync } from 'node:child_process';

// Read-only prerequisite admission. Consumers own pins, package roots and setup.
export async function checkNodeInputs(packageRoot) {
  const root = resolve(packageRoot);
  const nodeVersion = (await readFile(join(root, '.nvmrc'), 'utf8')).trim();
  const manifest = JSON.parse(await readFile(join(root, 'package.json'), 'utf8'));
  const lock = JSON.parse(await readFile(join(root, 'package-lock.json'), 'utf8'));
  assert.match(nodeVersion, /^\d+\.\d+\.\d+$/);
  assert.equal(process.versions.node, nodeVersion, `Select Node ${nodeVersion} from ${root}/.nvmrc`);
  assert.match(manifest.packageManager, /^npm@\d+\.\d+\.\d+$/);
  assert.deepEqual(lock.packages[''].dependencies, manifest.dependencies, 'Root production dependencies differ from the lock');
  assert.deepEqual(lock.packages[''].devDependencies, manifest.devDependencies, 'Root development dependencies differ from the lock');
  assert.deepEqual(lock.packages[''].engines, manifest.engines, 'Root engine declarations differ from the lock');
  return { root, nodeVersion, packageManager: manifest.packageManager };
}

export async function checkNodeTools(packageRoot, npm = 'npm') {
  const selection = await checkNodeInputs(packageRoot);
  const actual = execFileSync(npm, ['--version'], {
    encoding: 'utf8', timeout: 30_000, maxBuffer: 4096, stdio: ['ignore', 'pipe', 'pipe'],
  }).trim();
  assert.equal(`npm@${actual}`, selection.packageManager, `Select ${selection.packageManager} from ${selection.root}/package.json`);
  return selection;
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  assert(process.argv.length === 3 || process.argv.length === 4, 'usage: check-node-tools.mjs PACKAGE_ROOT [NPM_EXECUTABLE]');
  const selected = await checkNodeTools(process.argv[2], process.argv[3]);
  console.log(`Node/npm inputs verified: Node ${selected.nodeVersion}, ${selected.packageManager}, matching root manifest/lock declarations.`);
}
