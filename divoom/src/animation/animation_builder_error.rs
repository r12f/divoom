use thiserror::Error;

/// Building animations can fail due to different reasons, such as failing to load image file or decode image file.
/// Hence we have defined the error code here.
#[derive(Debug, Error)]
pub enum DivoomAnimationBuilderError {
    #[error("Failed to load file from disk")]
    IOError {
        #[from]
        source: std::io::Error,
    },

    #[error("Failed to decode file")]
    DecodeError {
        #[from]
        source: gif::DecodingError,
    },

    #[error("Canvas size not supported. Only 16, 32, and 64 are supported.")]
    UnsupportedCanvasSize,
}

/// Result that wraps the error.
pub type DivoomAnimationBuilderResult<T> = std::result::Result<T, DivoomAnimationBuilderError>;
