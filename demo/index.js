import { __wbg_set_wasm, get_spectrogram } from "./wasm/wasm_spectrogram_bg.js";

const wasm = await WebAssembly.instantiateStreaming(
  fetch("./wasm/wasm_spectrogram_bg.wasm")
);
__wbg_set_wasm(wasm.instance.exports);

let height = 1920;
let width = 1080;
let overlap = 25;
let frameSize = 1024;

/** @type {Float32Array} */
let samples;

const getCanvasArr = () => {
  console.time("wasm parse");
  const canvasArray = get_spectrogram(samples, width, height, overlap, frameSize);
  console.timeEnd("wasm parse");

  return canvasArray;
};

/** @type {HTMLCanvasElement} */
const canvas = document.querySelector("canvas");

const paintSpectrogram = () => {
  const canvasArray = getCanvasArr();

  console.time("fixing clamped array");
  const newArr = new Uint8ClampedArray(4 * 1920 * 1080);
  const clampedArray = new Uint8ClampedArray(canvasArray.buffer);
  for (let i = 0, j = 0; i < newArr.length; i++) {
    if ((i + 1) % 4 === 0) {
      // every 5th position (0-indexed)
      newArr[i] = 255;
    } else {
      newArr[i] = clampedArray[j];
      j++;
    }
  }
  console.timeEnd("fixing clamped array");

  const ctx = canvas.getContext("2d");

  console.time("canvas paint");
  const imageData = new ImageData(newArr, 1920, 1080);
  ctx.putImageData(imageData, 0, 0);
  console.timeEnd("canvas paint");
};

const overlapRange = document.querySelector(`input[type=range].overlap`);
overlapRange.addEventListener("input", () => {
  overlap = overlapRange.value;
  document.querySelector("span.overlap").textContent = overlap;
  if (samples) {
    paintSpectrogram();
  }
});

const frameSizeRange = document.querySelector(`input[type=range].frame-size`);
frameSizeRange.addEventListener("input", () => {
  frameSize = 2 << (7 + Number(frameSizeRange.value));
  document.querySelector("span.frame-size").textContent = frameSize;
  if (samples) {
    paintSpectrogram();
  }
});

const fileInput = document.querySelector("input[type=file]");
fileInput.addEventListener("input", async () => {
  /** @type {File} */
  const file = fileInput.files[0];
  const buffer = await file.arrayBuffer();

  const sampleRate =
    file.type === "audio/wav"
      ? new DataView(buffer).getUint32(24, true)
      : 44_100;

  console.time("audio data parse");
  const audioCtx = new AudioContext({ sampleRate });
  const audioBuffer = await audioCtx.decodeAudioData(buffer);
  samples = audioBuffer.getChannelData(0);
  console.timeEnd("audio data parse");

  paintSpectrogram();
});
