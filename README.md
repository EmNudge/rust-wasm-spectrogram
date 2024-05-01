# Rust Wasm Spectrogram

A test to see how efficient we can get a wasm-based spectrogram build.

Test with [Where were you a year ago](https://haskinslabs.org/about-us/features-and-demos/sinewave-synthesis/s1spectro).

I used a makefile because why not. Run the demo with:
```sh
make serve
# or
npx serve .
```

Timing data is available in the console using `console.time()` to see where the bottlenecks in our process might be.