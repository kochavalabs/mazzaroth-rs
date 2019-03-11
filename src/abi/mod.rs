pub mod decoder;
pub mod encoder;

pub mod types;

/// Error for decoding rust types from decoder
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum AbiError {
    /// Unexpected end of the decoder
    UnexpectedEof,

    /// Bad format
    BadFormat,
}

/// AbiType trait
pub trait AbiType: Sized {
    /// Define how objects should be decoded.
    fn decode(bytes: Vec<u8>) -> Result<Self, AbiError>;

    /// Define how objects should be encoded.
    fn encode(self) -> Vec<u8>;
}
