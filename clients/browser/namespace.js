// Local namespace representation only; this never proves provider provisioning.
export function validNamespace(value, header = false) {
  if (typeof value !== 'string' || value.length === 0 || value.length > 256 || value.trim() !== value ||
    /[\u0000-\u001f\u007f-\u009f\ud800-\udfff]/u.test(value) || new TextEncoder().encode(value).length > 256) return false;
  if (header) {
    try { new Headers({ 'X-Caffeine-Project-ID': value }); }
    catch { return false; }
  }
  return true;
}
