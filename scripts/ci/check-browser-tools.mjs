import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { checkNodeInputs } from './check-node-tools.mjs';
await checkNodeInputs(fileURLToPath(new URL('../../tests/browser/', import.meta.url)));

const json = async path => JSON.parse(await readFile(new URL(path, import.meta.url), 'utf8'));
const client = await json('../../clients/browser/package.json');
const fixture = await json('../../tests/browser/package.json');
const lock = await json('../../tests/browser/package-lock.json');
const sdk = await json('../../tests/browser/node_modules/@icp-sdk/core/package.json');
assert.equal(client.peerDependencies['@icp-sdk/core'], fixture.devDependencies['@icp-sdk/core']);
assert.equal(client.peerDependencies['@caffeineai/object-storage'], fixture.devDependencies['@caffeineai/object-storage']);
assert.equal(sdk.version, client.peerDependencies['@icp-sdk/core']);
// A matching root pin alone does not exclude a second or aliased SDK identity.
const sdkPackages = Object.entries(lock.packages).filter(([path, entry]) =>
  path.endsWith('node_modules/@icp-sdk/core') || entry.name === '@icp-sdk/core' ||
  entry.resolved?.includes('/@icp-sdk/core/'));
assert.equal(sdkPackages.length, 1, 'The browser graph must contain one SDK Core package');
assert.equal(sdkPackages[0][0], 'node_modules/@icp-sdk/core', 'SDK Core must not be aliased');
assert.equal(sdkPackages[0][1].version, sdk.version, 'Installed SDK Core differs from the lock');
const fixtureRequire = createRequire(new URL('../../tests/browser/package.json', import.meta.url));
const providerRequire = createRequire(new URL('../../tests/browser/node_modules/@caffeineai/object-storage/package.json', import.meta.url));
assert.equal(providerRequire.resolve('@icp-sdk/core/agent'), fixtureRequire.resolve('@icp-sdk/core/agent'),
  'Caffeine and Blob must resolve the same installed SDK Core Agent');
