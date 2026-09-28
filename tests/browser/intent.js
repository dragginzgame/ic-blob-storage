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
export async function inspect(key) {
  const db = await open();
  try {
    return await new Promise((resolve, reject) => {
      const tx = db.transaction('intents', 'readonly');
      const request = tx.objectStore('intents').get(key);
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
export function claim(key, envelope, requestId, abortAfterPut = false) {
  return change(key, row => {
    if (!row || row.cancelled || row.phase !== 'saved') throw new Refusal('dispatch-blocked');
    return { ...row, phase: 'uncertain', envelope, requestId };
  }, abortAfterPut);
}
export function observe(key, requestId) {
  return change(key, row => {
    if (!row || !['uncertain', 'observed'].includes(row.phase) || row.requestId !== requestId) {
      throw new Refusal('observation-binding');
    }
    // Cancellation is never cleared, including by a delayed successful certificate.
    return { ...row, phase: 'observed' };
  });
}
export function cancel(key) {
  return change(key, row => {
    if (!row) throw new Refusal('missing');
    return { ...row, cancelled: true };
  });
}
