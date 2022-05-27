mod image_diff;
mod random_shape;
use image::RgbImage;
use crate::random_shape::RandomShape;
use crate::random_shape::RandomCircle;
use std::vec::Vec;
use std::iter;

// Sort shapes by how close to the target the current image becomes after drawing the
// shape on top.
fn sort_generation(target_img: &image::RgbImage, current_img: &image::RgbImage, gen: &mut Vec<RandomCircle>) {
    gen.sort_by_cached_key(|shape| {
        let new_img = shape.draw(&current_img); 
        image_diff::image_diff(&target_img, &new_img)
    });
}

// Takes in the target image along with the current generation of shapes.
// Returns the next generation of shapes.
fn next_generation(target_img: &image::RgbImage, current_img: &image::RgbImage, current_gen: &Vec<RandomCircle>) -> Vec<RandomCircle> {
    let mut newvec = current_gen.to_vec();

    sort_generation(&target_img, &current_img, &mut newvec);
    // Kill worst 90 shapes and replace them with mutated children of the survivors.
    newvec.truncate(10);
    let children: Vec<RandomCircle> = newvec.iter().map(|shape| { iter::repeat_with(|| shape.mutate()).take(9) })
        .flatten().collect();
    newvec.extend(children);
    return newvec;
}

fn main() {
    let base_img = image::open("test.jpg").unwrap().to_rgb8();
    let base_img = image::imageops::resize(&base_img, 200, 200, image::imageops::FilterType::Nearest);

    let (imgx, imgy) = base_img.dimensions();
    let mut imgbuf = RgbImage::new(imgx, imgy);

    const NUM_EPOCHS: i32 = 100;
    const NUM_GENS: i32 = 50;

    for i in 1..=NUM_EPOCHS {

        let mut shapes: Vec<RandomCircle> = (0..100).map(|_| {
            random_shape::RandomCircle::new(imgx, imgy)
        }).collect();

        for i in 0..NUM_GENS {
            shapes = next_generation(&base_img, &imgbuf, &shapes);
            println!("Done generation {} of {}", i, NUM_GENS);
        }
        sort_generation(&base_img, &imgbuf, &mut shapes);
        let minshape = shapes.iter().min_by_key(|s| image_diff::image_diff(&base_img, &s.draw(&imgbuf))).unwrap();
        imgbuf = minshape.draw(&imgbuf);
        imgbuf.save(format!("out-{}.jpg", i)).expect("Could not save image");

        println!("Done epoch {} of {}", i, NUM_EPOCHS);
    }

    imgbuf.save("out.jpg").expect("Could not save image");
}
