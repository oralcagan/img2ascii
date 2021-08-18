#[derive(Debug)]
pub enum Error {
    Image(image::ImageError),
    Io(std::io::Error),
    FileNotFound(String),
}

impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Self {
        Self::Image(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

pub type ConverterResult<T> = Result<T, crate::error::Error>;
