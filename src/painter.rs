use crate::colormap;
use image::{ImageBuffer, Rgb, };

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

    pub fn get_buffer(self) -> Vec<u8> {
        self.0.into_vec()
    }
}
