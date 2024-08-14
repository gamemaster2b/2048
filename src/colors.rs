use bevy::{color::Oklcha, prelude::Color};

pub const BOARD: Color = Color::oklcha(0.191, 0.036, 273.2, 1.0);

pub const TILE_PLACEHOLDER: Color = Color::Oklcha(Oklcha {
    lightness: 0.6299,
    chroma: 0.14,
    hue: 318.22,
    alpha: 1.0,
});
pub const CLEAR_COLOR: Color = Color::Oklcha(Oklcha {
    lightness: 0.2706,
    chroma: 0.035,
    hue: 267.69,
    alpha: 1.0,
});
