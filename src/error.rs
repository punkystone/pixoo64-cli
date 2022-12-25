use std::fmt::Display;

use pixoo64::error::{send_error::SendError, set_error::SetError};
#[allow(clippy::module_name_repetitions)]
pub struct ArgumentError;
#[allow(clippy::module_name_repetitions, clippy::enum_variant_names)]
pub enum RunError {
    PixooSendError(SendError),
    PixooSetError(SetError),
    ArgumentError(ArgumentError),
}
impl Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::PixooSendError(error) => write!(f, "{error}"),
            RunError::PixooSetError(error) => write!(f, "{error}"),
            RunError::ArgumentError(_) => write!(f, "Error parsing arguments"),
        }
    }
}
impl From<SendError> for RunError {
    fn from(value: SendError) -> Self {
        RunError::PixooSendError(value)
    }
}

impl From<SetError> for RunError {
    fn from(value: SetError) -> Self {
        RunError::PixooSetError(value)
    }
}
