mod vec3;
mod ray;

use vec3::{Vec3, Color, Point3, dot};
use ray::Ray;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = dot(&oc, r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0. {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / (2.*a);
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::build(0., 0., -1.), 0.5, r);
    if t > 0. {
        let n = (r.at(t) - Vec3::build(0., 0., -1.)).unit_vector();
        return Color::build(n.x()*0.5+1., n.y()+1., n.z()+1.);
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    Color::build(1.0, 1.0, 1.0)*(1.0-t) + Color::build(0.5, 0.7, 1.0)*t
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 1000;
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
