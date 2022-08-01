# Image Metadata Exporter

## Description

A command-line application that extracts a fixed set of metadata from one or more JPEG images specified on the command line, and serializes the data for each to a JSON file. Start by extracting the following fields from the filesystem metadata:
{
    filename
    size
    created_time
    modified_time
}

The output JSON document will be a text file with a top-level JSON object containing each of the property values for the file. The output file name will be the input file name with .json as its extension (replacing the input file's .jpg or .jpeg extension). Numeric values should be serialized as numbers, text values should be serialized as strings, and date/time values should be serialized as strings in a valid ISO8601 format (either UTC or local). Any unspecified or unavailable values should be omitted in the output.

## Setup

### Install Rust and Cargo (on Mac OSX)
Running
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Compile Project
```
git clone git@github.com:Torresnceuit/ImageMetadataExporter.git
cd ImageMetadataExporter
cargo build
```

### Run tests

```
cargo test
```

### Run Executable ImageMetadataExporter
In target/debug folder, execute the application
```
cd target/debug
./image_metadata_exporter -f file1 file2 file3
```
Using --help for usage. Enjoy!!!




