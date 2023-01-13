use bevy_text::{TextStyle, TextSection};
use bevy_ecs::system::Commands;
use bevy_ui::node_bundles::{NodeBundle, TextBundle, ButtonBundle};
use bevy_hierarchy::{ChildBuilder, BuildChildren};


/// Spawns a [`NodeBundle`] as the root.
/// Also specifies children in the form of a callback.
pub fn node_root(
    commands: &mut Commands,
    mut style: impl FnMut() -> NodeBundle,
    children: impl FnMut(&mut ChildBuilder)
) {
    commands.spawn(style()).with_children(children);
}

/// Spawns a [`NodeBundle`] as a child of the parent specified.
pub fn node(
    parent: &mut ChildBuilder,
    mut style: impl FnMut() -> NodeBundle
) {
    parent.spawn(style());
}

/// Spawns a [`NodeBundle`] as a child of the parent specified.
/// Also specifies children in the form of a callback.
pub fn node_with(
    parent: &mut ChildBuilder,
    mut style: impl FnMut() -> NodeBundle,
    children: impl FnMut(&mut ChildBuilder)
) {
    parent.spawn(style()).with_children(children);
}

/// Spawns a [`TextBundle`] as a child of the parent specified.
pub fn text(
    parent: &mut ChildBuilder,
    text: &str,
    mut style: impl FnMut() -> TextBundle,
    mut font: impl FnMut() -> TextStyle
) {
    let mut bundle = style();
    let sections = &mut bundle.text.sections;
    sections.push(TextSection {
        value: text.to_string(),
        style: font(),
    });
    parent.spawn(bundle);
}

/// Spawns a [`ButtonBundle`] as a child of the parent specified.
/// Also specifies children in the form of a callback.
pub fn button_with(
    parent: &mut ChildBuilder,
    mut style: impl FnMut() -> ButtonBundle,
    children: impl FnMut(&mut ChildBuilder)
) {
    parent.spawn(style()).with_children(children);
}

/// Spawns a [`ButtonBundle`] as a child of the parent specified.
pub fn button(
    parent: &mut ChildBuilder,
    mut style: impl FnMut() -> ButtonBundle
) {
    parent.spawn(style());
}

/// Spawns a [`ButtonBundle`] as a child of the parent specified.
pub fn text_button<F: FnMut() -> TextStyle + Clone>(
    parent: &mut ChildBuilder,
    txt: &str,
    style: impl FnMut() -> ButtonBundle,
    font: F
) {
    button_with(parent, style, move |p| {
        let font = font.clone();
        text(p, txt, || TextBundle::default(), font)
    });
}
