// Passive value checks only; callers own authority, limits and refusal codes.
const utf8 = new TextEncoder();

// Ordered JSON representation equality; this is not general deep or byte equality.
export const equal = (a, b) => JSON.stringify(a) === JSON.stringify(b);

export const integer = (value, min, max) => Number.isSafeInteger(value) && value >= min && value <= max;

export function validFields(value, required, optional = []) {
  return !!value && Object.getPrototypeOf(value) === Object.prototype &&
    required.every(key => Object.hasOwn(value, key)) &&
    Object.keys(value).every(key => required.includes(key) || optional.includes(key));
}

// String length bounds the encoding allocation first; UTF-8 bytes own the limit.
export function validUtf8Text(value, maximum, minimum = 0) {
  return typeof value === 'string' && value.length >= minimum &&
    value.length <= maximum && utf8.encode(value).length <= maximum;
}

