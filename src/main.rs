use std::path::{Path, PathBuf};

use clap::{Arg, App};

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
            .takes_value(true)])
        .get_matches();

    let filePath = matches.value_of("file").unwrap();
    let fileDirectory = Path::new(filePath).parent().unwrap();

    println!("{}", filePath);
    println!("{}", fileDirectory.display());

    Ok(())
}
