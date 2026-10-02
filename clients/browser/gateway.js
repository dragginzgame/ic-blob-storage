const hex = bytes => Array.from(bytes, b => b.toString(16).padStart(2, '0')).join('');
const equal = (a, b) => JSON.stringify(a) === JSON.stringify(b);

/** Local coordination refusal, not a provider response or reconciliation result. */
export class GatewayRefusal extends Error {
  constructor(code) { super(code); this.name = 'GatewayRefusal'; this.code = code; }
}
const require = (condition, code) => { if (!condition) throw new GatewayRefusal(code); };

/**
 * A fetch hook for one uninterrupted Caffeine upload (retry:false, concurrency:1).
 * The caller's store must atomically implement the gateway contract in README.md.
 * This records opaque requests; it does not implement or validate provider formats.
 * Recreating this hook cannot resume an already claimed transfer.
 */
export async function createGatewayTransport({ certificate, intents, origin,
  maxRequests, maxRequestBytes, maxTotalRequestBytes, fetch: transport = globalThis.fetch.bind(globalThis) }) {
  const endpoint = new URL(origin);
  require(endpoint.protocol === 'https:', 'origin');
  require(!endpoint.username && !endpoint.password && endpoint.pathname === '/' &&
    !endpoint.search && !endpoint.hash, 'origin');
  // Reject unsupported request-stream environments before certificate issuance.
  // A buffered fallback reintroduces transparent PUT replay after connection loss.
  let duplex = false;
  try {
    const probe = new Request(endpoint, { method: 'PUT', body: new ReadableStream({
      start(controller) { controller.close(); }
    }), get duplex() { duplex = true; return 'half'; } });
    require(duplex && !probe.headers.has('content-type'), 'request-streaming');
  } catch { throw new GatewayRefusal('request-streaming'); }
  require(Number.isSafeInteger(maxRequests) && maxRequests > 0 && maxRequests <= 256 &&
    Number.isSafeInteger(maxRequestBytes) && maxRequestBytes > 0 &&
    maxRequestBytes <= 2 * 1024 * 1024 && Number.isSafeInteger(maxTotalRequestBytes) &&
    maxTotalRequestBytes > 0 && maxTotalRequestBytes <= maxRequests * maxRequestBytes, 'limits');
  for (const method of ['claimGateway', 'observeGateway']) {
    require(typeof intents[method] === 'function', 'store');
  }
  const binding = structuredClone((await certificate.inspect()).binding);
  const scope = Object.freeze({ origin: endpoint.origin, maxRequests, maxRequestBytes, maxTotalRequestBytes });
  // A local execution fence, never a service operation ID or restore authority.
  const owner = crypto.randomUUID();
  let index = 0;
  let claimedBytes = 0;
  const copy = value => structuredClone(value);
  function retained(row, request, phase) {
    require(row && equal(row.binding, binding) && row.phase === 'observed', 'intent-binding');
    const gateway = row.gateway;
    require(gateway?.owner === owner && equal(gateway.scope, scope) &&
      Array.isArray(gateway.requests) && gateway.requests.length === index + 1 &&
      gateway.requests.length <= maxRequests, 'gateway-binding');
    const last = gateway.requests[index];
    require(equal(last.request, request) && last.phase === phase, 'request-binding');
    return last;
  }
  return async (url, init = {}) => {
    const target = new URL(url);
    require(target.origin === endpoint.origin && !target.username && !target.password &&
      !target.hash && target.href.length <= 4096 && init.method === 'PUT', 'request');
    // Snapshot before any await. Only the body forms used by the pinned upstream
    // client are supported; streams and mutable Request objects are not accepted.
    require(typeof init.body === 'string' || init.body instanceof Uint8Array, 'body');
    require((typeof init.body === 'string' ? init.body.length : init.body.byteLength)
      <= maxRequestBytes, 'request-size');
    const body = typeof init.body === 'string' ? new TextEncoder().encode(init.body) : init.body.slice();
    require(body.length <= maxRequestBytes, 'request-size');
    require(body.length <= maxTotalRequestBytes - claimedBytes, 'request-budget');
    const headers = Array.from(new Headers(init.headers).entries());
    require(headers.length <= 16 && headers.reduce((n, [k, v]) => n + k.length + v.length, 0)
      <= 4096, 'headers');
    const signal = init.signal ? AbortSignal.any([init.signal, AbortSignal.timeout(20_000)])
      : AbortSignal.timeout(20_000);
    signal.throwIfAborted();
    const request = { url: target.href, method: 'PUT', headers, bodyBytes: body.length,
      bodySha256: hex(new Uint8Array(await crypto.subtle.digest('SHA-256', body))) };
    const current = await certificate.inspect();
    require(equal(current.binding, binding) && current.phase === 'observed' && !current.cancelled,
      'gateway-blocked');
    signal.throwIfAborted();
    const row = await intents.claimGateway(copy(binding), copy(scope), owner, index, copy(request));
    retained(row, request, 'uncertain');
    claimedBytes += body.length;
    require(!row.cancelled, 'gateway-blocked');
    // A cancellation/abort after claim may leave an unsent but uncertain request.
    // Never clear that claim or automatically retry it.
    signal.throwIfAborted();
    // The already fingerprinted snapshot is emitted once and immediately closed.
    // Keep the SDK's exact PUT payload; never buffer or replay it in a fallback.
    const stream = new ReadableStream({ start(controller) {
      controller.enqueue(body); controller.close();
    } });
    const response = await transport(target.href, { method: 'PUT', headers, body: stream,
      duplex: 'half', signal,
      redirect: 'error', credentials: 'omit', cache: 'no-store', referrerPolicy: 'no-referrer' });
    require(!response.redirected && response.status >= 200 && response.status <= 599, 'response');
    const chunks = [];
    let length = 0;
    if (response.body) {
      const reader = response.body.getReader();
      for (;;) {
        const { done, value } = await reader.read();
        if (done) break;
        length += value.length;
        if (length > 64 * 1024) { await reader.cancel(); throw new GatewayRefusal('response-size'); }
        chunks.push(value);
      }
    }
    const bytes = new Uint8Array(length);
    let offset = 0;
    for (const chunk of chunks) { bytes.set(chunk, offset); offset += chunk.length; }
    const observed = await intents.observeGateway(copy(binding), copy(scope), owner, index,
      copy(request), response.status);
    const last = retained(observed, request, 'responded');
    require(last.status === response.status, 'response-binding');
    require(!observed.cancelled, 'gateway-blocked');
    signal.throwIfAborted();
    index += 1;
    // HTTP observation is not provider completion or permission to replay.
    return new Response(response.body === null ? null : bytes, {
      status: response.status, statusText: response.statusText, headers: response.headers });
  };
}
