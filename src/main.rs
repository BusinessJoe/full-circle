use clap::Parser;

use wallpaper_evolution::evolve;

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
    scale: f64,
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
