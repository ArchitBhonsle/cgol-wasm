const fps_slider = document.getElementById('fps-slider');
const fps_value = document.getElementById('fps-value');

fps.addEventListener('change', e => {
  fps_value.innerHTML = `${e.target.value} fps`;
});
