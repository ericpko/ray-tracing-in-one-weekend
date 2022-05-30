pub struct Image {
    pub aspect_ratio: f32,
    // pixel dimensions
    pub width: usize,
    pub height: usize,
    // flattened 1D image array of bytes
    pub pixels: Vec<u8>,
}

impl Image {
    pub fn new(aspect_ratio: f32, width: usize) -> Self {
        let height = (width as f32 / aspect_ratio) as usize;
        let pixels = Vec::with_capacity(3 * width * height);
        Self {
            aspect_ratio,
            width,
            height,
            pixels,
        }
    }
}
