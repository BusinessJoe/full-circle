use crate::random_shape::RandomCircle;
use crate::random_shape::RandomShape;
use std::iter;

use crate::image_diff::image_diff;
use image::{RgbaImage, Rgba};
use std::path::Path;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

pub mod image_diff;
pub mod random_shape;
mod utils;

#[wasm_bindgen]
pub struct TestStruct {
    target_img: image::RgbaImage,
    current_img: image::RgbaImage,
    scaled_target_img: image::RgbaImage,
    scaled_current_img: image::RgbaImage,
    scale: f64,
    current_score: i64,
}

#[wasm_bindgen]
impl TestStruct {
    #[wasm_bindgen(constructor)]
    pub fn new(path: &str) -> Self {
        utils::set_panic_hook();

        let scale = 6.0;
        let (width, height) = (600, 600);
        let mut target_img = RgbaImage::new(width, height);
        for x in 0..width {
            for y in 0..height {
                target_img.put_pixel(x, y, Rgba([(x*255/width).try_into().unwrap(), 127, (y*255/height).try_into().unwrap(), 255]));
            }
        }

        let (scaled_width, scaled_height) = ((width as f64 / scale) as u32, (height as f64 / scale) as u32);
        // Create a scaled down target image for faster drawing and scoring
        let scaled_target_img = image::imageops::resize(
            &target_img,
            scaled_width,
            scaled_height,
            image::imageops::FilterType::Nearest,
        );

        Self {
            target_img,
            current_img: RgbaImage::new(width, height),
            scaled_target_img,
            scaled_current_img: RgbaImage::new(scaled_width, scaled_height),
            scale,
            current_score: i64::from(width*height*255*3),
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let mut data = self.current_img.to_vec();

        let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), 600, 600)?;
        ctx.put_image_data(&data, 0.0, 0.0)
    }

    pub fn try_epoch(&mut self, generation_size: usize, num_gens: u32) -> bool {
        match epoch(
            generation_size,
            num_gens,
            &self.target_img,
            &self.current_img,
            &self.scaled_target_img,
            &self.scaled_current_img,
            self.scale,
            self.current_score,
        ) {
            Some((best_shape, new_score)) => {
                self.current_score = new_score;
                self.current_img = best_shape.draw(&self.current_img, self.scale);
                self.scaled_current_img = best_shape.draw(&self.scaled_current_img, 1.0);
                true
            },
            None => false
        }
    }
}

/*
#[wasm_bindgen]
pub struct Evolution {
    target_img: image::RgbaImage,
    current_img: image::RgbaImage,
    scaled_target_img: image::RgbaImage,
    scaled_current_img: image::RgbaImage,
    scale: f64,
    current_score: i64,
}

// Public methods, exported to JS
#[wasm_bindgen]
impl Evolution {
    #[wasm_bindgen(constructor)]
    pub fn new(target_path: &str, scale: f64) -> Self {
        utils::set_panic_hook();

        let target_img = image::open(target_path).unwrap().to_rgb8();
        let (imgx, imgy) = target_img.dimensions();

        let current_score = (imgx * imgy * 255 * 3) as i64;

        // Unscaled image used for output
        let current_img = RgbaImage::new(imgx, imgy);

        // Create a scaled down target image for faster drawing and scoring
        let scaled_target_img = image::imageops::resize(
            &target_img,
            (imgx as f64 / scale) as u32,
            (imgy as f64 / scale) as u32,
            image::imageops::FilterType::Nearest,
        );

        let (imgx, imgy) = scaled_target_img.dimensions();
        let scaled_current_img = RgbaImage::new(imgx, imgy);

        Self {
            target_img,
            current_img,
            scaled_target_img,
            scaled_current_img,
            scale,
            current_score,
        }
    }

    // Attempts to add a shape to the current image by performing an epoch.
    // Returns true on success, false otherwise.
    pub fn try_epoch(&mut self, generation_size: usize, num_gens: u32) -> bool {
        match epoch(
            generation_size,
            num_gens,
            &self.target_img,
            &self.current_img,
            &self.scaled_target_img,
            &self.scaled_current_img,
            self.scale,
            self.current_score,
        ) {
            Some((best_shape, new_score)) => {
                self.current_score = new_score;
                self.current_img = best_shape.draw(&self.current_img, self.scale);
                self.scaled_current_img = best_shape.draw(&self.scaled_current_img, 1.0);
                true
            },
            None => false
        }
    }

    pub fn draw_current(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let mut data = self.current_img.to_vec();
        let (width, height) = self.current_img.dimensions();
        let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
        ctx.put_image_data(&data, 0.0, 0.0)
    }
}
*/

// ============================

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
// ============================

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
) -> Option<(impl RandomShape, i64)> {
    let (imgx, imgy) = scaled_target_img.dimensions();

    let mut shapes: Vec<RandomCircle> = iter::repeat_with(|| RandomCircle::new(imgx, imgy))
        .take(100)
        .collect();

    for _i in 0..num_gens {
        shapes = next_generation(scaled_target_img, scaled_current_img, &shapes);
    }

    let best_shape = shapes
        .into_iter()
        .min_by_key(|shape| shape.score(target_img, current_img, scale))
        .unwrap();

    // Calculate the score for the current image at full scale.
    let new_score = current_score + best_shape.score(target_img, current_img,  scale);

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
            score
        ) {
            Some((best_shape, new_score)) => {
                score = new_score;
                outbuf = best_shape.draw(&outbuf, scale);
                imgbuf = best_shape.draw(&imgbuf, 1.0);
            },
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
