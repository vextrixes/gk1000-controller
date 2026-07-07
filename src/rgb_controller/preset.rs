use crate::Color;
use crate::Effects;

#[derive(PartialEq, Debug)]
pub struct Preset {
    pub effect: Effects,
    pub color: Color,
    pub full_color: bool,
    pub brightness: u8,
    pub speed: u8,
    pub direction: u8,
    pub keymap: Option<[[u8; 3]; 144]>,
}

impl Preset {
    pub fn new(
        effect: Effects,
        color: Color,
        full_color: bool,
        brightness: u8,
        speed: u8,
        direction: u8,
        keymap: Option<[[u8; 3]; 144]>,
    ) -> Self {
        Self {
            effect,
            color,
            full_color,
            brightness: brightness.clamp(0, 16),
            speed: speed.clamp(1, 16),
            direction: direction.clamp(0, 3),
            keymap,
        }
    }
}

impl Default for Preset {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }
}
