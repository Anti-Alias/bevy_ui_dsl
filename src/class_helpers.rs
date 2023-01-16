use bevy_ui::{Val, Size, UiRect, BackgroundColor};
use bevy_render::color::Color;

pub fn size(width: Val, height: Val) -> Size {
    Size::new(width, height)
}

pub fn rect(left: Val, right: Val, top: Val, bottom: Val) -> UiRect {
    UiRect::new(left, right, bottom, top)
}

pub fn all(value: Val) -> UiRect {
    UiRect::all(value)
}

pub fn horozontal(value: Val) -> UiRect {
    UiRect::horizontal(value)
}

pub fn vertical(value: Val) -> UiRect {
    UiRect::vertical(value)
}

pub fn left(value: Val) -> UiRect {
    UiRect::left(value)
}

pub fn right(value: Val) -> UiRect {
    UiRect::right(value)
}

pub fn top(value: Val) -> UiRect {
    UiRect::top(value)
}

pub fn bottom(value: Val) -> UiRect {
    UiRect::bottom(value)
}

pub fn pc(num: impl Tof32) -> Val {
    Val::Percent(num.to_f32())
}

pub fn px(num: impl Tof32) -> Val {
    Val::Px(num.to_f32())
}

pub fn rgb8(r: u8, g: u8, b: u8) -> Color {
    Color::rgb_u8(r, g, b)
}

pub fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::rgba_u8(r, g, b, a)
}

pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::rgb(r, g, b)
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::rgba(r, g, b, a)
}

pub fn brgb8(r: u8, g: u8, b: u8) -> BackgroundColor {
    Color::rgb_u8(r, g, b).into()
}

pub fn brgba8(r: u8, g: u8, b: u8, a: u8) -> BackgroundColor {
    Color::rgba_u8(r, g, b, a).into()
}

pub fn brgb(r: f32, g: f32, b: f32) -> BackgroundColor {
    Color::rgb(r, g, b).into()
}

pub fn brgba(r: f32, g: f32, b: f32, a: f32) -> BackgroundColor {
    Color::rgba(r, g, b, a).into()
}


pub const fn auto() -> Val {
    Val::Auto
}

pub const fn undefined() -> Val {
    Val::Undefined
}

pub trait Tof32 {
    fn to_f32(self) -> f32;
}

impl Tof32 for f32 {
    fn to_f32(self) -> f32 {
        self
    }
}

impl Tof32 for f64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for i8 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for i16 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for i64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for i128 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for u8 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for u16 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for u32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for u64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl Tof32 for u128 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

/// Includes const background colors
pub mod bcolor {
    use bevy_render::color::Color;
    use bevy_ui::BackgroundColor;

    pub const ALICE_BLUE: BackgroundColor = BackgroundColor(Color::rgb(0.94, 0.97, 1.0));
    pub const ANTIQUE_WHITE: BackgroundColor = BackgroundColor(Color::rgb(0.98, 0.92, 0.84));
    pub const AQUAMARINE: BackgroundColor = BackgroundColor(Color::rgb(0.49, 1.0, 0.83));
    pub const AZURE: BackgroundColor = BackgroundColor(Color::rgb(0.94, 1.0, 1.0));
    pub const BEIGE: BackgroundColor = BackgroundColor(Color::rgb(0.96, 0.96, 0.86));
    pub const BISQUE: BackgroundColor = BackgroundColor(Color::rgb(1.0, 0.89, 0.77));
    pub const BLACK: BackgroundColor = BackgroundColor(Color::rgb(0.0, 0.0, 0.0));
    pub const BLUE: BackgroundColor = BackgroundColor(Color::rgb(0.0, 0.0, 1.0));
    pub const CRIMSON: BackgroundColor = BackgroundColor(Color::rgb(0.86, 0.08, 0.24));
    pub const CYAN: BackgroundColor = BackgroundColor(Color::rgb(0.0, 1.0, 1.0));
    pub const DARK_GRAY: BackgroundColor = BackgroundColor(Color::rgb(0.25, 0.25, 0.25));
    pub const DARK_GREEN: BackgroundColor = BackgroundColor(Color::rgb(0.0, 0.5, 0.0));
    pub const FUCHSIA: BackgroundColor = BackgroundColor(Color::rgb(1.0, 0.0, 1.0));
    pub const GOLD: BackgroundColor = BackgroundColor(Color::rgb(1.0, 0.84, 0.0));
    pub const GRAY: BackgroundColor = BackgroundColor(Color::rgb(0.5, 0.5, 0.5));
    pub const GREEN: BackgroundColor = BackgroundColor(Color::rgb(0.0, 1.0, 0.0));
    pub const INDIGO: BackgroundColor = BackgroundColor(Color::rgb(0.29, 0.0, 0.51));
    pub const LIME_GREEN: BackgroundColor = BackgroundColor(Color::rgb(0.2, 0.8, 0.2));
    pub const MAROON: BackgroundColor = BackgroundColor(Color::rgb(0.5, 0.0, 0.0));
    pub const MIDNIGHT_BLUE: BackgroundColor = BackgroundColor(Color::rgb(0.1, 0.1, 0.44));
    pub const NAVY: BackgroundColor = BackgroundColor(Color::rgb(0.0, 0.0, 0.5));
    pub const NONE: BackgroundColor = BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0));
    pub const OLIVE: BackgroundColor = BackgroundColor(Color::rgb(0.5, 0.5, 0.0));
    pub const ORANGE: BackgroundColor = BackgroundColor(Color::rgb(1.0, 0.65, 0.0));
    pub const ORANGE_RED: BackgroundColor = BackgroundColor(Color::rgb(1.0, 0.27, 0.0));
    pub const PINK: BackgroundColor = BackgroundColor(Color::rgb(1.0, 0.08, 0.58));
    pub const PURPLE: BackgroundColor = BackgroundColor(Color::rgb(0.5, 0.0, 0.5));
    pub const RED: BackgroundColor = BackgroundColor(Color::rgb(1.0, 0.0, 0.0));
    pub const SALMON: BackgroundColor = BackgroundColor(Color::rgb(0.98, 0.5, 0.45));
    pub const SEA_GREEN: BackgroundColor = BackgroundColor(Color::rgb(0.18, 0.55, 0.34));
    pub const SILVER: BackgroundColor = BackgroundColor(Color::rgb(0.75, 0.75, 0.75));
    pub const TEAL: BackgroundColor = BackgroundColor(Color::rgb(0.0, 0.5, 0.5));
    pub const TOMATO: BackgroundColor = BackgroundColor(Color::rgb(1.0, 0.39, 0.28));
    pub const TURQUOISE: BackgroundColor = BackgroundColor(Color::rgb(0.25, 0.88, 0.82));
    pub const VIOLET: BackgroundColor = BackgroundColor(Color::rgb(0.93, 0.51, 0.93));
    pub const WHITE: BackgroundColor = BackgroundColor(Color::rgb(1.0, 1.0, 1.0));
    pub const YELLOW: BackgroundColor = BackgroundColor(Color::rgb(1.0, 1.0, 0.0));
    pub const YELLOW_GREEN: BackgroundColor = BackgroundColor(Color::rgb(0.6, 0.8, 0.2));
}

/// Includes const colors
pub mod color {
    use bevy_render::color::Color;

    pub const ALICE_BLUE: Color = Color::rgb(0.94, 0.97, 1.0);
    pub const ANTIQUE_WHITE: Color = Color::rgb(0.98, 0.92, 0.84);
    pub const AQUAMARINE: Color = Color::rgb(0.49, 1.0, 0.83);
    pub const AZURE: Color = Color::rgb(0.94, 1.0, 1.0);
    pub const BEIGE: Color = Color::rgb(0.96, 0.96, 0.86);
    pub const BISQUE: Color = Color::rgb(1.0, 0.89, 0.77);
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
    pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
    pub const CRIMSON: Color = Color::rgb(0.86, 0.08, 0.24);
    pub const CYAN: Color = Color::rgb(0.0, 1.0, 1.0);
    pub const DARK_GRAY: Color = Color::rgb(0.25, 0.25, 0.25);
    pub const DARK_GREEN: Color = Color::rgb(0.0, 0.5, 0.0);
    pub const FUCHSIA: Color = Color::rgb(1.0, 0.0, 1.0);
    pub const GOLD: Color = Color::rgb(1.0, 0.84, 0.0);
    pub const GRAY: Color = Color::rgb(0.5, 0.5, 0.5);
    pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
    pub const INDIGO: Color = Color::rgb(0.29, 0.0, 0.51);
    pub const LIME_GREEN: Color = Color::rgb(0.2, 0.8, 0.2);
    pub const MAROON: Color = Color::rgb(0.5, 0.0, 0.0);
    pub const MIDNIGHT_BLUE: Color = Color::rgb(0.1, 0.1, 0.44);
    pub const NAVY: Color = Color::rgb(0.0, 0.0, 0.5);
    pub const NONE: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    pub const OLIVE: Color = Color::rgb(0.5, 0.5, 0.0);
    pub const ORANGE: Color = Color::rgb(1.0, 0.65, 0.0);
    pub const ORANGE_RED: Color = Color::rgb(1.0, 0.27, 0.0);
    pub const PINK: Color = Color::rgb(1.0, 0.08, 0.58);
    pub const PURPLE: Color = Color::rgb(0.5, 0.0, 0.5);
    pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
    pub const SALMON: Color = Color::rgb(0.98, 0.5, 0.45);
    pub const SEA_GREEN: Color = Color::rgb(0.18, 0.55, 0.34);
    pub const SILVER: Color = Color::rgb(0.75, 0.75, 0.75);
    pub const TEAL: Color = Color::rgb(0.0, 0.5, 0.5);
    pub const TOMATO: Color = Color::rgb(1.0, 0.39, 0.28);
    pub const TURQUOISE: Color = Color::rgb(0.25, 0.88, 0.82);
    pub const VIOLET: Color = Color::rgb(0.93, 0.51, 0.93);
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    pub const YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
    pub const YELLOW_GREEN: Color = Color::rgb(0.6, 0.8, 0.2);
}