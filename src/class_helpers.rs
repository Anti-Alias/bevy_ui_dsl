//! A module that provides helper constants and functions that make creating classes simpler.
//! Feature flag 'class_helpers' must be enabled to use this module.

use bevy_color::Color;
use bevy_ui::{
    AlignContent, AlignItems, AlignSelf, BackgroundColor, Direction, Display, FlexDirection,
    FlexWrap, JustifyContent, Overflow, OverflowAxis, PositionType, UiRect, Val,
};

pub const ALIGN_FLEX_START: AlignItems = AlignItems::FlexStart;
pub const ALIGN_FLEX_END: AlignItems = AlignItems::FlexEnd;
pub const ALIGN_CENTER: AlignItems = AlignItems::Center;
pub const ALIGN_BASELINE: AlignItems = AlignItems::Baseline;
pub const ALIGN_STRETCH: AlignItems = AlignItems::Stretch;

pub const ALIGN_SELF_AUTO: AlignSelf = AlignSelf::FlexStart;
pub const ALIGN_SELF_FLEX_START: AlignSelf = AlignSelf::FlexEnd;
pub const ALIGN_SELF_FLEX_END: AlignSelf = AlignSelf::FlexEnd;
pub const ALIGN_SELF_CENTER: AlignSelf = AlignSelf::Center;
pub const ALIGN_SELF_BASELINE: AlignSelf = AlignSelf::Baseline;
pub const ALIGN_SELF_STRETCH: AlignSelf = AlignSelf::Stretch;

pub const ALIGN_CONTENT_AUTO: AlignContent = AlignContent::FlexStart;
pub const ALIGN_CONTENT_FLEX_START: AlignContent = AlignContent::FlexEnd;
pub const ALIGN_CONTENT_FLEX_END: AlignContent = AlignContent::FlexEnd;
pub const ALIGN_CONTENT_CENTER: AlignContent = AlignContent::Center;
pub const ALIGN_CONTENT_STRETCH: AlignContent = AlignContent::Stretch;
pub const ALIGN_CONTENT_SPACE_BETWEEN: AlignContent = AlignContent::SpaceBetween;
pub const ALIGN_CONTENT_SPACE_AROUND: AlignContent = AlignContent::SpaceAround;

pub const INHERIT: Direction = Direction::Inherit;
pub const LEFT_TO_RIGHT: Direction = Direction::LeftToRight;
pub const RIGHT_TO_LEFT: Direction = Direction::RightToLeft;

pub const FLEX: Display = Display::Flex;
pub const NONE: Display = Display::None;

pub const ROW: FlexDirection = FlexDirection::Row;
pub const COLUMN: FlexDirection = FlexDirection::Column;
pub const ROW_REVERSE: FlexDirection = FlexDirection::RowReverse;
pub const COLUMN_REVERSE: FlexDirection = FlexDirection::ColumnReverse;

pub const JUSTIFY_FLEX_START: JustifyContent = JustifyContent::FlexStart;
pub const JUSTIFY_FLEX_END: JustifyContent = JustifyContent::FlexEnd;
pub const JUSTIFY_CENTER: JustifyContent = JustifyContent::Center;
pub const JUSTIFY_SPACE_BETWEEN: JustifyContent = JustifyContent::SpaceBetween;
pub const JUSTIFY_SPACE_AROUND: JustifyContent = JustifyContent::SpaceAround;
pub const JUSTIFY_SPACE_EVENLY: JustifyContent = JustifyContent::SpaceEvenly;

pub const VISIBLE: Overflow = Overflow {
    x: OverflowAxis::Visible,
    y: OverflowAxis::Visible,
};
pub const CLIP: Overflow = Overflow {
    x: OverflowAxis::Clip,
    y: OverflowAxis::Clip,
};

pub const RELATIVE: PositionType = PositionType::Relative;
pub const ABSOLUTE: PositionType = PositionType::Absolute;

pub const NO_WRAP: FlexWrap = FlexWrap::NoWrap;
pub const WRAP: FlexWrap = FlexWrap::Wrap;

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
    Color::srgb_u8(r, g, b)
}

pub fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::srgba_u8(r, g, b, a)
}

pub fn rgb(r: f32, g: f32, b: f32) -> Color {
    Color::srgb(r, g, b)
}

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::srgba(r, g, b, a)
}

pub fn brgb8(r: u8, g: u8, b: u8) -> BackgroundColor {
    Color::srgb_u8(r, g, b).into()
}

pub fn brgba8(r: u8, g: u8, b: u8, a: u8) -> BackgroundColor {
    Color::srgba_u8(r, g, b, a).into()
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
    use bevy_color::Color;
    use bevy_ui::BackgroundColor;

    pub const ALICE_BLUE: BackgroundColor = BackgroundColor(Color::srgb(0.94, 0.97, 1.0));
    pub const ANTIQUE_WHITE: BackgroundColor = BackgroundColor(Color::srgb(0.98, 0.92, 0.84));
    pub const AQUAMARINE: BackgroundColor = BackgroundColor(Color::srgb(0.49, 1.0, 0.83));
    pub const AZURE: BackgroundColor = BackgroundColor(Color::srgb(0.94, 1.0, 1.0));
    pub const BEIGE: BackgroundColor = BackgroundColor(Color::srgb(0.96, 0.96, 0.86));
    pub const BISQUE: BackgroundColor = BackgroundColor(Color::srgb(1.0, 0.89, 0.77));
    pub const BLACK: BackgroundColor = BackgroundColor(Color::srgb(0.0, 0.0, 0.0));
    pub const BLUE: BackgroundColor = BackgroundColor(Color::srgb(0.0, 0.0, 1.0));
    pub const CRIMSON: BackgroundColor = BackgroundColor(Color::srgb(0.86, 0.08, 0.24));
    pub const CYAN: BackgroundColor = BackgroundColor(Color::srgb(0.0, 1.0, 1.0));
    pub const DARK_GRAY: BackgroundColor = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
    pub const DARK_GREEN: BackgroundColor = BackgroundColor(Color::srgb(0.0, 0.5, 0.0));
    pub const FUCHSIA: BackgroundColor = BackgroundColor(Color::srgb(1.0, 0.0, 1.0));
    pub const GOLD: BackgroundColor = BackgroundColor(Color::srgb(1.0, 0.84, 0.0));
    pub const GRAY: BackgroundColor = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
    pub const GREEN: BackgroundColor = BackgroundColor(Color::srgb(0.0, 1.0, 0.0));
    pub const INDIGO: BackgroundColor = BackgroundColor(Color::srgb(0.29, 0.0, 0.51));
    pub const LIME_GREEN: BackgroundColor = BackgroundColor(Color::srgb(0.2, 0.8, 0.2));
    pub const MAROON: BackgroundColor = BackgroundColor(Color::srgb(0.5, 0.0, 0.0));
    pub const MIDNIGHT_BLUE: BackgroundColor = BackgroundColor(Color::srgb(0.1, 0.1, 0.44));
    pub const NAVY: BackgroundColor = BackgroundColor(Color::srgb(0.0, 0.0, 0.5));
    pub const NONE: BackgroundColor = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0));
    pub const OLIVE: BackgroundColor = BackgroundColor(Color::srgb(0.5, 0.5, 0.0));
    pub const ORANGE: BackgroundColor = BackgroundColor(Color::srgb(1.0, 0.65, 0.0));
    pub const ORANGE_RED: BackgroundColor = BackgroundColor(Color::srgb(1.0, 0.27, 0.0));
    pub const PINK: BackgroundColor = BackgroundColor(Color::srgb(1.0, 0.08, 0.58));
    pub const PURPLE: BackgroundColor = BackgroundColor(Color::srgb(0.5, 0.0, 0.5));
    pub const RED: BackgroundColor = BackgroundColor(Color::srgb(1.0, 0.0, 0.0));
    pub const SALMON: BackgroundColor = BackgroundColor(Color::srgb(0.98, 0.5, 0.45));
    pub const SEA_GREEN: BackgroundColor = BackgroundColor(Color::srgb(0.18, 0.55, 0.34));
    pub const SILVER: BackgroundColor = BackgroundColor(Color::srgb(0.75, 0.75, 0.75));
    pub const TEAL: BackgroundColor = BackgroundColor(Color::srgb(0.0, 0.5, 0.5));
    pub const TOMATO: BackgroundColor = BackgroundColor(Color::srgb(1.0, 0.39, 0.28));
    pub const TURQUOISE: BackgroundColor = BackgroundColor(Color::srgb(0.25, 0.88, 0.82));
    pub const VIOLET: BackgroundColor = BackgroundColor(Color::srgb(0.93, 0.51, 0.93));
    pub const WHITE: BackgroundColor = BackgroundColor(Color::srgb(1.0, 1.0, 1.0));
    pub const YELLOW: BackgroundColor = BackgroundColor(Color::srgb(1.0, 1.0, 0.0));
    pub const YELLOW_GREEN: BackgroundColor = BackgroundColor(Color::srgb(0.6, 0.8, 0.2));
}

/// Includes const colors
pub mod color {
    use bevy_color::Color;

    pub const ALICE_BLUE: Color = Color::srgb(0.94, 0.97, 1.0);
    pub const ANTIQUE_WHITE: Color = Color::srgb(0.98, 0.92, 0.84);
    pub const AQUAMARINE: Color = Color::srgb(0.49, 1.0, 0.83);
    pub const AZURE: Color = Color::srgb(0.94, 1.0, 1.0);
    pub const BEIGE: Color = Color::srgb(0.96, 0.96, 0.86);
    pub const BISQUE: Color = Color::srgb(1.0, 0.89, 0.77);
    pub const BLACK: Color = Color::srgb(0.0, 0.0, 0.0);
    pub const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
    pub const CRIMSON: Color = Color::srgb(0.86, 0.08, 0.24);
    pub const CYAN: Color = Color::srgb(0.0, 1.0, 1.0);
    pub const DARK_GRAY: Color = Color::srgb(0.25, 0.25, 0.25);
    pub const DARK_GREEN: Color = Color::srgb(0.0, 0.5, 0.0);
    pub const FUCHSIA: Color = Color::srgb(1.0, 0.0, 1.0);
    pub const GOLD: Color = Color::srgb(1.0, 0.84, 0.0);
    pub const GRAY: Color = Color::srgb(0.5, 0.5, 0.5);
    pub const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
    pub const INDIGO: Color = Color::srgb(0.29, 0.0, 0.51);
    pub const LIME_GREEN: Color = Color::srgb(0.2, 0.8, 0.2);
    pub const MAROON: Color = Color::srgb(0.5, 0.0, 0.0);
    pub const MIDNIGHT_BLUE: Color = Color::srgb(0.1, 0.1, 0.44);
    pub const NAVY: Color = Color::srgb(0.0, 0.0, 0.5);
    pub const NONE: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);
    pub const OLIVE: Color = Color::srgb(0.5, 0.5, 0.0);
    pub const ORANGE: Color = Color::srgb(1.0, 0.65, 0.0);
    pub const ORANGE_RED: Color = Color::srgb(1.0, 0.27, 0.0);
    pub const PINK: Color = Color::srgb(1.0, 0.08, 0.58);
    pub const PURPLE: Color = Color::srgb(0.5, 0.0, 0.5);
    pub const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    pub const SALMON: Color = Color::srgb(0.98, 0.5, 0.45);
    pub const SEA_GREEN: Color = Color::srgb(0.18, 0.55, 0.34);
    pub const SILVER: Color = Color::srgb(0.75, 0.75, 0.75);
    pub const TEAL: Color = Color::srgb(0.0, 0.5, 0.5);
    pub const TOMATO: Color = Color::srgb(1.0, 0.39, 0.28);
    pub const TURQUOISE: Color = Color::srgb(0.25, 0.88, 0.82);
    pub const VIOLET: Color = Color::srgb(0.93, 0.51, 0.93);
    pub const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
    pub const YELLOW: Color = Color::srgb(1.0, 1.0, 0.0);
    pub const YELLOW_GREEN: Color = Color::srgb(0.6, 0.8, 0.2);
}
