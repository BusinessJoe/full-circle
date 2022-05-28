pub fn image_diff(a: &image::RgbImage, b: &image::RgbImage) -> i64 {
    assert!(
        a.dimensions() == b.dimensions(),
        "Images have different sizes"
    );

    let sums = sums_chunked(a, b);
    sums.0 + sums.1 + sums.2
}

fn sums_chunked(samples_a: &[u8], samples_b: &[u8]) -> (i64, i64, i64) {
    samples_a
        .chunks_exact(3)
        .zip(samples_b.chunks_exact(3))
        .fold((0, 0, 0), |(r, g, b), (p_a, p_b)| {
            (
                r + (p_a[0] as i64 - p_b[0] as i64).abs(),
                g + (p_a[1] as i64 - p_b[1] as i64).abs(),
                b + (p_a[2] as i64 - p_b[2] as i64).abs(),
            )
        })
}
