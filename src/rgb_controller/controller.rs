use crate::Effects;
use crate::HidWrapperError;
use crate::hid_wrapper::HidWrapper;

pub struct RGBController {
    hid_wrapper: HidWrapper,
}

impl Default for RGBController {
    /// Makes a new instance of HidWrapper (No shit Sherlock)
    ///
    /// # Panics
    /// if ```HidWrapperError::HidApiError``` is returned from new()
    fn default() -> Self {
        match Self::new() {
            Ok(hid_wrapper) => hid_wrapper,
            Err(err) => panic!("RGBController Default() failed to create new RGBController: {err}"),
        }
    }
}

impl RGBController {
    /// Makes a new instance of HidWrapper (No shit Sherlock)
    ///
    /// # Errors
    /// ```HidWrapperError::HidApiError```
    pub fn new() -> Result<RGBController, HidWrapperError> {
        Ok(RGBController {
            hid_wrapper: HidWrapper::new()?,
        })
    }

    pub fn is_device_connected(&mut self) -> Result<bool, HidWrapperError> {
        self.hid_wrapper.is_device_connected()
    }

    /// Starts the RGB conversation
    ///
    /// # Errors
    /// ```HidWrapperError::HidApiError```
    /// ```HidWrapperError::NoHidDeviceError```
    fn prepare_device(&mut self) -> Result<(), HidWrapperError> {
        self.hid_wrapper.send_report(&[0x04, 0xab])?;
        self.hid_wrapper.get_report()?;

        self.hid_wrapper.send_report(&[
            0x26, 0x36, 0xf2, 0x27, 0xb7, 0x2a, 0x56, 0x77, 0x98, 0x59, 0x9c, 0x6d, 0x9f, 0xc6,
            0xb, 0x44, 0xec, 0xa8, 0x47, 0x21, 0xe2, 0x65, 0x76, 0xa3, 0x70, 0xbe, 0xa2, 0xbb,
            0x95, 0x47, 0x5e, 0xbe, 0x23, 0x1b, 0xad, 0xb8, 0xc3, 0xcd, 0x1f, 0x4, 0x39, 0xa2,
            0x3d, 0x19, 0xe9, 0xfd, 0x27, 0xa7, 0x9a, 0x1a, 0x7c, 0xe3, 0x7b, 0xff, 0x7a, 0x1e,
            0x63, 0xb0, 0x67, 0x9d, 0x5d, 0xa3, 0x2f, 0xa5,
        ])?;

        self.hid_wrapper.send_report(&[0x04, 0x02])?;
        self.hid_wrapper.get_report()?;

        self.hid_wrapper
            .send_report(&[0x04, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12])?;
        self.hid_wrapper.get_report()?;
        Ok(())
    }

    /// Some middle bytes IDK what to name ts
    /// Is a required part of the RGB conversation
    ///
    /// # Errors
    /// ```HidWrapperError::HidApiError```
    /// ```HidWrapperError::NoHidDeviceError```
    fn mid(&mut self) -> Result<(), HidWrapperError> {
        //TODO: Rename func and possibly change packets depending on mode
        self.hid_wrapper.send_report(&[
            0x1, 0x0, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x0, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0x2, 0x0, 0x0, 0xff, 0x0, 0x0, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0x3, 0x0, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0x4, 0x5a, 0xff, 0xff, 0x0, 0xff, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
        ])?;

        self.hid_wrapper.send_report(&[
            0x5, 0x0, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0x6, 0x0, 0x0, 0xff, 0x0, 0x0, 0x0, 0x0, 0x0, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0x7, 0x5a, 0xff, 0xff, 0x0, 0xff, 0x0, 0x0, 0x0, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0x8, 0x0, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x0, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
        ])?;

        self.hid_wrapper.send_report(&[
            0x9, 0x0, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0xa, 0x0, 0x0, 0xff, 0x0, 0x0, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0xb, 0x0, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0xc, 0x0, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
        ])?;

        self.hid_wrapper.send_report(&[
            0xd, 0x0, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0xe, 0x0, 0x0, 0xff, 0x0, 0x0, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0xf, 0x0, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0x10, 0x0, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
        ])?;

        self.hid_wrapper.send_report(&[
            0x11, 0x0, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0x12, 0x0, 0x0, 0xff, 0x0, 0x0, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0x13, 0x0, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x1, 0x10, 0xc, 0x0, 0x0, 0x0, 0xaa, 0x55,
            0x80, 0xff, 0xff, 0xff, 0x0, 0xff, 0x0, 0x0, 0x0, 0x10, 0x0, 0x0, 0x0, 0x0, 0xaa, 0x55,
        ])?;

        Ok(())
    }

    /// Sets colors for each key (only if StaticPerKey effect is set) base on a 2d array
    /// Is a required part of the RGB conversation
    ///
    /// # Errors
    /// ```HidWrapperError::HidApiError```
    /// ```HidWrapperError::NoHidDeviceError```
    fn custom_keys(&mut self, keymap: &[[u8; 3]; 144]) -> Result<(), HidWrapperError> {
        for i in 0..9 {
            //Magic num 9 == len of keymap / 16
            self.hid_wrapper.send_report(&[
                0x80,
                keymap[i * 16][0],
                keymap[i * 16][1],
                keymap[i * 16][2],
                0x80,
                keymap[i * 16 + 1][0],
                keymap[i * 16 + 1][1],
                keymap[i * 16 + 1][2],
                0x80,
                keymap[i * 16 + 2][0],
                keymap[i * 16 + 2][1],
                keymap[i * 16 + 2][2],
                0x80,
                keymap[i * 16 + 3][0],
                keymap[i * 16 + 3][1],
                keymap[i * 16 + 3][2],
                0x80,
                keymap[i * 16 + 4][0],
                keymap[i * 16 + 4][1],
                keymap[i * 16 + 4][2],
                0x80,
                keymap[i * 16 + 5][0],
                keymap[i * 16 + 5][1],
                keymap[i * 16 + 5][2],
                0x80,
                keymap[i * 16 + 6][0],
                keymap[i * 16 + 6][1],
                keymap[i * 16 + 6][2],
                0x80,
                keymap[i * 16 + 7][0],
                keymap[i * 16 + 7][1],
                keymap[i * 16 + 7][2],
                0x80,
                keymap[i * 16 + 8][0],
                keymap[i * 16 + 8][1],
                keymap[i * 16 + 8][2],
                0x80,
                keymap[i * 16 + 9][0],
                keymap[i * 16 + 9][1],
                keymap[i * 16 + 9][2],
                0x80,
                keymap[i * 16 + 10][0],
                keymap[i * 16 + 10][1],
                keymap[i * 16 + 10][2],
                0x80,
                keymap[i * 16 + 11][0],
                keymap[i * 16 + 11][1],
                keymap[i * 16 + 11][2],
                0x80,
                keymap[i * 16 + 12][0],
                keymap[i * 16 + 12][1],
                keymap[i * 16 + 12][2],
                0x80,
                keymap[i * 16 + 13][0],
                keymap[i * 16 + 13][1],
                keymap[i * 16 + 13][2],
                0x80,
                keymap[i * 16 + 14][0],
                keymap[i * 16 + 14][1],
                keymap[i * 16 + 14][2],
                0x80,
                keymap[i * 16 + 15][0],
                keymap[i * 16 + 15][1],
                keymap[i * 16 + 15][2],
            ])?;
        }
        Ok(())
    }

    /// Applies an effect to the device
    /// Is a required part of the RGB conversation
    ///
    /// # Errors
    /// ```HidWrapperError::HidApiError```
    /// ```HidWrapperError::NoHidDeviceError```
    fn apply_to_device(&mut self) -> Result<(), HidWrapperError> {
        self.hid_wrapper.send_report(&[0x04, 0x02])?;
        self.hid_wrapper.get_report()?;

        self.hid_wrapper.send_report(&[0x04, 0xf0])?;

        Ok(())
    }

    /// Sets an effect
    ///
    /// # Errors
    /// ```HidWrapperError::HidApiError```
    /// ```HidWrapperError::NoHidDeviceError```
    #[allow(clippy::too_many_arguments)]
    pub fn set_effect(
        &mut self,
        effect: Effects,
        red: u8,
        green: u8,
        blue: u8,
        full_color: bool,
        brightness: u8,
        speed: u8,
        direction: u8,
        keymap: Option<[[u8; 3]; 144]>,
    ) -> Result<(), HidWrapperError> {
        self.hid_wrapper.open_device()?;

        self.prepare_device()?;

        self.mid()?;

        self.hid_wrapper.send_report(&[])?;
        self.hid_wrapper.send_report(&[])?;
        self.hid_wrapper.send_report(&[])?;

        match keymap {
            Some(map) => self.custom_keys(&map)?,
            None => self.custom_keys(&[[0, 255, 0]; 144])?,
        }

        self.hid_wrapper.send_report(&[
            effect.to_u8(),
            red,
            green,
            blue,
            0,
            0,
            0,
            0,
            u8::from(full_color),
            brightness.clamp(0, 16),
            speed.clamp(1, 16),
            direction.clamp(0, 3),
            0,
            0,
            0xaa,
            0x55,
        ])?;

        self.apply_to_device()?;

        self.hid_wrapper.close_device();

        Ok(())
    }
    pub fn set_static_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Static,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            1,
            0,
            None,
        )
    }

    pub fn set_single_off_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::SingleOff,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_single_on_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::SingleOn,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_glittering_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Glittering,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_rain_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Rain,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_colorful_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Colorful,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_breath_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Breath,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_spectrum_effect(
        &mut self,
        brightness: u8,
        speed: u8,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Spectrum,
            0,
            0,
            0,
            false,
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_centrifugal_wave_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::CentrifugalWave,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_vertical_wave_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        direction: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::VerticalWave,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            direction.clamp(2, 3),
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_horizontal_wave_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        direction: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::HorizontalWave,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            direction.clamp(0, 1),
            None,
        )
    }

    pub fn set_rotating_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Rotating,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_explosion_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Explosion,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_launch_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Launch,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_ripples_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Ripples,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_snake_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Snake,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_pulse_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Pulse,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_tilt_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Tilt,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_shuttle_effect(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        brightness: u8,
        speed: u8,
        full_color: Option<bool>,
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::Shuttle,
            red,
            green,
            blue,
            full_color.unwrap_or(false),
            brightness,
            speed,
            0,
            None,
        )
    }

    pub fn set_static_per_key_effect(
        &mut self,
        brightness: u8,
        keymap: [[u8; 3]; 144],
    ) -> Result<(), HidWrapperError> {
        self.set_effect(
            Effects::StaticPerKey,
            0,
            0,
            0,
            false,
            brightness,
            1,
            0,
            Some(keymap),
        )
    }
}
