// Owned HTTP/2 substitute shared by single-file and serial standalone journeys.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';

export function standaloneGateway(config, bundle, sizes = [1024], workerBundle,
  bodies = sizes.map(size => Buffer.alloc(size, 42))) {
  assert.deepEqual(bodies.map(body => body.length), sizes);
  const chunkBytes = 1024 * 1024;
  const requestBudget = bodies.reduce((sum, body) => sum + 1 + Math.ceil(body.length / chunkBytes), 0);
  const state = { puts: [], gets: [], reads: [], arrivals: [], objects: new Map() };
  const record = (path, bytes) => ({ path, bytes: bytes.length,
    sha256: createHash('sha256').update(bytes).digest('hex') });
  const handle = (req, res) => {
    if (config.browserOrigin) {
      res.setHeader('access-control-allow-origin', config.browserOrigin);
      if (req.method === 'OPTIONS') {
        res.writeHead(204, { 'access-control-allow-methods': 'GET, PUT',
          'access-control-allow-headers': 'content-type, x-caffeine-project-id' });
        res.end(); return;
      }
    }
    const url = new URL(req.url, 'http://localhost');
    if (req.method === 'PUT') {
      assert.equal(req.httpVersion, '2.0');
      const chunks = []; let length = 0;
      req.on('data', bytes => {
        length += bytes.length;
        if (length > (url.pathname === '/v1/chunk/' ? chunkBytes : 65536)) req.destroy(new Error('request limit'));
        else chunks.push(bytes);
      });
      req.on('end', () => {
        try {
          const bytes = Buffer.concat(chunks);
          state.arrivals.push(record(req.url, bytes));
          assert(state.arrivals.length <= requestBudget);
          let object;
          if (url.pathname === '/v1/blob-tree/') {
            const tree = JSON.parse(bytes), root = tree.blob_tree.tree.hash;
            assert.equal(tree.owner, config.service); assert.equal(tree.project_id, config.project);
            assert.equal(tree.num_blob_bytes, sizes[state.objects.size]);
            assert(tree.auth.OwnerEgressSignature.length > 0);
            assert(!state.objects.has(root));
            assert.equal(tree.blob_tree.chunk_hashes.length, Math.ceil(tree.num_blob_bytes / chunkBytes));
            object = { tree, index: state.objects.size, chunks: [] };
            state.objects.set(root, object); state.tree = tree;
          } else {
            assert.equal(url.pathname, '/v1/chunk/');
            object = state.objects.get(url.searchParams.get('blob_hash')); assert(object);
            assert.equal(object.body, undefined);
            assert.equal(url.searchParams.get('owner_id'), config.service);
            assert.equal(req.headers['x-caffeine-project-id'], config.project);
            assert.equal(url.searchParams.get('project_id'), config.project);
            const index = object.chunks.length;
            assert.equal(url.searchParams.get('chunk_index'), String(index));
            assert.equal(url.searchParams.get('chunk_hash'), object.tree.blob_tree.chunk_hashes[index]);
            assert.deepEqual(bytes, bodies[object.index].subarray(index * chunkBytes, (index + 1) * chunkBytes));
            object.chunks.push(bytes);
            if (object.chunks.length === object.tree.blob_tree.chunk_hashes.length) {
              object.body = Buffer.concat(object.chunks);
              assert.deepEqual(object.body, bodies[object.index]); state.body = object.body;
            }
          }
          state.puts.push(record(req.url, bytes));
          // Whole chunk received, then reply lost; never infer absence or replay.
          if (config.lostFinalReply && object.index === 0 && object.body) {
            req.stream.session.destroy(); return;
          }
          res.writeHead(200, { 'content-type': 'application/json' });
          res.end(JSON.stringify({ status: object.body ? 'blob_complete' : 'chunk_received' }));
        } catch (error) { state.failure = String(error); res.writeHead(500); res.end(); }
      });
      return;
    }
    if (url.pathname === '/v1/blob/') {
      try {
        assert.equal(req.httpVersion, '2.0'); assert.equal(req.method, 'GET');
        const object = state.objects.get(url.searchParams.get('blob_hash')); assert(object?.body);
        const browserReads = config.media && !config.corruptRead ? sizes.length + 1 : 0;
        assert(state.gets.length < (config.refuseFirstRead ? 3 : 2 * sizes.length + (config.overlap ? 1 : 0) + browserReads));
        assert.equal(url.searchParams.get('owner_id'), config.service);
        assert.equal(url.searchParams.get('project_id'), config.project);
        state.gets.push(req.url);
        state.reads.push({ path: req.url, origin: req.headers.origin ?? null,
          cookie: req.headers.cookie ?? null, authorization: req.headers.authorization ?? null });
        if (config.refuseFirstRead && state.gets.length === 1) {
          req.stream.close(7); return; // NGHTTP2_REFUSED_STREAM before headers.
        }
        const reply = Buffer.from(object.body);
        if (config.corruptRead) reply[reply.length > chunkBytes ? chunkBytes + 10 : 0] ^= 1;
        res.writeHead(200, { 'content-type': 'image/png', 'content-length': reply.length }); res.end(reply);
      } catch (error) { state.failure = String(error); res.writeHead(500); res.end(); }
      return;
    }
    if (req.url === '/publication-worker.js' && workerBundle) {
      res.writeHead(200, { 'content-type': 'text/javascript' }); res.end(workerBundle);
    } else if (req.url === '/standalone.js') {
      res.writeHead(200, { 'content-type': 'text/javascript' }); res.end(bundle);
    } else {
      res.writeHead(200, { 'content-type': 'text/html' });
      res.end('<!doctype html><script type="module" src="/standalone.js"></script>');
    }
  };
  return { handle, state };
}
