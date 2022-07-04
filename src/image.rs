use std::sync::{Arc, Mutex};

pub struct Image {
    pub aspect_ratio: f32,
    // pixel dimensions
    pub width: usize,
    pub height: usize,

    pub samples_per_pixel: u32,
    pub max_depth: i32, // ray reflection bounce limit
    // flattened 1D image array of bytes
    pub pixels: Arc<Mutex<Vec<u8>>>,
}

impl Image {
    pub fn new(aspect_ratio: f32, width: usize, samples_per_pixel: u32, max_depth: i32) -> Self {
        let height = (width as f32 / aspect_ratio) as usize;
        // let pixels = Vec::with_capacity(3 * width * height);
        let pixels = Arc::new(Mutex::new(vec![0u8; 3 * width * height]));
        Self {
            aspect_ratio,
            width,
            height,
            samples_per_pixel,
            max_depth,
            pixels,
        }
    }

    // pub fn color_pixel(&mut self, i: usize, j: usize, rgb: &[u8]) {
    //     // let rgb_image = self.pixels.lock().unwrap();
    //     // rgb_image[3 * (j * self.width + i) + 0] = rgb[0];
    //     // rgb_image[3 * (j * self.width + i) + 1] = rgb[1];
    //     // rgb_image[3 * (j * self.width + i) + 2] = rgb[2];
    //     let mut pixels = Arc::clone(&self.pixels);
    //     if let Ok(test) = pixels.lock() {
    //         test[3 * (j * self.width + i) + 0] = rgb[0];
    //         test[3 * (j * self.width + i) + 1] = rgb[1];
    //         test[3 * (j * self.width + i) + 2] = rgb[2];
    //     }
    // }

    pub fn write_ppm(&self) -> std::io::Result<()> {
        let width = self.width;
        let height = self.height;
        let mut image_vec = format!("P6\n{width} {height}\n255\n").as_bytes().to_owned();

        // let pixels = self.pixels.lock().unwrap();
        if let Ok(pixels) = self.pixels.lock() {
            image_vec.extend(pixels.iter());
        }
        // image_vec.extend(pixels);
        std::fs::write("image.ppm", image_vec)?;
        Ok(())
    }
}
