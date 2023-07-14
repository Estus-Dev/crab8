use egui::{Context, FontData, FontFamily};

const PIXELOID: &[u8] = include_bytes!("../assets/fonts/pixeloid-font/PixeloidMono-VGj6x.ttf");

pub fn set_font_data(context: &Context) {
    let mut fonts = egui::FontDefinitions::empty();

    fonts
        .font_data
        .insert("pixeloid".to_owned(), FontData::from_static(PIXELOID));

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "pixeloid".to_owned());

    // We're going to simply force proportional fonts to pixeloid anyway
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "pixeloid".to_owned());

    context.set_fonts(fonts);
}
