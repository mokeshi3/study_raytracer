mod vec3;
mod ray;

use vec3::{Vec3, Color, Point3};
use ray::Ray;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    Color::build(1.0, 1.0, 1.0)*(1.0-t) + Color::build(0.5, 0.7, 1.0)*t
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::build(0., 0., 0.);
    let horizontal = Vec3::build(viewport_width, 0., 0.);
    let vertical = Vec3::build(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal/2. - vertical/2. - Vec3::build(0., 0., focal_length);

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH-1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT-1) as f64;
            let r = Ray::build(origin, lower_left_corner + horizontal*u + vertical*v - origin);
            let pixel_color = ray_color(&r);
            pixel_color.println_color();
        }
    }
}
