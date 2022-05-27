use image::RgbImage;
use std::vec::Vec;
use std::path::Path;
use std::env;

use wallpaper_evolution::{sort_generation, next_generation, image_diff};
use wallpaper_evolution::random_shape::{RandomShape, RandomCircle};
use wallpaper_evolution::random_shape;

fn evolve(input_path: &str, num_epochs: u32, num_gens: u32, output_folder: &str, scale: f32) {
    let target_img = image::open(input_path).unwrap().to_rgb8();

    let (imgx, imgy) = target_img.dimensions();

    // Unscaled image used for output
    let mut outbuf = RgbImage::new(imgx, imgy);

    // Create a scaled down target image for faster drawing and scoring
    let scaled_target_img = image::imageops::resize(
        &target_img, 
        (imgx as f32 / scale) as u32, (imgy as f32 / scale) as u32, 
        image::imageops::FilterType::Nearest);

    let (imgx, imgy) = scaled_target_img.dimensions();
    let mut imgbuf = RgbImage::new(imgx, imgy);

    for i in 1..=num_epochs {

        let mut shapes: Vec<RandomCircle> = (0..100).map(|_| {
            random_shape::RandomCircle::new(imgx, imgy)
        }).collect();

        for i in 0..num_gens {
            shapes = next_generation(&scaled_target_img, &imgbuf, &shapes);
            //println!("Done generation {} of {}", i, NUM_GENS);
        }
        sort_generation(&scaled_target_img, &imgbuf, &mut shapes);
        let minshape = shapes.iter().min_by_key(|s| image_diff::image_diff(&scaled_target_img, &s.draw(&imgbuf, 1.0))).unwrap();
        imgbuf = minshape.draw(&imgbuf, 1.0);

        // Calculate the score for the current image
        let score = image_diff::image_diff(&scaled_target_img, &imgbuf);
        
        // Draw the unscaled shape to the output buffer and save it as a file periodically
        outbuf = minshape.draw(&outbuf, scale);

        if i % 10 == 0 {
            outbuf.save(Path::new(output_folder).join(format!("out-{}-{}.jpg", i, score))).expect("Could not save image");
        }

        println!("Done epoch {} of {}", i, num_epochs);
    }

    outbuf.save(Path::new(output_folder).join("out.jpg")).expect("Could not save image");
}

fn main() {
    let mut args = env::args();
    let target_path = args.nth(1).unwrap();
    let scale = args.next().unwrap();
    let scale = match scale.parse() {
        Ok(s) => s,
        Err(_) => panic!("Could not parse {} as a scale", scale),
    };
    evolve(&target_path, 500, 80, "out", scale);
}
