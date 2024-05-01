# Rust Wasm Spectrogram

A test to see how efficient we can get a wasm-based spectrogram build. It uses [wasm-pack](https://github.com/rustwasm/wasm-pack) to build the wasm bundle.

Run the demo with
```sh
nodemon --exec 'wasm-pack build -d demo/wasm' -e rs,toml # in one terminal
npx serve demo # in another
```

A good test file is "[Where were you a year ago](https://haskinslabs.org/about-us/features-and-demos/sinewave-synthesis/s1spectro)".

Timing data is available in the console using `console.time()` to see where the bottlenecks in our process might be.