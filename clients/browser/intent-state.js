// Internal journal state rules. Persistence and caller snapshots stay with adapters.
/** Local journal refusal. Never authorizes repeating a certificate or provider request. */
export class IntentRefusal extends Error {
  constructor(code) { super(code); this.name = 'IntentRefusal'; this.code = code; }
}
export const require = (condition, code) => { if (!condition) throw new IntentRefusal(code); };
export const equal = (a, b) => JSON.stringify(a) === JSON.stringify(b);
export const bound = (value, saved) => require(value && equal(value.binding, saved), 'intent-binding');

// Arguments have been validated/snapshotted by the adapter before storage awaits.
// Return a fresh row: a refusal cannot partly mutate the adapter's original state.
export function claimGateway(existing, saved, limits, token, index, exact) {
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
  return { ...existing, gateway: { ...gateway,
    requests: [...gateway.requests, { request: exact, phase: 'uncertain' }] } };
}

export function observeGateway(existing, saved, limits, token, index, exact, status) {
  bound(existing, saved);
  const gateway = existing.gateway, entry = gateway?.requests[index];
  require(gateway?.owner === token && equal(gateway.scope, limits) &&
    gateway.requests.length === index + 1 && entry?.phase === 'uncertain' &&
    equal(entry.request, exact), 'gateway-observation');
  return { ...existing, gateway: { ...gateway,
    requests: gateway.requests.map((entry, i) => i === index ?
      { request: exact, phase: 'responded', status } : entry) } };
}
