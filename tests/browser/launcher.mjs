// Actual maintained Node/Chromium/profile boundary; no IC/provider request.
import { chromium } from 'playwright';
import { Ed25519KeyIdentity } from '@icp-sdk/core/identity';
import { createServer } from 'node:net';
import { mkdir, writeFile, readFile, lstat, symlink, rename, unlink } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { resolve, join } from 'node:path';
import assert from 'node:assert/strict';
import { launchPublicationBrowser, LauncherRefusal } from '../../clients/browser/launcher.mjs';
const report = resolve(process.argv[2]);
await mkdir(report, { mode: 0o700 });
const reservation = createServer();
await new Promise(resolve => reservation.listen(0, '127.0.0.1', resolve));
const port = reservation.address().port;
const signer = Ed25519KeyIdentity.generate(new Uint8Array(32).fill(42));
const cases = [], requests = [], unexpected = [];
const origin = `http://127.0.0.1:${port}`;
const assetOrigins = new Set([origin]);
let launches = 0;
const engine = { async launchPersistentContext(...args) {
  launches++;
  const context = await chromium.launchPersistentContext(...args);
  context.on('request', request => {
    requests.push({ method: request.method(), path: new URL(request.url()).pathname });
    if (!assetOrigins.has(new URL(request.url()).origin)) unexpected.push(new URL(request.url()).origin);
  });
  return context;
} };
const options = mode => ({ profile: join(report, 'profile'), assetPort: port, nativeSession: null,
  hostBundle: resolve('.tmp/browser/publication-host.js'), workerBundle: resolve('.tmp/browser/publication-worker.js'),
  bootstrap: { schema: 1, operation: 'bootstrap', signer: { kind: 'ed25519', json: JSON.stringify(signer.toJSON()) },
    configuration: { host: origin, rootKey: [1], service: 'aaaaa-aa', tenant: 'rrkah-fqaaa-aaaaa-aaaaq-cai',
      uploader: signer.getPrincipal().toText(), project: 'bootstrap-fixture', bucket: 'test', origin: 'https://substitute.invalid',
      maxBodyBytes: 1024, maxRequests: 2, maxRequestBytes: 65536, maxTotalRequestBytes: 131072, maxJobs: 32, timeoutSeconds: 30 },
    journal: { database: 'native-browser-v1', mode, maxSlots: 2 } } });
const binding = { key: 'aaaaa-aa:rrkah-fqaaa-aaaaa-aaaaq-cai:1', service: 'aaaaa-aa',
  tenant: 'rrkah-fqaaa-aaaaa-aaaaq-cai', uploader: signer.getPrincipal().toText(),
  project: 'bootstrap-fixture', bucket: 'test', operation: '1', root: 'sha256:' + '1'.repeat(64), permission: [68, 73, 68, 76] };
const bodyBytes = Buffer.from([0, 128, 255]);
const body = join(report, 'body.bin'); await writeFile(body, bodyBytes, { flag: 'wx', mode: 0o600 });
const transfer = { binding, body, bytes: '3', body_sha256: createHash('sha256').update(bodyBytes).digest('hex'),
  manifest_json: '{"tree_type":"DSBMTWH"}', preparation: {} };
const refusal = code => error => error instanceof LauncherRefusal && error.code === code;
let bridge;
try {
  await assert.rejects(launchPublicationBrowser(engine, options('create')), refusal('launch'));
  await assert.rejects(lstat(join(report, 'profile')), { code: 'ENOENT' }); cases.push('occupied_port_refuses_before_profile');
  await new Promise(resolve => reservation.close(resolve));
  const selected = options('create');
  const starting = launchPublicationBrowser(engine, selected);
  selected.profile = join(report, 'caller-mutated-profile'); selected.assetPort = 1;
  selected.bootstrap.signer.json = 'caller-mutated-key';
  bridge = await starting;
  await assert.rejects(lstat(selected.profile), { code: 'ENOENT' });
  cases.push('launch_selection_owned_before_filesystem_awaits');
  const bindingPath = join(report, 'profile', 'publication-binding.json');
  const profileBinding = await readFile(bindingPath);
  const saved = JSON.parse(profileBinding);
  assert.equal(saved.origin, origin); assert.equal(saved.profile, join(report, 'profile'));
  assert.equal(saved.signer.sha256, createHash('sha256').update(JSON.stringify(options('create').bootstrap.signer)).digest('hex'));
  assert.equal(profileBinding.includes(signer.toJSON()[1]), false);
  assert.equal((await lstat(bindingPath)).mode & 0o777, 0o600);
  cases.push('private_passive_binding_retains_scope_without_signer_secret');
  const inspection = await bridge.execute({ id: 1, index: 0, action: 'inspect', binding });
  assert.equal(inspection.state, 'inspected'); assert.deepEqual(inspection.journal, { present: false });
  cases.push('explicit_profile_and_origin_start_without_effects');
  const pending = bridge.execute({ id: 2, index: 0, action: 'upload', transfer });
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'inspect', binding }), refusal('busy'));
  transfer.body_sha256 = '0'.repeat(64); // The bridge already owns the original request.
  const result = await pending; assert.equal(result.error, 'root'); assert.equal(result.id, 2);
  transfer.body_sha256 = createHash('sha256').update(bodyBytes).digest('hex'); cases.push('owned_request_and_single_job');
  const changed = structuredClone(transfer); changed.body_sha256 = '0'.repeat(64);
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('body-digest'));
  changed.body_sha256 = transfer.body_sha256; changed.bytes = '2';
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('body-size'));
  changed.bytes = '1025';
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('body-size'));
  changed.bytes = '03';
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('body-size'));
  cases.push('changed_truncated_oversized_and_noncanonical_body_refuse');
  changed.bytes = '3'; changed.body = join(report, 'link.bin'); await symlink(body, changed.body);
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('file'));
  changed.body = report;
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('body-size'));
  cases.push('symlinks_and_directories_are_not_bodies');
  changed.body = join(report, 'pipe');
  await promisify(execFile)('mkfifo', [changed.body]);
  await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }), refusal('body-size'));
  cases.push('fifo_without_writer_refuses_without_blocking');
  for (const hints of [{ filename: null }, { filename: 'β'.repeat(2049) },
    {cache_control:null}, {cache_control:'β'.repeat(2049)}, { inferred_type: 'text/plain' }]) {
    changed.body = body; changed.preparation = hints;
    await assert.rejects(bridge.execute({ id: 3, index: 0, action: 'upload', transfer: changed }),
      refusal(Object.hasOwn(hints, 'inferred_type') ? 'configuration' : 'metadata-hint'));
  }
  cases.push('metadata_hints_have_exact_fields_and_utf8_bounds');
  assert.deepEqual((await bridge.execute({ id: 3, index: 0, action: 'inspect', binding })).journal, { present: false });
  await bridge.close(); await bridge.close();
  await assert.rejects(bridge.execute({ id: 4, index: 0, action: 'inspect', binding }), refusal('closed'));
  await assert.rejects(launchPublicationBrowser(engine, options('create')), refusal('launch'));
  cases.push('close_preserves_profile_and_refuses_replacement');
  const foreignPort = createServer();
  await new Promise(resolve => foreignPort.listen(0, '127.0.0.1', resolve));
  const changedOrigin = options('open'); changedOrigin.assetPort = foreignPort.address().port;
  assetOrigins.add(`http://127.0.0.1:${changedOrigin.assetPort}`);
  await new Promise(resolve => foreignPort.close(resolve));
  await assert.rejects(launchPublicationBrowser(engine, changedOrigin), refusal('profile-binding'));
  const missing = options('open'); missing.profile = join(report, 'missing-profile');
  await assert.rejects(launchPublicationBrowser(engine, missing), refusal('launch'));
  await assert.rejects(lstat(missing.profile), { code: 'ENOENT' });
  const linked = options('open'); linked.profile = join(report, 'linked-profile');
  await symlink(join(report, 'profile'), linked.profile);
  await assert.rejects(launchPublicationBrowser(engine, linked), refusal('profile'));
  cases.push('changed_origin_missing_and_symlink_profiles_refuse_without_replacement');
  const beforeRefusals = launches;
  for (const change of [
    value => { value.bootstrap.signer.json = JSON.stringify(Ed25519KeyIdentity.generate(new Uint8Array(32).fill(43)).toJSON()); },
    value => { value.bootstrap.signer.kind = 'secp256k1'; },
    value => { value.bootstrap.configuration.rootKey = [2]; },
    ...['host', 'service', 'tenant', 'uploader', 'project', 'bucket', 'origin'].map(key =>
      value => { value.bootstrap.configuration[key] += '-changed'; }),
    value => { value.bootstrap.journal.database += '-changed'; },
    value => { value.bootstrap.journal.maxSlots++; },
  ]) {
    const candidate = options('open'); change(candidate);
    await assert.rejects(launchPublicationBrowser(engine, candidate), refusal('profile-binding'));
  }
  const changedBundle = join(report, 'changed-bundle.js');
  await writeFile(changedBundle, '// changed executable', { flag: 'wx', mode: 0o600 });
  for (const key of ['hostBundle', 'workerBundle']) {
    const candidate = options('open'); candidate[key] = changedBundle;
    await assert.rejects(launchPublicationBrowser(engine, candidate), refusal('profile-binding'));
  }
  assert.equal(launches, beforeRefusals);
  assert.deepEqual(await readFile(bindingPath), profileBinding);
  cases.push('changed_signer_trust_scope_journal_and_assets_refuse_before_chromium');
  for (const bytes of [Buffer.from('{'), Buffer.from(JSON.stringify({ ...saved, extra: true })),
    Buffer.from(JSON.stringify({ ...saved, format: 'unknown' }))]) {
    await writeFile(bindingPath, bytes);
    await assert.rejects(launchPublicationBrowser(engine, options('open')), refusal('profile-binding'));
  }
  await writeFile(bindingPath, profileBinding);
  const retainedBinding = join(report, 'original-binding.json');
  await rename(bindingPath, retainedBinding);
  await assert.rejects(launchPublicationBrowser(engine, options('open')), refusal('profile-binding'));
  await symlink(retainedBinding, bindingPath);
  await assert.rejects(launchPublicationBrowser(engine, options('open')), refusal('profile-binding'));
  await unlink(bindingPath); await rename(retainedBinding, bindingPath);
  const copied = options('open'); copied.profile = join(report, 'copied-profile');
  await mkdir(copied.profile, { mode: 0o700 });
  await writeFile(join(copied.profile, 'publication-binding.json'), profileBinding, { flag: 'wx', mode: 0o600 });
  await assert.rejects(launchPublicationBrowser(engine, copied), refusal('profile-binding'));
  assert.equal(launches, beforeRefusals);
  assert.deepEqual(await readFile(bindingPath), profileBinding);
  cases.push('partial_missing_symlink_and_relocated_bindings_refuse_without_replacement');
  bridge = await launchPublicationBrowser(engine, options('open'));
  assert.deepEqual((await bridge.execute({ id: 1, index: 0, action: 'inspect', binding })).journal, { present: false });
  await bridge.close(); cases.push('same_origin_and_profile_reopen_original_journal');
  const deadline = options('open'); deadline.bootstrap.configuration.timeoutSeconds = 1;
  bridge = await launchPublicationBrowser(engine, deadline);
  await new Promise(resolve => setTimeout(resolve, 1500));
  await assert.rejects(bridge.execute({ id: 1, index: 0, action: 'inspect', binding }), refusal('closed'));
  await bridge.close(); assert((await lstat(join(report, 'profile'))).isDirectory()); cases.push('deadline_closes_browser_without_erasing_profile');
  const interrupted = options('create'); interrupted.profile = join(report, 'interrupted-profile');
  let observedBindings = 0;
  const interruptedEngine = { async launchPersistentContext(profile) {
    const retained = JSON.parse(await readFile(join(profile, 'publication-binding.json')));
    assert.equal(retained.origin, origin); assert.deepEqual(retained.journal, saved.journal);
    observedBindings++; throw new Error('private launch diagnostic');
  } };
  await assert.rejects(launchPublicationBrowser(interruptedEngine, interrupted), refusal('launch'));
  const original = await readFile(join(interrupted.profile, 'publication-binding.json'));
  interrupted.bootstrap.journal.mode = 'open';
  await assert.rejects(launchPublicationBrowser(interruptedEngine, interrupted), refusal('launch'));
  assert.equal(observedBindings, 2);
  assert.deepEqual(await readFile(join(interrupted.profile, 'publication-binding.json')), original);
  cases.push('launch_failure_retains_binding_before_browser_and_reopen_never_replaces_it');
  const native = options('create'); native.profile = join(report, 'native-profile');
  native.nativeSession = join(report, 'native-session'); await mkdir(native.nativeSession);
  const hash = bytes => createHash('sha256').update(bytes).digest('hex');
  const inventory = Buffer.from('{}'), installation = Buffer.from([68, 73, 68, 76]);
  const nativeIntent = { format: 'ic-blob-storage/publication-session:retained-browser-handoffs',
    operation: 'publish_session', source_session: null, files: 1,
    network: 'local', namespace: '1', operator: binding.uploader, verifier: binding.uploader,
    max_steps: 9, max_service_updates: 2, max_service_queries: 50, timeout_seconds: 30,
    max_provider_requests: 0, automatic_retries: 0,
    input_verification: 'one_complete_startup_pass_and_selected_body_before_setup',
    service: binding.service, tenant: binding.tenant, uploader: binding.uploader,
    service_url: origin, gateway: native.bootstrap.configuration.origin,
    root_key_sha256: hash(Buffer.from([1])), inventory_sha256: hash(inventory), installation_sha256: hash(installation),
    browser: { format: 'ic-blob-storage/browser-selection', session: native.nativeSession, profile: native.profile,
      project: native.bootstrap.configuration.project, bucket: native.bootstrap.configuration.bucket, asset_port: port,
      signer_sha256: hash(JSON.stringify(native.bootstrap.signer)), host_sha256: hash(await readFile(native.hostBundle)),
      worker_sha256: hash(await readFile(native.workerBundle)), database: native.bootstrap.journal.database,
      max_slots: native.bootstrap.journal.maxSlots } };
  const nativeReady = { schema: 1, operation: 'publish_session', event: 'ready', files: 1,
    max_steps: 9, provider_requests: 0, input_verification: 'session_start', retry_authorized: false, publication_lease: false,
    inventory_sha256: nativeIntent.inventory_sha256, installation_sha256: nativeIntent.installation_sha256 };
  const intentPath = join(native.nativeSession, 'intent.json'), readyPath = join(native.nativeSession, 'ready.json');
  await writeFile(intentPath, JSON.stringify(nativeIntent)); await writeFile(readyPath, JSON.stringify(nativeReady));
  await writeFile(join(native.nativeSession, 'inventory.json'), inventory);
  await writeFile(join(native.nativeSession, 'installation.candid'), installation);
  const beforeNative = launches;
  for (const [key, value] of [['service', 'foreign'], ['root_key_sha256', '0'.repeat(64)],
    ['gateway', 'https://foreign.invalid'], ['inventory_sha256', '0'.repeat(64)], ['browser', null]]) {
    await writeFile(intentPath, JSON.stringify({ ...nativeIntent, [key]: value }));
    await assert.rejects(launchPublicationBrowser(engine, native), refusal('native-session'));
  }
  await writeFile(intentPath, JSON.stringify(nativeIntent));
  await rename(readyPath, join(native.nativeSession, 'retained-ready.json'));
  await assert.rejects(launchPublicationBrowser(engine, native), refusal('native-session'));
  await rename(join(native.nativeSession, 'retained-ready.json'), readyPath);
  assert.equal(launches, beforeNative); await assert.rejects(lstat(native.profile), { code: 'ENOENT' });
  cases.push('mismatched_or_partial_native_intent_refuses_before_profile_and_chromium');
  bridge = await launchPublicationBrowser(engine, native);
  assert.deepEqual((await bridge.execute({ id: 1, index: 0, action: 'inspect', binding })).journal, { present: false });
  await assert.rejects(bridge.execute({ id: 2, index: 0, action: 'upload', transfer }), refusal('native-phase'));
  const initialPhase = join(native.nativeSession, 'step-0000'), recoveryPhase = join(native.nativeSession, 'step-0001');
  await mkdir(initialPhase); await mkdir(recoveryPhase);
  const handoff = { format: 'ic-blob-storage/browser-handoff', session: native.nativeSession,
    file_index: 0, source_run: join(native.nativeSession, 'setup'), source_transfer: null, transfer };
  await writeFile(join(initialPhase, 'request.json'), JSON.stringify({ phase: 'transfer', index: 0, source_transfer: null }));
  await writeFile(join(initialPhase, 'transfer.json'), JSON.stringify(handoff));
  await writeFile(join(recoveryPhase, 'request.json'), JSON.stringify({ phase: 'transfer', index: 0, source_transfer: initialPhase }));
  const recovering = { ...handoff, source_transfer: initialPhase };
  await writeFile(join(recoveryPhase, 'transfer.json'), JSON.stringify(recovering));
  const command = { id: 2, index: 0, action: 'transfer', nativePhase: recoveryPhase };
  const recovered = await bridge.execute(command);
  assert.equal(recovered.action, 'recover-certificate'); assert.equal(recovered.error, 'history-missing');
  assert.equal(recovered.retry_authorized, false);
  for (const value of [{ ...recovering, file_index: 1 }, { ...recovering, extra: true },
    { ...recovering, transfer: { ...transfer, body_sha256: '0'.repeat(64) } }]) {
    await writeFile(join(recoveryPhase, 'transfer.json'), JSON.stringify(value));
    await assert.rejects(bridge.execute({ ...command, id: 3 }), refusal('native-phase'));
  }
  await writeFile(join(recoveryPhase, 'transfer.json'), '{');
  await assert.rejects(bridge.execute({ ...command, id: 3 }), refusal('native-phase'));
  await writeFile(join(recoveryPhase, 'transfer.json'), JSON.stringify(recovering));
  assert.deepEqual((await bridge.execute({ id: 3, index: 0, action: 'inspect', binding })).journal, { present: false });
  cases.push('native_handoff_recovery_without_browser_history_never_becomes_upload_and_tampering_refuses');
  await bridge.close(); native.bootstrap.journal.mode = 'open';
  const originalNativeBinding = await readFile(join(native.profile, 'publication-binding.json'));
  assert.equal(JSON.parse(originalNativeBinding).native_session.intent_sha256, hash(await readFile(intentPath)));
  const bypass = { ...native, nativeSession: null };
  await assert.rejects(launchPublicationBrowser(engine, bypass), refusal('profile-binding'));
  // Even a scope-equivalent reserialization changes original byte provenance.
  await writeFile(intentPath, JSON.stringify(nativeIntent, null, 2));
  await assert.rejects(launchPublicationBrowser(engine, native), refusal('profile-binding'));
  await writeFile(intentPath, JSON.stringify(nativeIntent));
  bridge = await launchPublicationBrowser(engine, native);
  let controls = 0;
  const stoppedFinal = { report: { operation: 'publish_session', state: 'blocked' }, exit_code: 0 };
  const peer = (phase, ready = nativeReady, finish = async () => stoppedFinal) => ({ ready, phase, finish });
  await assert.rejects(bridge.driveSession(peer(async () => { controls++; },
    { ...nativeReady, inventory_sha256: '0'.repeat(64) })), refusal('native-session'));
  assert.equal(controls, 0);
  const budgeted = await bridge.driveSession(peer(async selected => {
    controls++; assert.deepEqual(selected, { phase: 'status', index: 0 });
    return { schema: 1, event: 'phase', step: 0, next_index: 0, report: { file_live: false },
      next_frame: { phase: 'prepare', index: 0, source_run: null } };
  }, { ...nativeReady, max_steps: 1 }));
  assert.equal(controls, 1); assert.equal(budgeted.code, 'step_budget_exhausted');
  assert.equal(budgeted.browser_jobs, 0);
  assert.deepEqual(budgeted.native_result, stoppedFinal);
  const circularFrame = { phase: 'map' }; circularFrame.extra = circularFrame;
  for (const next_frame of [{ phase: 'transfer', index: 1, source_transfer: null },
    { phase: 'map', retry: true }, { phase: 'prepare', index: 0, source_run: 'relative' },
    { phase: 'constructor' }, { phase: 'isPrototypeOf' },
    { phase: { toString: 'PRIVATE_PHASE_DIAGNOSTIC' } },
    { phase: 'map', PRIVATE_VALUE: 1n }, circularFrame]) {
    let finished = false;
    await assert.rejects(bridge.driveSession(peer(async () => ({ schema: 1, event: 'phase',
      step: 0, next_index: 0, report: {}, next_frame }), nativeReady,
    async () => { finished = true; return stoppedFinal; })),
    error => refusal('native-control')(error) && !String(error).includes('PRIVATE_'));
    assert.equal(finished, false);
    native.bootstrap.journal.mode = 'open'; bridge = await launchPublicationBrowser(engine, native);
  }
  let unblock;
  const driving = bridge.driveSession(peer(async () => new Promise(resolve => { unblock = resolve; })));
  await assert.rejects(bridge.execute({ id: 1, index: 0, action: 'inspect', binding }), refusal('busy'));
  await bridge.close(); await assert.rejects(driving, refusal('closed'));
  unblock({ schema: 1, event: 'phase', step: 0, next_index: 0, report: {}, next_frame: null });
  bridge = await launchPublicationBrowser(engine, native);
  cases.push('native_driver_bounds_control_hints_and_excludes_concurrent_jobs_until_closed');
  // Metadata-only peers exercise the completion boundary, not service authority.
  const map = { schema: 1, operation: 'publish_map', authentication: 'query_signatures',
    inventory_sha256: nativeReady.inventory_sha256, installation_sha256: nativeReady.installation_sha256,
    files: [{ index: 0 }], blockers: [], atomic_snapshot: false, publication_lease: false,
    provider_requests: 0, public_serving_qualified: false, retry_authorized: false, all_references_live: true };
  const mapPeer = (report = map, finish = async () => ({ report, exit_code: 0 })) => peer(async selected => ({
    schema: 1, event: 'phase', step: selected.phase === 'status' ? 0 : 1,
    next_index: 1, report: selected.phase === 'map' ? report : { file_live: true },
    next_frame: selected.phase === 'map' ? null : { phase: 'map' },
  }), nativeReady, finish);
  for (const finish of [
    async () => ({ report: { ...map, files: [] }, exit_code: 0 }),
    async () => ({ report: { error: 'transport' }, exit_code: 3 }),
    async () => ({ report: map, exit_code: 7 }),
    async () => ({ report: { ...map, PRIVATE_VALUE: 1n }, exit_code: 0 }),
    async () => {
      const final = { report: { ...map }, exit_code: 0 }; final.report.extra = final; return final;
    },
    async () => { throw new Error('lost final'); },
  ]) {
    await assert.rejects(bridge.driveSession(mapPeer(map, finish)), refusal('native-control'));
    bridge = await launchPublicationBrowser(engine, native);
  }
  for (const change of [{ inventory_sha256: '0'.repeat(64) }, { files: [] },
    { files: [{ index: 1 }] }, { blockers: [{ code: 'incomplete' }] }, { publication_lease: true },
    { files: [{ index: 0, details: 'x'.repeat(8 * 1024 * 1024) }] }]) {
    await assert.rejects(bridge.driveSession(mapPeer({ ...map, ...change })), refusal('native-control'));
    bridge = await launchPublicationBrowser(engine, native);
  }
  const complete = await bridge.driveSession(mapPeer());
  assert.equal(complete.state, 'complete'); assert.deepEqual(complete.native_result, { report: map, exit_code: 0 });
  let finalSignal, releaseFinal, enterFinal;
  const atFinal = new Promise(resolve => { enterFinal = resolve; });
  const finishing = bridge.driveSession(mapPeer(map, signal => {
    finalSignal = signal; enterFinal(); return new Promise(resolve => { releaseFinal = resolve; });
  }));
  await Promise.race([atFinal, finishing.then(() => assert.fail('expected pending final result'))]);
  await assert.rejects(bridge.execute({ id: 1, index: 0, action: 'inspect', binding }), refusal('busy'));
  await bridge.close(); await assert.rejects(finishing, refusal('closed')); assert.equal(finalSignal.aborted, true);
  releaseFinal({ report: map, exit_code: 0 });
  bridge = await launchPublicationBrowser(engine, native);
  cases.push('native_driver_requires_exact_current_complete_map_final_report_and_exit_and_cancels_finish');
  await bridge.close(); assert.deepEqual(await readFile(join(native.profile, 'publication-binding.json')), originalNativeBinding);
  cases.push('original_native_session_binding_reopens_and_cannot_be_bypassed_or_rewritten');
  const shutdownEngine = mode => ({ async launchPersistentContext(...args) {
    const context = await engine.launchPersistentContext(...args);
    return {
      async close() { await context.close(); throw new Error('PRIVATE_SHUTDOWN_DIAGNOSTIC'); },
      async newPage() {
        if (mode === 'startup') throw new Error('PRIVATE_STARTUP_DIAGNOSTIC');
        const page = await context.newPage(); let evaluations = 0;
        return {
          goto: (...args) => page.goto(...args),
          evaluate: (...args) => {
            if (mode === 'execute' && ++evaluations > 1) throw new Error('PRIVATE_JOB_DIAGNOSTIC');
            return page.evaluate(...args);
          },
        };
      },
    };
  } });
  const redacted = code => error => refusal(code)(error) && !String(error).includes('PRIVATE_');
  // Inject only shutdown diagnostics around real contexts; no platform effects
  // or journal outcomes are substituted. Cleanup must release the original port.
  await assert.rejects(launchPublicationBrowser(shutdownEngine('startup'), options('open')), redacted('launch'));
  bridge = await launchPublicationBrowser(shutdownEngine('execute'), options('open'));
  await assert.rejects(bridge.execute({ id: 1, index: 0, action: 'inspect', binding }), redacted('browser'));
  await assert.rejects(bridge.execute({ id: 2, index: 0, action: 'inspect', binding }), redacted('closed'));
  await assert.rejects(bridge.close(), redacted('browser'));
  bridge = await launchPublicationBrowser(shutdownEngine('close'), options('open'));
  await assert.rejects(bridge.close(), redacted('browser'));
  await assert.rejects(bridge.close(), redacted('browser'));
  bridge = await launchPublicationBrowser(shutdownEngine('close'), native);
  await assert.rejects(bridge.driveSession(peer(async () => { throw new Error('PRIVATE_CONTROL_DIAGNOSTIC'); })),
    redacted('native-control'));
  await assert.rejects(bridge.close(), redacted('browser'));
  bridge = await launchPublicationBrowser(engine, options('open'));
  assert.deepEqual((await bridge.execute({ id: 1, index: 0, action: 'inspect', binding })).journal, { present: false });
  await bridge.close(); assert.deepEqual(await readFile(bindingPath), profileBinding);
  cases.push('shutdown_diagnostics_are_redacted_preserve_primary_failure_and_release_original_port_and_profile');
  assert.deepEqual(unexpected, []); assert(requests.every(request => request.method === 'GET'));
  await writeFile(join(report, 'summary.json'), JSON.stringify({ schema: 1, cases, requests,
    assetOrigin: origin, unexpected, liveProviderRequests: 0, paidEffects: 0, automaticRetries: 0 }, null, 2), { flag: 'wx', mode: 0o600 });
  console.log(`PASS ${cases.length} native browser/profile/body boundary cases`);
} finally { await bridge?.close(); reservation.close(); }
