import("../pkg/index.js").catch(console.error);
import "../css/style.css";

function ready(fn) {
  if (document.readyState != 'loading'){
    fn();
  } else {
    document.addEventListener('DOMContentLoaded', fn);
  }
}

function setCanvasSize() {
    var canvasElem = document.getElementById('fp-canvas');
    canvasElem.setAttribute("width", window.innerWidth);
    canvasElem.setAttribute("height", window.innerHeight);
}

ready(function () {
    setCanvasSize();
})

window.addEventListener('resize', setCanvasSize)