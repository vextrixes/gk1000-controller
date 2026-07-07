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
    pub fn save_to_file(&self, _file: &mut File) -> Result<(), SavePresetError> {
        if !self.is_valid() {
            return Err(SavePresetError::InvalidPreset);
        }
        match _file.write_all(&self._encode_to_buffer()?) {
            Err(err) => Err(SavePresetError::FileError(err)),
            _ => Ok(()),
        }
    }

    pub(crate) fn _encode_to_buffer(&self) -> Result<Vec<u8>, SavePresetError> {
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

    pub(crate) fn _decode_from_buffer(buffer: &mut VecDeque<u8>) -> Result<Self, LoadPresetError> {
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
