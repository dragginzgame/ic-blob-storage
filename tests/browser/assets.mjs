// Frozen test bundles only. Callers own routes, headers, observations and transport faults.
export function serveAssets(assets, entry, headers = {}) {
  return (req, res) => {
    const script = Object.hasOwn(assets, req.url);
    res.writeHead(200, { ...headers, 'content-type': script ? 'text/javascript' : 'text/html' });
    res.end(script ? assets[req.url] : `<!doctype html><script type="module" src="${entry}"></script>`);
  };
}
