#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

pub fn map_hot(percentage: f32) -> Color {
    let black = Color(0, 0, 0);
    let purple = Color(100, 19, 200);
    let red = Color(225, 16, 0);
    let yellow = Color(255, 215, 25);
    let white = Color(255, 255, 255);

    interpolate_colors(percentage, &[black, purple, red, yellow, white])
}

fn interpolate_colors(percentage: f32, colors: &[Color]) -> Color {
    let base_index = (colors.len() - 1) as f32 * percentage;
    let prev_index = base_index.floor();
    let next_index = base_index.ceil();

    let ratio: f32 = 1f32 / ((colors.len() - 1) as f32);
    let mapped_percentage = (percentage - ratio * prev_index) / (ratio * next_index);

    let color1 = colors[prev_index as usize];
    let color2 = colors[next_index as usize];

    interpolate(mapped_percentage, color1, color2)
}

fn interpolate(percentage: f32, color1: Color, color2: Color) -> Color {
    Color(
        (color2.0.abs_diff(color1.0) as f32 * percentage + (color1.0 as f32)) as u8,
        (color2.1.abs_diff(color1.1) as f32 * percentage + (color1.1 as f32)) as u8,
        (color2.2.abs_diff(color1.2) as f32 * percentage + (color1.2 as f32)) as u8,
    )
}
