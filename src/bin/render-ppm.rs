use indicatif::{ProgressBar, ProgressStyle};
use ray::vec3::vec3;

use std::{env, str};

use ray::ray::Ray;
use ray::vec3::WriteColor;

/// Render a reference image in Portable Pixmap (PPM) format.

fn color_ray(r: Ray, center: vec3) -> vec3 {
    if let Some(t) = hit_sphere(center, 0.5, &r) {
        let n = (r.at(t) - center).to_unit();
        return 0.5 * (n + 1);
    }
    let unit = &r.orient.to_unit();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * vec3::new(1.0, 1.0, 1.0) + t * vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: vec3, radius: f64, r: &Ray) -> Option<f64> {
    let oc = r.origin - center;
    let a = r.orient.dot(&r.orient);
    let half_b = oc.dot(&r.orient);
    let c = oc.dot(&oc) - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;
    if discriminant > 0.0 {
        Some((-half_b - discriminant.sqrt()) / a)
    } else {
        None
    }
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
