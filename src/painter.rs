use crate::colormap::{ map_hot, Color };

pub struct ImagePainter {
    pub buffer: Box<[u8]>,
    pub width: usize,
    pub height: usize,
}

impl ImagePainter {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height * 3].into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn place_point(&mut self, x: usize, y: usize, color: Color) {
        let base_index = x * y;
        self.buffer[base_index] = color.0;
        self.buffer[base_index + 1] = color.1;
        self.buffer[base_index + 2] = color.2;
    }

    pub fn place_point_perc(&mut self, x_perc: f32, y_perc: f32, mag_perc: f32) {
        let x_pos = x_perc * (self.width as f32);
        let y_pos = y_perc * (self.height as f32);
        let color = map_hot(mag_perc);

        self.place_point(x_pos as usize, y_pos as usize, color);
    }
}
