mod color;
mod hid_wrapper;
mod keymap;
mod rgb_controller;
#[cfg(test)]
mod tests;

pub use color::Color;
pub use hid_wrapper::HidWrapperError;
pub use keymap::{Keymap, KeymapKeys};
pub use rgb_controller::controller::RGBController;
pub use rgb_controller::effect_info::{
    DirectionPolicy, EFFECTS, EffectInfo, HorizontalDirections, VerticalDirections,
};
pub use rgb_controller::effects::Effects;
pub use rgb_controller::preset::Preset;

#[cfg(feature = "preset_file_saving")]
mod save_preset;
#[cfg(feature = "preset_file_saving")]
pub use save_preset::{LoadPresetError, SavePresetError};

#[cfg(feature = "access_hid_wrapper")]
pub use hid_wrapper::HidWrapper;
