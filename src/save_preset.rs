use crate::{DirectionPolicy, EFFECTS, EffectInfo, Effects, Preset};
use std::{
    collections::VecDeque,
    fs::File,
    io::{Read, Write},
};

#[derive(Debug)]
pub enum SavePresetError {
    InvalidPreset,
    FileError(std::io::Error),
    NoKeymapGiven,
}

#[derive(Debug)]
pub enum LoadPresetError {
    InvalidOrCorruptedFile,
    OpenFileError(std::io::Error),
}

const MAGIC_NUMBER_1: u8 = 0b10100100;
const MAGIC_NUMBER_2: u8 = 0b00010000;
const VERSION: u8 = 1;

impl Preset {
    /// Method to save a preset to a file.
    ///
    /// # Example
    /// ```
    /// let preset = gk1000_controller::Preset::default();
    /// let mut file = std::fs::File::create("/tmp/file.gk1k").expect("Failed to create");
    /// preset.save_to_file(&mut file).expect("Failed to Save");
    /// ```
    /// Make sure you use a function to create a writable, overridable file.
    ///
    /// # Errors
    /// If this function fails a `SavePresetError` will be returned
    pub fn save_to_file(&self, _file: &mut File) -> Result<(), SavePresetError> {
        if !self.is_valid() {
            return Err(SavePresetError::InvalidPreset);
        }
        match _file.write_all(&self._encode_to_buffer()?) {
            Err(err) => Err(SavePresetError::FileError(err)),
            _ => Ok(()),
        }
    }

    /// Method to save a preset to a buffer.
    ///
    /// This method is not supposed to be used to save presets to a file
    ///
    /// # Example
    /// ```
    /// let preset = gk1000_controller::Preset::default();
    /// if preset.is_valid(){
    /// let buffer = preset._encode_to_buffer().expect("Failed to Encode");
    /// }
    /// ```
    ///
    /// # Errors
    /// If this function fails a `SavePresetError` will be returned
    pub fn _encode_to_buffer(&self) -> Result<Vec<u8>, SavePresetError> {
        let effect_info: EffectInfo = EFFECTS[usize::from(self.effect.to_u8() - 1)].clone();
        let mut buffer: Vec<u8> = vec![MAGIC_NUMBER_1, MAGIC_NUMBER_2, VERSION];

        buffer
            .push(self.effect.to_u8() + (u8::from(self.full_color) * 128) + (self.direction * 32)); //Effect id, full_color, and direction in a single byte

        if !self.full_color && effect_info.can_set_color {
            buffer.extend(&self.color.to_array());
        }

        if effect_info.can_set_speed {
            buffer.push(self.speed);
        }

        buffer.push(self.brightness);

        if effect_info.requires_keymap {
            for key_color in self.keymap.ok_or(SavePresetError::NoKeymapGiven)?.iter() {
                buffer.extend(key_color);
            }
        }

        Ok(buffer)
    }

    /// Function to load a preset from a file.
    ///
    /// # Example
    /// ```
    /// let mut file = std::fs::File::open("/tmp/file.gk1k").expect("Failed to open");
    /// let preset = gk1000_controller::Preset::load_from_file(&mut file).expect("Failed to load");
    /// ```
    ///
    /// # Errors
    /// If this function fails a `LoadPresetError` will be returned
    pub fn load_from_file(_file: &mut File) -> Result<Self, LoadPresetError> {
        let mut buffer: Vec<u8> = vec![];
        match _file.read_to_end(&mut buffer) {
            Ok(_) => {
                let mut buffer: VecDeque<u8> = buffer.into();

                Self::_decode_from_buffer(&mut buffer)
            }
            Err(err) => Err(LoadPresetError::OpenFileError(err)),
        }
    }

    /// Function to load a preset from a buffer.
    ///
    /// This function is not supposed to be used to load presets from a file
    ///
    /// # Example
    /// ```
    /// let mut buffer: std::collections::VecDeque<u8> = gk1000_controller::Preset::default()._encode_to_buffer().expect("Failed to Save").into();
    /// let preset = gk1000_controller::Preset::_decode_from_buffer(&mut buffer).expect("Failed to decode");
    /// ```
    ///
    /// # Errors
    /// If this function fails a `LoadPresetError` will be returned
    pub fn _decode_from_buffer(buffer: &mut VecDeque<u8>) -> Result<Self, LoadPresetError> {
        let mut save: Preset = Default::default();

        if buffer.pop_front() != Some(MAGIC_NUMBER_1) || buffer.pop_front() != Some(MAGIC_NUMBER_2)
        {
            return Err(LoadPresetError::InvalidOrCorruptedFile);
        }

        if buffer
            .pop_front()
            .ok_or(LoadPresetError::InvalidOrCorruptedFile)?
            != VERSION
        {
            return Err(LoadPresetError::InvalidOrCorruptedFile);
            //todo!("TOO BAD, Invalid VERSION")
        }

        let effect_byte = buffer
            .pop_front()
            .ok_or(LoadPresetError::InvalidOrCorruptedFile)?;

        save.full_color = (effect_byte & 0b1000_0000) != 0;
        let direction: u8 = (effect_byte >> 5) & 0b11;
        let effect_id: u8 = effect_byte & 0b0001_1111;

        save.effect = Effects::from_u8(effect_id).ok_or(LoadPresetError::InvalidOrCorruptedFile)?;
        let effect_info: EffectInfo = EFFECTS[usize::from(save.effect.to_u8() - 1)].clone();

        if !save.full_color && effect_info.can_set_color {
            save.color.red = buffer
                .pop_front()
                .ok_or(LoadPresetError::InvalidOrCorruptedFile)?;
            save.color.green = buffer
                .pop_front()
                .ok_or(LoadPresetError::InvalidOrCorruptedFile)?;
            save.color.blue = buffer
                .pop_front()
                .ok_or(LoadPresetError::InvalidOrCorruptedFile)?;
        }

        if effect_info.can_set_direction != DirectionPolicy::None {
            save.direction = direction;
        }

        if effect_info.can_set_speed {
            save.speed = buffer
                .pop_front()
                .ok_or(LoadPresetError::InvalidOrCorruptedFile)?;
        }

        save.brightness = buffer
            .pop_front()
            .ok_or(LoadPresetError::InvalidOrCorruptedFile)?;

        if effect_info.requires_keymap {
            let mut keymap_buffer: [[u8; 3]; 144] = [[0; 3]; 144];
            #[allow(clippy::needless_range_loop)]
            for i in 0..keymap_buffer.len() {
                keymap_buffer[i] = [
                    buffer
                        .pop_front()
                        .ok_or(LoadPresetError::InvalidOrCorruptedFile)?,
                    buffer
                        .pop_front()
                        .ok_or(LoadPresetError::InvalidOrCorruptedFile)?,
                    buffer
                        .pop_front()
                        .ok_or(LoadPresetError::InvalidOrCorruptedFile)?,
                ];
            }
            save.keymap = Some(keymap_buffer);
        }
        Ok(save)
    }
}
