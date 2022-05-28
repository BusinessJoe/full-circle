use std::path::Path;
use std::vec::Vec;

use clap::Parser;
use image::RgbImage;

use wallpaper_evolution::random_shape;
use wallpaper_evolution::random_shape::{RandomCircle, RandomShape};
use wallpaper_evolution::{image_diff, next_generation, sort_generation};

fn evolve(input_path: &str, num_epochs: u32, num_gens: u32, output_folder: &str, scale: f32) {
    let target_img = image::open(input_path).unwrap().to_rgb8();

    let (imgx, imgy) = target_img.dimensions();

    // Unscaled image used for output
    let mut outbuf = RgbImage::new(imgx, imgy);

    // Create a scaled down target image for faster drawing and scoring
    let scaled_target_img = image::imageops::resize(
        &target_img,
        (imgx as f32 / scale) as u32,
        (imgy as f32 / scale) as u32,
        image::imageops::FilterType::Nearest,
    );

    let (imgx, imgy) = scaled_target_img.dimensions();
    let mut imgbuf = RgbImage::new(imgx, imgy);

    for i in 1..=num_epochs {
        let mut shapes: Vec<RandomCircle> = (0..100)
            .map(|_| random_shape::RandomCircle::new(imgx, imgy))
            .collect();

        for i in 0..num_gens {
            shapes = next_generation(&scaled_target_img, &imgbuf, &shapes);
            //println!("Done generation {} of {}", i, NUM_GENS);
        }
        sort_generation(&scaled_target_img, &imgbuf, &mut shapes);
        let minshape = shapes
            .iter()
            .min_by_key(|s| image_diff::image_diff(&scaled_target_img, &s.draw(&imgbuf, 1.0)))
            .unwrap();
        imgbuf = minshape.draw(&imgbuf, 1.0);

        // Calculate the score for the current image
        let score = image_diff::image_diff(&scaled_target_img, &imgbuf);

        // Draw the unscaled shape to the output buffer and save it as a file periodically
        outbuf = minshape.draw(&outbuf, scale);

        if i % 10 == 0 {
            outbuf
                .save(Path::new(output_folder).join(format!("out-{}-{}.jpg", i, score)))
                .expect("Could not save image");
        }

        println!("Done epoch {} of {}", i, num_epochs);
    }

    outbuf
        .save(Path::new(output_folder).join("out.jpg"))
        .expect("Could not save image");
}

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    #[clap(short, long)]
    input_path: String,

    #[clap(short, long)]
    output_folder: String,

    #[clap(short, long, default_value_t = 200)]
    epochs: u32,

    #[clap(short, long, default_value_t = 50)]
    gens: u32,

    #[clap(short, long)]
    scale: f32,
}

fn main() {
    let args = Args::parse();

    evolve(
        &args.input_path,
        args.epochs,
        args.gens,
        &args.output_folder,
        args.scale,
    );
}
