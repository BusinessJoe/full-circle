use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use image::RgbaImage;
use std::iter;
use std::time::Duration;
use shape_evolution::random_shape::RandomCircle;
use shape_evolution::{evolve, evolve::sort_generation};

// Generate a population of 100 random circles with given radius
fn randomize_generation(radius: i32, imgx: u32, imgy: u32) -> Vec<RandomCircle> {
    iter::repeat_with(|| RandomCircle::new(imgx, imgy))
        .map(|shape| RandomCircle { radius, ..shape })
        .take(100)
        .collect()
}

fn bench_sort_generation(
    mut generation: Vec<RandomCircle>,
    target_img: &RgbaImage,
    current_img: &RgbaImage,
) {
    generation = sort_generation(target_img, current_img, generation);
}

fn criterion_benchmark(c: &mut Criterion) {
    let target_img = image::open("../rap.jpeg").unwrap().to_rgba8();
    let (imgx, imgy) = target_img.dimensions();
    let current_img = RgbaImage::new(imgx, imgy);

    let mut group = c.benchmark_group("evolution");
    group.sample_size(50);
    for i in [300, 200, 100, 75, 50, 25, 5].iter() {
        group.bench_with_input(BenchmarkId::new("sort radius", i), i, |b, i| {
            let generation = randomize_generation(*i, imgx, imgy);
            b.iter(|| bench_sort_generation(generation.clone(), &target_img, &current_img))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
