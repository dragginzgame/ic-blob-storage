import { constants } from 'node:fs';
import { open, lstat, mkdir } from 'node:fs/promises';
import { createServer } from 'node:http';
import { createHash } from 'node:crypto';
import { isAbsolute } from 'node:path';

/** Finite process-bridge refusal; never includes keys, paths or remote errors. */
export class LauncherRefusal extends Error {
  constructor(code) { super(code); this.name = 'LauncherRefusal'; this.code = code; }
}
const require = (condition, code) => { if (!condition) throw new LauncherRefusal(code); };
const exact = (value, keys, optional = []) => require(value &&
  Object.getPrototypeOf(value) === Object.prototype && keys.every(key => Object.hasOwn(value, key)) &&
  Object.keys(value).every(key => [...keys, ...optional].includes(key)), 'configuration');
const integer = (value, maximum) => Number.isSafeInteger(value) && value > 0 && value <= maximum;

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
  exact(transfer.preparation, [], ['content_type', 'filename']);
  for (const value of Object.values(transfer.preparation)) require(typeof value === 'string' &&
    Buffer.byteLength(value) <= 4096, 'metadata-hint');
  const bytes = await boundedFile(transfer.body, maximum, size);
  require(createHash('sha256').update(bytes).digest('hex') === transfer.body_sha256, 'body-digest');
  const hints = transfer.preparation;
  return { bytes, job: { binding: structuredClone(transfer.binding), snapshot: {
    bodySha256: transfer.body_sha256, manifestJSON: transfer.manifest_json,
    ...(Object.hasOwn(hints, 'content_type') ? { contentType: hints.content_type } : {}),
    ...(Object.hasOwn(hints, 'filename') ? { filename: hints.filename } : {}) } } };
}

/** Own one persistent Playwright Chromium context and fixed loopback asset origin.
 * The caller supplies its installed Chromium engine, trusted bundles, exact SDK
 * signer/bootstrap and original profile/origin. This bridge owns no durable phase,
 * service dispatch, retry, verifier or publication decision. Browser results remain
 * redacted observations; the native session must independently establish completion.
 */
export async function launchPublicationBrowser(chromium, options, browserOptions = {}) {
  exact(options, ['profile', 'assetPort', 'hostBundle', 'workerBundle', 'bootstrap']);
  require(typeof chromium?.launchPersistentContext === 'function' &&
    typeof options.profile === 'string' && isAbsolute(options.profile) &&
    integer(options.assetPort, 65535), 'configuration');
  const rootKey = options.bootstrap?.configuration?.rootKey;
  require((rootKey instanceof Uint8Array || Array.isArray(rootKey)) &&
    integer(rootKey.length, 1024) && Array.from(rootKey).every(byte =>
      Number.isInteger(byte) && byte >= 0 && byte <= 255), 'configuration');
  const bootstrap = structuredClone({ ...options.bootstrap,
    configuration: { ...options.bootstrap.configuration, rootKey: Uint8Array.from(rootKey) } });
  const maximum = bootstrap?.configuration?.maxBodyBytes;
  const seconds = bootstrap?.configuration?.timeoutSeconds;
  require(integer(maximum, 1024 * 1024 * 1024) && integer(seconds, 3600) &&
    ['create', 'open'].includes(bootstrap?.journal?.mode), 'configuration');
  bootstrap.configuration.rootKey = Uint8Array.from(bootstrap.configuration.rootKey);
  // These bounds cover executable assets, not a second credential/config route.
  const hostBundle = await boundedFile(options.hostBundle, 16 * 1024 * 1024);
  const workerBundle = await boundedFile(options.workerBundle, 16 * 1024 * 1024);
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
  let context, page, closed = false, busy = false, timer, closing;
  function close() {
    if (closing) return closing;
    closed = true; clearTimeout(timer);
    // Closing the whole context kills the worker and preserves its profile.
    closing = (async () => {
      try { await context?.close(); } finally {
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
    await close();
    if (error instanceof LauncherRefusal) throw error;
    throw new LauncherRefusal('launch');
  }
  return Object.freeze({
    origin,
    async execute(request) {
      require(!closed, 'closed'); require(!busy, 'busy'); busy = true;
      try {
        request = structuredClone(request);
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
      } catch (error) {
        if (error instanceof LauncherRefusal) throw error;
        const code = closed ? 'closed' : 'browser';
        await close();
        throw new LauncherRefusal(code);
      } finally { busy = false; }
    },
    close,
  });
}
