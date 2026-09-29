// Private two-slot browser fixture. No reset, eviction, identity allocation or provider retry.
const NAME = 'blob-certificate-fixture-v1';
const MAX_SLOTS = 2;
export class Refusal extends Error {
  constructor(code) { super(code); this.code = code; }
}
async function open() {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open(NAME, 1);
    request.onupgradeneeded = () => request.result.createObjectStore('intents', { keyPath: 'key' });
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
}
// Read and write within the same serialized transaction, across tabs. No network awaits.
async function change(key, fn, abortAfterPut = false) {
  const db = await open();
  try {
    return await new Promise((resolve, reject) => {
      const tx = db.transaction('intents', 'readwrite', { durability: 'strict' });
      const store = tx.objectStore('intents');
      let result, failure;
      const rows = store.getAll();
      rows.onsuccess = () => {
        try {
          const next = fn(rows.result.find(row => row.key === key), rows.result.length);
          if (next) {
            store.put(next); result = next;
            if (abortAfterPut) { failure = new Refusal('fixture-write-abort'); tx.abort(); }
          }
        } catch (error) { failure = error; tx.abort(); }
      };
      tx.oncomplete = () => resolve(result);
      tx.onabort = () => reject(failure ?? tx.error);
      tx.onerror = () => {}; // onabort owns failure.
    });
  } finally { db.close(); }
}
export async function inspect(binding) {
  const db = await open();
  try {
    return await new Promise((resolve, reject) => {
      const tx = db.transaction('intents', 'readonly');
      const request = tx.objectStore('intents').get(binding.key);
      tx.oncomplete = () => resolve(request.result);
      tx.onabort = () => reject(tx.error);
    });
  } finally { db.close(); }
}
export function save(binding) {
  return change(binding.key, (existing, count) => {
    if (existing) {
      if (JSON.stringify(existing.binding) !== JSON.stringify(binding)) throw new Refusal('conflict');
      return existing;
    }
    if (count >= MAX_SLOTS) throw new Refusal('capacity');
    return { key: binding.key, binding, cancelled: false, phase: 'saved' };
  });
}
function bound(row, binding) {
  if (!row || JSON.stringify(row.binding) !== JSON.stringify(binding)) throw new Refusal('intent-binding');
}
export function claim(binding, envelope, requestId, abortAfterPut = false) {
  return change(binding.key, row => {
    bound(row, binding);
    if (!row || row.cancelled || row.phase !== 'saved') throw new Refusal('dispatch-blocked');
    return { ...row, phase: 'uncertain', envelope, requestId };
  }, abortAfterPut);
}
export function observe(binding, requestId) {
  return change(binding.key, row => {
    bound(row, binding);
    if (!row || !['uncertain', 'observed'].includes(row.phase) || row.requestId !== requestId) {
      throw new Refusal('observation-binding');
    }
    // Cancellation is never cleared, including by a delayed successful certificate.
    return { ...row, phase: 'observed' };
  });
}
export function cancel(binding) {
  return change(binding.key, row => {
    bound(row, binding);
    return { ...row, cancelled: true };
  });
}

// Gateway claims and cancellation share the certificate row and transaction.
export function claimGateway(binding, scope, owner, index, request, abortAfterPut = false) {
  return change(binding.key, row => {
    bound(row, binding);
    if (row.cancelled || row.phase !== 'observed') throw new Refusal('gateway-blocked');
    const gateway = row.gateway ?? { scope, owner, requests: [] };
    if (gateway.owner !== owner || JSON.stringify(gateway.scope) !== JSON.stringify(scope)) {
      throw new Refusal('gateway-session');
    }
    if (index !== gateway.requests.length || index >= scope.maxRequests) throw new Refusal('gateway-capacity');
    const used = gateway.requests.reduce((n, entry) => n + entry.request.bodyBytes, 0);
    if (request.bodyBytes > scope.maxTotalRequestBytes - used) throw new Refusal('gateway-budget');
    const previous = gateway.requests.at(-1);
    if (previous && (previous.phase !== 'responded' || previous.status < 200 || previous.status >= 300)) {
      throw new Refusal('gateway-uncertain');
    }
    if (gateway.requests.some(entry => entry.request.url === request.url)) throw new Refusal('gateway-repeat');
    gateway.requests.push({ request, phase: 'uncertain' });
    return { ...row, gateway };
  }, abortAfterPut);
}
export function observeGateway(binding, scope, owner, index, request, status, abortAfterPut = false) {
  return change(binding.key, row => {
    bound(row, binding);
    const gateway = row.gateway;
    const entry = gateway?.requests[index];
    if (gateway?.owner !== owner || JSON.stringify(gateway.scope) !== JSON.stringify(scope) ||
      gateway.requests.length !== index + 1 || !entry ||
      JSON.stringify(entry.request) !== JSON.stringify(request) || entry.phase !== 'uncertain') {
      throw new Refusal('gateway-observation');
    }
    gateway.requests[index] = { request, phase: 'responded', status };
    // A late HTTP response records history even when cancellation won the race.
    return { ...row, gateway };
  }, abortAfterPut);
}
