use crate::image_diff::image_diff;
use crate::mutate::Mutate;
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

pub trait RandomShape: Mutate {
    #[must_use]
    fn draw(&self, image: &image::RgbaImage) -> image::RgbaImage;

    // Returns the same output as draw, but cropped to the bounding box returned by
    // get_bounds().
    #[must_use]
    fn draw_subimage(&self, image: &image::RgbaImage) -> image::RgbaImage;

    #[must_use]
    fn get_bounds(&self) -> Option<BoundingBox>;

    // Calculates and returns how close the current image becomes to the target after this shape is
    // drawn. Smaller scores are better.
    #[must_use]
    fn score(
        &self,
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
    ) -> i128;

    fn scale_up(&self, scale: f64) -> Self;
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

impl RandomShape for RandomCircle {
    fn draw(&self, image: &image::RgbaImage) -> image::RgbaImage {
        imageproc::drawing::draw_filled_circle(image, self.center, self.radius, self.color)
    }

    fn draw_subimage(&self, image: &image::RgbaImage) -> image::RgbaImage {
        let bounds = self.get_bounds().unwrap();
        let image = image
            .view(bounds.x, bounds.y, bounds.width, bounds.height)
            .to_image();
        let center = (
            (self.center.0 - i32::try_from(bounds.x).unwrap()),
            (self.center.1 - i32::try_from(bounds.y).unwrap()),
        );
        // Pass a reference to image, since the new value of image is no longer a reference.
        imageproc::drawing::draw_filled_circle(&image, center, self.radius, self.color)
    }

    fn get_bounds(&self) -> Option<BoundingBox> {
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
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
            width: (x2 - x + 1).try_into().unwrap(),
            height: (y2 - y + 1).try_into().unwrap(),
        })
    }

    fn score(
        &self,
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
    ) -> i128 {
        if self.get_bounds() == None {
            return 0; // If the bounds lay outside the image, this shape does not change the image
        }
        self.score_bresenham(target_img, current_img)

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

    fn scale_up(&self, scale: f64) -> Self {
        Self {
            imgx: (f64::from(self.imgx) * scale).round() as u32,
            imgy: (f64::from(self.imgy) * scale).round() as u32,
            center: (
                (f64::from(self.center.0) * scale).round() as i32, 
                (f64::from(self.center.1) * scale).round() as i32,
            ),
            radius: (f64::from(self.radius) * scale).round() as i32,
            color: self.color,
        }
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
    #[allow(unused)]
    fn score_small(
        &self,
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
        prev_score: u128,
    ) -> u128 {
        let bounds = self.get_bounds().unwrap();

        let cropped_target = target_img
            .view(bounds.x, bounds.y, bounds.width, bounds.height)
            .to_image();
        let cropped_current = current_img
            .view(bounds.x, bounds.y, bounds.width, bounds.height)
            .to_image();

        let new_img = self.draw_subimage(current_img);

        let prev_cropped_score = image_diff(&cropped_target, &cropped_current);
        let new_cropped_score = image_diff(&cropped_target, &new_img);

        prev_score + new_cropped_score - prev_cropped_score
    }

    // On shapes with large bounding boxes, it's best to avoid cropping and simply draw and score
    // on the original target image.
    #[allow(unused)]
    fn score_large(
        &self,
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
    ) -> u128 {
        let new_img = self.draw(current_img);
        image_diff(target_img, &new_img)
    }

    fn pixel_diff(p1: &[u8], p2: &[u8]) -> i128 {
        (p1[0].abs_diff(p2[0])) as i128
            + (p1[1].abs_diff(p2[1])) as i128
            + (p1[2].abs_diff(p2[2])) as i128
    }

    // Calculates the score difference after drawing a horizontal line across current_img.
    fn score_diff_for_line_horizontal(
        target_img: &image::RgbaImage,
        current_img: &image::RgbaImage,
        x0: i32,
        x1: i32,
        y: i32,
        color: Rgba<u8>,
    ) -> i128 {
        let mut diff: i128 = 0;

        let (width, height): (u32, u32) = target_img.dimensions();

        // Check some preconditions
        if !(x0 < width.try_into().unwrap() && x1 >= 0 && x0 <= x1) {
            return 0;
        }

        if 0 <= y && u32::try_from(y).unwrap() < height {
            // Clamp the lower and upper bounds to fit in the image
            let x0: u32 = cmp::max(x0, 0).try_into().unwrap();
            // here x1 becomes a exclusive upper bound
            let temp = cmp::min(x1 + 1, width.try_into().unwrap());
            let x1: u32 = temp.try_into().unwrap();

            // Convert y to a u32
            let y: u32 = y.try_into().unwrap();

            // Loop over every pixel along the line and calculate the potential difference in score
            // of applying the color to that pixel
            for x in x0..x1 {
                // x and y have already been bounds-checked, so we can index directly into
                // the underlying pixel buffer without worry.
                let index: usize = 4 * usize::try_from(x + width * y).unwrap();
                let target_pixel = &target_img.as_raw()[index..index + 4];
                let current_pixel = &current_img.as_raw()[index..index + 4];

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
    ) -> i128 {
        let mut diff: i128 = 0;

        let mut error = -self.radius;
        let mut x = self.radius;
        let mut y = 0;

        while x >= y {
            let last_y = y;
            error += y;
            y += 1;
            error += y;

            diff += Self::score_plot4points(
                target_img,
                current_img,
                self.center.0,
                self.center.1,
                x,
                last_y,
                self.color,
            );

            if error >= 0 {
                if x != last_y {
                    diff += Self::score_plot4points(
                        target_img,
                        current_img,
                        self.center.0,
                        self.center.1,
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
    ) -> i128 {
        let mut diff: i128 = 0;
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
        prev_score: u128,
    ) {
        match shape.get_bounds() {
            Some(_b) => {}
            None => return,
        };
        let score_small = shape.score_small(target_img, current_img, prev_score);
        let score_large = shape.score_large(target_img, current_img);
        let score_bresenham = shape.score_bresenham(target_img, current_img);
        assert_eq!(score_small, score_large);

        // The Bresenham algorithm isn't exactly the same as the others - we're happy with it being
        // within a 10% margin.
        assert!(
            (i128::try_from(score_small).unwrap() - score_bresenham).abs() as f64 / (score_small as f64) < 0.10,
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
            assert_scoring_equal(&shape, &target_img, &current_img, prev_score);
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
            assert_scoring_equal(&shape, &target_img, &current_img, prev_score);
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
        assert_scoring_equal(&shape, &target_img, &current_img, prev_score);
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

        assert_eq!(shape.get_bounds(), Some(expected_bounds));
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
            shape.score_small(&target_img, &current_img, prev_score),
            (imgx * imgy * 255 * 3) as u128
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
            shape.score_large(&target_img, &current_img),
            (imgx * imgy * 255 * 3) as u128
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
        assert_scoring_equal(&shape, &target_img, &current_img, prev_score);
    }
}
