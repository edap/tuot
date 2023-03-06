use image::ImageError;
use std::fmt;
use std::io;
use tobj::LoadError;

// https://learning-rust.github.io/docs/e7.custom_error_types.html

#[derive(Debug)]
pub struct RenderError {
    pub kind: String,
    pub message: String,
}

// Implement std::fmt::Display for RenderError
impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RenderError {{ kind: {}, message: {} }}",
            self.kind, self.message
        )
    }
}

impl From<io::Error> for RenderError {
    fn from(error: io::Error) -> Self {
        RenderError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

impl From<tobj::LoadError> for RenderError {
    fn from(error: tobj::LoadError) -> Self {
        RenderError {
            kind: String::from("tobj"),
            message: error.to_string(),
        }
    }
}

impl From<ImageError> for RenderError {
    fn from(error: ImageError) -> Self {
        RenderError {
            kind: String::from("image"),
            message: error.to_string(),
        }
    }
}
