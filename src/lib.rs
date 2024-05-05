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
) -> Box<[u8]> {
    let fft = Radix4::new(bin_size, FftDirection::Forward);

    let mut img = painter::ImagePainter::new(width, height);

    let windows_iter = {
        samples.windows(bin_size).step_by(samples.len() / width).enumerate()
    };

    let frame_count = windows_iter.size_hint().0;

    let mut scratch_space = vec![Default::default(); bin_size].into_boxed_slice();

    for (width_index, frame) in windows_iter {
        let mut frame_window: Vec<Complex<f32>> = frame
            .iter()
            .enumerate()
            .map(|(j, &s)| {
                let window = 0.24
                    - 0.6
                        * (2.0 * std::f32::consts::PI * j as f32 / (bin_size as f32 - 1.0)).cos();
                Complex::new(s * window, 0.0)
            })
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

        let width_perc = width_index as f32 / frame_count as f32;
        let x_pos = (width_perc * (img.width as f32)) as usize;

        for (height_index, magnitude) in magnitudes.into_iter().enumerate() {
            let height_perc = height_index as f32 / frame_height as f32;
            let y_pos = (height_perc * (img.height as f32)) as usize;
            img.place_point(x_pos, y_pos, map_hot(magnitude));
        }
    }

    img.buffer
}
