use crate::mutate::Mutate;
use crate::random_shape::{RandomCircle, RandomShape};
use crate::image_diff::image_diff;
use image::RgbaImage;
use rand;
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
    gen.sort_by_cached_key(|shape| shape.score(target_img, current_img));
    gen
}

// Takes in the target image along with the current generation of shapes.
// Returns the next generation of shapes.
#[must_use]
pub fn next_generation(
    target_img: &image::RgbaImage,
    current_img: &image::RgbaImage,
    current_gen: &[RandomCircle],
    mutation_factor: f64,
) -> Vec<RandomCircle> {
    let rng = rand::thread_rng();
    let (imgx, imgy) = target_img.dimensions();
    let mut newvec = current_gen.to_vec();

    newvec = sort_generation(target_img, current_img, newvec);
    // Kill worst 80 shapes and replace them with mutated children of the survivors.
    newvec.truncate(20);
    let children: Vec<RandomCircle> = newvec
        .iter()
        .flat_map(|shape| {
            iter::repeat_with(|| shape.mutate(&mut rng.clone(), mutation_factor)).take(3)
        })
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
    current_score: u128,
) -> Option<(RandomCircle, u128)> {
    let (imgx, imgy) = target_img.dimensions();

    let mut shapes: Vec<RandomCircle> = iter::repeat_with(|| RandomCircle::new(imgx, imgy))
        .take(100)
        .collect();

    for i in 0..num_gens {
        let mutation_factor: f64 = 1.0 - 0.9 / f64::from(i * num_gens + 1);
        shapes = next_generation(
            target_img,
            current_img,
            &shapes,
            mutation_factor,
        );
    }

    let best_shape = shapes
        .into_iter()
        .min_by_key(|shape| shape.score(target_img, current_img))
        .unwrap();

    // Calculate the score for the current image at full scale.
    let delta = best_shape.score(target_img, current_img);
    let new_score = if delta >= 0 {
        current_score + delta as u128
    } else {
        current_score - delta.unsigned_abs() as u128
    };

    println!("score per pixel: {}", new_score as f64 / (imgx * imgy) as f64);

    // Save the shape if it was an improvement
    if new_score < current_score {
        Some((best_shape, new_score))
    } else {
        None
    }
}

pub fn evolve(input_path: &str, num_epochs: u32, num_gens: u32, output_folder: &str, scale_down: f64) {
    let target_img = image::open(input_path).unwrap().to_rgba8();
    let (width, height) = target_img.dimensions();
    let mut output_img = RgbaImage::new(width, height);

    let target_img = image::imageops::resize(
        &target_img,
        (f64::from(width) / scale_down) as u32,
        (f64::from(height) / scale_down) as u32,
        image::imageops::FilterType::Nearest,
    );
    let (width, height) = target_img.dimensions();

    let mut current_img = RgbaImage::new(width, height);
    let mut score = u128::from(width * height) * 255 * 3;

    for i in 1..=num_epochs {
        match epoch(
            100,
            num_gens,
            &target_img,
            &current_img,
            score,
        ) {
            Some((best_shape, new_score)) => {
                score = new_score;
                current_img = best_shape.draw(&current_img);
                output_img = best_shape.scale_up(scale_down).draw(&output_img);
            }
            None => {
                //println!("Discarded epoch");
            }
        }

        // Save the output buffer periodically.
        if i % 100 == 0 {
            output_img
                .save(Path::new(output_folder).join(format!("out-{}-{}.jpg", i, score)))
                .expect("Could not save image");
        }

        println!("Done epoch {} of {}", i, num_epochs);
    }

    output_img
        .save(Path::new(output_folder).join("out.jpg"))
        .expect("Could not save image");
}
