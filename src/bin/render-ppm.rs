use indicatif::{ProgressBar, ProgressStyle};
use ray::vec3::vec3;

use std::{env, str};

use ray::ray::Ray;
use ray::vec3::WriteColor;
use ray::hittable::Hittable;
use ray::sphere::Sphere;

/// Render a reference image in Portable Pixmap (PPM) format.

fn color_ray(r: &Ray, world: &Vec<Sphere>) -> vec3 {
    if let Some(hit) = world.hit(r, 0.0, std::f64::INFINITY) {
        return 0.5 * (hit.normal + 1);
    }
    let unit = &r.orient.to_unit();
    let t = 0.5 * (unit.y + 1.0);
    (1.0 - t) * vec3::newi(1, 1, 1) + t * vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let width: u64 = args[1].parse().unwrap();
    let height: u64 = args[2].parse().unwrap();

    // build a world
    let world = vec![
        Sphere { center: vec3::newi(0, 0, -1), radius: 0.5 },
        Sphere { center: vec3::new(0.0, -100.5, -1.0), radius: 100.0 },
    ];

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
            let color = color_ray(&r, &world);
            color.write(&mut std::io::stdout());
            pb.inc(1);
        }
    }
    pb.finish()
}
