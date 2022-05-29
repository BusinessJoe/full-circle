use image::Pixel;
use image::GenericImageView;
use rand::Rng;
use std::cmp;
use crate::image_diff::image_diff;

#[derive(Debug)]
pub struct BoundingBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub trait RandomShape {
    #[must_use]
    fn draw(&self, image: &image::RgbImage, scale: f32) -> image::RgbImage;

    // Returns the same output as draw, but cropped to the bounding box returned by
    // get_bounds().
    #[must_use]
    fn draw_subimage(&self, image: &image::RgbImage, scale: f32) -> image::RgbImage;

    #[must_use]
    fn mutate(&self) -> Self;

    #[must_use]
    fn get_bounds(&self) -> Option<BoundingBox>;

    #[must_use]
    fn score(&self, target_img: &image::RgbImage, current_img: &image::RgbImage) -> i64;
}

#[derive(Clone, Debug)]
pub struct RandomCircle {
    pub imgx: u32,
    pub imgy: u32,
    pub center: (i32, i32),
    pub radius: i32,
    pub color: image::Rgb<u8>,
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
    let drad = rng.gen_range(-20..=20);
    cmp::max(radius + drad, 1)
}

#[must_use]
fn mutate_color(color: image::Rgb<u8>, rng: &mut rand::rngs::ThreadRng) -> image::Rgb<u8> {
    let dr = rng.gen_range(-20..=20);
    let dg = rng.gen_range(-20..=20);
    let db = rng.gen_range(-20..=20);

    let r = clamp_channel(i32::from(color.channels()[0]) + dr);
    let g = clamp_channel(i32::from(color.channels()[1]) + dg);
    let b = clamp_channel(i32::from(color.channels()[2]) + db);

    image::Rgb([r, g, b])
}

impl RandomShape for RandomCircle {
    fn draw(&self, image: &image::RgbImage, scale: f32) -> image::RgbImage {
        let center = (
            (self.center.0 as f32 * scale) as i32,
            (self.center.1 as f32 * scale) as i32,
        );
        let radius = (self.radius as f32 * scale) as i32;
        imageproc::drawing::draw_filled_circle(image, center, radius, self.color)
    }

    fn draw_subimage(&self, image: &image::RgbImage, scale: f32) -> image::RgbImage {
        let bounds = self.get_bounds().unwrap();
        let image = image.view(bounds.x, bounds.y, bounds.width, bounds.height).to_image();
        let center = (
            ((self.center.0 - bounds.x as i32) as f32 * scale) as i32,
            ((self.center.1 - bounds.y as i32) as f32 * scale) as i32,
        );
        let radius = (self.radius as f32 * scale) as i32;
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

    fn get_bounds(&self) -> Option<BoundingBox> {
        let x = cmp::max(self.center.0 - self.radius, 0);
        let y = cmp::max(self.center.1 - self.radius, 0);
        let x2 = cmp::min(self.center.0 + self.radius, (self.imgx-1) as i32);
        let y2 = cmp::min(self.center.1 + self.radius, (self.imgy-1) as i32);

        // Return none if bounds are not contained within image.
        if x >= self.imgx.try_into().unwrap() || y >= self.imgy.try_into().unwrap() || x2 < 0 || y2 < 0 {
            return None;
        }

        Some(BoundingBox {
            x: x as u32,
            y: y as u32,
            width: (x2 - x).abs() as u32, //TODO: determine if and why this abs() is required
            height: (y2 - y).abs() as u32,
        })
    }

    fn score(&self, target_img: &image::RgbImage, current_img: &image::RgbImage) -> i64 {
        let (imgx, imgy) = target_img.dimensions();
        let bounds = match self.get_bounds() {
            Some(b) => b,
            None => return 0, // If the bounds lay outside the image, this shape does not change the image
        };

        if bounds.width * bounds.height < imgx * imgy / 4 {
            self.score_small(target_img, current_img)
        } else {
            self.score_large(target_img, current_img)
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
            color: image::Rgb([
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
            ]),
        }
    }

    // We can use the bounds of the shape to crop the target and current image to a smaller area
    // where all the drawing and scoring can be done. This greatly improves performance on shapes
    // with smaller bounding boxes.
    fn score_small(&self, target_img: &image::RgbImage, current_img: &image::RgbImage) -> i64 {
        let bounds = self.get_bounds().unwrap();

        let cropped_target = target_img.view(bounds.x, bounds.y, bounds.width, bounds.height).to_image();
        let cropped_current = current_img.view(bounds.x, bounds.y, bounds.width, bounds.height).to_image();

        let new_img = self.draw_subimage(current_img, 1.0);

        let prev_score = image_diff(&cropped_target, &cropped_current);
        let new_score = image_diff(&cropped_target, &new_img);
        new_score - prev_score
    }

    // On shapes with large bounding boxes, it's best to avoid cropping and simply draw and score 
    // on the original target image.
    fn score_large(&self, target_img: &image::RgbImage, current_img: &image::RgbImage) -> i64 {
        let new_img = self.draw(current_img, 1.0);
        image_diff(target_img, &new_img)
    }

}
