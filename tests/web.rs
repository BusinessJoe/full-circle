extern crate wallpaper_evolution;
use wallpaper_evolution::web::load_image;

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test::wasm_bindgen_test]
pub async fn load_png() -> () {
    let url = "https://upload.wikimedia.org/wikipedia/commons/thumb/4/47/PNG_transparency_demonstration_1.png/420px-PNG_transparency_demonstration_1.png";
    let image = load_image(url).await.unwrap();
    assert_eq!(image.width(), 420);
    assert_eq!(image.height(), 315);
}
