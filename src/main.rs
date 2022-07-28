mod cli;
mod image_builder;
mod identicon;

use cli::Cli;
use structopt::StructOpt;
use image_builder::ImageBuilder;

fn main() {
  let args = Cli::from_args();

  let builder = ImageBuilder::new(args.username.to_owned());

  let path = builder.hash_username()
    .pick_color()
    .set_grid();
    // .filter_odd_squares()
    // .build_pixel_map()
    // .draw_image()
    // .build();

    println!("{:?}", path);
}
