use bevy_asset::AssetServer;
use bevy_text::{TextStyle, TextSection};
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use bevy_ui::{Size, Val, FlexWrap};
use bevy_ui::node_bundles::{NodeBundle, TextBundle, ButtonBundle, ImageBundle};
use bevy_hierarchy::BuildChildren;
use super::UiChildBuilder;


/// Spawns a [`NodeBundle`] as the root with children.
pub fn root<'w, 's>(
    class: impl FnOnce() -> NodeBundle,
    assets: &AssetServer,
    commands: &mut Commands,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    commands
        .spawn(class())
        .with_children(|builder| {
            let mut builder = UiChildBuilder {
                builder,
                assets
            };
            children(&mut builder);
        })
        .id()
}

/// Spawns a [`NodeBundle`] without children.
pub fn rect(
    parent: &mut UiChildBuilder,
    class: impl FnOnce() -> NodeBundle
) -> Entity {
    parent.spawn(class()).id()
}

/// Spawns a [`NodeBundle`] with children.
pub fn node(
    class: impl FnOnce() -> NodeBundle,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    parent.spawn(class()).with_children(children).id()
}

/// Spawns a [`TextBundle`].
pub fn text(
    text: &str,
    class: impl FnOnce() -> TextBundle,
    text_style: impl FnOnce(&AssetServer) -> TextStyle,
    parent: &mut UiChildBuilder
) -> Entity {
    let mut bundle = class();
    let sections = &mut bundle.text.sections;
    sections.push(TextSection {
        value: text.to_string(),
        style: text_style(parent.assets),
    });
    parent.spawn(bundle).id()
}

/// Spawns a [`ButtonBundle`] with children.
pub fn button(
    class: impl FnOnce(&AssetServer) -> ButtonBundle,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    parent
        .spawn(class(parent.assets))
        .with_children(children)
        .id()
}

/// Spawns a [`ButtonBundle`] without children.
pub fn simple_button(
    parent: &mut UiChildBuilder,
    class: impl FnOnce() -> ButtonBundle
) -> Entity {
    parent.spawn(class()).id()
}

/// Spawns a [`ButtonBundle`] with a single [`TextBundle`] as its child.
pub fn text_button(
    txt: &str,
    class: impl FnOnce(&AssetServer) -> ButtonBundle,
    text_style: impl FnOnce(&AssetServer) -> TextStyle,
    parent: &mut UiChildBuilder
) -> Entity {
    button(class, parent, |p| {
        let text_bundle = || TextBundle::default();
        text(txt, text_bundle, text_style, p);
    })
}

/// Spawns an [`ImageBundle`] with children.
pub fn image(
    class: impl FnOnce(&AssetServer) -> ImageBundle,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    parent
        .spawn(class(parent.assets))
        .with_children(children).id()
}

/// Spawns an [`ImageBundle`] without children.
pub fn simple_image(
    class: impl FnOnce(&AssetServer) -> ImageBundle,
    parent: &mut UiChildBuilder
) -> Entity {
    parent.spawn(class(parent.assets)).id()
}

/// Spawns a [`NodeBundle`] which children [`NodeBundle`]s acting as the cells of a grid.
/// The callback allows for spawing children in those cells.
pub fn grid(
    rows: usize,
    columns: usize,
    class: impl FnOnce() -> NodeBundle,
    parent: &mut UiChildBuilder,
    mut children: impl FnMut(&mut UiChildBuilder, usize, usize)
) -> Entity {
    // Spawns container
    let mut container_bundle = class();
    container_bundle.style.flex_wrap = FlexWrap::Wrap;
    let mut container = parent.spawn(container_bundle);

    // Spawns cells as children of the container
    let mut cell_bundle = NodeBundle::default();
    cell_bundle.style.size = Size::new(
        Val::Percent(100.0 / rows as f32),
        Val::Percent(100.0 / columns as f32)
    );
    for row in 0..rows {
        for col in 0..columns {
            container = container.with_children(|container| {
                container
                    .spawn(cell_bundle.clone())
                    .with_children(|cell| children(cell, row, col));
            });
        }
    }
    container.id()
}