// In-memory substitute only. IndexedDB durability is exercised by the Chromium suite.
import assert from 'node:assert/strict';
import { GatewayRefusal } from '../../clients/browser/gateway.js';

export function fixtureIntents(binding) {
  let row = { binding, phase: 'observed', cancelled: false };
  const same = (a, b) => assert.deepEqual(a, b);
  return {
    inspect: async () => structuredClone(row),
    claimGateway: async (binding, scope, owner, index, request) => {
      same(binding, row.binding);
      if (row.cancelled) throw new GatewayRefusal('gateway-blocked');
      const gateway = row.gateway ?? { scope, owner, requests: [] };
      if (gateway.owner !== owner) throw new GatewayRefusal('gateway-session');
      same(gateway.scope, scope);
      if (index !== gateway.requests.length || index >= scope.maxRequests) throw new GatewayRefusal('gateway-capacity');
      const previous = gateway.requests.at(-1);
      if (previous && (previous.phase !== 'responded' || previous.status < 200 || previous.status >= 300)) {
        throw new GatewayRefusal('gateway-uncertain');
      }
      if (gateway.requests.some(entry => entry.request.url === request.url)) throw new GatewayRefusal('gateway-repeat');
      const used = gateway.requests.reduce((n, entry) => n + entry.request.bodyBytes, 0);
      if (request.bodyBytes > scope.maxTotalRequestBytes - used) throw new GatewayRefusal('gateway-budget');
      gateway.requests.push({ request, phase: 'uncertain' });
      row = { ...row, gateway };
      return structuredClone(row);
    },
    observeGateway: async (binding, scope, owner, index, request, status) => {
      same(binding, row.binding); same(scope, row.gateway.scope);
      assert.equal(owner, row.gateway.owner);
      assert.equal(row.gateway.requests.length, index + 1);
      same(request, row.gateway.requests[index].request);
      assert.equal(row.gateway.requests[index].phase, 'uncertain');
      row.gateway.requests[index] = { request, phase: 'responded', status };
      return structuredClone(row);
    },
  };
}
