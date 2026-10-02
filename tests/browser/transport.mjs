// Owned socket observations, not a Caffeine provider or billing qualification.
import { chromium } from 'playwright';
import { createServer } from 'node:https';
import { createSecureServer } from 'node:http2';
import { readFile, writeFile, mkdir } from 'node:fs/promises';
import { resolve, join } from 'node:path';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
import { loopbackTLS } from './tls.mjs';

const output = resolve(process.argv[2]);
const cut = process.argv[3] ?? 'connection-close';
assert(['connection-close', 'refused-stream', 'data-close'].includes(cut));
await mkdir(output); // A fresh capture is required, including after failure.
const record = (name, value) => writeFile(join(output, name), `${JSON.stringify(value, null, 2)}\n`, { flag: 'wx' });
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const modes = ['buffered', 'keepalive-false', 'xhr', 'stream'];
await record('plan.json', { schema: 1, kind: 'owned_loopback_transport', started: new Date().toISOString(),
  runnerSha256: digest(await readFile(new URL(import.meta.url))),
  modes, cut, protocols: ['http/1.1', 'h2'], maxArrivalsPerCase: 3, requestBytes: 1024,
  caseTimeoutMs: 10000, runTimeoutMs: 90000, providerRequests: 0, attachedCycles: '0',
  certificate: 'generated temporary loopback TLS identity',
  journal: 'maintained IndexedDB; certificate observation is a local substitute',
  controls: 'buffered/XHR explicitly consume and replace the maintained outgoing stream; never production alternatives',
  cleanup: 'Close owned browser/sockets; remove temporary TLS keys; retain captures' });
const gateway = await readFile(new URL('../../clients/browser/gateway.js', import.meta.url));
const store = await readFile(new URL('../../.tmp/browser/store.js', import.meta.url));
const tls = await loopbackTLS();
let browser;
const results = [];
const deadline = setTimeout(() => { console.error('Transport observation exceeded 90 seconds'); process.exit(1); }, 90000);
try {
  const { cert, key } = tls;
  browser = await chromium.launch(tls.launch);
  for (const protocol of ['http/1.1', 'h2']) for (const mode of modes) {
    const arrivals = [], sockets = new Set();
    let serverFailure;
    const handle = (req, res) => {
      if (req.method === 'PUT') {
        const chunks = []; let length = 0;
        let completed = false;
        req.on('error', () => {});
        req.on('data', bytes => {
          length += bytes.length;
          if (length > 1024) { serverFailure = 'request-size'; req.destroy(); }
          else chunks.push(bytes);
          if (cut === 'data-close' && length === 1024) receive();
        });
        function receive() {
          if (completed) return;
          completed = true;
          const body = Buffer.concat(chunks);
          const arrival = { method: req.method, path: req.url, protocol: req.httpVersion,
            bytes: body.length, sha256: digest(body) };
          arrivals.push(arrival);
          if (req.url !== '/upload' || !body.equals(Buffer.alloc(1024, 42)) || arrivals.length > 3) {
            serverFailure = 'unexpected-request'; res.destroy(); return;
          }
          // Lose headers after receipt, then reject any transparent repeat so the
          // exact browser outcome and journal status remain inspectable.
          if (arrivals.length === 1) {
            if (req.stream && cut === 'refused-stream') req.stream.close(7); // NGHTTP2_REFUSED_STREAM.
            else if (req.stream) req.stream.session.destroy(); else res.destroy();
          } else { res.writeHead(503); res.end('repeat observed'); }
        }
        req.on('end', receive);
        return;
      }
      const scripts = { '/gateway.js': gateway, '/store.js': store };
      if (scripts[req.url]) {
        res.writeHead(200, { 'content-type': 'text/javascript' }); res.end(scripts[req.url]);
      } else {
        res.writeHead(200, { 'content-type': 'text/html' });
        res.end('<!doctype html><script src="/store.js" type="module"></script><script type="module">import {createGatewayTransport} from "/gateway.js"; window.createTransport = createGatewayTransport;</script>');
      }
    };
    const server = protocol === 'h2' ? createSecureServer({ cert, key, allowHTTP1: false }, handle)
      : createServer({ cert, key, ALPNProtocols: ['http/1.1'] }, handle);
    server.on('connection', socket => { sockets.add(socket); socket.on('close', () => sockets.delete(socket)); });
    server.on('session', session => session.on('error', () => {}));
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    const origin = `https://127.0.0.1:${server.address().port}`;
    const context = await browser.newContext();
    try {
      const page = await context.newPage();
      page.setDefaultTimeout(10000);
      await page.goto(origin); await page.waitForFunction(() => !!window.createTransport && !!window.createIntentStore);
      // No Playwright request routing: it could turn a stream back into a buffer.
      const observed = await page.evaluate(async ({ origin, mode }) => {
        // Navigation uses a different credentials mode. Warm the actual fetch
        // connection pool, as the SDK's successful tree PUT does before a chunk.
        const warm = await fetch(`${origin}/warm`, { credentials: 'omit', cache: 'no-store',
          redirect: 'error', referrerPolicy: 'no-referrer' });
        await warm.text();
        const binding = { key: 'aaaaa-aa:aaaaa-aa:1', service: 'aaaaa-aa', tenant: 'aaaaa-aa',
          uploader: 'rrkah-fqaaa-aaaaa-aaaaq-cai', operation: '1', root: `sha256:${'1'.repeat(64)}`,
          project: 'substitute', bucket: 'substitute', permission: [68, 73, 68, 76], icOrigin: origin, icRootKey: [1] };
        const intents = await createIntentStore({ database: 'transport-case', maxSlots: 1, mode: 'create' });
        await intents.save(binding); await intents.claim(binding, [1], '2'.repeat(64));
        await intents.observe(binding, '2'.repeat(64));
        const certificate = { inspect: () => intents.inspect(binding) };
        let dispatches = 0;
        if (mode === 'stream' && origin.startsWith('https:')) {
          const before = await certificate.inspect();
          const OriginalRequest = window.Request;
          try {
            window.Request = class { constructor() { this.headers = new Headers(); } };
            let unsupported;
            try { await createTransport({ certificate, intents, origin,
              maxRequests: 1, maxRequestBytes: 1024, maxTotalRequestBytes: 1024 }); }
            catch (error) { unsupported = error.code; }
            if (unsupported !== 'request-streaming') throw new Error('unsupported stream accepted');
          } finally { window.Request = OriginalRequest; }
          if (JSON.stringify(await certificate.inspect()) !== JSON.stringify(before)) throw new Error('unsupported stream changed intent');
        }
        const hook = await createTransport({ certificate, intents, origin,
          maxRequests: 1, maxRequestBytes: 1024, maxTotalRequestBytes: 1024,
          fetch: async (url, init) => {
            dispatches++;
            if (!(init.body instanceof ReadableStream) || init.duplex !== 'half') throw new Error('missing production stream');
            if (mode !== 'stream') {
              init = { ...init, body: new Uint8Array(await new Response(init.body).arrayBuffer()) };
              delete init.duplex;
            }
            if (mode === 'xhr') return new Promise((resolve, reject) => {
              const xhr = new XMLHttpRequest(); xhr.open(init.method, url); xhr.timeout = 10000;
              for (const [name, value] of init.headers) xhr.setRequestHeader(name, value);
              xhr.onload = () => resolve(new Response(xhr.response, { status: xhr.status }));
              xhr.onerror = xhr.ontimeout = () => reject(new TypeError('XHR failure'));
              xhr.send(init.body);
            });
            if (mode === 'keepalive-false') init = { ...init, keepalive: false };
            return fetch(url, init);
          } });
        let outcome;
        try { const response = await hook(`${origin}/upload`, { method: 'PUT', body: new Uint8Array(1024).fill(42) });
          outcome = { status: response.status }; }
        catch (error) { outcome = { error: error.name, code: error.code ?? null }; }
        const row = await intents.inspect(binding);
        return { dispatches, outcome, gateway: row.gateway };
      }, { origin, mode });
      const result = { mode, protocol, cut, browser: browser.version(), origin, observed, arrivals, serverFailure: serverFailure ?? null };
      await record(`${protocol === 'h2' ? 'h2' : 'h1'}-${mode}.json`, result);
      assert.equal(serverFailure, undefined);
      assert.equal(observed.dispatches, 1);
      assert.equal(observed.gateway.requests.length, 1);
      if (mode === 'stream') {
        assert.equal(arrivals.length, protocol === 'h2' ? 1 : 0);
        assert.equal(observed.gateway.requests[0].phase, 'uncertain');
        assert.equal(observed.outcome.error, 'TypeError');
      } else {
        assert(arrivals.length > observed.dispatches, 'Buffered control should expose unmetered replay');
        assert.equal(observed.outcome.status, 503);
      }
      for (const arrival of arrivals) assert.equal(arrival.protocol, protocol === 'h2' ? '2.0' : '1.1');
      for (const arrival of arrivals) assert.equal(arrival.sha256, digest(Buffer.alloc(1024, 42)));
      results.push(result);
      console.log(`${protocol} ${mode}: ${arrivals.length} PUT arrivals, ${observed.outcome.status ?? observed.outcome.error}`);
    } finally {
      await context.close();
      for (const socket of sockets) socket.destroy();
      await new Promise(resolve => server.close(resolve));
    }
  }
  await record('summary.json', { schema: 1, results, qualification: 'Owned transports only; no deployed provider economics',
    completed: new Date().toISOString() });
} finally {
  clearTimeout(deadline); await browser?.close(); await tls.close();
}
