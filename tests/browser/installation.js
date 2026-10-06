// Passive service configuration fixture; not a provider wire contract or live target.
import { IDL } from '@icp-sdk/core/candid';
import { Principal } from '@icp-sdk/core/principal';

export function configuration(service, namespace) {
  const fields = (names, type) => Object.fromEntries(names.map(name => [name, type]));
  const resources = IDL.Record({
    ...fields(['max_tenants', 'max_headers', 'max_header_bytes', 'max_chunks',
      'max_tenant_chunks', 'max_objects', 'max_tenant_objects', 'max_references_per_object',
      'max_receipts_per_object', 'max_active', 'max_tenant_active'], IDL.Nat32),
    max_object_bytes: IDL.Nat64,
    ...fields(['max_physical_bytes', 'max_liability_bytes', 'max_tenant_logical_bytes'], IDL.Nat),
  });
  const type = IDL.Record({
    service: IDL.Principal, operator: IDL.Principal, payment_account: IDL.Principal,
    namespace: IDL.Nat, resources,
    billing: IDL.Record({ cashier: IDL.Principal,
      ...fields(['reserve', 'minimum_balance', 'target_balance'], IDL.Nat),
      ...fields(['max_gateway_entries', 'max_gateway_unique'], IDL.Nat32) }),
    funding: IDL.Record({ allocated: IDL.Nat, renewal_ceiling: IDL.Nat,
      reserve: IDL.Nat, max_attempts: IDL.Nat32 }),
    reads: IDL.Record({ ...fields(['sessions', 'tenant_sessions', 'reply_bytes'], IDL.Nat32),
      bytes: IDL.Nat64, tenant_bytes: IDL.Nat64 }),
  });
  return new Uint8Array(IDL.encode([type], [{
    service: Principal.fromText(service), operator: Principal.fromText('r7inp-6aaaa-aaaaa-aaabq-cai'),
    payment_account: Principal.fromText(service), namespace: BigInt(namespace),
    resources: { max_tenants: 1, max_object_bytes: 10485760n, max_headers: 8, max_header_bytes: 1024,
      max_chunks: 20, max_tenant_chunks: 20, max_objects: 2, max_tenant_objects: 2,
      max_physical_bytes: 20971520n, max_liability_bytes: 20971520n, max_tenant_logical_bytes: 20971520n,
      max_references_per_object: 2, max_receipts_per_object: 3, max_active: 1, max_tenant_active: 1 },
    billing: { cashier: Principal.fromText('72ch2-fiaaa-aaaar-qbsvq-cai'), reserve: 100n,
      minimum_balance: 100n, target_balance: 200n, max_gateway_entries: 2, max_gateway_unique: 1 },
    funding: { allocated: 1000n, renewal_ceiling: 1000n, reserve: 100n, max_attempts: 1 },
    reads: { sessions: 1, tenant_sessions: 1, reply_bytes: 1024, bytes: 1024n, tenant_bytes: 1024n },
  }]));
}
