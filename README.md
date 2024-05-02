# Rust Wasm Spectrogram

A test to see how efficient we can get a wasm-based spectrogram build.


## Usage

I use [wasm-pack](https://github.com/rustwasm/wasm-pack) for the build and [watchexec](https://github.com/watchexec/watchexec/tree/main) to rerun commands.
```sh
watchexec -e rs,toml -- wasm-pack build -d demo/wasm
```

In a separate terminal, I run a web server with [serve](https://www.npmjs.com/package/serve)
```sh
npx serve demo
```

## Playing With The Demo

A good test file is "[Where were you a year ago](https://haskinslabs.org/about-us/features-and-demos/sinewave-synthesis/s1spectro)". It will be cached in your browser, so it's preserved between reloads.

Timing data is available in the console using `console.time()` to see where the bottlenecks in our process might be.