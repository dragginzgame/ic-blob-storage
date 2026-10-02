// Integration harness only: exercise the maintained store in real Chromium.
import { createIndexedDBIntentStore } from '../../clients/browser/intents.js';
window.createIntentStore = createIndexedDBIntentStore;
