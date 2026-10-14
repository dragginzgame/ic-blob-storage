import assert from 'node:assert/strict';
import { mkdtemp, writeFile, readFile, rm } from 'node:fs/promises';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { checkNodeInputs, checkNodeTools } from './check-node-tools.mjs';

const root = await mkdtemp(join(tmpdir(), 'shared-node-tools.'));
let complete = false;
try {
  const manifest = { packageManager: 'npm@12.2.0', dependencies: { prod: '1.0.0' }, devDependencies: { dev: '2.0.0' }, engines: { node: '>=20' } };
  const file = name => join(root, name);
  const json = (name, value) => writeFile(file(name), JSON.stringify(value));
  await writeFile(file('.nvmrc'), process.versions.node + '\n');
  await json('package.json', manifest);
  await json('package-lock.json', { packages: { '': manifest } });
  const npm = file('selected npm');
  await writeFile(npm, '#!/bin/sh\n[ "$1" = --version ] || exit 2\nprintf "12.2.0\\n"\n', { mode: 0o700 });
  const original = await readFile(file('package-lock.json'));
  assert.equal((await checkNodeTools(root, npm)).packageManager, 'npm@12.2.0');
  for (const field of ['dependencies', 'devDependencies', 'engines']) {
    await json('package-lock.json', { packages: { '': { ...manifest, [field]: {} } } });
    await assert.rejects(checkNodeInputs(root));
  }
  await writeFile(file('package-lock.json'), original);
  await writeFile(file('.nvmrc'), '0.0.0\n');
  await assert.rejects(checkNodeInputs(root));
  await writeFile(file('.nvmrc'), process.versions.node);
  await writeFile(npm, '#!/bin/sh\nprintf "0.0.0\\n"\n');
  await assert.rejects(checkNodeTools(root, npm));
  await writeFile(npm, '#!/bin/sh\nexit 23\n');
  await assert.rejects(checkNodeTools(root, npm));
  await assert.rejects(checkNodeTools(root, file('missing npm')));
  assert.deepEqual(await readFile(file('package-lock.json')), original);
  complete = true;
  console.log('Node/npm input and executable checks passed; no setup or lock mutation.');
} finally {
  if (complete) await rm(root, { recursive: true });
  else console.error(`Failed Node tool fixture retained: ${root}`);
}
