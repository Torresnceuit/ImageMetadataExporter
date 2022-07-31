use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path};
use exif::{Exif, Tag, In};
use serde_json::Value;

pub fn extract_exif_metadata_from_image(file_arg: &str)
{
    let file_path = Path::new(file_arg);
    let file = std::fs::File::open(file_path);
    let file_name_with_extension = file_path.file_name().unwrap().to_str().unwrap();
    let directory_path = file_path.parent().unwrap();
    if file.is_err() {
        eprintln!("WARN: Could not find file: {}", file_name_with_extension);
        return;
    }
    let file = file.unwrap();

    let mut bufreader = std::io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    let exif = exif_reader.read_from_container(&mut bufreader).unwrap();

    let image_size = match file.metadata() {
        Ok(metadata) => Some(metadata.len()),
        Err(e) => {
            eprintln!("WARN: Can not get image {} size due to metadata error: {}", file_name_with_extension, e);
            None
        }
    };

    let mut json = parsing_metadata(exif);
    json["file_name"] = serde_json::json!(&file_name_with_extension);
    json["size"] = serde_json::json!(image_size);

    let file_name_split_vec = file_name_with_extension.split(".").collect::<Vec<_>>();
    let file_name_raw = file_name_split_vec[0];
    println!("{}", file_name_raw);

    export_exif_to_json(directory_path, file_name_raw, json);
}

pub fn export_exif_to_json<P>(directory_path: P, file_name: &str, json: Value)
    where
        P: AsRef<Path>,
{
    let full_json_file_name = format!("{}.json", file_name);
    let json_file_path = directory_path.as_ref().join(&full_json_file_name);

    let file = File::create(json_file_path);
    if file.is_err() {
        eprintln!("WARN: Could not create file: {}", full_json_file_name);
        return;
    }
    let file = file.unwrap();

    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &json).unwrap();
    writer.flush().unwrap();
}

pub fn parsing_metadata(exif: Exif) -> Value {
    let camera_model = match exif.get_field(Tag::Model, In::PRIMARY) {
        Some(camera_model) => Some(
            camera_model
                .display_value()
                .with_unit(&exif)
                .to_string()
                .replace("\"", ""),
        ),
        None => None,
    };

    let serial_number = match exif.get_field(Tag::BodySerialNumber, In::PRIMARY) {
        Some(serial_number) => Some(
            serial_number
                .display_value()
                .with_unit(&exif)
                .to_string()
                .replace("\"", ""),
        ),
        None => None,
    };

    // TAG description: time that img_spec are created
    let created_time = match exif.get_field(Tag::DateTimeDigitized, In::PRIMARY) {
        Some(created_time) => Some(created_time.display_value().with_unit(&exif).to_string()),
        None => None,
    };

    // TAG description: time that change file
    let modified_time = match exif.get_field(Tag::DateTime, In::PRIMARY) {
        Some(modified_time) => Some(modified_time.display_value().with_unit(&exif).to_string()),
        None => None,
    };

    let orientation = match exif.get_field(Tag::Orientation, In::PRIMARY) {
        Some(orientation) => orientation.value.get_uint(0),
        None => None,
    };

    // TAG description: capture time
    let capture_time = match exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
        Some(capture_time) => Some(capture_time.display_value().with_unit(&exif).to_string()),
        None => None,
    };

    serde_json::json!({
        "created_time": created_time,
        "modified_time": modified_time,
        "orientation": orientation,
        "capture_time": capture_time,
        "camera_model": camera_model,
        "serial_number": serial_number,
    })
}