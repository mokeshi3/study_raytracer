mod vec3;

use vec3::Vec3;

fn main() {
    // Image
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let pixel = Vec3::new(
                i as f32 / (IMAGE_WIDTH as f32 - 1.),
                j as f32 / (IMAGE_HEIGHT as f32 - 1.),
                0.25,
            );

            pixel.println_color();
        }
    }
}
