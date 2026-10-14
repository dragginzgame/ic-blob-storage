// In-memory substitute only. IndexedDB durability is exercised by the Chromium suite.
import { IntentRefusal, claimGateway, observeGateway } from '../../clients/browser/intent-state.js';
import { GatewayRefusal } from '../../clients/browser/gateway.js';

export function fixtureIntents(binding) {
  let row = { binding: structuredClone(binding), phase: 'observed', cancelled: false };
  const change = (transition, args) => {
    try {
      row = transition(row, ...structuredClone(args));
      return structuredClone(row);
    } catch (error) {
      if (error instanceof IntentRefusal) throw new GatewayRefusal(error.code);
      throw error;
    }
  };
  return {
    inspect: async () => structuredClone(row),
    claimGateway: async (...args) => change(claimGateway, args),
    observeGateway: async (...args) => change(observeGateway, args),
  };
}
