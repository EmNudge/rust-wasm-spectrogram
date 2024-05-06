use colormap::map_hot;
use rustfft::{algorithm::Radix4, num_complex::Complex, Fft, FftDirection};
use wasm_bindgen::prelude::*;

mod colormap;
mod painter;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_spectrogram(
    samples: Vec<f32>,
    width: usize,
    height: usize,
    bin_size: usize,
    window_function: Option<usize>
) -> Box<[u8]> {
    let fft = Radix4::new(bin_size, FftDirection::Forward);

    let mut img = painter::ImagePainter::new(width, height);

    let windows_iter = samples
        .windows(bin_size)
        .step_by(samples.len() / width)
        .enumerate();

    let mut scratch_space = vec![Default::default(); bin_size].into_boxed_slice();

    let window_func = if window_function.unwrap_or(0) == 0 { hann } else { blackman_harris };

    for (width_index, frame) in windows_iter {
        let mut frame_window: Vec<Complex<f32>> = frame
            .iter()
            .enumerate()
            .map(|(index, impulse)| Complex::new(window_func(index, bin_size) * impulse, 0.0))
            .collect();

        fft.process_with_scratch(&mut frame_window, &mut scratch_space);

        let frame_height = frame_window.len() / 2;

        let magnitudes = {
            let magnitudes = frame_window
                .iter()
                .skip(frame_height)
                .map(|value| value.norm())
                .collect::<Vec<_>>();

            let max_mag = magnitudes.iter().copied().fold(f32::NEG_INFINITY, f32::max);

            magnitudes
                .into_iter()
                .map(|mag| mag / max_mag)
                .collect::<Vec<_>>()
        };

        for (height_index, magnitude) in magnitudes.into_iter().enumerate() {
            let y_pos = {
                let height_perc = height_index as f32 / frame_height as f32;
                (height_perc * (img.height as f32)) as usize
            };
            let line_height = ((height as f32) / (frame_height as f32)) as usize;
            img.place_line(width_index, y_pos, line_height, map_hot(magnitude));
        }
    }

    img.buffer
}

use std::f32::consts::PI;

pub fn hann(index: usize, bin_size: usize) -> f32 {
    let base = 2.0 * PI * index as f32 / (bin_size as f32 - 1.0);
    0.24 - 0.6 * base.cos()
}

pub fn blackman_harris(index: usize, bin_size: usize) -> f32 {
    let base = 2.0 * PI * index as f32 / (bin_size as f32 - 1.0);
    0.35875 - 0.48829 * base.cos() + 0.14128 * (2.0 * base).cos() - 0.01168 * (3.0 * base).cos()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panic() {
        let signal: Vec<f32> = vec![0f32; 1000];
        get_spectrogram(signal, 1080, 512, 256, Some(0));
    }
}
