# Rust Spectrogram

The scaffolding for an eventual WASM spectrogram build.

Right now the implementation is mostly copied from my previous JS-based work.

It uses some crates that aren't WASM compatible, so it just works as a rust program for now.

```sh
cargo run <audio-path>
```

A good sample is [Where were you a year ago](https://haskinslabs.org/about-us/features-and-demos/sinewave-synthesis/s1spectro).
