import { __wbg_set_wasm, get_spectrogram } from "./wasm/wasm_spectrogram_bg.js";
import {
  getAudioSignalFromBuffer,
  getBufferFromCache,
  makeSlider,
  placeFileInCache,
} from "./lib.js";

const wasm = await WebAssembly.instantiateStreaming(
  fetch("./wasm/wasm_spectrogram_bg.wasm")
);
__wbg_set_wasm(wasm.instance.exports);

let width = 1080;
let height = 720;
let frameSize = 1024;

/** @type {Float32Array} */
let samples;

const getCanvasArr = () => {
  console.time("wasm parse");
  const canvasArray = get_spectrogram(
    samples,
    width,
    height,
    frameSize
  );
  console.timeEnd("wasm parse");

  return canvasArray;
};

/** @type {HTMLCanvasElement} */
const canvas = document.querySelector("canvas");
const ctx = canvas.getContext("2d");
ctx.imageSmoothingEnabled = false;

const paintSpectrogram = () => {
  const canvasArray = new Uint8ClampedArray(getCanvasArr());
  console.log({ width, height })
  const imageData = new ImageData(canvasArray, width, height);

  console.time("canvas paint");
  ctx.putImageData(imageData, 0, 0);
  console.timeEnd("canvas paint");
};

makeSlider("Overlap", [2, 50], 25, (num) => {
  overlap = num;
  if (samples) paintSpectrogram();
});
makeSlider("Bin Power", [1, 4], 25, (num) => {
  frameSize = 2 << (7 + num);
  if (samples) paintSpectrogram();
});
makeSlider("Width", [500, 1080], 1080, (num) => {
  width = num;
  ctx.clearRect(0, 0, 1080, 1080);
  if (samples) paintSpectrogram();
});
makeSlider("Height", [500, 720], 720, (num) => {
  height = num;
  ctx.clearRect(0, 0, 1920, 720);
  if (samples) paintSpectrogram();
});

const fileInput = document.querySelector("input[type=file]");

getBufferFromCache("audio-file-buffer").then(async (blob) => {
  if (!blob) return;

  const buffer = await blob.arrayBuffer();
  samples = await getAudioSignalFromBuffer(buffer);
  paintSpectrogram();
});

fileInput.addEventListener("input", async () => {
  /** @type {File} */
  const file = fileInput.files[0];
  if (!file) return;

  const buffer = await file.arrayBuffer();

  void placeFileInCache(file, "audio-file-buffer");

  samples = await getAudioSignalFromBuffer(buffer, file.type);
  paintSpectrogram();
});
