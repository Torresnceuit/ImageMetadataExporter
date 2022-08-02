extern crate core;

use clap::{Arg, App};

use utils::extract_exif_metadata_from_image;

mod utils;
mod err;

fn main() -> Result<(), String> {
    let matches = App::new("Image Metadata Exporter")
        .author("Torres Nguyen <Torresnceuit@gmail.com>")
        .about("Export Image Metadata to JSON")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("0.1.0"))
        .arg_required_else_help(true)
        .args(&[Arg::new("file")
            .long("file")
            .short('f')
            .help("Path to image")
            .max_values(3) // Allow up to 3 files passed through command line argument
            .takes_value(true)])
        .get_matches();

    let file_arg_vec = matches
        .get_many::<String>("file")
        .unwrap()
        .collect::<Vec<_>>();
    dbg!(&file_arg_vec);

    for file_arg in file_arg_vec.into_iter() {
        let _ = extract_exif_metadata_from_image(file_arg);
    }
    Ok(())
}
