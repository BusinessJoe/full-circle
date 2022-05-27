use rand::Rng;
use std::cmp;
use image::Pixel;

pub trait RandomShape {
    fn draw(&self, image: &image::RgbImage) -> image::RgbImage;
    fn mutate(&self) -> Self;
}

#[derive(Clone)]
pub struct RandomCircle {
    imgx: u32,
    imgy: u32,
    center: (i32, i32),
    radius: i32,
    color: image::Rgb<u8>,
}

fn clamp_channel(c: i32) -> u8 {
    cmp::max(0, cmp::min(255, c)) as u8
}

fn mutate_radius(radius: &i32, rng: &mut rand::rngs::ThreadRng) -> i32 {
    let drad = rng.gen_range(-20..=20);
    radius + drad
}

fn mutate_color(color: &image::Rgb<u8>, rng: &mut rand::rngs::ThreadRng) -> image::Rgb<u8> {
    let dr = rng.gen_range(-20..=20);
    let dg = rng.gen_range(-20..=20);
    let db = rng.gen_range(-20..=20);

    let r = clamp_channel(color.channels()[0] as i32 + dr);
    let g = clamp_channel(color.channels()[1] as i32 + dg);
    let b = clamp_channel(color.channels()[2] as i32 + db);

    image::Rgb([r, g, b])
}

impl RandomShape for RandomCircle {
    fn draw(&self, image: &image::RgbImage) -> image::RgbImage {
        imageproc::drawing::draw_filled_circle(image, self.center, self.radius, self.color)
    }

    fn mutate(&self) -> Self {
        let mut rng = rand::thread_rng();

        let color = mutate_color(&self.color, &mut rng);
        let radius = mutate_radius(&self.radius, &mut rng);
        RandomCircle {
            imgx: self.imgx,
            imgy: self.imgy,
            center: self.center,
            radius,
            color,
        }
    }
}

impl RandomCircle {
    pub fn new(imgx: u32, imgy: u32) -> Self {
        let simgx = imgx as i32;
        let simgy = imgy as i32;

        let mut rng = rand::thread_rng();
        let max_radius = cmp::max(simgx, simgy);

        Self { 
            imgx, imgy,
            center: (rng.gen_range(0..simgx), rng.gen_range(0..simgy)),
            radius: rng.gen_range(1..max_radius),
            color: image::Rgb([
                rng.gen_range(0..=255), 
                rng.gen_range(0..=255), 
                rng.gen_range(0..=255),
            ]),
        }
    }
}
