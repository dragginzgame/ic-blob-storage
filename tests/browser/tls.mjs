// Test-only temporary TLS identity. Never disables validation for unrelated hosts.
import { mkdtemp, readFile, writeFile, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { X509Certificate, createHash } from 'node:crypto';
import { createSecureServer } from 'node:http2';

export async function loopbackTLS() {
  const directory = await mkdtemp(join(tmpdir(), 'blob-browser-tls-'));
  try {
    const openssl = args => promisify(execFile)('openssl', args, { timeout: 10000 });
    await openssl(['req', '-x509', '-newkey', 'rsa:2048', '-nodes',
      '-keyout', join(directory, 'root.key'), '-out', join(directory, 'root.pem'), '-days', '1',
      '-subj', '/CN=Blob local fixture CA', '-addext', 'basicConstraints=critical,CA:TRUE',
      '-addext', 'keyUsage=critical,keyCertSign,cRLSign']);
    await openssl(['req', '-new', '-newkey', 'rsa:2048', '-nodes',
      '-keyout', join(directory, 'key.pem'), '-out', join(directory, 'leaf.csr'),
      '-subj', '/CN=localhost']);
    await writeFile(join(directory, 'leaf.ext'), 'basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature,keyEncipherment\nextendedKeyUsage=serverAuth\nsubjectAltName=IP:127.0.0.1\n');
    await openssl(['x509', '-req', '-in', join(directory, 'leaf.csr'), '-CA', join(directory, 'root.pem'),
      '-CAkey', join(directory, 'root.key'), '-CAcreateserial', '-out', join(directory, 'cert.pem'),
      '-days', '1', '-extfile', join(directory, 'leaf.ext')]);
    const cert = await readFile(join(directory, 'cert.pem')), key = await readFile(join(directory, 'key.pem'));
    const root = await readFile(join(directory, 'root.pem'));
    const spki = new X509Certificate(cert).publicKey.export({ type: 'spki', format: 'der' });
    return { cert, key, root, launch: { headless: true,
      args: [`--ignore-certificate-errors-spki-list=${createHash('sha256').update(spki).digest('base64')}`] },
    close: () => rm(directory, { recursive: true }) };
  } catch (error) { await rm(directory, { recursive: true }); throw error; }
}

export async function loopbackH2(tls, handler) {
  const sockets = new Set();
  const server = createSecureServer({ cert: tls.cert, key: tls.key, allowHTTP1: false }, (req, res) => {
    req.on('error', () => {}); res.on('error', () => {});
    if (req.method === 'OPTIONS') {
      res.writeHead(204, { 'access-control-allow-origin': '*', 'access-control-allow-methods': 'PUT',
        'access-control-allow-headers': req.headers['access-control-request-headers'] ?? '' });
      res.end(); return;
    }
    res.setHeader('access-control-allow-origin', '*'); handler(req, res);
  });
  server.on('connection', socket => { sockets.add(socket); socket.on('close', () => sockets.delete(socket)); });
  server.on('session', session => session.on('error', () => {}));
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  return { origin: `https://127.0.0.1:${server.address().port}`,
    close: async () => { for (const socket of sockets) socket.destroy();
      await new Promise(resolve => server.close(resolve)); } };
}
