use hidapi::HidError;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum HidWrapperError {
    #[allow(dead_code)]
    HidApiError(HidError),
    NoHidDeviceError,
}

impl Display for HidWrapperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HidWrapperError::HidApiError(err) => {
                write!(f, "Hid Api Error: {err}")
            }
            HidWrapperError::NoHidDeviceError => {
                write!(f, "No Hid Device Found")
            }
        }
    }
}

impl Error for HidWrapperError {}

impl From<HidError> for HidWrapperError {
    fn from(error: HidError) -> Self {
        match error {
            HidError::HidApiError { ref message } => {
                if message == "No HID devices with requested VID/PID found in the system." {
                    HidWrapperError::NoHidDeviceError
                } else {
                    HidWrapperError::HidApiError(error)
                }
            }
            _ => HidWrapperError::HidApiError(error),
        }
    }
}
