import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

// The private browser build root owns its exact preparation tools and lock.
const root = new URL('../../tests/browser/', import.meta.url);
const nodeVersion = (await readFile(new URL('.nvmrc', root), 'utf8')).trim();
const manifest = JSON.parse(await readFile(new URL('package.json', root), 'utf8'));
const lock = JSON.parse(await readFile(new URL('package-lock.json', root), 'utf8'));
assert.match(nodeVersion, /^\d+\.\d+\.\d+$/);
assert.equal(process.versions.node, nodeVersion, `Select Node ${nodeVersion} from tests/browser/.nvmrc`);
assert.match(manifest.packageManager, /^npm@\d+\.\d+\.\d+$/);
assert.deepEqual(lock.packages[''].devDependencies, manifest.devDependencies, 'Prepare the matching browser lock with npm ci');
assert.deepEqual(lock.packages[''].engines, manifest.engines, 'The browser lock must match the supported Node range');
console.log(`Browser inputs verified: Node ${nodeVersion}, matching manifest/lock declarations.`);
