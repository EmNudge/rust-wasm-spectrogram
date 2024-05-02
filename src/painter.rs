use crate::colormap::Color;

pub struct ImagePainter {
    pub buffer: Box<[u8]>,
    pub width: usize,
    pub height: usize,
}

impl ImagePainter {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height * 4].into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn place_point(&mut self, x: usize, y: usize, color: Color) {
        let base_index = (y * self.width * 4) + (x * 4);

        self.buffer[base_index] = color.0;
        self.buffer[base_index + 1] = color.1;
        self.buffer[base_index + 2] = color.2;
        self.buffer[base_index + 3] = 255;
    }
}
