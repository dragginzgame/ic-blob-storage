// Test-only platform fault injection; all journal rules live in the reusable store.
import { createIndexedDBIntentStore } from '../../clients/browser/intents.js';
const options = { database: 'blob-certificate-fixture-v1', maxSlots: 2 };
let abortWrite = false, storePromise;
const factory = {
  open(...args) {
    const opening = indexedDB.open(...args);
    opening.addEventListener('success', () => {
      const db = opening.result;
      const transaction = db.transaction.bind(db);
      db.transaction = (...args) => {
        const tx = transaction(...args), objectStore = tx.objectStore.bind(tx);
        tx.objectStore = name => {
          const store = objectStore(name), put = store.put.bind(store);
          store.put = (...args) => {
            const request = put(...args);
            if (abortWrite) tx.abort();
            return request;
          };
          return store;
        };
        return tx;
      };
    });
    return opening;
  },
};
export function initialize() {
  storePromise = createIndexedDBIntentStore({ ...options, mode: 'create', indexedDB: factory });
  return storePromise.then(() => undefined);
}
const store = () => storePromise ??= createIndexedDBIntentStore({ ...options, mode: 'open', indexedDB: factory });
async function call(method, args, abort = false) {
  const intents = await store();
  abortWrite = abort;
  try { return await intents[method](...args); }
  catch (error) {
    if (abort && (error?.name === 'AbortError' || error?.code === 'store-aborted')) {
      const failure = new Error('fixture-write-abort'); failure.code = 'fixture-write-abort'; throw failure;
    }
    throw error;
  } finally { abortWrite = false; }
}
export const save = binding => call('save', [binding]);
export const inspect = binding => call('inspect', [binding]);
export const claim = (binding, envelope, id, abort) => call('claim', [binding, envelope, id], abort);
export const observe = (binding, id) => call('observe', [binding, id]);
export const cancel = binding => call('cancel', [binding]);
export const claimGateway = (binding, scope, owner, index, request, abort) =>
  call('claimGateway', [binding, scope, owner, index, request], abort);
export const observeGateway = (binding, scope, owner, index, request, status, abort) =>
  call('observeGateway', [binding, scope, owner, index, request, status], abort);
