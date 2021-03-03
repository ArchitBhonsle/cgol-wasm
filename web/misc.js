// canvas resize
const canvas = document.getElementById('canvas');
canvas.style.width = '100%';
canvas.style.height = '100%';
canvas.width = canvas.offsetWidth;
canvas.height = canvas.offsetHeight;

const pausePlay = document.getElementById('pause-play');
const randomize = document.getElementById('randomize');
const clear = document.getElementById('clear');
const fps_input = document.getElementById('fps-input');
const help = document.getElementById('help');

// keyboard controls
window.addEventListener(
  'keydown',
  function (event) {
    if (event.defaultPrevented) return;

    switch (event.key) {
      case 'p':
        pausePlay.click();
        break;
      case 'r':
        randomize.click();
        break;
      case 'c':
        clear.click();
        break;
      case 'ArrowUp':
        fps_input.stepUp();
        break;
      case 'ArrowDown':
        fps_input.stepDown();
        break;
      case '?':
        help.click();
        break;
      default:
        return;
    }

    // Cancel the default action to avoid it being handled twice
    event.preventDefault();
  },
  true
);

// modal
const modalContainer = document.getElementById('modal-container');
const modal = document.getElementById('modal');
const modalClose = document.getElementById('modal-close');

const visitedBefore = localStorage.getItem('cgol-wasm');
if (visitedBefore) modalContainer.style.display = 'none';
else localStorage.setItem('cgol-wasm', 'meow');

help.onclick = function () {
  modalContainer.style.display =
    modalContainer.style.display === 'block' ? 'none' : 'block';
};

modalClose.onclick = function () {
  modalContainer.style.display = 'none';
};

window.onclick = function (event) {
  if (event.target == modalContainer) {
    modalContainer.style.display = 'none';
  }
};
