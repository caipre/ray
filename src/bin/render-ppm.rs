use indicatif::{ProgressBar, ProgressStyle};
use ray::vec3::vec3;

use std::{env, str};

use ray::ray::Ray;
use ray::vec3::WriteColor;

/// Render a reference image in Portable Pixmap (PPM) format.

fn color_ray(r: Ray, center: vec3) -> vec3 {
    if hit_sphere(center, 0.5, &r) {
        return vec3::newi(1, 0, 0);
    }
    let unit = &r.orient.to_unit();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * vec3::new(1.0, 1.0, 1.0) + t * vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = r.orient.dot(&r.orient);
    let b = 2.0 * oc.dot(&r.orient);
    let c = oc.dot(&oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4 as f64 * a * c;
    discriminant > 0 as f64
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
            let r = Ray {
                origin,
                orient: lower_left + u * horizontal + v * vertical,
            };
            let color = color_ray(r, vec3::newi(0, 0, -1));
            color.write(&mut std::io::stdout());
            pb.inc(1);
        }
    }
    pb.finish()
}
