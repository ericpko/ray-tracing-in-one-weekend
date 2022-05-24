// Image dimensions in pixels
const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

pub fn render() -> anyhow::Result<()> {
    pretty_env_logger::init();
    log::info!("rendering image...");

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / (IMAGE_WIDTH as f32 - 1.);
            let g = j as f32 / (IMAGE_HEIGHT as f32 - 1.);
            let b = 0.25;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            println!("{ir} {ig} {ib}");
        }
    }

    Ok(())
}
