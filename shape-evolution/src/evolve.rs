use image::RgbaImage;
use crate::random_shape::{RandomCircle, RandomShape};
use std::iter;
use std::path::Path;

// Sort shapes by how close to the target the current image becomes after drawing the
// shape on top.
#[must_use]
pub fn sort_generation(
    target_img: &image::RgbaImage,
    current_img: &image::RgbaImage,
    mut gen: Vec<RandomCircle>,
) -> Vec<RandomCircle> {
    gen.sort_by_cached_key(|shape| shape.score(target_img, current_img, 1.0));
    gen
}

// Takes in the target image along with the current generation of shapes.
// Returns the next generation of shapes.
#[must_use]
pub fn next_generation(
    target_img: &image::RgbaImage,
    current_img: &image::RgbaImage,
    current_gen: &[RandomCircle],
) -> Vec<RandomCircle> {
    let (imgx, imgy) = target_img.dimensions();
    let mut newvec = current_gen.to_vec();

    newvec = sort_generation(target_img, current_img, newvec);
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

// Perform a single epoch, returning the best resulting shape and its corresponding score.
// If no shape could be found which lowers the score, this function returns None.
pub fn epoch(
    generation_size: usize,
    num_gens: u32,
    target_img: &image::RgbaImage,
    current_img: &image::RgbaImage,
    scaled_target_img: &image::RgbaImage,
    scaled_current_img: &image::RgbaImage,
    scale: f64,
    current_score: i64,
) -> Option<(RandomCircle, i64)> {
    let (imgx, imgy) = scaled_target_img.dimensions();

    let mut shapes: Vec<RandomCircle> = iter::repeat_with(|| RandomCircle::new(imgx, imgy))
        .take(100)
        .collect();

    for _i in 0..num_gens {
        shapes = next_generation(scaled_target_img, scaled_current_img, &shapes);
    }

    let best_shape = shapes
        .into_iter()
        .min_by_key(|shape| shape.score(scaled_target_img, scaled_current_img, 1.0))
        .unwrap();

    // Calculate the score for the current image at full scale.
    let new_score = current_score + best_shape.score(target_img, current_img, scale);

    println!("score diff {}", new_score - current_score);

    // Save the shape if it was an improvement
    if new_score < current_score {
        Some((best_shape, new_score))
    } else {
        None
    }
}

pub fn evolve(input_path: &str, num_epochs: u32, num_gens: u32, output_folder: &str, scale: f64) {
    let target_img = image::open(input_path).unwrap().to_rgba8();

    let (imgx, imgy) = target_img.dimensions();
    let mut score = (imgx * imgy * 255 * 3) as i64;

    // Unscaled image used for output
    let mut outbuf = RgbaImage::new(imgx, imgy);

    // Create a scaled down target image for faster drawing and scoring
    let scaled_target_img = image::imageops::resize(
        &target_img,
        (imgx as f64 / scale) as u32,
        (imgy as f64 / scale) as u32,
        image::imageops::FilterType::Nearest,
    );

    let (imgx, imgy) = scaled_target_img.dimensions();
    let mut imgbuf = RgbaImage::new(imgx, imgy);

    for i in 1..=num_epochs {
        match epoch(
            100,
            num_gens,
            &target_img,
            &outbuf,
            &scaled_target_img,
            &imgbuf,
            scale,
            score,
        ) {
            Some((best_shape, new_score)) => {
                score = new_score;
                outbuf = best_shape.draw(&outbuf, scale);
                imgbuf = best_shape.draw(&imgbuf, 1.0);
            }
            None => {
                println!("Discarded epoch");
            }
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
