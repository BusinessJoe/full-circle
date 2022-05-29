use crate::random_shape::RandomCircle;
use crate::random_shape::RandomShape;
use std::iter;

use image::RgbImage;
use image::GenericImageView;
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
) -> Vec<RandomCircle> {
    gen.sort_by_cached_key(|shape| {
        shape.score(target_img, current_img)
    });
    gen
}

// Takes in the target image along with the current generation of shapes.
// Returns the next generation of shapes.
#[must_use]
pub fn next_generation(
    target_img: &image::RgbImage,
    current_img: &image::RgbImage,
    current_gen: &[RandomCircle],
) -> Vec<RandomCircle> {
    let (imgx, imgy) = target_img.dimensions();
    let mut newvec = current_gen.to_vec();

    newvec = sort_generation(target_img, current_img, newvec);
    // Kill worst 90 shapes and replace them with mutated children of the survivors.
    newvec.truncate(10);
    let children: Vec<RandomCircle> = newvec
        .iter()
        .flat_map(|shape| iter::repeat_with(|| shape.mutate()).take(8))
        .collect();
    newvec.extend(children);
    newvec.extend(iter::repeat_with(|| RandomCircle::new(imgx, imgy)).take(10));
    newvec
}

pub fn evolve(input_path: &str, num_epochs: u32, num_gens: u32, output_folder: &str, scale: f32) {
    let target_img = image::open(input_path).unwrap().to_rgb8();

    let (imgx, imgy) = target_img.dimensions();

    // Unscaled image used for output
    let mut outbuf = RgbImage::new(imgx, imgy);

    // Create a scaled down target image for faster drawing and scoring
    let scaled_target_img = image::imageops::resize(
        &target_img,
        (imgx as f32 / scale) as u32,
        (imgy as f32 / scale) as u32,
        image::imageops::FilterType::Nearest,
    );

    let (imgx, imgy) = scaled_target_img.dimensions();
    let mut imgbuf = RgbImage::new(imgx, imgy);

    for i in 1..=num_epochs {
        let mut shapes: Vec<RandomCircle> = (0..100)
            .map(|_| random_shape::RandomCircle::new(imgx, imgy))
            .collect();

        for _i in 0..num_gens {
            shapes = next_generation(&scaled_target_img, &imgbuf, &shapes);
            //println!("Done generation {} of {}", i, NUM_GENS);
        }
        shapes = sort_generation(&scaled_target_img, &imgbuf, shapes);
        let minshape = shapes
            .iter()
            .min_by_key(|s| image_diff::image_diff(&scaled_target_img, &s.draw(&imgbuf, 1.0)))
            .unwrap();
        imgbuf = minshape.draw(&imgbuf, 1.0);

        // Calculate the score for the current image
        let score = image_diff::image_diff(&scaled_target_img, &imgbuf);

        // Draw the unscaled shape to the output buffer and save it as a file periodically
        outbuf = minshape.draw(&outbuf, scale);

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




