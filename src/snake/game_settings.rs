use crate::modes::color_mode::ColorMode;

#[derive(Clone, Copy)]
pub struct GameSettings {
    pub max_width: u16,
    pub max_height: u16,
    pub color_mode: ColorMode
}