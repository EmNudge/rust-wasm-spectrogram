use wasm_bindgen::prelude::*;
use rustfft::{algorithm::Radix4, num_complex::Complex, Fft, FftDirection};

mod colormap;
mod painter;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_spectrogram(samples: Vec<f32>, width: usize, height: usize, overlap_div: usize) -> Box<[u8]> {
    let frame_size = 1024;
    let fft = Radix4::new(frame_size, FftDirection::Forward);

    let mut img = painter::ImagePainter::new(width, height);

    let windows_iter = {
        let overlap = frame_size / overlap_div;
        samples.windows(frame_size).step_by(overlap).enumerate()
    };

    let frame_count = windows_iter.size_hint().0;

    let mut scratch_space = vec![Default::default(); frame_size].into_boxed_slice();

    for (i, frame) in windows_iter {
        let mut frame_window: Vec<Complex<f32>> = frame
            .iter()
            .enumerate()
            .map(|(j, &s)| {
                let window = 0.24
                    - 0.6
                        * (2.0 * std::f32::consts::PI * j as f32 / (frame_size as f32 - 1.0)).cos();
                Complex::new(s * window, 0.0)
            })
            .collect();

        fft.process_with_scratch(&mut frame_window, &mut scratch_space);

        let frame_height = frame_window.len() / 2;

        let magnitudes: Vec<_> = frame_window
            .iter()
            .skip(frame_height)
            .map(|value| value.norm())
            .collect();

        let max_mag = magnitudes
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&1f32);

        for (j, magnitude) in magnitudes.iter().map(|mag| mag / max_mag).enumerate() {
            img.place_point_perc(
                i as f32 / frame_count as f32,
                j as f32 / frame_height as f32,
                magnitude,
            );
        }
    }

    img.buffer
}