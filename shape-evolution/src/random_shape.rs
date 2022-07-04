use crate::image_diff::image_diff;
use image::GenericImageView;
use image::{Pixel, Rgba};
use rand::Rng;
use serde::{
    de::Deserializer,
    ser::{SerializeTuple, Serializer},
    Deserialize, Serialize,
};
use std::cmp;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq)]
pub struct BoundingBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub trait RandomShape {
    #[must_use]
    fn draw(&self, image: &image::RgbaImage, scale: f64) -> image::RgbaImage;

    // Returns the same output as draw, but cropped to the bounding box returned by
    // get_bounds().
    #[must_use]
    fn draw_subimage(&self, image: &image::RgbaImage, scale: f64) -> image::RgbaImage;

    #[must_use]
    fn mutate(&self) -> Self;

    #[must_use]
    fn get_bounds(&self, scale: f64) -> Option<BoundingBox>;

    // Calculates and returns how close the current image becomes to the target after this shape is
    // drawn. Smaller scores are better.
    #[must_use]
    fn score(
        &self,
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
        scale: f64,
    ) -> i64;
}

// Serializer and deserializer for an Rgba<u8> struct. Used by RandomCircle for its color field.
fn serialize_rgba<S>(color: &image::Rgba<u8>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut tup = ser.serialize_tuple(4)?;
    tup.serialize_element(&color[0])?;
    tup.serialize_element(&color[1])?;
    tup.serialize_element(&color[2])?;
    tup.serialize_element(&color[3])?;
    tup.end()
}
fn deserialize_rgba<'de, D>(de: D) -> Result<image::Rgba<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let tup: (u8, u8, u8, u8) = Deserialize::deserialize(de)?;
    Ok(image::Rgba([tup.0, tup.1, tup.2, tup.3]))
}

// RandomCircle definition for wasm.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RandomCircle {
    pub imgx: u32,
    pub imgy: u32,

    #[wasm_bindgen(skip)]
    pub center: (i32, i32),

    pub radius: i32,

    #[wasm_bindgen(skip)]
    #[serde(
        serialize_with = "serialize_rgba",
        deserialize_with = "deserialize_rgba"
    )]
    pub color: image::Rgba<u8>,
}

// RandomCircle definition not for wasm.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RandomCircle {
    pub imgx: u32,
    pub imgy: u32,
    pub center: (i32, i32),
    pub radius: i32,
    #[serde(
        serialize_with = "serialize_rgba",
        deserialize_with = "deserialize_rgba"
    )]
    pub color: image::Rgba<u8>,
}

// Wasm impl
#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl RandomCircle {
    #[wasm_bindgen(getter)]
    pub fn center(&self) -> js_sys::Int32Array {
        js_sys::Int32Array::from(&[self.center.0, self.center.1][..])
    }
    #[wasm_bindgen(setter)]
    pub fn set_center(&mut self, center: &[i32]) {
        self.center = (center[0], center[1]);
    }
    #[wasm_bindgen(getter)]
    pub fn color(&self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(&[self.color[0], self.color[1], self.color[2], self.color[3]][..])
    }
    #[wasm_bindgen(setter)]
    pub fn set_color(&mut self, color: &[u8]) {
        self.color = image::Rgba([color[0], color[1], color[2], color[3]]);
    }
}

#[must_use]
fn clamp_channel(c: i32) -> u8 {
    cmp::max(0, cmp::min(255, c)) as u8
}

#[must_use]
fn mutate_center(center: (i32, i32), rng: &mut rand::rngs::ThreadRng) -> (i32, i32) {
    let dc1 = rng.gen_range(-5..=5);
    let dc2 = rng.gen_range(-5..=5);
    (center.0 + dc1, center.1 + dc2)
}

#[must_use]
fn mutate_radius(radius: i32, rng: &mut rand::rngs::ThreadRng) -> i32 {
    let drad = rng.gen_range(-20..=2);
    cmp::max(radius + drad, 1)
}

#[must_use]
fn mutate_color(color: image::Rgba<u8>, rng: &mut rand::rngs::ThreadRng) -> image::Rgba<u8> {
    let dr = rng.gen_range(-20..=20);
    let dg = rng.gen_range(-20..=20);
    let db = rng.gen_range(-20..=20);

    let r = clamp_channel(i32::from(color.channels()[0]) + dr);
    let g = clamp_channel(i32::from(color.channels()[1]) + dg);
    let b = clamp_channel(i32::from(color.channels()[2]) + db);

    image::Rgba([r, g, b, 255])
}

impl RandomShape for RandomCircle {
    fn draw(&self, image: &image::RgbaImage, scale: f64) -> image::RgbaImage {
        let center = (
            (self.center.0 as f64 * scale) as i32,
            (self.center.1 as f64 * scale) as i32,
        );
        let radius = (self.radius as f64 * scale) as i32;
        imageproc::drawing::draw_filled_circle(image, center, radius, self.color)
    }

    fn draw_subimage(&self, image: &image::RgbaImage, scale: f64) -> image::RgbaImage {
        let bounds = self.get_bounds(scale).unwrap();
        let image = image
            .view(bounds.x, bounds.y, bounds.width, bounds.height)
            .to_image();
        let center = (
            (self.center.0 as f64 * scale - bounds.x as f64) as i32,
            (self.center.1 as f64 * scale - bounds.y as f64) as i32,
        );
        let radius = (self.radius as f64 * scale) as i32;
        // Pass a reference to image, since the new value of image is no longer a reference.
        imageproc::drawing::draw_filled_circle(&image, center, radius, self.color)
    }

    fn mutate(&self) -> Self {
        let mut rng = rand::thread_rng();

        let center = mutate_center(self.center, &mut rng);
        let color = mutate_color(self.color, &mut rng);
        let radius = mutate_radius(self.radius, &mut rng);
        Self {
            imgx: self.imgx,
            imgy: self.imgy,
            center,
            radius,
            color,
        }
    }

    fn get_bounds(&self, scale: f64) -> Option<BoundingBox> {
        let x = cmp::max(self.center.0 - self.radius - 1, 0);
        let y = cmp::max(self.center.1 - self.radius - 1, 0);
        let x2 = cmp::min(self.center.0 + self.radius + 1, (self.imgx - 1) as i32);
        let y2 = cmp::min(self.center.1 + self.radius + 1, (self.imgy - 1) as i32);

        // Return none if bounds are not contained within image.
        if x >= self.imgx.try_into().unwrap()
            || y >= self.imgy.try_into().unwrap()
            || x2 < 0
            || y2 < 0
        {
            return None;
        }

        Some(BoundingBox {
            x: (x as f64 * scale) as u32,
            y: (y as f64 * scale) as u32,
            width: ((x2 - x + 1) as f64 * scale) as u32,
            height: ((y2 - y + 1) as f64 * scale) as u32,
        })
    }

    fn score(
        &self,
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
        scale: f64,
    ) -> i64 {
        if self.get_bounds(scale) == None {
            return 0; // If the bounds lay outside the image, this shape does not change the image
        }
        self.score_bresenham(target_img, current_img, scale)

        /*
        // Compare the area of the bounding box to the area of the target image - if the bounding
        // box is sufficiently small, use the scoring algorithm for smaller shapes.
        if bounds.width * bounds.height < imgx * imgy / 4 {
            self.score_small(target_img, current_img, scale, prev_score)
        } else {
            self.score_large(target_img, current_img, scale)
        }
        */
    }
}

impl RandomCircle {
    #[must_use]
    pub fn new(imgx: u32, imgy: u32) -> Self {
        let simgx = imgx as i32;
        let simgy = imgy as i32;

        let mut rng = rand::thread_rng();
        let max_radius = cmp::max(simgx, simgy);

        Self {
            imgx,
            imgy,
            center: (rng.gen_range(0..simgx), rng.gen_range(0..simgy)),
            radius: rng.gen_range(1..max_radius),
            color: image::Rgba([
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                255,
            ]),
        }
    }

    // We can use the bounds of the shape to crop the target and current image to a smaller area
    // where all the drawing and scoring can be done. This greatly improves performance on shapes
    // with smaller bounding boxes.
    fn score_small(
        &self,
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
        scale: f64,
        prev_score: i64,
    ) -> i64 {
        let bounds = self.get_bounds(scale).unwrap();

        let cropped_target = target_img
            .view(bounds.x, bounds.y, bounds.width, bounds.height)
            .to_image();
        let cropped_current = current_img
            .view(bounds.x, bounds.y, bounds.width, bounds.height)
            .to_image();

        let new_img = self.draw_subimage(current_img, scale);

        let prev_cropped_score = image_diff(&cropped_target, &cropped_current);
        let new_cropped_score = image_diff(&cropped_target, &new_img);

        prev_score + new_cropped_score - prev_cropped_score
    }

    // On shapes with large bounding boxes, it's best to avoid cropping and simply draw and score
    // on the original target image.
    fn score_large(
        &self,
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
        scale: f64,
    ) -> i64 {
        let new_img = self.draw(current_img, scale);
        image_diff(target_img, &new_img)
    }

    fn pixel_diff(p1: &[u8], p2: &[u8]) -> i64 {
        (i64::from(p1[0]) - i64::from(p2[0])).abs()
            + (i64::from(p1[1]) - i64::from(p2[1])).abs()
            + (i64::from(p1[2]) - i64::from(p2[2])).abs()
    }

    // Calculates the score difference after drawing a horizontal line across current_img.
    fn score_diff_for_line_horizontal(
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
        x0: i32,
        x1: i32,
        y: i32,
        color: Rgba<u8>,
    ) -> i64 {
        let mut diff = 0i64;

        let (width, height) = target_img.dimensions();

        if 0 <= y && y < height.try_into().unwrap() {
            // Clamp the lower and upper bounds to fit in the image
            let x0: u32 = cmp::max(x0, 0).try_into().unwrap();
            let x1: u32 = cmp::min(x1, (width-1).try_into().unwrap()).try_into().unwrap();

            // Convert y to a u32
            let y: u32 = y.try_into().unwrap();

            // Loop over every pixel along the line and calculate the potential difference in score
            // of applying the color to that pixel
            for x in x0..=x1 {
                // x and y have already been bounds-checked, so we can index directly into
                // the underlying pixel buffer without worry.
                let index: usize = (4 * x + width * y).try_into().unwrap();
                let target_pixel = &target_img.as_raw()[index..index+4];
                let current_pixel = &current_img.as_raw()[index..index+4];

                diff += Self::pixel_diff(target_pixel, color.channels())
                    - Self::pixel_diff(target_pixel, current_pixel);
            }
        }

        //let line_iterator = BresenhamLineIter::new((x0 as f32, y as f32), (x1 as f32, y as f32));

        diff
    }

    fn score_bresenham(
        &self,
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
        scale: f64,
    ) -> i64 {
        let mut diff = 0i64;

        let mut error = (-self.radius as f64 * scale) as i32;
        let mut x = (self.radius as f64 * scale) as i32;
        let mut y = 0;

        while x >= y {
            let last_y = y;
            error += y;
            y += 1;
            error += y;

            diff += Self::score_plot4points(
                target_img,
                current_img,
                (self.center.0 as f64 * scale) as i32,
                (self.center.1 as f64 * scale) as i32,
                x,
                last_y,
                self.color,
            );

            if error >= 0 {
                if x != last_y {
                    diff += Self::score_plot4points(
                        target_img,
                        current_img,
                        (self.center.0 as f64 * scale) as i32,
                        (self.center.1 as f64 * scale) as i32,
                        last_y,
                        x,
                        self.color,
                    );
                }

                error -= x;
                x -= 1;
                error -= x;
            }
        }

        diff
    }

    fn score_plot4points(
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
        cx: i32,
        cy: i32,
        x: i32,
        y: i32,
        color: image::Rgba<u8>,
    ) -> i64 {
        let mut diff = 0i64;
        diff += Self::score_diff_for_line_horizontal(
            target_img,
            current_img,
            cx - x,
            cx + x,
            cy + y,
            color,
        );
        if y != 0 {
            diff += Self::score_diff_for_line_horizontal(
                target_img,
                current_img,
                cx - x,
                cx + x,
                cy - y,
                color,
            );
        }
        diff
    }
}

#[cfg(test)]
mod tests {
    use crate::image_diff::image_diff;
    use crate::random_shape::{BoundingBox, RandomCircle, RandomShape};
    use image::RgbaImage;
    use std::iter;

    fn assert_scoring_equal(
        shape: &RandomCircle,
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
        prev_score: i64,
        scale: f64,
    ) {
        match shape.get_bounds(scale) {
            Some(_b) => {}
            None => return,
        };
        let score_small = shape.score_small(target_img, current_img, scale, prev_score);
        let score_large = shape.score_large(target_img, current_img, scale);
        let score_bresenham = shape.score_bresenham(target_img, current_img, scale);
        assert_eq!(score_small, score_large);

        // The Bresenham algorithm isn't exactly the same as the others - we're happy with it being
        // within a 10% margin.
        assert!(
            (score_small - score_bresenham).abs() as f64 / (score_small as f64) < 0.10,
            "{} !~= {}",
            score_small,
            score_bresenham
        );
    }

    #[test]
    fn test_scoring_algs_equal() {
        let (imgx, imgy) = (50, 75);

        // Create 1000 random shapes for testing
        let shapes = iter::repeat_with(|| RandomCircle::new(imgx, imgy)).take(1000);

        let target_img = RgbaImage::new(imgx, imgy);
        let current_img = RgbaImage::new(imgx, imgy);
        let prev_score = image_diff(&target_img, &current_img);

        for shape in shapes {
            assert_scoring_equal(&shape, &target_img, &current_img, prev_score, 1.0);
        }
    }

    #[test]
    fn test_scoring_algs_equal_single_point() {
        let (imgx, imgy) = (5, 7);

        // Create 1000 random shapes for testing
        let shapes = iter::repeat_with(|| RandomCircle {
            center: (2, 3),
            radius: 0,
            color: image::Rgba([10, 10, 10, 255]),
            ..RandomCircle::new(imgx, imgy)
        })
        .take(1000);

        let target_img = RgbaImage::new(imgx, imgy);
        let current_img = RgbaImage::new(imgx, imgy);
        let prev_score = image_diff(&target_img, &current_img);

        for shape in shapes {
            assert_scoring_equal(&shape, &target_img, &current_img, prev_score, 1.0);
        }
    }

    #[test]
    fn test_scoring_algs_equal_scale_5() {
        let (imgx, imgy) = (10, 15);
        let scale = 5;

        // Create 1000 random shapes for testing
        let shapes = iter::repeat_with(|| RandomCircle::new(imgx, imgy)).take(1000);

        let target_img = RgbaImage::new(imgx * scale, imgy * scale);
        let current_img = RgbaImage::new(imgx * scale, imgy * scale);
        let prev_score = image_diff(&target_img, &current_img);

        assert_eq!(prev_score, 0);

        for shape in shapes {
            assert_scoring_equal(&shape, &target_img, &current_img, prev_score, scale as f64);
        }
    }

    #[test]
    fn test_score_algs_equal_shape_outside_canvas() {
        let (imgx, imgy) = (50, 75);

        let target_img = RgbaImage::new(imgx, imgy);
        let current_img = RgbaImage::new(imgx, imgy);
        let prev_score = image_diff(&target_img, &current_img);

        let shape = RandomCircle {
            imgx,
            imgy,
            center: (-100, -100),
            radius: 1,
            color: image::Rgba([255, 255, 255, 255]),
        };
        assert_scoring_equal(&shape, &target_img, &current_img, prev_score, 1.0);
    }

    #[test]
    fn test_shape_fills_canvas_bounds() {
        let (imgx, imgy) = (50, 75);

        let shape = RandomCircle {
            imgx,
            imgy,
            center: (100, 100),
            radius: 1000,
            color: image::Rgba([255, 255, 255, 255]),
        };

        let expected_bounds = BoundingBox {
            x: 0,
            y: 0,
            width: imgx,
            height: imgy,
        };

        assert_eq!(shape.get_bounds(1.0), Some(expected_bounds));
    }

    #[test]
    fn test_score_small_shape_fills_canvas() {
        let (imgx, imgy) = (50, 75);

        let target_img = RgbaImage::new(imgx, imgy);
        let current_img = RgbaImage::new(imgx, imgy);
        let prev_score = image_diff(&target_img, &current_img);

        let shape = RandomCircle {
            imgx,
            imgy,
            center: (100, 100),
            radius: 1000,
            color: image::Rgba([255, 255, 255, 255]),
        };
        assert_eq!(
            shape.score_small(&target_img, &current_img, 1.0, prev_score),
            (imgx * imgy * 255 * 3) as i64
        );
    }

    #[test]
    fn test_score_large_shape_fills_canvas() {
        let (imgx, imgy) = (50, 75);

        let target_img = RgbaImage::new(imgx, imgy);
        let current_img = RgbaImage::new(imgx, imgy);
        let prev_score = image_diff(&target_img, &current_img);

        assert_eq!(prev_score, 0);

        let shape = RandomCircle {
            imgx,
            imgy,
            center: (100, 100),
            radius: 1000,
            color: image::Rgba([255, 255, 255, 255]),
        };
        assert_eq!(
            shape.score_large(&target_img, &current_img, 1.0),
            (imgx * imgy * 255 * 3) as i64
        );
    }

    #[test]
    fn test_score_algs_equal_shape_fills_canvas() {
        let (imgx, imgy) = (50, 75);

        let target_img = RgbaImage::new(imgx, imgy);
        let current_img = RgbaImage::new(imgx, imgy);
        let prev_score = image_diff(&target_img, &current_img);

        let shape = RandomCircle {
            imgx,
            imgy,
            center: (100, 100),
            radius: 1000,
            color: image::Rgba([255, 255, 255, 255]),
        };
        assert_scoring_equal(&shape, &target_img, &current_img, prev_score, 1.0);
    }
}
