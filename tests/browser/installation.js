// Passive service configuration fixture; not a provider wire contract or live target.

export function configuration(service, namespace) {
  const proposal = {
    service: service, operator: 'r7inp-6aaaa-aaaaa-aaabq-cai',
    payment_account: service, namespace: BigInt(namespace),
    resources: { max_tenants: 1, max_object_bytes: 10485760n, max_headers: 8, max_header_bytes: 1024,
      max_chunks: 20, max_tenant_chunks: 20, max_objects: 2, max_tenant_objects: 2,
      max_physical_bytes: 20971520n, max_liability_bytes: 20971520n, max_tenant_logical_bytes: 20971520n,
      max_references_per_object: 2, max_receipts_per_object: 3, max_active: 1, max_tenant_active: 1 },
    billing: { cashier: '72ch2-fiaaa-aaaar-qbsvq-cai', reserve: 100n,
      minimum_balance: 100n, target_balance: 200n, max_gateway_entries: 2, max_gateway_unique: 1 },
    funding: { allocated: 1000n, renewal_ceiling: 1000n, reserve: 100n, max_attempts: 1 },
  };
  return new TextEncoder().encode(JSON.stringify(proposal, (_key, value) =>
    typeof value === "bigint" ? JSON.rawJSON(value.toString()) : value));
}
