use image;

const HEIGHT: u32 = 2048;
const WIDTH: u32 = 2048;

fn main() {
    let mut buf = image::ImageBuffer::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let p = buf.get_pixel_mut(x, y);
            let r = (f64::from(x) / f64::from(WIDTH) * 255.99) as u8;
            let g = (f64::from(y) / f64::from(HEIGHT) * 255.99) as u8;
            let b = (0.2 * 255.99) as u8;
            *p = image::Rgb([r, g, b]);
        }
    }

    buf.save("out.png").expect("Unable to write output file");
}
