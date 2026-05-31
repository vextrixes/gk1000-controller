use crate::hid_wrapper::constants::{
    GET_REPORT_REPORT, MAX_SEND_REPORT_LEN, PRODUCT_ID, VENDOR_ID,
};
use crate::hid_wrapper::error::HidWrapperError;
use hidapi::{HidApi, HidDevice};

pub struct HidWrapper {
    hid_api: HidApi,
    hid_device: Option<HidDevice>,
}

impl Default for HidWrapper {
    /// Makes a new instance of HidWrapper (No shit Sherlock)
    ///
    /// # Panics
    /// If ```HidWrapperError::HidApiError``` is returned from new()
    fn default() -> Self {
        match Self::new() {
            Ok(hid_wrapper) => hid_wrapper,
            Err(err) => panic!("hid wrapper Default() failed to create new HidWrapper: {err}"),
        }
    }
}

impl HidWrapper {
    /// Makes a new instance of HidWrapper (No shit Sherlock)
    ///
    /// # Errors
    /// ```HidWrapperError::HidApiError```
    pub fn new() -> Result<HidWrapper, HidWrapperError> {
        Ok(HidWrapper {
            hid_api: match HidApi::new() {
                Ok(hidapi) => hidapi,
                Err(err) => return Err(HidWrapperError::from(err)),
            },
            hid_device: None,
        })
    }

    pub fn is_device_connected(&mut self) -> Result<bool, HidWrapperError> {
        match self.hid_api.reset_devices() {
            Ok(_) => {}
            Err(err) => return Err(HidWrapperError::from(err)),
        };
        match self.hid_api.add_devices(VENDOR_ID, PRODUCT_ID) {
            Ok(_) => {}
            Err(err) => return Err(HidWrapperError::from(err)),
        };
        Ok(self.hid_api.device_list().count() >= 1)
    }

    /// Opens a device
    ///
    /// # Errors
    /// ```HidWrapperError::HidApiError```
    /// ```HidWrapperError::NoHidDeviceError```
    pub fn open_device(&mut self) -> Result<(), HidWrapperError> {
        self.hid_device = match self.hid_api.open(VENDOR_ID, PRODUCT_ID) {
            Ok(hid_device) => Some(hid_device),
            Err(err) => return Err(HidWrapperError::from(err)),
        };
        Ok(())
    }

    /// Drops the device
    ///
    pub fn close_device(&mut self) {
        self.hid_device = None;
    }

    /// Sends a set report (65 bytes len)
    ///
    /// # Errors
    /// ```HidWrapperError::HidApiError```
    /// ```HidWrapperError::NoHidDeviceError```
    pub fn send_report(&mut self, data: &[u8]) -> Result<(), HidWrapperError> {
        match &self.hid_device {
            Some(hid_device) => {
                let mut out = [0; MAX_SEND_REPORT_LEN];
                out[1..1 + data.len()].copy_from_slice(data);
                match hid_device.send_feature_report(&out) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(HidWrapperError::from(err)),
                }
            }
            None => Err(HidWrapperError::NoHidDeviceError),
        }
    }

    /// Sends a get report (64 bytes len)
    ///
    /// # Errors
    /// ```HidWrapperError::HidApiError```
    /// ```HidWrapperError::NoHidDeviceError```
    pub fn get_report(&mut self) -> Result<usize, HidWrapperError> {
        match &self.hid_device {
            Some(hid_device) => {
                let mut request = GET_REPORT_REPORT;
                request[1] = 64;
                match hid_device.get_feature_report(&mut request) {
                    Ok(usize) => Ok(usize),
                    Err(err) => Err(HidWrapperError::from(err)),
                }
            }
            None => Err(HidWrapperError::NoHidDeviceError),
        }
    }
}
