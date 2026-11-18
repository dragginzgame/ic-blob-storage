function selector(value) {
  if (typeof value !== "string" || !/^[\x21-\x7e]{1,128}$/.test(value)) {
    throw new TypeError("Expected a bounded printable provider selector");
  }
  return value;
}

export function endpoint(value) {
  const url = new URL(value);
  if ((url.protocol !== "https:" && !(url.protocol === "http:"
      && ["localhost", "127.0.0.1", "[::1]"].includes(url.hostname)))
      || url.username || url.password || url.href.includes("#") || url.href.length > 4096) {
    throw new TypeError("Expected bounded HTTPS or owned loopback HTTP URL without credentials or fragment");
  }
  return url;
}

export function providerBinding(options) {
  const url = endpoint(options.gateway);
  if (url.href !== url.origin + "/") throw new TypeError("Gateway must be an origin");
  return Object.freeze({ gateway: url.origin, owner: selector(options.owner),
    project: selector(options.project), bucket: selector(options.bucket) });
}
