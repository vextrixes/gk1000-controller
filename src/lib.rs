mod color;
mod hid_wrapper;
mod rgb_controller;

pub use color::Color;
pub use hid_wrapper::HidWrapperError;
pub use rgb_controller::controller::RGBController;
pub use rgb_controller::effects::Effects;
pub use rgb_controller::keymap::{Keymap, KeymapKeys};

#[cfg(feature = "access_hid_wrapper")]
pub use hid_wrapper::HidWrapper;
