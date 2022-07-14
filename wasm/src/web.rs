use anyhow::{anyhow, Result};
use exif;
use js_sys::ArrayBuffer;
use js_sys::Uint8Array;
use std::io::{BufRead, Cursor, Seek};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Request, RequestInit, RequestMode, Response, WorkerGlobalScope};

pub async fn load_image(url: &str) -> Result<image::RgbaImage, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    println!("{}", url);
    let request = Request::new_with_str_and_init(url, &opts).unwrap();

    request.headers().set("Accept", "*/*")?;

    let global_scope = js_sys::global();
    console::log_1(&global_scope.to_string());
    let worker_scope = global_scope
        .dyn_into::<WorkerGlobalScope>()
        .expect("Expected global scope to be WorkerGlobalScope");

    let resp_value = JsFuture::from(worker_scope.fetch_with_request(&request))
        .await
        .unwrap();

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let bytes_value = JsFuture::from(resp.array_buffer()?).await?;
    let bytes: ArrayBuffer = bytes_value.dyn_into().unwrap();

    let array = Uint8Array::new(&bytes);
    let bytes: Vec<u8> = array.to_vec();

    match image::load_from_memory(&bytes) {
        Ok(dyn_image) => Ok(dyn_image.into_rgba8()),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

pub fn load_image_from_buffer(buffer: &ArrayBuffer) -> Result<image::RgbaImage, JsValue> {
    let array = Uint8Array::new(buffer);
    let bytes: Vec<u8> = array.to_vec();

    let mut img = match image::load_from_memory(&bytes) {
        Ok(img) => img,
        Err(e) => return Err(JsValue::from_str(&e.to_string())),
    };

    let mut cursor = Cursor::new(bytes);
    let orientation = read_exif_orientation(&mut cursor);
    if let Ok(orientation) = orientation {
        img = correct_orientation(img, orientation);
    }

    Ok(img.into_rgba8())
}

fn read_exif_orientation<R>(reader: &mut R) -> Result<u32>
where
    R: BufRead + Seek,
{
    let exif_data = exif::Reader::new().read_from_container(reader)?;
    let orientation = exif_data
        .get_field(exif::Tag::Orientation, exif::In::PRIMARY)
        .ok_or_else(|| anyhow!("Orientation tag is missing"))?;
    match orientation.value.get_uint(0) {
        Some(v @ 1..=8) => Ok(v),
        _ => Err(anyhow!("Orientation value is broken")),
    }
}

fn correct_orientation(img: image::DynamicImage, orientation: u32) -> image::DynamicImage {
    match orientation {
        1 => img,
        2 => img.fliph(),
        3 => img.rotate180(),
        4 => img.rotate180().fliph(),
        5 => img.rotate90().fliph(),
        6 => img.rotate90(),
        7 => img.rotate270().fliph(),
        8 => img.rotate270(),
        _ => panic!("Invalid exif orientation value"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImage;

    fn testing_image() -> image::DynamicImage {
        let mut img: image::GrayImage = image::ImageBuffer::new(2, 2);
        img.put_pixel(0, 0, [0].into());
        img.put_pixel(1, 0, [1].into());
        img.put_pixel(0, 1, [2].into());
        img.put_pixel(1, 1, [3].into());
        image::DynamicImage::ImageLuma8(img)
    }

    fn assert_image(img: image::DynamicImage, vals: [u8; 4]) {
        let img = img.as_luma8().expect("was not provided a luma8 image");
        assert_eq!(img.width(), 2);
        assert_eq!(img.height(), 2);
        let pixels: Vec<u8> = img.pixels().map(|luma| luma.0[0]).collect();
        let vals = vals.to_vec();
        assert_eq!(pixels, vals);
    }

    fn test_orientation(n: u32, expected: [u8; 4]) {
        let img = correct_orientation(testing_image(), n);
        assert_image(img, expected);
    }

    #[test]
    fn test_orientation_1() {
        test_orientation(1, [0, 1, 2, 3]);
    }
    #[test]
    fn test_orientation_2() {
        test_orientation(2, [1, 0, 3, 2]);
    }
    #[test]
    fn test_orientation_3() {
        test_orientation(3, [3, 2, 1, 0]);
    }
    #[test]
    fn test_orientation_4() {
        test_orientation(4, [2, 3, 0, 1]);
    }
    #[test]
    fn test_orientation_5() {
        test_orientation(5, [0, 2, 1, 3]);
    }
    #[test]
    fn test_orientation_6() {
        test_orientation(6, [2, 0, 3, 1]);
    }
    #[test]
    fn test_orientation_7() {
        test_orientation(7, [3, 1, 2, 0]);
    }
    #[test]
    fn test_orientation_8() {
        test_orientation(8, [1, 3, 0, 2]);
    }
}
