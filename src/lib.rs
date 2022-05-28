#![allow(unused)] // ! mute unused warnings for now
mod ray;
use ray::Ray;

// Image dimensions in pixels
const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

pub fn render() -> anyhow::Result<()> {
    pretty_env_logger::init();
    log::info!("rendering image...");

    let mut image: Vec<u8> = Vec::with_capacity(3 * IMAGE_WIDTH * IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / (IMAGE_WIDTH as f32 - 1.);
            let g = j as f32 / (IMAGE_HEIGHT as f32 - 1.);
            let b = 0.25;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            image.extend([ir, ig, ib]);
        }
    }
    write_ppm(image)?;

    Ok(())
}

fn write_ppm(image: Vec<u8>) -> std::io::Result<()> {
    let mut image_vec = format!("P6\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n")
        .as_bytes()
        .to_owned(); // &[u8] -> Vec<u8>

    image_vec.extend(image);
    std::fs::write("image.ppm", image_vec)?;
    Ok(())
}
