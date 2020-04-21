use indicatif::{ProgressBar, ProgressStyle};
use ray::vec3::vec3;

use std::{str, env};

use ray::vec3::WriteColor;
use ray::ray::Ray;

/// Render a reference image in Portable Pixmap (PPM) format.

fn color_ray(r: Ray) -> vec3 {
    let unit = r.orient.to_unit();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * vec3::new(1.0, 1.0, 1.0) + t * vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let width: u64 = args[1].parse().unwrap();
    let height: u64 = args[2].parse().unwrap();

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    let pb = ProgressBar::new(width * height);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:42.cyan/blue}] ({eta})")
            .progress_chars("=> "),
    );

    let lower_left = vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vec3::new(4.0, 0.0, 0.0);
    let vertical = vec3::new(0.0, 2.0, 0.0);
    let origin = vec3::ORIGIN;

    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f64 / width as f64;
            let v = j as f64 / height as f64;
            let r = Ray { origin, orient: lower_left + u * horizontal + v * vertical };
            let color = color_ray(r);
            color.write(&mut std::io::stdout());
            pb.inc(1);
        }
    }
    pb.finish()
}
