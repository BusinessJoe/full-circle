#[must_use]
pub fn image_diff(a: &image::RgbImage, b: &image::RgbImage) -> i64 {
    debug_assert!(
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
                r + (i64::from(p_a[0]) - i64::from(p_b[0])).abs(),
                g + (i64::from(p_a[1]) - i64::from(p_b[1])).abs(),
                b + (i64::from(p_a[2]) - i64::from(p_b[2])).abs(),
            )
        })
}
