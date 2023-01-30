use bevy_asset::AssetServer;
use bevy_text::{TextStyle, TextSection};
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use bevy_ui::{Size, Val, FlexWrap, Style, JustifyContent, AlignItems};
use bevy_ui::node_bundles::{NodeBundle, TextBundle, ButtonBundle, ImageBundle};
use bevy_hierarchy::BuildChildren;
use super::{Class, AssetClass, UiChildBuilder};


/// Spawns a [`NodeBundle`] as the root with children.
pub fn root(
    class: impl Class<NodeBundle>,
    assets: &AssetServer,
    commands: &mut Commands,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle);
    commands
        .spawn(bundle)
        .with_children(|builder| {
            let mut builder = UiChildBuilder {
                builder,
                assets
            };
            children(&mut builder);
        })
        .id()
}

/// Spawns a [`NodeBundle`] with children.
pub fn node(
    class: impl Class<NodeBundle>,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle);
    parent.spawn(bundle).with_children(children).id()
}

/// Spawns a [`TextBundle`].
pub fn text(
    text: &str,
    class: impl AssetClass<TextBundle>,
    text_class: impl AssetClass<TextStyle>,
    parent: &mut UiChildBuilder
) -> Entity {
    let mut bundle = TextBundle::default();
    class.apply(parent.assets, &mut bundle);
    let sections = &mut bundle.text.sections;
    let mut style = TextStyle::default();
    text_class.apply(parent.assets, &mut style);
    sections.push(TextSection {
        value: text.to_string(),
        style,
    });
    parent.spawn(bundle).id()
}

/// Spawns a [`ButtonBundle`] with children.
pub fn button(
    class: impl AssetClass<ButtonBundle>,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    let mut bundle = ButtonBundle::default();
    class.apply(parent.assets, &mut bundle);
    parent
        .spawn(bundle)
        .with_children(children)
        .id()
}

/// Spawns a [`ButtonBundle`] without children.
pub fn simple_button(
    class: impl AssetClass<ButtonBundle>,
    parent: &mut UiChildBuilder
) -> Entity {
    let mut bundle = ButtonBundle::default();
    class.apply(parent.assets, &mut bundle);
    parent.spawn(bundle).id()
}

/// Spawns a [`ButtonBundle`] with a single [`TextBundle`] as its child.
pub fn text_button(
    txt: &str,
    class: impl AssetClass<ButtonBundle>,
    text_style: impl AssetClass<TextStyle>,
    parent: &mut UiChildBuilder
) -> Entity {
    fn c_text(_a: &AssetServer,_b: &mut TextBundle) {}       // No need to overwrite the default!
    button(class, parent, |p| {
        text(txt, c_text, text_style, p);
    })
}

/// Spawns an [`ImageBundle`].
pub fn image(
    class: impl AssetClass<ImageBundle>,
    parent: &mut UiChildBuilder
) -> Entity {
    let mut bundle = ImageBundle::default();
    class.apply(parent.assets, &mut bundle);
    parent.spawn(bundle).id()
}

/// Spawns an [`ImageBundle`] with children.
pub fn image_pane(
    class: impl AssetClass<ImageBundle>,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    let mut bundle = ImageBundle::default();
    class.apply(parent.assets, &mut bundle);
    parent
        .spawn(bundle)
        .with_children(children).id()
}

/// Spawns a [`NodeBundle`] composed of [`NodeBundle`] cells in the form of a grid.
/// The callback function argument spawns the contents of those cells.
pub fn grid(
    rows: usize,
    columns: usize,
    class: impl Class<NodeBundle>,
    parent: &mut UiChildBuilder,
    mut children: impl FnMut(&mut UiChildBuilder, usize, usize)
) -> Entity {
    // Spawns container
    let mut container_bundle = NodeBundle::default();
    class.apply(&mut container_bundle);
    container_bundle.style.flex_wrap = FlexWrap::Wrap;
    let mut container = parent.spawn(container_bundle);

    // Spawns cells as children of the container
    let cell_bundle = NodeBundle {
        style: Style {
            size: Size::new(
                Val::Percent(100.0 / columns as f32),
                Val::Percent(100.0 / rows as f32)
            ),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    };
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