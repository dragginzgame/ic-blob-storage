// Collect only a caller-admitted body. Fetch policy, absent-body semantics,
// refusal identity and durable observations remain with the transport owner.
export async function readResponseBody(body, maximum, refusal) {
  const reader = body.getReader(), chunks = [];
  let length = 0;
  for (;;) {
    const { done, value } = await reader.read();
    if (done) break;
    length += value.length;
    if (length > maximum) { await reader.cancel(); throw refusal; }
    chunks.push(value);
  }
  const bytes = new Uint8Array(length);
  let offset = 0;
  for (const chunk of chunks) { bytes.set(chunk, offset); offset += chunk.length; }
  return bytes;
}
