use crate::random_shape;
use image::Pixel;
use rand;
use rand::Rng;
use std::cmp;

pub trait Mutate {
    fn mutate(&self, rng: &mut rand::rngs::ThreadRng, factor: f64) -> Self;
}

pub trait BoundedMutate {
    fn bounded_mutate(&self, rng: &mut rand::rngs::ThreadRng, max_change: i32) -> Self;
}

impl Mutate for random_shape::RandomCircle {
    fn mutate(&self, rng: &mut rand::rngs::ThreadRng, factor: f64) -> Self {
        Self {
            imgx: self.imgx,
            imgy: self.imgy,
            center: self
                .center
                .bounded_mutate(rng, (f64::from(2 * self.radius) * factor) as i32),
            radius: self
                .radius
                .bounded_mutate(rng, (f64::from(self.radius) / 2.0 * factor) as i32),
            color: self.color.bounded_mutate(rng, (20.0 * factor) as i32),
        }
    }
}

impl BoundedMutate for i32 {
    fn bounded_mutate(&self, rng: &mut rand::rngs::ThreadRng, max_change: i32) -> Self {
        let delta = rng.gen_range(-max_change..=max_change);
        *self + delta
    }
}

impl BoundedMutate for (i32, i32) {
    fn bounded_mutate(&self, rng: &mut rand::rngs::ThreadRng, max_change: i32) -> Self {
        let mut get_delta = || rng.gen_range(-max_change..=max_change);
        (self.0 + get_delta(), self.1 + get_delta())
    }
}

impl BoundedMutate for image::Rgba<u8> {
    fn bounded_mutate(&self, rng: &mut rand::rngs::ThreadRng, max_change: i32) -> Self {
        let mut get_delta = || rng.gen_range(-max_change..=max_change);
        let clamp_channel = |c: i32| u8::try_from(cmp::max(0, cmp::min(255, c))).unwrap();

        let r = clamp_channel(i32::from(self.channels()[0]) + get_delta());
        let g = clamp_channel(i32::from(self.channels()[1]) + get_delta());
        let b = clamp_channel(i32::from(self.channels()[2]) + get_delta());

        image::Rgba([r, g, b, 255])
    }
}
