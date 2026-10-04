// Native owns phase decisions. This bounded loop owns no durable state or retries.
import { isAbsolute } from 'node:path';
import { isDeepStrictEqual } from 'node:util';

export async function driveSession({ ready, phase, finish, execute, refuse, signal }) {
  const require = condition => { if (!condition) refuse('native-control'); };
  const integer = (value, maximum) => Number.isSafeInteger(value) && value >= 0 && value <= maximum;
  const exact = (value, fields) => value && Object.getPrototypeOf(value) === Object.prototype &&
    Object.keys(value).length === fields.length && fields.every(field => Object.hasOwn(value, field));
  function frame(value) {
    const shapes = { status: ['phase', 'index'], prepare: ['phase', 'index', 'source_run'],
      transfer: ['phase', 'index', 'source_transfer'], verify: ['phase', 'index', 'source_observation'], map: ['phase'] };
    require(typeof value?.phase === 'string' && Object.hasOwn(shapes, value.phase));
    const fields = shapes[value.phase];
    require(exact(value, fields));
    if (value.phase !== 'map') require(integer(value.index, ready.files - 1));
    for (const field of fields.filter(field => field.startsWith('source_'))) {
      require(value[field] === null || (typeof value[field] === 'string' && isAbsolute(value[field])));
    }
    return value;
  }
  async function response(action) {
    let abort;
    const stopped = new Promise((_, reject) => {
      abort = () => reject(new Error('closed'));
      signal.addEventListener('abort', abort, { once: true });
    });
    try {
      signal.throwIfAborted();
      const value = structuredClone(await Promise.race([action(), stopped]));
      require(Buffer.byteLength(JSON.stringify(value)) <= 8 * 1024 * 1024);
      return value;
    } catch { refuse(signal.aborted ? 'closed' : 'native-control'); }
    finally { signal.removeEventListener('abort', abort); }
  }
  async function conclude(report, complete, code) {
    const final = await response(() => finish(signal));
    require(exact(final, ['report', 'exit_code']) && final.report &&
      Object.getPrototypeOf(final.report) === Object.prototype &&
      (final.exit_code === 0 ? !Object.hasOwn(final.report, 'error') :
        [2, 3].includes(final.exit_code) && typeof final.report.error === 'string'));
    if (complete) {
      require(final.exit_code === 0 && isDeepStrictEqual(final.report, report) &&
        exact(report, ['schema', 'operation', 'authentication', 'inventory_sha256', 'installation_sha256',
          'files', 'blockers', 'atomic_snapshot', 'publication_lease', 'provider_requests',
          'public_serving_qualified', 'retry_authorized', 'all_references_live']) &&
        report.schema === 1 && report.operation === 'publish_map' && report.authentication === 'query_signatures' &&
        report.inventory_sha256 === ready.inventory_sha256 && report.installation_sha256 === ready.installation_sha256 &&
        report.atomic_snapshot === false && report.publication_lease === false && report.provider_requests === 0 &&
        report.public_serving_qualified === false && report.retry_authorized === false &&
        Array.isArray(report.blockers) && report.blockers.length === 0 &&
        Array.isArray(report.files) && report.files.length === ready.files &&
        report.files.every((file, index) => file?.index === index));
    }
    return { schema: 1, operation: 'publication_driver', state: complete ? 'complete' : 'stopped',
      ...(code ? { code } : {}), report, native_result: final, browser_jobs: jobs, retry_authorized: false };
  }
  let selected = { phase: 'status', index: 0 }, jobs = 0;
  for (let step = 0; step < ready.max_steps; step++) {
    const event = await response(() => phase(structuredClone(selected), signal));
    require(exact(event, ['schema', 'event', 'step', 'next_index', 'report', 'next_frame']) &&
      event.schema === 1 && event.event === 'phase' && event.step === step &&
      integer(event.next_index, ready.files) && event.report &&
      Object.getPrototypeOf(event.report) === Object.prototype);
    const next = event.next_frame === null ? null : frame(event.next_frame);
    if (selected.phase === 'transfer' && event.report.operation === 'publish_session_transfer') {
      require(event.report.file_index === selected.index && typeof event.report.native_phase === 'string' &&
        isAbsolute(event.report.native_phase) && event.report.retry_authorized === false);
      // A completed or uncertain SDK result still needs the native verifier.
      // A process/control exception stops this invocation; it is never retried.
      jobs++;
      await execute({ index: selected.index, action: 'transfer', nativePhase: event.report.native_phase });
    }
    if (next === null) {
      const complete = selected.phase === 'map' && event.report.all_references_live === true;
      if (complete) require(event.next_index === ready.files);
      return conclude(event.report, complete);
    }
    if (step + 1 === ready.max_steps) return conclude(event.report, false, 'step_budget_exhausted');
    selected = next;
  }
  refuse('native-control');
}
