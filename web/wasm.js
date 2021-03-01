import init, { start } from './wasm/cgol.js';

async function run() {
  await init();
  start('canvas', 'button', 'fps-slider', 30, 2, '#333', '#DDD');
}

run();
