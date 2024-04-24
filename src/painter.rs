use crate::colormap;
use image::{ImageBuffer, Rgb};
use rand::Rng;

pub struct ImagePainter(ImageBuffer<Rgb<u8>, Vec<u8>>);

impl ImagePainter {
    pub fn new(width: u32, height: u32) -> Self {
        Self(ImageBuffer::new(width, height))
    }

    pub fn place_point(&mut self, x_perc: f32, y_perc: f32, mag_perc: f32) {
        let x_pos = x_perc * (self.0.width() as f32);
        let y_pos = y_perc * (self.0.height() as f32);
        let color = colormap::map_hot(mag_perc).as_rgb();
        self.0.put_pixel(x_pos as u32, y_pos as u32, color);
    }

    pub fn save_rand(&self) {
        let path = {
            let mut rng = rand::thread_rng();
            let random_ascii = (0..5)
                .map(|_| rng.gen_range(b'a'..=b'z') as char)
                .collect::<String>();

            let path_str = format!("spectrogram-{}.png", random_ascii);
            std::path::PathBuf::from(path_str)
        };

        dbg!(&path);

        self.0.save(path).unwrap();
    }
}
