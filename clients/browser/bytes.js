// Representation only; callers own hashing, binding and byte admission.
export const hex = bytes => Array.from(bytes, byte => byte.toString(16).padStart(2, '0')).join('');
