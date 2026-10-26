import assert from 'node:assert/strict';
import { readFile, readdir } from 'node:fs/promises';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { join, extname } from 'node:path';
import { checkNodeInputs } from './check-node-tools.mjs';
await checkNodeInputs(fileURLToPath(new URL('../../tests/browser/', import.meta.url)));

const json = async path => JSON.parse(await readFile(new URL(path, import.meta.url), 'utf8'));
const client = await json('../../clients/browser/package.json');
const fixture = await json('../../tests/browser/package.json');
const lock = await json('../../tests/browser/package-lock.json');
const platformName = '@tokostudios/ic-platform';
const platform = await json('../../tests/browser/node_modules/@tokostudios/ic-platform/package.json');
const sdk = await json('../../tests/browser/node_modules/@icp-sdk/core/package.json');
assert.match(client.dependencies[platformName], /^\d+\.\d+\.\d+$/);
assert.equal(client.dependencies[platformName], fixture.devDependencies[platformName]);
assert.equal(platform.name, platformName);
assert.equal(platform.version, client.dependencies[platformName]);
assert.equal(lock.packages[`node_modules/${platformName}`].version, platform.version);
assert.equal(client.peerDependencies['@caffeineai/object-storage'], fixture.devDependencies['@caffeineai/object-storage']);
assert.equal(sdk.version, platform.dependencies['@icp-sdk/core']);
assert.equal(fixture.overrides['@caffeineai/object-storage']['@icp-sdk/core'], sdk.version);
// A matching root pin alone does not exclude a second or aliased SDK identity.
const sdkPackages = Object.entries(lock.packages).filter(([path, entry]) =>
  path.endsWith('node_modules/@icp-sdk/core') || entry.name === '@icp-sdk/core' ||
  entry.resolved?.includes('/@icp-sdk/core/'));
assert.equal(sdkPackages.length, 1, 'The browser graph must contain one SDK Core package');
assert.equal(sdkPackages[0][0], 'node_modules/@icp-sdk/core', 'SDK Core must not be aliased');
assert.equal(sdkPackages[0][1].version, sdk.version, 'Installed SDK Core differs from the lock');
const fixtureRequire = createRequire(new URL('../../tests/browser/package.json', import.meta.url));
const platformRequire = createRequire(new URL('../../tests/browser/node_modules/@tokostudios/ic-platform/package.json', import.meta.url));
const providerRequire = createRequire(new URL('../../tests/browser/node_modules/@caffeineai/object-storage/package.json', import.meta.url));
assert.equal(providerRequire.resolve('@icp-sdk/core/agent'), platformRequire.resolve('@icp-sdk/core/agent'),
  'Caffeine and Platform must resolve the same installed SDK Core Agent');
assert.equal(fixtureRequire.resolve('@icp-sdk/core/agent'), platformRequire.resolve('@icp-sdk/core/agent'),
  'The fixture must resolve the canonical SDK Core Agent');

// Product, build and test sources use Platform; provider/Platform transitive SDK
// imports and the reviewed Caffeine source patch keep their external ownership.
const directSdkImport = /(?:\bfrom\s+|\bimport\s*(?:\(\s*)?|\brequire\s*\(\s*)['"`]@icp-sdk\/core(?:\/|['"`])/;
async function checkSources(directory) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    if (entry.name === 'node_modules') continue;
    const path = join(directory, entry.name);
    if (entry.isDirectory()) { await checkSources(path); continue; }
    if (!entry.isFile()) continue;
    if (entry.name === 'package.json') {
      const manifest = JSON.parse(await readFile(path, 'utf8'));
      for (const field of ['dependencies', 'devDependencies', 'peerDependencies', 'optionalDependencies']) {
        for (const [name, version] of Object.entries(manifest[field] ?? {})) {
          assert(name !== '@icp-sdk/core' && !name.startsWith('@icp-sdk/core/') &&
            !/^npm:@icp-sdk\/core(?:@|$)/.test(String(version)),
            `${path}: direct SDK Core dependency (${field}/${name}); use Platform`);
        }
      }
      for (const [name, script] of Object.entries(manifest.scripts ?? {})) {
        assert(!directSdkImport.test(script), `${path}: direct SDK Core import in script ${name}; use Platform`);
      }
    } else if (['.js', '.mjs', '.cjs', '.ts', '.mts', '.cts', '.jsx', '.tsx'].includes(extname(path))) {
      const source = await readFile(path, 'utf8');
      assert(!directSdkImport.test(source),
        `${path}: direct SDK Core import; use Platform`);
    }
  }
}
for (const root of ['clients/browser', 'tests/browser', 'scripts']) {
  await checkSources(fileURLToPath(new URL(`../../${root}`, import.meta.url)));
}
