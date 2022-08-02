use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("File I/O error!")]
    IOError,
    #[error("Invalid input!")]
    InvalidInput,
    #[error("Can not read exif metadata!")]
    ExifMetadataError,
    #[error("Can not export to Json!")]
    ExportJsonError,
}