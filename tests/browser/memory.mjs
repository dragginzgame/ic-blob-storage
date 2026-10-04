// Read-only local instrumentation shared by preparation and successful-transfer fixtures.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

export async function attachBrowserMemory(context, origin) {
  const root = await context.browser().newBrowserCDPSession();
  const pending = new Map();
  let messageId = 0, workerSession;
  function reply(event) {
    if (event.sessionId !== workerSession) return;
    const message = JSON.parse(event.message), waiter = pending.get(message.id);
    if (!waiter) return;
    clearTimeout(waiter.timer); pending.delete(message.id);
    if (message.error) waiter.reject(new Error(JSON.stringify(message.error)));
    else waiter.resolve(message.result);
  }
  root.on('Target.receivedMessageFromTarget', reply);
  try {
    const { targetInfos } = await root.send('Target.getTargets');
    const workers = targetInfos.filter(t => t.type === 'worker' && t.url === `${origin}/worker.js`);
    assert.equal(workers.length, 1, 'one actual publication worker');
    ({ sessionId: workerSession } = await root.send('Target.attachToTarget', { targetId: workers[0].targetId, flatten: false }));
  } catch (error) { root.off('Target.receivedMessageFromTarget', reply); await root.detach(); throw error; }
  return {
    async workerCommand(method) {
      assert(pending.size < 4, 'bounded observer requests');
      const id = ++messageId;
      const response = new Promise((resolve, reject) => {
        const timer = setTimeout(() => { pending.delete(id); reject(new Error('worker measurement timeout')); }, 5000);
        pending.set(id, { resolve, reject, timer });
      });
      response.catch(() => {});
      try {
        await root.send('Target.sendMessageToTarget', { sessionId: workerSession,
          message: JSON.stringify({ id, method, params: {} }) });
        return await response;
      } finally {
        const waiter = pending.get(id);
        if (waiter) { clearTimeout(waiter.timer); pending.delete(id); }
      }
    },
    async browserRSS() {
      const { processInfo } = await root.send('SystemInfo.getProcessInfo');
      const processes = await Promise.all(processInfo.map(async ({ id, type }) => {
        // Never enumerate unrelated host processes; Chrome supplies its own PIDs.
        assert(Number.isSafeInteger(id) && id > 0);
        const status = await readFile(`/proc/${id}/status`, 'utf8');
        const matched = /^VmRSS:\s+(\d+) kB$/m.exec(status);
        assert(matched, 'owned browser process RSS available');
        return { pid: id, type, rss_bytes: Number(matched[1]) * 1024 };
      }));
      return { processes, sum_rss_bytes: processes.reduce((sum, p) => sum + p.rss_bytes, 0) };
    },
    async close() {
      root.off('Target.receivedMessageFromTarget', reply);
      for (const waiter of pending.values()) { clearTimeout(waiter.timer); waiter.reject(new Error('observer closed')); }
      pending.clear(); await root.detach();
    },
  };
}

export async function sampleBrowserMemory(monitor) {
  const started = performance.now();
  const result = { node_before: process.memoryUsage(), worker_before: await monitor.workerCommand('Runtime.getHeapUsage'),
    browser_before: await monitor.browserRSS(), browser_rss_samples: [], worker_heap_samples: [], error: null };
  result.sampled_node_peak_rss = result.node_before.rss;
  result.sampled_node_peak_array_buffers = result.node_before.arrayBuffers;
  let running = true;
  const timer = setInterval(() => {
    const memory = process.memoryUsage();
    result.sampled_node_peak_rss = Math.max(result.sampled_node_peak_rss, memory.rss);
    result.sampled_node_peak_array_buffers = Math.max(result.sampled_node_peak_array_buffers, memory.arrayBuffers);
  }, 10);
  // The whole serial fixture is bounded to 180 seconds; keep a separate finite sample ceiling.
  async function poll(rows, read) {
    try {
      while (running) {
        if (rows.length >= 10_000) throw new Error('memory sample capacity');
        rows.push({ elapsed_ms: performance.now() - started, ...await read() });
        await new Promise(resolve => setTimeout(resolve, 20));
      }
    } catch (error) { result.error ??= String(error); }
  }
  const tasks = [poll(result.browser_rss_samples, () => monitor.browserRSS()),
    poll(result.worker_heap_samples, () => monitor.workerCommand('Runtime.getHeapUsage'))];
  let stopped;
  return { result, stop() {
    stopped ??= (async () => {
      clearInterval(timer); running = false; await Promise.all(tasks);
      result.elapsed_ms = performance.now() - started;
      try {
        result.worker_after = await monitor.workerCommand('Runtime.getHeapUsage');
        result.browser_after = await monitor.browserRSS();
      } catch (error) { result.error ??= String(error); }
      result.sampled_browser_peak_sum_rss = Math.max(result.browser_before.sum_rss_bytes,
        ...result.browser_rss_samples.map(s => s.sum_rss_bytes));
      result.sampled_worker_peak_backing_storage = Math.max(result.worker_before.backingStorageSize,
        result.worker_after?.backingStorageSize ?? 0, ...result.worker_heap_samples.map(s => s.backingStorageSize));
      return result;
    })();
    return stopped;
  } };
}
