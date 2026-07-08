use crate::Color;
use crate::EFFECTS;
use crate::EffectInfo;
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

    /// Validates if the preset is a valid preset that can be applied
    pub fn is_valid(&self) -> bool {
        if self.brightness > 16 || self.direction > 3 || 1 > self.speed || self.speed > 16 {
            return false;
        }
        let effect_info: &EffectInfo = &EFFECTS[usize::from(self.effect.to_u8() - 1)];
        if effect_info.requires_keymap && self.keymap.is_none() {
            return false;
        }
        // These are uneded and cause issues
        // if effect_info.can_set_full_color && self.full_color{
        //     return false;
        // }
        // if effect_info.can_set_direction == DirectionPolicy::None && self.direction != 0{
        //     return false;
        // }
        // if effect_info.can_set_color && self.color != Color::default(){
        //     return false;
        // }
        true
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
