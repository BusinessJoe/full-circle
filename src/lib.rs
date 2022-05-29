use crate::random_shape::RandomCircle;
use crate::random_shape::RandomShape;
use std::iter;

use crate::image_diff::image_diff;
use image::RgbImage;
use std::path::Path;

pub mod image_diff;
pub mod random_shape;

// Sort shapes by how close to the target the current image becomes after drawing the
// shape on top.
#[must_use]
pub fn sort_generation(
    target_img: &image::RgbImage,
    current_img: &image::RgbImage,
    mut gen: Vec<RandomCircle>,
    prev_score: i64,
) -> Vec<RandomCircle> {
    gen.sort_by_cached_key(|shape| shape.score(target_img, current_img, prev_score, 1.0));
    gen
}

// Takes in the target image along with the current generation of shapes.
// Returns the next generation of shapes.
#[must_use]
pub fn next_generation(
    target_img: &image::RgbImage,
    current_img: &image::RgbImage,
    current_gen: &[RandomCircle],
    prev_score: i64,
) -> Vec<RandomCircle> {
    let (imgx, imgy) = target_img.dimensions();
    let mut newvec = current_gen.to_vec();

    newvec = sort_generation(target_img, current_img, newvec, prev_score);
    // Kill worst 80 shapes and replace them with mutated children of the survivors.
    newvec.truncate(20);
    let children: Vec<RandomCircle> = newvec
        .iter()
        .flat_map(|shape| iter::repeat_with(|| shape.mutate()).take(3))
        .collect();
    newvec.extend(children);
    newvec.extend(iter::repeat_with(|| RandomCircle::new(imgx, imgy)).take(20));
    newvec
}

pub fn evolve(input_path: &str, num_epochs: u32, num_gens: u32, output_folder: &str, scale: f64) {
    let target_img = image::open(input_path).unwrap().to_rgb8();

    let (imgx, imgy) = target_img.dimensions();
    let mut score = (imgx * imgy * 255 * 3) as i64;

    // Unscaled image used for output
    let mut outbuf = RgbImage::new(imgx, imgy);

    // Create a scaled down target image for faster drawing and scoring
    let scaled_target_img = image::imageops::resize(
        &target_img,
        (imgx as f64 / scale) as u32,
        (imgy as f64 / scale) as u32,
        image::imageops::FilterType::Nearest,
    );

    let (imgx, imgy) = scaled_target_img.dimensions();
    let mut imgbuf = RgbImage::new(imgx, imgy);

    for i in 1..=num_epochs {
        let mut shapes: Vec<RandomCircle> = iter::repeat_with(|| RandomCircle::new(imgx, imgy))
            .take(100)
            .collect();

        let scaled_score = image_diff(&scaled_target_img, &imgbuf);
        for _i in 0..num_gens {
            shapes = next_generation(&scaled_target_img, &imgbuf, &shapes, scaled_score);
            //println!("Done generation {} of {}", i, NUM_GENS);
        }

        let best_shape = shapes
            .iter()
            .min_by_key(|shape| shape.score(&target_img, &outbuf, score, scale))
            .unwrap();

        // Calculate the score for the current image at full scale.
        let new_score = best_shape.score(&target_img, &outbuf, score, scale);

        println!("score diff {}", new_score - score);

        // Save the shape if it was an improvement
        if new_score < score {
            imgbuf = best_shape.draw(&imgbuf, 1.0);
            score = new_score;

            // Draw the unscaled shape to the output buffer.
            outbuf = best_shape.draw(&outbuf, scale);
        } else {
            println!("Discarded epoch");
        }

        // Save the output buffer periodically.
        if i % 10 == 0 {
            outbuf
                .save(Path::new(output_folder).join(format!("out-{}-{}.jpg", i, score)))
                .expect("Could not save image");
        }

        println!("Done epoch {} of {}", i, num_epochs);
    }

    outbuf
        .save(Path::new(output_folder).join("out.jpg"))
        .expect("Could not save image");
}
