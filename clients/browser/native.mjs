// Private pipes only. Native owns phase decisions and all signed service claims.
import { spawn } from 'node:child_process';
import { isAbsolute } from 'node:path';
import { TextDecoder } from 'node:util';

/** Redacted transport failure; never includes command arguments or remote bodies. */
export class NativeSessionRefusal extends Error {
  constructor(code) { super(code); this.name = 'NativeSessionRefusal'; this.code = code; }
}

/** Launch one explicitly selected native session. No discovery, restart or retry. */
export async function startPublicationSession(options) {
  const refuse = code => { throw new NativeSessionRefusal(code); };
  if (!options || typeof options !== 'object') refuse('configuration');
  const { binary, cwd, timeoutSeconds, signal } = options;
  let args;
  try { args = structuredClone(options.args); } catch { refuse('configuration'); }
  const env = { ...process.env };
  if (typeof binary !== 'string' || !isAbsolute(binary) || typeof cwd !== 'string' || !isAbsolute(cwd) ||
      !Number.isSafeInteger(timeoutSeconds) || timeoutSeconds < 1 || timeoutSeconds > 600 ||
      !Array.isArray(args) || args[0] !== 'publish-session' || args.length > 128 ||
      args.some(arg => typeof arg !== 'string' || arg.includes('\0')) ||
      Buffer.byteLength(JSON.stringify(args)) > 65536 ||
      (signal !== undefined && !(signal instanceof AbortSignal)) ||
      (options.env !== undefined && (!options.env || Object.getPrototypeOf(options.env) !== Object.prototype))) refuse('configuration');
  for (const [key, value] of Object.entries(options.env ?? {})) {
    if (!key || key.includes('=') || key.includes('\0') ||
        (value !== null && (typeof value !== 'string' || value.includes('\0')))) refuse('configuration');
    if (value === null) delete env[key]; else env[key] = value;
  }
  if (signal?.aborted) refuse('closed');
  const deferred = () => {
    let resolve, reject;
    const promise = new Promise((yes, no) => { resolve = yes; reject = no; });
    // Exit/failure may precede the caller consuming ready or the final result.
    promise.catch(() => {});
    return { promise, resolve, reject };
  };
  const ready = deferred(), result = deferred(), exited = deferred();
  let child;
  try { child = spawn(binary, args, { cwd, env, shell: false, stdio: ['pipe', 'pipe', 'ignore'] }); }
  catch { refuse('spawn'); }
  let failure, pending, terminal, started = false, ending = false, step = 0;
  const stop = code => {
    if (failure) return;
    failure = new NativeSessionRefusal(code);
    ready.reject(failure); pending?.reject(failure); result.reject(failure);
    child.kill('SIGKILL');
  };
  const abort = () => stop('closed');
  signal?.addEventListener('abort', abort, { once: true });
  const timer = setTimeout(() => stop('deadline'), timeoutSeconds * 1000);
  child.on('error', () => stop('spawn'));
  child.stdin.on('error', () => stop('transport'));
  const decoder = new TextDecoder('utf-8', { fatal: true });
  let fragments = [], bytes = 0;
  function line(raw) {
    let value;
    try { value = JSON.parse(decoder.decode(raw)); } catch { stop('reply'); return; }
    if (!value || Object.getPrototypeOf(value) !== Object.prototype || terminal) { stop('reply'); return; }
    if (value.event === 'ready' && !started && !pending && !ending) {
      started = true; ready.resolve(value);
    } else if (value.event === 'phase' && started && pending && value.step === step) {
      const waiting = pending; pending = undefined; step++; waiting.resolve(value);
    } else if (!Object.hasOwn(value, 'event')) {
      terminal = value;
      if (!started) ready.reject(new NativeSessionRefusal('native-failure'));
      pending?.reject(new NativeSessionRefusal('native-failure')); pending = undefined;
    } else stop('reply');
  }
  child.stdout.on('data', chunk => {
    for (let offset = 0; offset < chunk.length && !failure;) {
      const newline = chunk.indexOf(10, offset), end = newline < 0 ? chunk.length : newline + 1;
      const part = chunk.subarray(offset, end); bytes += part.length;
      if (bytes > (started ? 8 * 1024 * 1024 : 16384)) { stop('reply-limit'); break; }
      fragments.push(part); offset = end;
      if (newline >= 0) {
        line(Buffer.concat(fragments, bytes)); fragments = []; bytes = 0;
      }
    }
  });
  child.on('close', (code, killed) => {
    clearTimeout(timer); signal?.removeEventListener('abort', abort);
    if (!failure) {
      if (bytes || !terminal || pending || killed ||
          (code === 0 ? Object.hasOwn(terminal, 'error') :
            ![2, 3].includes(code) || typeof terminal.error !== 'string')) stop('exit');
      else result.resolve({ report: terminal, exit_code: code });
    }
    exited.resolve();
  });
  const close = async () => { if (!terminal || child.exitCode === null) stop('closed'); await exited.promise; };
  try {
    const currentReady = await ready.promise;
    return {
      ready: currentReady,
      phase: async (frame, phaseSignal) => {
        if (failure) throw failure;
        if (terminal || ending) refuse('closed');
        if (pending) refuse('busy');
        let record;
        try { record = JSON.stringify(structuredClone(frame)) + '\n'; }
        catch { refuse('configuration'); }
        if (Buffer.byteLength(record) > 8192) refuse('frame-limit');
        if (phaseSignal !== undefined && !(phaseSignal instanceof AbortSignal)) refuse('configuration');
        if (phaseSignal?.aborted) { stop('closed'); throw failure; }
        const waiting = deferred(); pending = waiting;
        phaseSignal?.addEventListener('abort', abort, { once: true });
        try { child.stdin.write(record); return await waiting.promise; }
        finally { phaseSignal?.removeEventListener('abort', abort); }
      },
      // Close control input and require both a complete final record and exit.
      // On a partial session this records native transport refusal, not rollback.
      finish: async finishSignal => {
        if (finishSignal !== undefined && !(finishSignal instanceof AbortSignal)) refuse('configuration');
        if (finishSignal?.aborted) { stop('closed'); throw failure; }
        finishSignal?.addEventListener('abort', abort, { once: true });
        ending = true; if (!child.stdin.destroyed) child.stdin.end();
        try { return await result.promise; }
        finally { finishSignal?.removeEventListener('abort', abort); }
      },
      close,
    };
  } catch (error) { await close(); throw error; }
}
