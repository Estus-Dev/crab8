pub(crate) const LOGO: &[u8] = include_bytes!("../../../../docs/images/logo-2x.png");

pub(crate) fn load_egui_image(buffer: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::load_from_memory(buffer)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let image_data = image_buffer.as_flat_samples();

    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        image_data.as_slice(),
    ))
}
