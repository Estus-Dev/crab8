use crab8::screen::Screen;

pub trait DrawScreen {
    fn draw_screen(&self, frame: &mut [u8]);
}

impl DrawScreen for Screen {
    fn draw_screen(&self, frame: &mut [u8]) {
        const PIXEL_LIT: [u8; 4] = [255, 255, 255, 255];
        const PIXEL_OFF: [u8; 4] = [0, 0, 0, 255];
        let (width, _) = self.size();

        for (i, frame_pixel) in frame.chunks_exact_mut(4).enumerate() {
            let y = i / width;
            let x = i % width;

            frame_pixel.copy_from_slice(if self.lit(x, y) {
                &PIXEL_LIT
            } else {
                &PIXEL_OFF
            });
        }
    }
}
