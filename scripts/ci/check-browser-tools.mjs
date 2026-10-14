import { fileURLToPath } from 'node:url';
import { checkNodeInputs } from './check-node-tools.mjs';
await checkNodeInputs(fileURLToPath(new URL('../../tests/browser/', import.meta.url)));
