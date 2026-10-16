import { Principal } from '@icp-sdk/core/principal';
import { validNamespace } from './namespace.js';
import { integer } from './value-validation.js';

// Representation rules only. Callers retain their own roles, trust roots,
// transport restrictions, snapshots and durable transaction boundaries.
export const certificateBindingFields = Object.freeze(['key', 'service', 'tenant',
  'uploader', 'operation', 'root', 'project', 'bucket', 'permission']);

export function certificateBindingFailure(value) {
  if (!validNamespace(value.project, true) || !validNamespace(value.bucket)) return 'namespace';
  for (const field of ['service', 'tenant', 'uploader']) {
    try {
      if (typeof value[field] !== 'string' || value[field].length > 63 ||
        Principal.fromText(value[field]).toText() !== value[field]) return 'principal';
    } catch { return 'principal'; }
  }
  if (value.uploader === Principal.anonymous().toText()) return 'identity';
  if (typeof value.operation !== 'string' || !/^(0|[1-9][0-9]{0,38})$/.test(value.operation) ||
    BigInt(value.operation) >= (1n << 128n)) return 'operation';
  if (typeof value.root !== 'string' || !/^sha256:[0-9a-f]{64}$/.test(value.root)) return 'root';
  if (!Array.isArray(value.permission) || value.permission.length === 0 ||
    value.permission.length > 65536 || !value.permission.every(byte =>
      Number.isInteger(byte) && byte >= 0 && byte <= 255)) return 'permission';
  if (value.key !== `${value.service}:${value.tenant}:${value.operation}`) return 'key';
  return undefined;
}

export function validGatewayLimits({ maxRequests, maxRequestBytes, maxTotalRequestBytes }) {
  return integer(maxRequests, 1, 256) &&
    integer(maxRequestBytes, 1, 2 * 1024 * 1024) &&
    integer(maxTotalRequestBytes, 1, maxRequests * maxRequestBytes);
}

export function validJournalConfiguration({ database, maxSlots, mode }) {
  return typeof database === 'string' && database.length > 0 && database.length <= 128 &&
    integer(maxSlots, 1, 1_000_000) &&
    ['create', 'open'].includes(mode);
}

// Service origins permit HTTPS or loopback HTTP, without URL-carried credentials
// or routing suffixes. Callers parse/bound inputs and may require HTTPS separately.
export function validServiceEndpoint(url) {
  return !url.username && !url.password && url.pathname === '/' && !url.search &&
    !url.hash && (url.protocol === 'https:' || (url.protocol === 'http:' &&
      ['localhost', '127.0.0.1', '[::1]'].includes(url.hostname)));
}
