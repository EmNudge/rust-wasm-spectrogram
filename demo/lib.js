/** @type {(blog: File, name: string) => void} */
export const placeFileInCache = async (file, name) => {
  const cache = await caches.open('blobCache');
  await cache.put(name, new Response(file));
};

/** @type {(name: string) => Blob} name */
export const getBufferFromCache = async (name) => {
  const cache = await caches.open('blobCache');
  return cache.match(name).then(resp => resp?.blob());
};

/** @type {(buffer: ArrayBuffer, type: string) => Promise<Float32Array>} */
export const getAudioSignalFromBuffer = async (buffer, type = "audio/wav") => {
  const sampleRate =
    type === "audio/wav" ? new DataView(buffer).getUint32(24, true) : 44_100;

  console.time("audio data parse");
  const audioCtx = new AudioContext({ sampleRate });
  const audioBuffer = await audioCtx.decodeAudioData(buffer);
  const data = audioBuffer.getChannelData(0);
  console.timeEnd("audio data parse");

  return data;
};

const slidersDiv = document.querySelector("div.sliders");

/**
 * @param {string} title 
 * @param {[number, number]} range 
 * @param {number} def 
 * @param {(value: number) => void} onVal 
 */
export const makeSlider = (title, range, def, onVal) => {
  const label = document.createElement('label');
  label.innerHTML = `<span>${title}: <output>${def}</output>`;
  label.innerHTML += `\n<input type="range" value="${def}" min="${range[0]}" max="${range[1]}" step="1">`;
  
  const output = label.querySelector('output');
  const input = label.querySelector('input');
  input.addEventListener('input', () => {
    const num = Number(input.value);
    output.textContent = num;
    onVal(num);
  })
  
  slidersDiv.append(label);
}