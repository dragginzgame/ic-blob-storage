import { servePublicationBootstrap } from './bootstrap.js';

// This is a DedicatedWorker entry, not a public window message handler. The
// creator transfers its one private port; signer/configuration arrive on it only.
let connected = false;
self.onmessage = ({ data, ports }) => {
  if (connected) return;
  connected = true;
  self.onmessage = null;
  if (data?.schema !== 1 || data.operation !== 'private-port' || ports.length !== 1 ||
    Object.keys(data).length !== 2) { self.close(); return; }
  servePublicationBootstrap(ports[0]);
};
