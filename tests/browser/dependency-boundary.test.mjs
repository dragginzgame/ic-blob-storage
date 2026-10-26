import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtemp, mkdir, readFile, writeFile, rm, cp } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';

const sdk = '@icp-sdk/core';
const platform = '@tokostudios/ic-platform';
const repo = new URL('../../', import.meta.url);

// Minimal installed packages exercise dependency admission, not SDK behavior.
async function fixture(root) {
  for (const path of ['scripts/ci', 'clients/browser', 'tests/browser',
    `tests/browser/node_modules/${sdk}`, `tests/browser/node_modules/${platform}`,
    'tests/browser/node_modules/@caffeineai/object-storage']) {
    await mkdir(join(root, path), { recursive: true });
  }
  for (const path of ['scripts/ci/check-browser-tools.mjs', 'scripts/ci/check-node-tools.mjs',
    'clients/browser/package.json', 'tests/browser/package.json', 'tests/browser/package-lock.json',
    'tests/browser/.nvmrc']) {
    await writeFile(join(root, path), await readFile(new URL(path, repo)));
  }
  for (const name of [sdk, platform, '@caffeineai/object-storage']) {
    const path = `tests/browser/node_modules/${name}/package.json`;
    await writeFile(join(root, path), await readFile(new URL(path, repo)));
  }
  for (const module of ['esm', 'cjs']) {
    await mkdir(join(root, `tests/browser/node_modules/${sdk}/lib/${module}/agent`), { recursive: true });
    await writeFile(join(root, `tests/browser/node_modules/${sdk}/lib/${module}/agent/index.js`), '');
  }
}

async function changeJson(root, path, change) {
  const value = JSON.parse(await readFile(join(root, path), 'utf8'));
  change(value);
  await writeFile(join(root, path), JSON.stringify(value));
}

test('browser dependency boundary admits Platform and refuses SDK escape paths', async t => {
  const cases = [
    ['canonical graph', async () => {}, null],
    ...['dependencies', 'devDependencies', 'peerDependencies', 'optionalDependencies'].map(field =>
      [field, root => changeJson(root, 'clients/browser/package.json', value => {
        value[field] = { ...value[field], [sdk]: '6.1.1' };
      }), 'direct SDK Core dependency']),
    ['dependency alias', root => changeJson(root, 'clients/browser/package.json', value => {
      value.dependencies['renamed-sdk'] = `npm:${sdk}@6.1.1`;
    }), 'direct SDK Core dependency'],
    ['product import', root => writeFile(join(root, 'clients/browser/escape.js'),
      `import { Principal } from '${sdk}/principal';`), 'direct SDK Core import'],
    ['test dynamic import', root => writeFile(join(root, 'tests/browser/escape.mjs'),
      `await import('${sdk}/agent');`), 'direct SDK Core import'],
    ['build require', root => writeFile(join(root, 'scripts/ci/escape.cjs'),
      `require('${sdk}/agent');`), 'direct SDK Core import'],
    ['npm script import', root => changeJson(root, 'clients/browser/package.json', value => {
      value.scripts = { escape: `node -e "require('${sdk}/agent')"` };
    }), 'direct SDK Core import in script'],
    ['re-export', root => writeFile(join(root, 'clients/browser/escape.js'),
      `export { Principal } from '${sdk}/principal';`), 'direct SDK Core import'],
    ...['nested', 'aliased'].map(kind => [kind, root => changeJson(root, 'tests/browser/package-lock.json', value => {
      const original = value.packages[`node_modules/${sdk}`];
      const path = kind === 'nested' ? `node_modules/provider/node_modules/${sdk}` : 'node_modules/renamed-sdk';
      value.packages[path] = { ...original, name: sdk };
    }), 'one SDK Core package']),
    ['installed shadow', root => cp(join(root, `tests/browser/node_modules/${sdk}`),
      join(root, `tests/browser/node_modules/@caffeineai/object-storage/node_modules/${sdk}`),
      { recursive: true }), 'same installed SDK Core Agent'],
  ];
  for (const [name, change, expected] of cases) {
    await t.test(name, async () => {
      const root = await mkdtemp(join(tmpdir(), 'blob-browser-boundary-'));
      try {
        await fixture(root);
        await change(root);
        const result = spawnSync(process.execPath, [join(root, 'scripts/ci/check-browser-tools.mjs')],
          { encoding: 'utf8', timeout: 30_000, maxBuffer: 1024 * 1024 });
        assert.ifError(result.error);
        if (expected) {
          assert.notEqual(result.status, 0, name);
          assert(result.stderr.includes(expected), result.stderr);
        } else assert.equal(result.status, 0, result.stderr);
      } finally { await rm(root, { recursive: true, force: true }); }
    });
  }
});
