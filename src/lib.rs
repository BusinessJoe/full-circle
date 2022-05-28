use crate::random_shape::RandomCircle;
use crate::random_shape::RandomShape;
use std::iter;

pub mod image_diff;
pub mod random_shape;

// Sort shapes by how close to the target the current image becomes after drawing the
// shape on top.
pub fn sort_generation(
    target_img: &image::RgbImage,
    current_img: &image::RgbImage,
    gen: &mut [RandomCircle],
) {
    gen.sort_by_cached_key(|shape| {
        let new_img = shape.draw(current_img, 1.0);
        image_diff::image_diff(target_img, &new_img)
    });
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

    sort_generation(target_img, current_img, &mut newvec);
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
