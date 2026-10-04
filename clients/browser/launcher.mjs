import { constants } from 'node:fs';
import { open, lstat, mkdir, realpath } from 'node:fs/promises';
import { createServer } from 'node:http';
import { createHash } from 'node:crypto';
import { isAbsolute, join, dirname, basename } from 'node:path';
import { isDeepStrictEqual } from 'node:util';
import { driveSession } from './session.mjs';

/** Finite process-bridge refusal; never includes keys, paths or remote errors. */
export class LauncherRefusal extends Error {
  constructor(code) { super(code); this.name = 'LauncherRefusal'; this.code = code; }
}
const require = (condition, code) => { if (!condition) throw new LauncherRefusal(code); };
const exact = (value, keys, optional = []) => require(value &&
  Object.getPrototypeOf(value) === Object.prototype && keys.every(key => Object.hasOwn(value, key)) &&
  Object.keys(value).every(key => [...keys, ...optional].includes(key)), 'configuration');
const integer = (value, maximum) => Number.isSafeInteger(value) && value > 0 && value <= maximum;
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const nativeIntentFields = ['format', 'source_session', 'browser', 'operation', 'network', 'service_url',
  'service', 'namespace', 'tenant', 'uploader', 'operator', 'verifier', 'gateway',
  'inventory_sha256', 'installation_sha256', 'root_key_sha256', 'files', 'max_steps',
  'max_service_updates', 'max_service_queries', 'timeout_seconds', 'max_provider_requests',
  'automatic_retries', 'input_verification'];

function nativeReady(ready, intent) {
  exact(ready, ['schema', 'operation', 'event', 'files', 'inventory_sha256', 'installation_sha256',
    'max_steps', 'provider_requests', 'input_verification', 'retry_authorized', 'publication_lease']);
  require(ready.schema === 1 && ready.operation === 'publish_session' && ready.event === 'ready' &&
    ready.retry_authorized === false && ready.publication_lease === false &&
    ready.provider_requests === 0 && ready.input_verification === 'session_start' &&
    integer(ready.files, 4096) && ready.files === intent.files &&
    integer(ready.max_steps, 8 * ready.files + 1) &&
    ready.inventory_sha256 === intent.inventory_sha256 &&
    ready.installation_sha256 === intent.installation_sha256, 'native-session');
}

// Open once, reject special files before reading, and never allocate past the
// caller's ceiling. A selected body is checked again in the browser before intent.
async function boundedFile(path, maximum, expected) {
  require(typeof path === 'string' && isAbsolute(path), 'file');
  let file;
  try {
    file = await open(path, constants.O_RDONLY | constants.O_NOFOLLOW | constants.O_NONBLOCK);
    const stat = await file.stat();
    require(stat.isFile() && integer(stat.size, maximum) &&
      (expected === undefined || stat.size === expected), 'body-size');
    const bytes = Buffer.alloc(stat.size);
    let offset = 0;
    while (offset < bytes.length) {
      const read = await file.read(bytes, offset, Math.min(65536, bytes.length - offset), null);
      require(read.bytesRead > 0, 'body-size'); offset += read.bytesRead;
    }
    require((await file.read(Buffer.alloc(1), 0, 1, null)).bytesRead === 0, 'body-size');
    return bytes;
  } catch (error) {
    if (error instanceof LauncherRefusal) throw error;
    throw new LauncherRefusal('file');
  } finally { await file?.close(); }
}

async function snapshot(transfer, maximum) {
  exact(transfer, ['binding', 'body', 'body_sha256', 'manifest_json', 'bytes', 'preparation']);
  require(typeof transfer.bytes === 'string' && /^[1-9]\d{0,9}$/.test(transfer.bytes), 'body-size');
  const size = Number(transfer.bytes);
  require(integer(size, maximum) && typeof transfer.body_sha256 === 'string' &&
    /^[0-9a-f]{64}$/.test(transfer.body_sha256), 'body-size');
  require(typeof transfer.manifest_json === 'string' &&
    Buffer.byteLength(transfer.manifest_json) <= 256 * 1024, 'manifest-size');
  exact(transfer.preparation, [], ['content_type', 'filename', 'cache_control']);
  for (const value of Object.values(transfer.preparation)) require(typeof value === 'string' &&
    Buffer.byteLength(value) <= 4096, 'metadata-hint');
  const bytes = await boundedFile(transfer.body, maximum, size);
  require(createHash('sha256').update(bytes).digest('hex') === transfer.body_sha256, 'body-digest');
  const hints = transfer.preparation;
  return { bytes, job: { binding: structuredClone(transfer.binding), snapshot: {
    bodySha256: transfer.body_sha256, manifestJSON: transfer.manifest_json,
    ...(Object.hasOwn(hints, 'content_type') ? { contentType: hints.content_type } : {}),
    ...(Object.hasOwn(hints, 'filename') ? { filename: hints.filename } : {}),
    ...(Object.hasOwn(hints, 'cache_control') ? { cacheControl: hints.cache_control } : {}) } } };
}

// Passive launch binding, not an effect journal. Write before Chromium can open
// IndexedDB; a partial first launch stays closed until its original record exists.
// Runtime budgets can change, but identity, trust, scope and executable assets cannot.
async function nativeSelection(options, bootstrap, hostBundle, workerBundle) {
  if (options.nativeSession === null) return null;
  try {
    require(typeof options.nativeSession === 'string' && isAbsolute(options.nativeSession), 'native-session');
    const metadata = await lstat(options.nativeSession);
    require(metadata.isDirectory() && !metadata.isSymbolicLink(), 'native-session');
    const session = await realpath(options.nativeSession);
    const bytes = await boundedFile(join(session, 'intent.json'), 16384);
    const intent = JSON.parse(bytes), selected = intent.browser, config = bootstrap.configuration;
    exact(intent, nativeIntentFields);
    require(intent.format === 'ic-blob-storage/publication-session:retained-browser-handoffs' &&
      intent.operation === 'publish_session' && intent.source_session === null, 'native-session');
    exact(selected, ['format', 'session', 'profile', 'project', 'bucket', 'asset_port',
      'signer_sha256', 'host_sha256', 'worker_sha256', 'database', 'max_slots']);
    require(selected.format === 'ic-blob-storage/browser-selection' && selected.session === session &&
      selected.profile === options.profile && selected.asset_port === options.assetPort &&
      selected.signer_sha256 === digest(JSON.stringify(bootstrap.signer)) &&
      selected.host_sha256 === digest(hostBundle) && selected.worker_sha256 === digest(workerBundle) &&
      selected.database === bootstrap.journal.database && selected.max_slots === bootstrap.journal.maxSlots &&
      selected.project === config.project && selected.bucket === config.bucket, 'native-session');
    require(['service', 'tenant', 'uploader'].every(key => intent[key] === config[key]) &&
      new URL(intent.service_url).origin === new URL(config.host).origin &&
      new URL(intent.gateway).origin === new URL(config.origin).origin &&
      intent.root_key_sha256 === digest(config.rootKey), 'native-session');
    const ready = JSON.parse(await boundedFile(join(session, 'ready.json'), 16384));
    nativeReady(ready, intent);
    require(ready.max_steps === intent.max_steps &&
      digest(await boundedFile(join(session, 'inventory.json'), 2 * 1024 * 1024)) === intent.inventory_sha256 &&
      digest(await boundedFile(join(session, 'installation.candid'), 16384)) === intent.installation_sha256,
    'native-session');
    return { binding: { session, intent_sha256: digest(bytes) }, intent };
  } catch { throw new LauncherRefusal('native-session'); }
}

async function bindProfile(profile, origin, bootstrap, hostBundle, workerBundle, native) {
  const configuration = bootstrap.configuration;
  const record = Buffer.from(JSON.stringify({
    format: 'ic-blob-storage/browser-profile:native-session-selection',
    native_session: native?.binding ?? null,
    profile: await realpath(profile), origin,
    signer: { kind: bootstrap.signer?.kind, sha256: digest(JSON.stringify(bootstrap.signer)) },
    configuration: Object.fromEntries(['host', 'service', 'tenant', 'uploader',
      'project', 'bucket', 'origin'].map(key => [key, configuration[key]])),
    root_sha256: digest(configuration.rootKey),
    journal: { database: bootstrap.journal.database, maxSlots: bootstrap.journal.maxSlots },
    host_sha256: digest(hostBundle), worker_sha256: digest(workerBundle),
  }) + '\n');
  const path = join(profile, 'publication-binding.json');
  try {
    require(record.length <= 16384, 'profile-binding');
    if (bootstrap.journal.mode === 'create') {
      const file = await open(path, constants.O_WRONLY | constants.O_CREAT |
        constants.O_EXCL | constants.O_NOFOLLOW, 0o600);
      try { await file.writeFile(record); await file.sync(); } finally { await file.close(); }
      const directory = await open(profile, constants.O_RDONLY | constants.O_DIRECTORY);
      try { await directory.sync(); } finally { await directory.close(); }
    } else {
      require((await boundedFile(path, 16384)).equals(record), 'profile-binding');
    }
  } catch { throw new LauncherRefusal('profile-binding'); }
}

// Native controls paid handoff selection; IndexedDB owns the actual SDK claim.
// Recovery reads the original claim and can only request certificate observation.
async function nativeTransfer(native, request) {
  try {
    exact(request, ['id', 'index', 'action', 'nativePhase']);
    require(request.action === 'transfer' && native && typeof request.nativePhase === 'string' &&
      isAbsolute(request.nativePhase), 'native-phase');
    const metadata = await lstat(request.nativePhase);
    require(metadata.isDirectory() && !metadata.isSymbolicLink(), 'native-phase');
    const phase = await realpath(request.nativePhase), directory = dirname(phase);
    const intentBytes = await boundedFile(join(directory, 'intent.json'), 16384);
    const intent = JSON.parse(intentBytes);
    exact(intent, nativeIntentFields);
    const immutable = value => {
      const { source_session, max_steps, max_service_queries, timeout_seconds, ...binding } = value;
      return binding;
    };
    require(isDeepStrictEqual(immutable(intent), immutable(native.intent)) &&
      (directory === native.binding.session ? digest(intentBytes) === native.binding.intent_sha256 :
        typeof intent.source_session === 'string') &&
      Number.isSafeInteger(request.index) && request.index >= 0 && request.index < intent.files,
    'native-phase');
    const ordinal = Number(basename(phase).slice(5));
    require(basename(phase) === `step-${String(ordinal).padStart(4, '0')}` &&
      Number.isSafeInteger(ordinal) && ordinal >= 0 && ordinal < intent.max_steps, 'native-phase');
    const frame = JSON.parse(await boundedFile(join(phase, 'request.json'), 8192));
    exact(frame, ['phase', 'index', 'source_transfer']);
    require(frame.phase === 'transfer' && frame.index === request.index, 'native-phase');
    const readHandoff = async path => {
      const value = JSON.parse(await boundedFile(join(path, 'transfer.json'), 512 * 1024));
      exact(value, ['format', 'session', 'file_index', 'source_run', 'source_transfer', 'transfer']);
      require(value.format === 'ic-blob-storage/browser-handoff' && value.session === native.binding.session &&
        value.file_index === request.index, 'native-phase');
      return value;
    };
    const record = await readHandoff(phase);
    require(record.source_transfer === frame.source_transfer, 'native-phase');
    if (record.source_transfer !== null) {
      require(typeof record.source_transfer === 'string' && isAbsolute(record.source_transfer), 'native-phase');
      const original = await readHandoff(record.source_transfer);
      require(isDeepStrictEqual(original, { ...record, source_transfer: null }), 'native-phase');
      return { id: request.id, index: request.index, action: 'recover-certificate', binding: record.transfer.binding };
    }
    return { id: request.id, index: request.index, action: 'upload', transfer: record.transfer };
  } catch { throw new LauncherRefusal('native-phase'); }
}

/** Own one persistent Playwright Chromium context and fixed loopback asset origin.
 * The caller supplies its installed Chromium engine, trusted bundles, exact SDK
 * signer/bootstrap and original profile/origin. Native owns phase decisions and
 * signed service dispatch; the bridge follows them without a durable cursor,
 * retry owner or verifier implementation. Browser results remain
 * redacted observations; the native session must independently establish completion.
 */
export async function launchPublicationBrowser(chromium, options, browserOptions = {}) {
  exact(options, ['profile', 'assetPort', 'hostBundle', 'workerBundle', 'bootstrap', 'nativeSession']);
  try { options = structuredClone(options); }
  catch { throw new LauncherRefusal('configuration'); }
  require(typeof chromium?.launchPersistentContext === 'function' &&
    typeof options.profile === 'string' && isAbsolute(options.profile) &&
    integer(options.assetPort, 65535), 'configuration');
  const rootKey = options.bootstrap?.configuration?.rootKey;
  require((rootKey instanceof Uint8Array || Array.isArray(rootKey)) &&
    integer(rootKey.length, 1024) && Array.from(rootKey).every(byte =>
      Number.isInteger(byte) && byte >= 0 && byte <= 255), 'configuration');
  const bootstrap = options.bootstrap;
  bootstrap.configuration.rootKey = Uint8Array.from(rootKey);
  const maximum = bootstrap?.configuration?.maxBodyBytes;
  const seconds = bootstrap?.configuration?.timeoutSeconds;
  require(integer(maximum, 1024 * 1024 * 1024) && integer(seconds, 3600) &&
    ['create', 'open'].includes(bootstrap?.journal?.mode), 'configuration');
  // These bounds cover executable assets, not a second credential/config route.
  const hostBundle = await boundedFile(options.hostBundle, 16 * 1024 * 1024);
  const workerBundle = await boundedFile(options.workerBundle, 16 * 1024 * 1024);
  const native = await nativeSelection(options, bootstrap, hostBundle, workerBundle);
  const origin = `http://127.0.0.1:${options.assetPort}`;
  const server = createServer((request, response) => {
    response.setHeader('cache-control', 'no-store');
    response.setHeader('x-content-type-options', 'nosniff');
    if (request.headers.host !== `127.0.0.1:${options.assetPort}` || request.method !== 'GET') {
      response.writeHead(403); response.end(); return;
    }
    const asset = { '/host.js': hostBundle, '/worker.js': workerBundle }[request.url];
    if (asset) { response.writeHead(200, { 'content-type': 'text/javascript' }); response.end(asset); }
    else if (request.url === '/') {
      response.writeHead(200, { 'content-type': 'text/html' }); response.end('<!doctype html><title>Publication worker</title>');
    } else { response.writeHead(404); response.end(); }
  });
  let context, page, closed = false, busy = false, timer, closing, lastId = 0;
  const lifetime = new AbortController();
  function close() {
    if (closing) return closing;
    closed = true; clearTimeout(timer); lifetime.abort();
    // Closing the whole context kills the worker and preserves its profile.
    closing = (async () => {
      try { await context?.close(); }
      catch { throw new LauncherRefusal('browser'); }
      finally {
        server.closeAllConnections(); await new Promise(resolve => server.close(resolve));
      }
    })();
    return closing;
  }
  try {
    // Bind the original origin before creating a profile. Port contention cannot
    // silently switch IndexedDB origins and replace recovery history.
    await new Promise((resolve, reject) => { server.once('error', reject); server.listen(options.assetPort, '127.0.0.1', resolve); });
    if (bootstrap.journal.mode === 'create') await mkdir(options.profile, { mode: 0o700 });
    const profile = await lstat(options.profile);
    require(profile.isDirectory() && !profile.isSymbolicLink(), 'profile');
    await bindProfile(options.profile, origin, bootstrap, hostBundle, workerBundle, native);
    context = await chromium.launchPersistentContext(options.profile,
      { ...browserOptions, headless: true, timeout: Math.min(seconds * 1000, 60000) });
    timer = setTimeout(() => { void close().catch(() => {}); }, seconds * 1000);
    page = await context.newPage();
    await page.goto(origin, { timeout: Math.min(seconds * 1000, 60000) });
    await page.evaluate(async value => {
      const { createPublicationWorkerHost } = await import('/host.js');
      value.configuration.rootKey = new Uint8Array(value.configuration.rootKey);
      globalThis.publicationHost = await createPublicationWorkerHost({ workerURL: '/worker.js', bootstrap: value });
    }, { ...bootstrap, configuration: { ...bootstrap.configuration,
      rootKey: Array.from(bootstrap.configuration.rootKey) } });
  } catch (error) {
    // Cleanup diagnostics must not replace the original finite refusal.
    await close().catch(() => {});
    if (error instanceof LauncherRefusal) throw error;
    throw new LauncherRefusal('launch');
  }
  async function execute(request) {
    request = structuredClone(request);
    if (integer(request?.id, 1_000_000)) lastId = Math.max(lastId, request.id);
    if (request?.action === 'transfer') request = await nativeTransfer(native, request);
    else if (native && request?.action === 'upload') throw new LauncherRefusal('native-phase');
    let job;
    if (request?.action === 'upload') {
      exact(request, ['id', 'index', 'action', 'transfer']);
      // Own metadata before filesystem awaits; caller mutation cannot change
      // the selected binding, hints or digest while reading the body.
      const owned = await snapshot(request.transfer, maximum);
      job = { schema: 1, id: request.id, index: request.index, action: 'upload', ...owned.job };
      await page.evaluate(size => { globalThis.publicationBody = new Uint8Array(size); }, owned.bytes.length);
      // Bound each CDP value; no giant JSON byte array or HTTP body route.
      for (let offset = 0; offset < owned.bytes.length; offset += 65536) {
        await page.evaluate(({ offset, bytes }) => publicationBody.set(bytes, offset),
          { offset, bytes: Array.from(owned.bytes.subarray(offset, offset + 65536)) });
      }
    } else {
      exact(request, ['id', 'index', 'action', 'binding']); job = { schema: 1, ...structuredClone(request) };
    }
    return await page.evaluate(async job => {
      if (job.action === 'upload') { job.snapshot.body = globalThis.publicationBody; delete globalThis.publicationBody; }
      return publicationHost.execute(job);
    }, job);
  }
  async function own(action) {
    require(!closed, 'closed'); require(!busy, 'busy'); busy = true;
    try { return await action(); }
    catch (error) {
      if (error instanceof LauncherRefusal) throw error;
      const code = closed ? 'closed' : 'browser';
      await close().catch(() => {});
      throw new LauncherRefusal(code);
    } finally { busy = false; }
  }
  return Object.freeze({
    origin,
    execute: request => own(() => execute(request)),
    driveSession: control => own(async () => {
      const { ready: currentReady, phase, finish } = control ?? {};
      require(native && native.intent.verifier !== null && typeof phase === 'function' && typeof finish === 'function', 'native-session');
      const ready = structuredClone(currentReady); nativeReady(ready, native.intent);
      try { return await driveSession({ ready, phase, finish, execute: request => execute({ ...request, id: lastId + 1 }), signal: lifetime.signal,
        refuse: code => { throw new LauncherRefusal(code); } }); }
      catch (error) { await close().catch(() => {}); throw error; }
    }),
    close,
  });
}
