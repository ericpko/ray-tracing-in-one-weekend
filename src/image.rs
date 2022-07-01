pub struct Image {
    pub aspect_ratio: f32,
    // pixel dimensions
    pub width: usize,
    pub height: usize,

    pub samples_per_pixel: u32,
    pub max_depth: i32, // ray reflection bounce limit
    // flattened 1D image array of bytes
    pub pixels: Vec<u8>,
}

impl Image {
    pub fn new(aspect_ratio: f32, width: usize, samples_per_pixel: u32, max_depth: i32) -> Self {
        let height = (width as f32 / aspect_ratio) as usize;
        let pixels = Vec::with_capacity(3 * width * height);
        Self {
            aspect_ratio,
            width,
            height,
            samples_per_pixel,
            max_depth,
            pixels,
        }
    }

    pub fn write_ppm(&self) -> std::io::Result<()> {
        let width = self.width;
        let height = self.height;
        let mut image_vec = format!("P6\n{width} {height}\n255\n").as_bytes().to_owned();

        image_vec.extend(&self.pixels);
        std::fs::write("image.ppm", image_vec)?;
        Ok(())
    }
}
