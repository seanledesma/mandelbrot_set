use clap::Parser;
use image::{Rgb, RgbImage};
use num::Complex;
use palette::{Hsv, Srgb, FromColor};

#[derive(Parser)]
/// Command line options for Mandelbrot image generation.
struct Opts {
    /// x-coordinate of the starting point. Short flag: `-x`
    #[arg(short = 'x', long, default_value = "-0.75", allow_hyphen_values(true))]
    x: f64,

    /// y-coordinate of the starting point. Short flag: `-y`
    #[arg(short = 'y', long, default_value = "0.0", allow_hyphen_values(true))]
    y: f64,

    /// Width of the region in the complex plane. Short flag: `-w`
    #[arg(short = 'w', long, default_value = "2.5", allow_hyphen_values(true))]
    width: f64,

    /// Height of the region in the complex plane. Short flag: `-t`
    #[arg(short = 't', long, default_value = "2.5", allow_hyphen_values(true))]
    height: f64,

    /// Width of the output image in pixels. Short flag: `-m`
    #[arg(short = 'm', long, default_value = "800", allow_hyphen_values(true))]
    image_width: u32,

    /// Height of the output image in pixels. Short flag: `-n`
    #[arg(short = 'n', long, default_value = "800", allow_hyphen_values(true))]
    image_height: u32,
}


fn mandelbrot(c: Complex<f64>, max_iterations: u32) -> u32 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..max_iterations {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iterations
}

fn generate_image(x: f64, y: f64, width: f64, height: f64, image_width: u32, image_height: u32) -> RgbImage {
    let mut img = RgbImage::new(image_width, image_height);
    let max_iterations = 256;

    let scale_x = width / image_width as f64;
    let scale_y = height / image_height as f64;

    let center_x = x - width / 2.0;
    let center_y = y - height / 2.0;

    for (img_x, img_y, pixel) in img.enumerate_pixels_mut() {
        let cx = center_x + (img_x as f64 * scale_x);
        let cy = center_y + (img_y as f64 * scale_y);

        let result = mandelbrot(Complex::new(cx, cy), max_iterations);

        //changing this from grayscale to color
        if result < max_iterations {
            let hue = 360.0 * (result as f64 / max_iterations as f64);
            let saturation = 1.6;
            let value = 0.8;

            let hsv = Hsv::new(hue, saturation, value);
            let rgb: Srgb<f64> = Srgb::from_color(hsv); // convert HSV to RGB
            let color = (
                (rgb.red * 255.0) as u8,
                (rgb.green * 255.0) as u8,
                (rgb.blue * 255.0) as u8,
            );
            *pixel = Rgb([color.0, color.1, color.2]);
        } else {
            *pixel = Rgb([0, 0, 0]); // black for points inside the set
        }

        // uncomment the following for grayscale
        // let color = (255.0 * (result as f64 / 256.0)) as u8;
        // *pixel = Rgb([color, color, color]);
    }


    img
}

fn main() {
    let opts: Opts = Opts::parse();

    let img = generate_image(opts.x, opts.y, opts.width, opts.height, opts.image_width, opts.image_height);
    img.save("mandelbrot.png").unwrap();
}
