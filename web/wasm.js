import init, { start } from './wasm/cgol.js';

async function run() {
  await init();
  start('canvas', 'button', 'fps-slider', 10, 0);
}

run();
