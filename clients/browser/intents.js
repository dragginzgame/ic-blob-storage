import { Principal } from '@icp-sdk/core/principal';
import { validNamespace } from './namespace.js';

/** Local journal refusal. Never authorizes repeating a certificate or provider request. */
export class IntentRefusal extends Error {
  constructor(code) { super(code); this.name = 'IntentRefusal'; this.code = code; }
}
const require = (condition, code) => { if (!condition) throw new IntentRefusal(code); };
const equal = (a, b) => JSON.stringify(a) === JSON.stringify(b);
const integer = (value, min, max) => Number.isSafeInteger(value) && value >= min && value <= max;
const hex = value => typeof value === 'string' && /^[0-9a-f]{64}$/.test(value);
const bytes = (value, max) => Array.isArray(value) && value.length > 0 &&
  value.length <= max && value.every(byte => integer(byte, 0, 255));
function fields(value, required, optional = []) {
  require(value && Object.getPrototypeOf(value) === Object.prototype &&
    required.every(key => Object.hasOwn(value, key)) &&
    Object.keys(value).every(key => [...required, ...optional].includes(key)), 'record');
}
function origin(value) {
  require(typeof value === 'string' && value.length <= 4096, 'origin');
  const url = new URL(value);
  require(value === url.origin && (url.protocol === 'https:' ||
    (url.protocol === 'http:' && ['localhost', '127.0.0.1', '[::1]'].includes(url.hostname))), 'origin');
}
function binding(value) {
  fields(value, ['key', 'service', 'tenant', 'uploader', 'operation', 'root',
    'project', 'bucket', 'permission', 'icOrigin', 'icRootKey']);
  require(validNamespace(value.project, true) && validNamespace(value.bucket), 'namespace');
  for (const field of ['service', 'tenant', 'uploader']) {
    require(typeof value[field] === 'string' && value[field].length <= 63 &&
      Principal.fromText(value[field]).toText() === value[field], 'principal');
  }
  require(value.uploader !== Principal.anonymous().toText(), 'principal');
  require(typeof value.operation === 'string' && /^(0|[1-9][0-9]{0,38})$/.test(value.operation) &&
    BigInt(value.operation) < (1n << 128n), 'operation');
  require(typeof value.root === 'string' && /^sha256:[0-9a-f]{64}$/.test(value.root), 'root');
  require(value.key === `${value.service}:${value.tenant}:${value.operation}`, 'key');
  require(bytes(value.permission, 65536) && bytes(value.icRootKey, 1024), 'bytes');
  origin(value.icOrigin);
  // Canonical property order makes comparisons independent of caller property order.
  return Object.fromEntries(['key', 'service', 'tenant', 'uploader', 'operation', 'root',
    'project', 'bucket', 'permission', 'icOrigin', 'icRootKey'].map(key => [key, value[key]]));
}
function scope(value) {
  fields(value, ['origin', 'maxRequests', 'maxRequestBytes', 'maxTotalRequestBytes']);
  origin(value.origin);
  require(integer(value.maxRequests, 1, 256) && integer(value.maxRequestBytes, 1, 2 * 1024 * 1024) &&
    integer(value.maxTotalRequestBytes, 1, value.maxRequests * value.maxRequestBytes), 'limits');
  return { origin: value.origin, maxRequests: value.maxRequests,
    maxRequestBytes: value.maxRequestBytes, maxTotalRequestBytes: value.maxTotalRequestBytes };
}
function owner(value) {
  require(typeof value === 'string' && /^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$/.test(value), 'owner');
}
function request(value, bounds) {
  fields(value, ['url', 'method', 'headers', 'bodyBytes', 'bodySha256']);
  require(typeof value.url === 'string' && value.url.length <= 4096, 'request');
  const url = new URL(value.url);
  require(url.href === value.url && url.origin === bounds.origin && !url.username &&
    !url.password && !url.hash && value.method === 'PUT' &&
    integer(value.bodyBytes, 0, bounds.maxRequestBytes) && hex(value.bodySha256), 'request');
  require(Array.isArray(value.headers) && value.headers.length <= 16 && value.headers.every(pair =>
    Array.isArray(pair) && pair.length === 2 && pair.every(part => typeof part === 'string')) &&
    value.headers.reduce((n, [k, v]) => n + k.length + v.length, 0) <= 4096, 'headers');
  require(equal(Array.from(new Headers(value.headers).entries()), value.headers), 'headers');
  return { url: value.url, method: value.method, headers: value.headers,
    bodyBytes: value.bodyBytes, bodySha256: value.bodySha256 };
}
function row(value) {
  if (value === undefined) return value;
  try {
    fields(value, ['key', 'binding', 'cancelled', 'phase'], ['envelope', 'requestId', 'gateway']);
    value.binding = binding(value.binding);
    require(value.key === value.binding.key && typeof value.cancelled === 'boolean' &&
      ['saved', 'uncertain', 'observed'].includes(value.phase), 'state');
    require(value.phase === 'saved' ? !Object.hasOwn(value, 'envelope') &&
      !Object.hasOwn(value, 'requestId') : bytes(value.envelope, 8192) && hex(value.requestId), 'state');
    if (Object.hasOwn(value, 'gateway')) {
      require(value.phase === 'observed', 'state');
      const gateway = value.gateway;
      fields(gateway, ['scope', 'owner', 'requests']);
      gateway.scope = scope(gateway.scope); owner(gateway.owner);
      require(Array.isArray(gateway.requests) &&
        integer(gateway.requests.length, 1, gateway.scope.maxRequests), 'history');
      let used = 0;
      const urls = new Set();
      for (const [index, entry] of gateway.requests.entries()) {
        fields(entry, ['request', 'phase'], ['status']);
        entry.request = request(entry.request, gateway.scope);
        used += entry.request.bodyBytes;
        require(used <= gateway.scope.maxTotalRequestBytes && !urls.has(entry.request.url), 'history');
        urls.add(entry.request.url);
        require(entry.phase === 'uncertain' ? !Object.hasOwn(entry, 'status') :
          entry.phase === 'responded' && integer(entry.status, 200, 599), 'history');
        require(index === gateway.requests.length - 1 ||
          (entry.phase === 'responded' && entry.status < 300), 'history');
      }
    }
    return value;
  } catch { throw new IntentRefusal('intent-corrupt'); }
}
const bound = (value, saved) => require(value && equal(value.binding, saved), 'intent-binding');

/**
 * Create a fresh v1 journal or reopen an explicitly named existing journal.
 * Capacity (1..64 lifetime rows) is immutable; no deletion, reset or retry API exists.
 * Strict IndexedDB commits serialize certificate, cancellation and gateway history
 * across tabs. Profile loss/rollback, eviction and hostile origin scripts remain
 * outside this local contract. See README.md before selecting trial storage.
 */
export async function createIndexedDBIntentStore({ database, maxSlots, mode,
  indexedDB: factory = globalThis.indexedDB }) {
  require(typeof database === 'string' && database.length > 0 && database.length <= 128 &&
    integer(maxSlots, 1, 64) && ['create', 'open'].includes(mode) && factory, 'configuration');
  const db = await new Promise((resolve, reject) => {
    let created = false, failure, settled = false;
    const fail = error => { settled = true; clearTimeout(timer); reject(error); };
    const opening = factory.open(database, 1);
    const timer = setTimeout(() => fail(new IntentRefusal('store-timeout')), 10_000);
    opening.onblocked = () => fail(new IntentRefusal('store-blocked'));
    opening.onupgradeneeded = event => {
      if (settled || mode !== 'create' || event.oldVersion !== 0) {
        failure = new IntentRefusal('store-missing'); opening.transaction.abort(); return;
      }
      created = true;
      opening.result.createObjectStore('intents', { keyPath: 'key' });
      opening.result.createObjectStore('configuration', { keyPath: 'key' })
        .put({ key: 'configuration', maxSlots });
    };
    opening.onerror = () => fail(failure ?? opening.error);
    opening.onsuccess = () => {
      const result = opening.result;
      if (settled) { result.close(); return; }
      clearTimeout(timer);
      if (mode === 'create' && !created) {
        result.close(); fail(new IntentRefusal('store-exists')); return;
      }
      settled = true; resolve(result);
    };
  });
  db.onversionchange = () => db.close();
  let closed = false;
  function transaction(saved, mutate) {
    require(!closed, 'store-closed');
    return new Promise((resolve, reject) => {
      let result, failure;
      const tx = db.transaction(['configuration', 'intents'], mutate ? 'readwrite' : 'readonly',
        mutate ? { durability: 'strict' } : {});
      const timer = setTimeout(() => {
        failure = new IntentRefusal('store-timeout'); tx.abort();
      }, 10_000);
      tx.oncomplete = () => { clearTimeout(timer); resolve(result); };
      tx.onabort = () => { clearTimeout(timer); reject(failure ?? tx.error ?? new IntentRefusal('store-aborted')); };
      tx.onerror = () => {}; // onabort owns request failures.
      const abort = error => { failure = error; tx.abort(); };
      if (mutate && tx.durability !== 'strict') { abort(new IntentRefusal('store-durability')); return; }
      const records = tx.objectStore('intents');
      const config = tx.objectStore('configuration').get('configuration');
      config.onsuccess = () => {
        try {
          fields(config.result, ['key', 'maxSlots']);
          require(config.result.key === 'configuration' && config.result.maxSlots === maxSlots, 'store-configuration');
          const count = records.count();
          count.onsuccess = () => {
            try {
              require(count.result <= maxSlots, 'store-capacity');
              if (!saved) return;
              const get = records.get(saved.key);
              get.onsuccess = () => {
                try {
                  const existing = row(get.result);
                  if (existing) bound(existing, saved);
                  result = mutate ? row(mutate(existing, count.result)) : existing;
                  if (mutate) records.put(result);
                } catch (error) { abort(error); }
              };
            } catch (error) { abort(error); }
          };
        } catch (error) { abort(error); }
      };
    });
  }
  try {
    require(db.version === 1 && equal(Array.from(db.objectStoreNames), ['configuration', 'intents']), 'store-schema');
    const schema = db.transaction(['configuration', 'intents'], 'readonly');
    for (const name of ['configuration', 'intents']) {
      const store = schema.objectStore(name);
      require(store.keyPath === 'key' && store.indexNames.length === 0, 'store-schema');
    }
    await transaction();
  } catch (error) { db.close(); throw error; }
  // Validate and snapshot all caller-owned arguments before the first storage await.
  const snapshot = value => binding(structuredClone(value));
  const change = (saved, fn) => transaction(saved, fn);
  return Object.freeze({
    close() { closed = true; db.close(); },
    inspect(input) { return transaction(snapshot(input)); },
    save(input) {
      const saved = snapshot(input);
      return change(saved, (existing, count) => {
        if (existing) return existing;
        require(count < maxSlots, 'capacity');
        return { key: saved.key, binding: saved, cancelled: false, phase: 'saved' };
      });
    },
    claim(input, envelope, requestId) {
      const saved = snapshot(input), exact = structuredClone(envelope);
      require(bytes(exact, 8192) && hex(requestId), 'envelope');
      return change(saved, existing => {
        bound(existing, saved);
        require(!existing.cancelled && existing.phase === 'saved', 'dispatch-blocked');
        return { ...existing, phase: 'uncertain', envelope: exact, requestId };
      });
    },
    observe(input, requestId) {
      const saved = snapshot(input); require(hex(requestId), 'request-id');
      return change(saved, existing => {
        bound(existing, saved);
        require(['uncertain', 'observed'].includes(existing.phase) && existing.requestId === requestId,
          'observation-binding');
        return { ...existing, phase: 'observed' };
      });
    },
    cancel(input) {
      const saved = snapshot(input);
      return change(saved, existing => { bound(existing, saved); return { ...existing, cancelled: true }; });
    },
    claimGateway(input, bounds, token, index, opaque) {
      const saved = snapshot(input), limits = scope(structuredClone(bounds)); owner(token);
      const exact = request(structuredClone(opaque), limits); require(integer(index, 0, 255), 'index');
      return change(saved, existing => {
        bound(existing, saved);
        require(!existing.cancelled && existing.phase === 'observed', 'gateway-blocked');
        const gateway = existing.gateway ?? { scope: limits, owner: token, requests: [] };
        require(gateway.owner === token && equal(gateway.scope, limits), 'gateway-session');
        require(index === gateway.requests.length && index < limits.maxRequests, 'gateway-capacity');
        const used = gateway.requests.reduce((n, entry) => n + entry.request.bodyBytes, 0);
        require(exact.bodyBytes <= limits.maxTotalRequestBytes - used, 'gateway-budget');
        const previous = gateway.requests.at(-1);
        require(!previous || (previous.phase === 'responded' && previous.status < 300), 'gateway-uncertain');
        require(!gateway.requests.some(entry => entry.request.url === exact.url), 'gateway-repeat');
        gateway.requests.push({ request: exact, phase: 'uncertain' });
        return { ...existing, gateway };
      });
    },
    observeGateway(input, bounds, token, index, opaque, status) {
      const saved = snapshot(input), limits = scope(structuredClone(bounds)); owner(token);
      const exact = request(structuredClone(opaque), limits);
      require(integer(index, 0, 255) && integer(status, 200, 599), 'observation');
      return change(saved, existing => {
        bound(existing, saved);
        const gateway = existing.gateway, entry = gateway?.requests[index];
        require(gateway?.owner === token && equal(gateway.scope, limits) &&
          gateway.requests.length === index + 1 && entry?.phase === 'uncertain' &&
          equal(entry.request, exact), 'gateway-observation');
        gateway.requests[index] = { request: exact, phase: 'responded', status };
        return { ...existing, gateway };
      });
    },
  });
}
