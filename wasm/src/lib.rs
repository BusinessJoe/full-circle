use image::RgbaImage;

use js_sys::ArrayBuffer;
use js_sys::Uint8Array;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{console, CanvasRenderingContext2d, ImageData};

use shape_evolution::evolve::epoch;
use shape_evolution::random_shape::{RandomCircle, RandomShape};

mod utils;
pub mod web;


#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct TestStruct {
    target_img: image::RgbaImage,
    current_img: image::RgbaImage,
    scaled_target_img: image::RgbaImage,
    scaled_current_img: image::RgbaImage,
    scale: f64,
    current_score: i64,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl TestStruct {
    pub async fn new_async(url: String) -> Self {
        utils::set_panic_hook();

        let target_img = web::load_image(&url).await.unwrap();
        let (width, height) = target_img.dimensions();

        let scale = std::cmp::max(width, height) as f64 / 50.0;
        let (scaled_width, scaled_height) = (
            (width as f64 / scale) as u32,
            (height as f64 / scale) as u32,
        );
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
            current_score: i64::from(width * height * 255 * 3),
        }
    }

    pub fn new_from_buffer(buffer: ArrayBuffer) -> Self {
        utils::set_panic_hook();

        let target_img = web::load_image_from_buffer(&buffer).unwrap();
        let (width, height) = target_img.dimensions();

        let scale = std::cmp::max(width, height) as f64 / 100.0;
        let (scaled_width, scaled_height) = (
            (width as f64 / scale) as u32,
            (height as f64 / scale) as u32,
        );
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
            current_score: i64::from(width * height * 255 * 3),
        }
    }

    pub fn get_image_data(&self) -> Result<JsValue, JsValue> {
        let (width, height) = self.current_img.dimensions();
        let mut data = self.current_img.to_vec();

        let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
        Ok(JsValue::from(data))
    }

    pub fn try_epoch(&mut self, generation_size: usize, num_gens: u32) -> Option<RandomCircle> {
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

                Some(best_shape)
            }
            None => {
                console::log_1(&JsValue::from_str("Discarded circle"));
                None
            }
        }
    }

    pub fn get_target_width(&self) -> u32 {
        self.target_img.width()
    }

    pub fn get_target_height(&self) -> u32 {
        self.target_img.height()
    }
}
