import { Principal } from '@icp-sdk/core/principal';
import { validNamespace } from './namespace.js';
import { certificateBindingFailure, certificateBindingFields, validGatewayLimits, validUtf8Text } from './validation.js';
export const actions = Object.freeze(['upload', 'inspect', 'recover-certificate', 'cancel']);

/** Local worker refusal; it never establishes completion or retry authority. */
export class WorkerRefusal extends Error {
  constructor(code) { super(code); this.name = 'WorkerRefusal'; this.code = code; }
}
const require = (condition, code) => { if (!condition) throw new WorkerRefusal(code); };
export const integer = (value, min, max) => Number.isSafeInteger(value) && value >= min && value <= max;
export function fields(value, required, optional = []) {
  require(value && Object.getPrototypeOf(value) === Object.prototype &&
    required.every(key => Object.hasOwn(value, key)) &&
    Object.keys(value).every(key => [...required, ...optional].includes(key)), 'message');
}
function endpoint(value, gateway) {
  require(typeof value === 'string' && value.length <= 4096, 'origin');
  let url;
  try { url = new URL(value); } catch { throw new WorkerRefusal('origin'); }
  require(!url.username && !url.password && url.pathname === '/' && !url.search &&
    !url.hash && (url.protocol === 'https:' || (!gateway && url.protocol === 'http:' &&
      ['localhost', '127.0.0.1', '[::1]'].includes(url.hostname))), 'origin');
  return url.origin;
}

// One authority/configuration predicate, reusable before any IndexedDB open.
export function publicationConfiguration(options) {
  fields(options, ['host', 'identity', 'rootKey', 'service', 'tenant', 'uploader',
    'project', 'bucket', 'intents', 'origin', 'maxBodyBytes', 'maxRequests',
    'maxRequestBytes', 'maxTotalRequestBytes', 'maxJobs', 'timeoutSeconds'],
    ['signal', 'certificateFetch', 'gatewayFetch']);
  const host = endpoint(options.host, false), origin = endpoint(options.origin, true);
  const scope = Object.fromEntries(['service', 'tenant', 'uploader', 'project', 'bucket']
    .map(key => [key, options[key]]));
  for (const key of ['service', 'tenant', 'uploader']) {
    require(typeof scope[key] === 'string' && scope[key].length <= 63 &&
      Principal.fromText(scope[key]).toText() === scope[key], 'principal');
  }
  const { identity, maxBodyBytes, maxRequests, maxRequestBytes,
    maxTotalRequestBytes, maxJobs, timeoutSeconds } = options;
  require(scope.uploader !== Principal.anonymous().toText() &&
    identity.getPrincipal().toText() === scope.uploader, 'identity');
  require(validNamespace(scope.project, true) && validNamespace(scope.bucket), 'namespace');
  require(options.rootKey instanceof Uint8Array && options.rootKey.length > 0 &&
    options.rootKey.length <= 1024, 'trust-root');
  const rootKey = options.rootKey.slice();
  require(integer(maxBodyBytes, 1, 1024 * 1024 * 1024) && integer(maxJobs, 1, 32769) &&
    integer(timeoutSeconds, 1, 3600) &&
    validGatewayLimits({ maxRequests, maxRequestBytes, maxTotalRequestBytes }), 'limits');
  return { host, origin, scope, rootKey };
}

// Bound and own a private job before cloning/awaits at either process boundary.
export function snapshotPublicationJob(request, scope, maxBodyBytes, lastId = 0) {
  fields(request, ['schema', 'id', 'action', 'index', 'binding'], ['snapshot']);
  require(request.schema === 1 && integer(request.id, 1, 1_000_000) &&
    request.id > lastId && actions.includes(request.action) &&
    integer(request.index, 0, 4095), 'message');
  const binding = request.binding;
  fields(binding, certificateBindingFields);
  require(Object.entries(scope).every(([key, value]) => binding[key] === value), 'scope');
  const refusal = certificateBindingFailure(binding);
  require(!refusal, refusal);
  if (request.action === 'upload') {
    fields(request.snapshot, ['body', 'bodySha256', 'manifestJSON'], ['contentType', 'filename', 'cacheControl']);
    require(request.snapshot.body instanceof Uint8Array &&
      integer(request.snapshot.body.length, 1, maxBodyBytes), 'body-size');
    require(typeof request.snapshot.bodySha256 === 'string' &&
      /^[0-9a-f]{64}$/.test(request.snapshot.bodySha256), 'body-digest');
    require(validUtf8Text(request.snapshot.manifestJSON, 256 * 1024, 1), 'manifest-size');
    for (const key of ['contentType', 'filename', 'cacheControl']) require(request.snapshot[key] === undefined ||
      validUtf8Text(request.snapshot[key], 4096), 'metadata-hint');
  } else require(!Object.hasOwn(request, 'snapshot'), 'message');
  // A bounded view can have a much larger backing buffer. Own only the selected
  // bytes before structured cloning or transferring to another process.
  if (request.action !== 'upload') return structuredClone(request);
  const { body, ...metadata } = request.snapshot;
  const selected = body.slice();
  const owned = structuredClone({ ...request, snapshot: metadata });
  owned.snapshot.body = selected;
  return owned;
}
