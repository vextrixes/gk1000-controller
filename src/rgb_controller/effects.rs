use Effects::{
    Breath, CentrifugalWave, Colorful, Explosion, Glittering, HorizontalWave, Launch, Pulse, Rain,
    Ripples, Rotating, Shuttle, SingleOff, SingleOn, Snake, Spectrum, Static, StaticPerKey, Tilt,
    VerticalWave,
};

/// Effects of the keyboard
pub enum Effects {
    Static,
    SingleOn,
    SingleOff,
    Glittering,
    Rain,
    Colorful,
    Breath,
    Spectrum,
    CentrifugalWave,
    VerticalWave,
    HorizontalWave,
    Rotating,
    Explosion,
    Launch,
    Ripples,
    Snake,
    Pulse,
    Tilt,
    Shuttle,
    StaticPerKey,
}

impl Effects {
    /// Returns an u8 of the effect used for communication with the device
    pub fn to_u8(&self) -> u8 {
        match self {
            Static => 1,
            SingleOn => 2,
            SingleOff => 3,
            Glittering => 4,
            Rain => 5,
            Colorful => 6,
            Breath => 7,
            Spectrum => 8,
            CentrifugalWave => 9,
            VerticalWave => 10,
            HorizontalWave => 11,
            Rotating => 12,
            Explosion => 13,
            Launch => 14,
            Ripples => 15,
            Snake => 16,
            Pulse => 17,
            Tilt => 18,
            Shuttle => 19,
            StaticPerKey => 20,
        }
    }
}
