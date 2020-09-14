fn main() {
    // Image
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r: f32 = i as f32 / (IMAGE_WIDTH as f32 - 1.);
            let g: f32 = j as f32 / (IMAGE_HEIGHT as f32 - 1.);
            let b: f32 = 0.25;

            let ir = (255.999*r) as i32;
            let ig = (255.999*g) as i32;
            let ib = (255.999*b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
