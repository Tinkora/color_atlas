use thiserror::Error;

/// Stable error type for the color_atlas toolkit.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("Invalid hex format: {0}")]
    InvalidHexFormat(String),

    #[error("Invalid hex length: expected 3, 6, or 8 hex chars after '#'")]
    InvalidHexLength,

    #[error("Invalid hex character in color string")]
    InvalidHexChar,

    #[error("JSON deserialization failed: {0}")]
    InvalidJson(String),

    #[error("Pixel data is empty")]
    EmptyPixels,

    #[error("Pixel buffer length {actual} does not match expected length {expected}")]
    InvalidPixelBuffer { expected: usize, actual: usize },

    #[error("Image contains {pixels} pixels; maximum supported is {max}")]
    ImageTooLarge { pixels: usize, max: usize },

    #[error("Palette count {requested} exceeds the maximum of {max}")]
    PaletteCountTooLarge { requested: u32, max: u32 },

    #[error("Not enough unique pixels (got {got}) for requested palette size ({needed})")]
    InsufficientPixels { got: usize, needed: u32 },

    #[error("Count must be at least 1, got {0}")]
    ZeroCount(u32),
}

impl CoreError {
    /// Returns a stable machine error code for Web, CLI, and Agent consumers.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidHexFormat(_) => "INVALID_HEX_FORMAT",
            Self::InvalidHexLength => "INVALID_HEX_LENGTH",
            Self::InvalidHexChar => "INVALID_HEX_CHAR",
            Self::InvalidJson(_) => "INVALID_JSON",
            Self::EmptyPixels => "EMPTY_PIXELS",
            Self::InvalidPixelBuffer { .. } => "INVALID_PIXEL_BUFFER",
            Self::ImageTooLarge { .. } => "IMAGE_TOO_LARGE",
            Self::PaletteCountTooLarge { .. } => "PALETTE_COUNT_TOO_LARGE",
            Self::InsufficientPixels { .. } => "INSUFFICIENT_PIXELS",
            Self::ZeroCount(_) => "ZERO_COUNT",
        }
    }
}
