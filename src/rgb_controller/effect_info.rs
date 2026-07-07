use crate::Effects;

#[derive(Clone)]
pub struct EffectInfo<'a> {
    pub name: &'a str,
    pub effect: Effects,
    pub can_set_color: bool,
    pub can_set_speed: bool,
    pub can_set_direction: DirectionPolicy,
    pub can_set_full_color: bool,
    pub requires_keymap: bool,
}

#[derive(PartialEq, Clone)]
pub enum DirectionPolicy {
    None,
    Horizontal01,
    Vertical23,
}

pub const EFFECTS: [EffectInfo; 20] = [
    EffectInfo {
        name: "Static",
        effect: Effects::Static,
        can_set_color: true,
        can_set_speed: false,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "SingleOn",
        effect: Effects::SingleOn,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "SingleOff",
        effect: Effects::SingleOff,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Glittering",
        effect: Effects::Glittering,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Rain",
        effect: Effects::Rain,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Colorful",
        effect: Effects::Colorful,
        can_set_color: false,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: false,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Breath",
        effect: Effects::Breath,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Spectrum",
        effect: Effects::Spectrum,
        can_set_color: false,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: false,
        requires_keymap: false,
    },
    EffectInfo {
        name: "CentrifugalWave",
        effect: Effects::CentrifugalWave,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "VerticalWave",
        effect: Effects::VerticalWave,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::Vertical23,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "HorizontalWave",
        effect: Effects::HorizontalWave,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::Horizontal01,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Rotating",
        effect: Effects::Rotating,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Explosion",
        effect: Effects::Explosion,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Launch",
        effect: Effects::Launch,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Ripples",
        effect: Effects::Ripples,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Snake",
        effect: Effects::Snake,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Pulse",
        effect: Effects::Pulse,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Tilt",
        effect: Effects::Tilt,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "Shuttle",
        effect: Effects::Shuttle,
        can_set_color: true,
        can_set_speed: true,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: true,
        requires_keymap: false,
    },
    EffectInfo {
        name: "StaticPerKey",
        effect: Effects::StaticPerKey,
        can_set_color: false,
        can_set_speed: false,
        can_set_direction: DirectionPolicy::None,
        can_set_full_color: false,
        requires_keymap: true,
    },
];

pub enum HorizontalDirections {
    Left,
    Right,
}

impl HorizontalDirections {
    pub fn to_u8(&self) -> u8 {
        match self {
            HorizontalDirections::Left => 0,
            HorizontalDirections::Right => 1,
        }
    }
}

pub enum VerticalDirections {
    Down,
    Up,
}

impl VerticalDirections {
    pub fn to_u8(&self) -> u8 {
        match self {
            VerticalDirections::Down => 2,
            VerticalDirections::Up => 3,
        }
    }
}
