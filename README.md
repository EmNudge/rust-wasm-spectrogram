# Rust Wasm Spectrogram

A test to see how efficient we can get a wasm-based spectrogram build.

You can view the live demo generated from this project at: https://wasm-spectrogram.pages.dev

## Local Development

I use [wasm-pack](https://github.com/rustwasm/wasm-pack) for the build and [watchexec](https://github.com/watchexec/watchexec/tree/main) to rerun commands.
```sh
watchexec -e rs,toml -- wasm-pack build -d demo/wasm
```

In a separate terminal, I run a web server with [serve](https://www.npmjs.com/package/serve)
```sh
npx serve demo
```

## Compiling

With wasm-pack, a bunch of optimizations are already enabled. However, our binary still sits around 35KB. We can shrink this down to another 10KB with the following settings:

```sh
wasm-pack build -d demo/wasm \
  -t web --release \
  --manifest-path ./Cargo.toml \
  -Z build-std=panic_abort,std -Z build-std-features=panic_immediate_abort
```

This brings us to ~25KB uncompressed and ~10KB gzipped.

Using `wasm-pack` also gets us a readme, a license, a `package.json`, and a `.gitignore`. When deploying the demo, I delete these files since I'm serving the whole folder.

```sh
find demo/wasm -type f -not -name 'wasm_*' -delete
```

## Playing With The Demo

A good test file is "[Where were you a year ago](https://haskinslabs.org/about-us/features-and-demos/sinewave-synthesis/s1spectro)". It will be cached in your browser, so it's preserved between reloads.

Timing data is available in the console using `console.time()` to see where the bottlenecks in our process might be.