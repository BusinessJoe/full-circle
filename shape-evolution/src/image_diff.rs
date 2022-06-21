// TODO: image_diff can use bounds to avoid iterating over large parts of the image.
// score_small might be able to use a subimage for the target image to avoid a copy
#[must_use]
pub fn image_diff(a: &image::RgbaImage, b: &image::RgbaImage) -> i64 {
    assert!(
        a.dimensions() == b.dimensions(),
        "Images have different sizes, {:?} != {:?}",
        a.dimensions(),
        b.dimensions()
    );

    sum_chunked(a, b)
}

// Ignores alpha channel
fn sum_chunked(samples_a: &[u8], samples_b: &[u8]) -> i64 {
    samples_a
        .chunks_exact(4)
        .zip(samples_b.chunks_exact(4))
        .fold(0, |sum, (p_a, p_b)| {
            sum + (i64::from(p_a[0]) - i64::from(p_b[0])).abs()
                + (i64::from(p_a[1]) - i64::from(p_b[1])).abs()
                + (i64::from(p_a[2]) - i64::from(p_b[2])).abs()
        })
}

#[cfg(test)]
mod tests {
    use crate::image_diff::image_diff;
    use image::RgbaImage;
    #[test]
    fn test_diff_black_white() {
        let (imgx, imgy) = (10, 20);
        let black = RgbaImage::from_fn(imgx, imgy, |_x, _y| image::Rgba([0, 0, 0, 255]));
        let white = RgbaImage::from_fn(imgx, imgy, |_x, _y| image::Rgba([255, 255, 255, 255]));

        assert_eq!(image_diff(&black, &white), (imgx * imgy * 255 * 3) as i64);
    }
}
