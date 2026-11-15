// Executed by Chromium, with the actual module and a separate-origin local HTTP peer.
import { prepare, readVerified, TransferError } from "/client.js";
import { initializeCustody, createTransferClaim } from "/custody.js";

const peer = new URL(location.href).searchParams.get("peer");
const scenario = new URL(location.href).searchParams.get("scenario");
const root = "sha256:bc608ea2567f6809b872afc8e9ec44fc0addf3fd58a00e7f7aa7757b35927a03";
const length = 5 * 1024 * 1024 + 17;
const check = (condition, message) => { if (!condition) throw new Error(message); };

try {
  check(["success", "lost-final-reply", "cancel-chunk"].includes(scenario), "known transport scenario");
  check(isSecureContext, "Web Crypto requires a secure context");
  const bytes = Uint8Array.from({ length }, (_, i) => (i * 31 + 7) % 251);
  const preparing = prepare(bytes);
  bytes.fill(0); // Mutation after entry must not change the prepared snapshot.
  const prepared = await preparing;
  check(prepared.declaration.root === root, "original six-chunk vector");
  const fixture = await (await fetch("/proof.json")).json();
  const proof = { ...fixture, ...Object.fromEntries(["request", "certificate", "rootKey", "badCertificate"]
    .map(key => [key, Uint8Array.from(atob(fixture[key]), c => c.charCodeAt(0))])) };
  const input = { ...proof, gateway: peer, project: "fixture-project", bucket: "fixture-bucket" };
  const custody = await createTransferClaim(input);
  const options = { gateway: peer, owner: proof.issuer, project: input.project, bucket: input.bucket,
    ...custody };
  if (new URL(location.href).searchParams.has("replay")) {
    let refused = false;
    try { await prepared.upload(options); }
    catch (error) {
      refused = error instanceof TransferError && error.stage === "claim" && !error.possibleExposure;
    }
    check(refused, "fresh page must refuse the persisted transfer claim");
    // Explicit read-only reconciliation after reload, including an uncertain PUT.
    let verified = false;
    try {
      const readback = await readVerified(`${peer}/uploaded`, prepared.declaration);
      verified = true;
      check(readback.root === root && readback.blob.size === length, "read back received upload bytes");
    } catch (error) { check(scenario === "cancel-chunk", String(error)); }
    check(verified === (scenario !== "cancel-chunk"), "incomplete cancelled transfer cannot verify");
    await fetch("/result", { method: "POST", body: JSON.stringify({ stage: "replay", refused, root, verified }) });
  } else {
    let corruptRefused = false;
    try { await createTransferClaim({ ...input, certificate: proof.badCertificate }); }
    catch { corruptRefused = true; }
    check(corruptRefused, "corrupt proof must not become transfer authority");
    async function refuseClaim(selected) {
      const fresh = await prepare(bodyBytes());
      let refused = false;
      try { await fresh.upload(selected); }
      catch (error) { refused = error instanceof TransferError && error.stage === "claim" && !error.possibleExposure; }
      check(refused, "custody refusal must precede every provider PUT");
    }
    function bodyBytes() { return Uint8Array.from({ length }, (_, i) => (i * 31 + 7) % 251); }
    await refuseClaim(options); // Missing custody must refuse, never provision implicitly.
    await initializeCustody(input);
    let replaceRefused = false;
    try { await initializeCustody(input); } catch { replaceRefused = true; }
    check(replaceRefused, "existing custody must not be replaced");
    const changed = { ...input, project: "different-project" };
    await refuseClaim({ ...options, project: changed.project, ...await createTransferClaim(changed) });
    const altered = new Uint8Array(custody.certificate.bytes);
    altered[altered.length - 1] ^= 1;
    await refuseClaim({ ...options, certificate: { root, bytes: altered } });
    if (scenario === "success") {
      options.claim = async (plan, signal) => {
        // Separate connections race for the same root; only one transaction wins.
        const claims = await Promise.allSettled([custody.claim(plan, signal), custody.claim(plan, signal)]);
        check(claims.filter(result => result.status === "fulfilled").length === 1,
          "concurrent claims must have exactly one winner");
        return true;
      };
    }
    const declaration = { ...prepared.declaration, headers: [...prepared.declaration.headers] };
    const pending = readVerified(`${peer}/ok`, declaration);
    declaration.root = `sha256:${"00".repeat(32)}`;
    declaration.headers[1] = "Content-Type: text/plain";
    const result = await pending;
    check(result.root === root && result.blob.size === length, "original read binding");
    check(result.blob.type === "application/octet-stream", "ignore response metadata");
    const body = new Uint8Array(await result.blob.arrayBuffer());
    check(body.every((value, i) => value === (i * 31 + 7) % 251), "verified bytes");

    const refused = [];
    for (const name of ["corrupt", "short", "extra", "partial", "redirect", "no-cors", "stall"]) {
      let failed = false;
      try {
        await readVerified(`${peer}/${name}`, prepared.declaration,
          { timeoutMs: name === "stall" ? 250 : 10_000 });
      } catch { failed = true; }
      check(failed, `read must refuse ${name}`);
      refused.push(name);
    }
    let cancelled = false;
    try {
      await readVerified(`${peer}/cancelled`, prepared.declaration,
        { signal: AbortSignal.abort(new Error("caller cancelled")) });
    } catch { cancelled = true; }
    check(cancelled, "pre-cancelled read");
    let outcome;
    try {
      const cancellation = new AbortController();
      const settled = prepared.upload({ ...options, signal: cancellation.signal })
        .then(value => ({ value }), error => ({ error }));
      if (scenario === "cancel-chunk") {
        await fetch("/chunk-pending"); // Test peer barrier, not a simulated Fetch response.
        cancellation.abort(new Error("caller cancelled while first chunk reply was pending"));
      }
      const { value: transfer, error } = await settled;
      if (error) throw error;
      check(scenario === "success" && transfer.outcome === "http-acknowledged"
        && transfer.acknowledged === 7, "tree and six chunks acknowledged");
      outcome = "http-acknowledged";
    } catch (error) {
      check(scenario !== "success" && error instanceof TransferError
        && error.stage === "chunk" && error.chunkIndex === (scenario === "cancel-chunk" ? 0 : 5)
        && error.possibleExposure && error.acknowledged === (scenario === "cancel-chunk" ? 1 : 6),
      "interrupted transfer must retain possible exposure and prior acknowledgments");
      outcome = "uncertain";
    }
    await fetch("/result", { method: "POST", body: JSON.stringify({ stage: "first", root, refused, cancelled, outcome }) });
    const next = new URL(location.href);
    next.searchParams.set("replay", "1");
    location.replace(next); // New module instance, same origin and committed claim.
  }
} catch (error) {
  await fetch("/result", { method: "POST", body: JSON.stringify({ error: String(error.stack ?? error) }) });
}
