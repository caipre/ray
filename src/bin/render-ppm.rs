use indicatif::{ProgressBar, ProgressStyle};
use ray::vec3::vec3;

use ray::vec3::WriteColor;

/// Render a reference image in Portable Pixmap (PPM) format.

const IMAGE_WIDTH: u64 = 400;
const IMAGE_HEIGHT: u64 = 200;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let pb = ProgressBar::new(IMAGE_WIDTH * IMAGE_HEIGHT);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:42.cyan/blue}] ({eta})")
            .progress_chars("=> "),
    );
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / IMAGE_WIDTH as f64;
            let g = j as f64 / IMAGE_HEIGHT as f64;
            let b = 0.2;
            let color = vec3::new(r, g, b);
            color.write(&mut std::io::stdout());
            pb.inc(1);
        }
    }
    pb.finish()
}
