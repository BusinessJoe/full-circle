use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wallpaper_evolution::next_generation;
use wallpaper_evolution::random_shape::{RandomShape, RandomCircle};
use wallpaper_evolution::random_shape;
use image::RgbImage;

fn generations(n: u64) {
    let base_img = image::open("test.jpg").unwrap().to_rgb8();
    let base_img = image::imageops::resize(&base_img, 100, 100, image::imageops::FilterType::Nearest);

    let (imgx, imgy) = base_img.dimensions();
    let mut imgbuf = RgbImage::new(imgx, imgy);

    let mut shapes: Vec<RandomCircle> = (0..100).map(|_| {
        random_shape::RandomCircle::new(imgx, imgy)
    }).collect();

    for i in 0..n {

        shapes = next_generation(&base_img, &imgbuf, &shapes);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("generations 10", |b| b.iter(|| generations(black_box(10))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
