import init from './wasm/cgol.js';

async function run() {
  await init();
  // const game = Game.new('canvas', 'button', 30, 2, '#333', '#DDD');
}

run();
