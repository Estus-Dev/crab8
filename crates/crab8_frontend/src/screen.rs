use crab8::screen::Screen;

pub trait DrawScreen {
    fn draw_screen(&self, frame: &mut [u8], colors: &[[u8; 4]]);
}

impl DrawScreen for Screen {
    fn draw_screen(&self, frame: &mut [u8], colors: &[[u8; 4]]) {
        let color_off: &[u8; 4] = colors.first().unwrap_or(&[0, 0, 0, 255]);
        let color_lit: &[u8; 4] = colors.get(1).unwrap_or(&[255, 255, 255, 255]);
        let (width, _) = self.size();

        for (i, frame_pixel) in frame.chunks_exact_mut(4).enumerate() {
            let y = i / width;
            let x = i % width;

            frame_pixel.copy_from_slice(if self.lit(x, y) { color_lit } else { color_off });
        }
    }
}
