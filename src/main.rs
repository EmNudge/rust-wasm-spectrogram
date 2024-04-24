use hound;
use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::{Fft, FftDirection};

mod colormap;
mod painter;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = args.get(1).expect("no file");

    let mut reader = hound::WavReader::open(path).expect("not a valid WAV file");

    let samples = reader
        .samples::<i16>()
        .filter_map(|s| s.ok().map(|s| s as f32))
        .collect::<Vec<f32>>();

    let frame_size = 1024;
    let fft = Radix4::new(frame_size, FftDirection::Forward);

    let img_height = 1080;
    let img_width = 1920;
    let mut img = painter::ImagePainter::new(img_width, img_height);

    let windows_iter = {
        let overlap = frame_size / 50;
        samples.windows(frame_size).step_by(overlap).enumerate()
    };

    let frame_count = windows_iter.size_hint().0;

    let mut scratch_space = vec![Default::default(); frame_size];

    for (i, frame) in windows_iter {
        if i % 1000 == 0 {
            println!("processed {} frames", i);
        }

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
            img.place_point(
                i as f32 / frame_count as f32,
                j as f32 / frame_height as f32,
                magnitude,
            );
        }
    }

    img.save_rand();
}
