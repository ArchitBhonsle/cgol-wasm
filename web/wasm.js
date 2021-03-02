import init, { start } from './wasm/cgol.js';

async function run() {
  await init();
  start('canvas', 'pause-play', 'fps-input', 'randomize', 'clear', 10, 0);
}

run();
