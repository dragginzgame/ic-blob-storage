// Private IC integration fixture. Rust supplies/validates opaque Candid using the
// maintained DTOs; this module defines neither a consumer API nor production intent storage.
import { HttpAgent } from '@icp-sdk/core/agent';
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';

function callers(config) {
  const endpoint = new URL(config.url);
  function agent(seed) {
    return new HttpAgent({ host: endpoint.origin,
      identity: Ed25519KeyIdentity.generate(new Uint8Array(32).fill(seed)),
      rootKey: new Uint8Array(config.rootKey), shouldFetchRootKey: false,
      shouldSyncTime: false, retryTimes: 0,
      fetch: (url, init) => {
        const target = new URL(url);
        const allowed = [config.tenant, config.service].some(id =>
          [`/api/v4/canister/${id}/call`, `/api/v3/canister/${id}/read_state`].includes(target.pathname));
        if (target.origin !== endpoint.origin || !allowed || target.search || target.hash || init.method !== 'POST') {
          throw new Error('fixture admission route');
        }
        return fetch(url, { ...init, redirect: 'error', signal: AbortSignal.timeout(20_000) });
      },
    });
  }
  const uploader = agent(42), outsider = agent(43);
  async function call(agent, canister, methodName, arg) {
    const result = await agent.update(canister, { effectiveCanisterId: canister,
      methodName, arg: new Uint8Array(arg), callSync: true });
    if (!result.reply || result.reply.length > 4096) throw new Error('fixture admission reply');
    return Array.from(result.reply);
  }
  return { uploader, outsider, call };
}

export async function admitAndPrepare(config, commands) {
  const { uploader, outsider, call } = callers(config);
  const authority = commands.verifyAuthority ? {
    foreignAdmission: await call(outsider, config.tenant, 'admit', commands.admission),
    directAdmission: await call(uploader, config.service, 'blob_admit_upload', commands.permission),
    foreignPreparation: await call(outsider, config.service, 'blob_prepare_upload', commands.preparation),
  } : null;
  const admission = await call(uploader, config.tenant, 'admit', commands.admission);
  const preparation = await call(uploader, config.service, 'blob_prepare_upload', commands.preparation);
  return { authority, admission, preparation };
}

export async function finishConsumer(config, cancelled) {
  const { uploader, call } = callers(config);
  const commands = config.consumer;
  // Always inspect/register through the consumer; a gateway result is not completion.
  const pending = await call(uploader, config.tenant, 'register', commands.registration);
  if (!cancelled) return { pending, cancelled: null, withdrawn: null };
  // Local browser cancellation and tenant withdrawal are separate explicit calls.
  const tombstone = await call(uploader, config.tenant, 'cancel', commands.asset);
  const withdrawn = await call(uploader, config.tenant, 'revoke', commands.revocation);
  return { pending, cancelled: tombstone, withdrawn };
}
