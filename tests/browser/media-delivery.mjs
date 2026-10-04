// Local media/CSP acceptance only. Rust owns canonical provider targets.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { createHash } from 'node:crypto';

export async function verifyMediaDelivery(context, config, bodies, gateway, state) {
  const page = context.pages().find(page => page.url() === `${config.browserOrigin}/`);
  assert(page); // A persistent context also contains an unrelated about:blank page.
  const decodedMedia = [], publicDelivery = [], images = [];
  const digest = bytes => createHash('sha256').update(bytes).digest('hex');
  const requests = await Promise.all(bodies.map(async (_, index) => {
    const request = JSON.parse(await readFile(join(config.report, `download-${index}`, 'download-request.json')));
    assert.equal(new URL(request.url).origin, gateway); assert.equal(request.method, 'GET');
    return request;
  }));
  for (const [index, body] of bodies.entries()) {
    const mime = config.contentTypes[index], isImage = mime.startsWith('image/');
    const expected = config.imageExpectations ? config.imageExpectations[index] : { width: body.readUInt32BE(16), height: body.readUInt32BE(20),
      pixel: index === 0 ? [200, 30, 60, 255] : [20, 180, 210, 255] };
    const delivered = await readFile(join(config.report, `download-${index}`, 'body.bin'));
    assert(delivered.equals(body));
    const sampled = await page.evaluate(async ({ bytes, url, mime }) => {
      async function sample(body, type) {
        if (type === 'model/gltf-binary') {
          const bytes = body instanceof Uint8Array ? body : new Uint8Array(body);
          const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
          if (view.getUint32(0, true) !== 0x46546c67 || view.getUint32(4, true) !== 2 ||
            view.getUint32(8, true) !== bytes.length || view.getUint32(16, true) !== 0x4e4f534a) throw new Error('fixture GLB header');
          const document = JSON.parse(new TextDecoder().decode(bytes.subarray(20, 20 + view.getUint32(12, true))));
          if (document.asset.version !== '2.0') throw new Error('fixture glTF version');
          // Structural observation only; no renderer or application adoption is implied.
          return { kind: 'glb', bytes: bytes.length, scenes: document.scenes.length,
            nodes: document.nodes.length, meshes: document.meshes.length };
        }
        const image = await createImageBitmap(new Blob([body], { type }));
        try {
          const canvas = new OffscreenCanvas(image.width, image.height), painter = canvas.getContext('2d');
          painter.drawImage(image, 0, 0);
          return { width: image.width, height: image.height, pixel: [...painter.getImageData(0, 0, 1, 1).data] };
        } finally { image.close(); }
      }
      const decoded = await sample(new Uint8Array(bytes), mime);
      const response = await fetch(url, { credentials: 'omit', redirect: 'error', cache: 'no-store',
        signal: AbortSignal.timeout(10000) });
      const body = await response.arrayBuffer(), hash = await crypto.subtle.digest('SHA-256', body);
      return { decoded, served: { origin: location.origin, status: response.status, type: response.type, url: response.url,
        mime: response.headers.get('content-type'), declaredBytes: response.headers.get('content-length'),
        cacheControl: response.headers.get('cache-control'),
        bytes: body.byteLength, sha256: [...new Uint8Array(hash)].map(value => value.toString(16).padStart(2, '0')).join(''),
        ...await sample(body, response.headers.get('content-type')) } };
    }, { bytes: [...delivered], url: requests[index].url, mime });
    if (expected) for (const [key, value] of Object.entries(expected)) assert.deepEqual(sampled.decoded[key], value);
    assert.deepEqual(sampled.served, { origin: config.browserOrigin, status: 200, type: 'cors', url: requests[index].url,
      mime, declaredBytes: String(body.length), cacheControl: config.cacheControls[index],
      bytes: body.length, sha256: digest(body), ...sampled.decoded });
    decodedMedia.push({ index, sha256: digest(body), ...sampled.decoded });
    publicDelivery.push({ index, afterFinalRelease: !!config.overlap && index === 0, ...sampled.served });
    if (isImage) {
      const imagePage = await context.newPage();
      try {
        await imagePage.goto(config.browserOrigin);
        const loaded = await image(imagePage, requests[index].url, gateway);
        assert.deepEqual(loaded, { origin: config.browserOrigin, loaded: true, violations: [], ...sampled.decoded });
        images.push({ index, afterFinalRelease: !!config.overlap && index === 0, ...loaded });
      } finally { await imagePage.close(); }
    }
  }
  const blockedPage = await context.newPage(), before = state.gets.length;
  let blockedImage, blockedProviderRequests;
  try {
    await blockedPage.goto(config.browserOrigin);
    assert(images.length > 0);
    blockedImage = await image(blockedPage, requests[images[0].index].url, "'none'");
    const { violations, ...outcome } = blockedImage;
    assert.deepEqual(outcome, { origin: config.browserOrigin, loaded: false });
    assert.equal(violations.length, 1);
    assert.equal(violations[0].directive, 'img-src');
    assert.equal(violations[0].disposition, 'enforce');
    assert.equal(new URL(violations[0].blockedURI).origin, gateway);
    blockedProviderRequests = state.gets.length - before;
    assert.equal(blockedProviderRequests, 0); // The browser refused before reaching the provider.
  } finally { await blockedPage.close(); }
  const opaque = await context.newPage();
  try {
    const refused = await opaque.evaluate(async url => {
      try {
        await fetch(url, { credentials: 'omit', redirect: 'error', cache: 'no-store', signal: AbortSignal.timeout(10000) });
        return { origin: location.origin, readable: true };
      } catch (error) { return { origin: location.origin, readable: false, error: error.name }; }
    }, requests[0].url);
    assert.deepEqual(refused, { origin: 'null', readable: false, error: 'TypeError' });
  } finally { await opaque.close(); }
  const reads = state.reads.filter(read => read.origin !== null);
  assert.deepEqual(reads.map(read => read.origin), [...config.contentTypes.flatMap(mime =>
    Array(mime.startsWith('image/') ? 2 : 1).fill(config.browserOrigin)), 'null']);
  for (const read of reads) { assert.equal(read.cookie, null); assert.equal(read.authorization, null); }
  return { decodedMedia, publicDelivery, opaqueOriginRefused: true,
    csp: { images, blockedImage, blockedProviderRequests } };
}

async function image(page, url, allowedOrigin) {
  return page.evaluate(async ({ url, allowedOrigin }) => {
    const violations = [];
    const violation = event => violations.push({ directive: event.effectiveDirective,
      disposition: event.disposition, blockedURI: event.blockedURI });
    document.addEventListener('securitypolicyviolation', violation);
    const policy = document.createElement('meta'); policy.httpEquiv = 'Content-Security-Policy';
    policy.content = `default-src 'none'; img-src ${allowedOrigin}; base-uri 'none'; form-action 'none'`;
    document.head.append(policy);
    const selected = new Image(); selected.crossOrigin = 'anonymous';
    let timer;
    try {
      const loaded = await new Promise((resolve, reject) => {
        timer = setTimeout(() => reject(new Error('fixture image deadline')), 10000);
        selected.onload = () => resolve(true); selected.onerror = () => resolve(false);
        document.body.append(selected); selected.src = url;
      });
      // Policy violations and image errors are delivered on separate event tasks.
      await new Promise(resolve => setTimeout(resolve, 0));
      const result = { origin: location.origin, loaded, violations };
      if (!loaded) return result;
      const canvas = new OffscreenCanvas(selected.naturalWidth, selected.naturalHeight), painter = canvas.getContext('2d');
      painter.drawImage(selected, 0, 0);
      return { ...result, width: selected.naturalWidth, height: selected.naturalHeight,
        pixel: [...painter.getImageData(0, 0, 1, 1).data] };
    } finally {
      clearTimeout(timer); selected.remove(); document.removeEventListener('securitypolicyviolation', violation);
    }
  }, { url, allowedOrigin });
}
