use bevy::{
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::ImageSampler,
    },
};
use chip_8::screen::Screen;

pub fn render_framebuffer(screen: &Screen) -> Image {
    const SCREEN_WIDTH: usize = 128;
    const SCREEN_HEIGHT: usize = 64;
    const PIXEL_CHANNELS: usize = 4;
    const PIXEL_SIZE: usize = 2;
    const PIXEL_LIT: [u8; 4] = [255, 255, 255, 255];
    const PIXEL_OFF: [u8; 4] = [0, 0, 0, 255];

    let mut pixel_data = [0; SCREEN_WIDTH * SCREEN_HEIGHT * PIXEL_CHANNELS];

    for y in 0..SCREEN_HEIGHT {
        let row_offset = SCREEN_WIDTH * PIXEL_CHANNELS * y;
        let row = y / PIXEL_SIZE;
        let row_pixels: Vec<u8> = screen
            .get_row(row)
            .iter()
            .flat_map(|&lit| {
                if lit {
                    PIXEL_LIT.repeat(2)
                } else {
                    PIXEL_OFF.repeat(2)
                }
            })
            .collect();

        pixel_data[row_offset..(row_offset + row_pixels.len())].copy_from_slice(&row_pixels);
    }

    let mut screen_data = Image::new_fill(
        Extent3d {
            width: SCREEN_WIDTH as u32,
            height: SCREEN_HEIGHT as u32,
            ..default()
        },
        TextureDimension::D2,
        &pixel_data,
        TextureFormat::Rgba8UnormSrgb,
    );

    screen_data.sampler_descriptor = ImageSampler::nearest();

    screen_data
}
