// Match wire::decimal's parse + Display equality, including grouping and signed zero.
const decimal = (signed: boolean) => (value: unknown): void => {
  const pattern = signed ? /^(0|-?[1-9][0-9]{0,2}(_[0-9]{3})*)$/ : /^(0|[1-9][0-9]{0,2}(_[0-9]{3})*)$/;
  // JS '$' also matches before a final newline: require the entire matched text.
  if (typeof value !== 'string' || pattern.exec(value)?.[0] !== value) {
    throw new WireError('value');
  }
};
export function createCodec(metadata: unknown, registry: unknown = {}) {
  return wireCodec(metadata, registry, { blobInt: decimal(true), blobNat: decimal(false) });
}
