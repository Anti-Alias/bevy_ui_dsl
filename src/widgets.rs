use bevy_asset::AssetServer;
use bevy_ecs::prelude::Bundle;
use bevy_text::{TextStyle, TextSection};
use bevy_ecs::entity::Entity;
use bevy_ecs::system::Commands;
use bevy_ui::{Val, FlexWrap, Style, JustifyContent, AlignItems};
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
    rooti(class, assets, commands, (), children)
}

/// Spawns a [`NodeBundle`] as the root with children.
pub fn rooti(
    class: impl Class<NodeBundle>,
    assets: &AssetServer,
    commands: &mut Commands,
    extras: impl Bundle,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle);
    commands
        .spawn((bundle, extras))
        .with_children(|builder| {
            children(&mut UiChildBuilder {
                builder,
                assets
            });
        })
        .id()
}


/// Spawns a clear [`NodeBundle`] that takes up the full space of its parent.
/// Often required for embedding other widgets after the initial widget is spawned.
pub fn blank(
    parent: Entity,
    class: impl Class<NodeBundle>,
    assets: &AssetServer,
    commands: &mut Commands,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    blanki(parent, class, assets, commands, (), children)
}

/// Spawns a clear [`NodeBundle`] that takes up the full space of its parent.
/// Often required for embedding other widgets after the initial widget is spawned.
pub fn blanki(
    parent: Entity,
    class: impl Class<NodeBundle>,
    assets: &AssetServer,
    commands: &mut Commands,
    extras: impl Bundle,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    commands
        .entity(parent)
        .with_children(|builder| {
            let mut bundle = NodeBundle::default();
            class.apply(&mut bundle);
            let mut builder = UiChildBuilder { builder, assets };
            builder.spawn((bundle, extras)).with_children(children);
        })
        .id()
}

/// Spawns a [`NodeBundle`] with children.
pub fn node(
    class: impl Class<NodeBundle>,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    nodei(class, (), parent, children)
}


/// Spawns a [`NodeBundle`] with children.
pub fn nodei(
    class: impl Class<NodeBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    let mut bundle = NodeBundle::default();
    class.apply(&mut bundle);
    
    let mut commands = parent.spawn(bundle);
    commands.insert(extras);
    commands.with_children(children).id()
}

/// Spawns a [`TextBundle`].
pub fn text(
    text: impl Into<String>,
    class: impl AssetClass<TextBundle>,
    text_class: impl AssetClass<TextStyle>,
    parent: &mut UiChildBuilder
) -> Entity {
    texti(text, class, text_class, (), parent)
}

/// Spawns a [`TextBundle`].
pub fn texti(
    text: impl Into<String>,
    class: impl AssetClass<TextBundle>,
    text_class: impl AssetClass<TextStyle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder
) -> Entity {
    let mut bundle = TextBundle::default();
    class.apply(parent.assets, &mut bundle);
    let sections = &mut bundle.text.sections;
    let mut style = TextStyle::default();
    text_class.apply(parent.assets, &mut style);
    sections.push(TextSection {
        value: text.into(),
        style,
    });
    parent.spawn((bundle, extras)).id()
}

/// Spawns a [`ButtonBundle`] with children.
pub fn button(
    class: impl AssetClass<ButtonBundle>,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    buttoni(class, (), parent, children)
}

/// Spawns a [`ButtonBundle`] with children.
pub fn buttoni(
    class: impl AssetClass<ButtonBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    let mut bundle = ButtonBundle::default();
    class.apply(parent.assets, &mut bundle);
    parent
        .spawn((bundle, extras))
        .with_children(children).id()
}

/// Spawns a [`ButtonBundle`] without children.
pub fn simple_button(
    class: impl AssetClass<ButtonBundle>,
    parent: &mut UiChildBuilder
) -> Entity {
    simple_buttoni(class, (), parent)
}

/// Spawns a [`ButtonBundle`] without children.
pub fn simple_buttoni(
    class: impl AssetClass<ButtonBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder
) -> Entity {
    let mut bundle = ButtonBundle::default();
    class.apply(parent.assets, &mut bundle);
    parent.spawn((bundle, extras)).id()
}

/// Spawns a [`ButtonBundle`] with a single [`TextBundle`] as its child.
pub fn text_button(
    txt: impl Into<String>,
    class: impl AssetClass<ButtonBundle>,
    text_style: impl AssetClass<TextStyle>,
    parent: &mut UiChildBuilder
) -> Entity {
    text_buttoni(txt, class, text_style, (), parent)
}

/// Spawns a [`ButtonBundle`] with a single [`TextBundle`] as its child.
pub fn text_buttoni(
    txt: impl Into<String>,
    class: impl AssetClass<ButtonBundle>,
    text_style: impl AssetClass<TextStyle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder
) -> Entity {
    buttoni(class, extras, parent, |p| {
        text(txt, (), text_style, p);
    })
}

/// Spawns an [`ImageBundle`].
pub fn image(
    class: impl AssetClass<ImageBundle>,
    parent: &mut UiChildBuilder
) -> Entity {
    imagei(class, (), parent)
}

/// Spawns an [`ImageBundle`].
pub fn imagei(
    class: impl AssetClass<ImageBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder
) -> Entity {
    let mut bundle = ImageBundle::default();
    class.apply(parent.assets, &mut bundle);
    parent.spawn((bundle, extras)).id()
}

/// Spawns an [`ImageBundle`] with children.
pub fn image_pane(
    class: impl AssetClass<ImageBundle>,
    parent: &mut UiChildBuilder,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    image_panei(class, parent, (), children)
}

/// Spawns an [`ImageBundle`] with children.
pub fn image_panei(
    class: impl AssetClass<ImageBundle>,
    parent: &mut UiChildBuilder,
    extras: impl Bundle,
    children: impl FnOnce(&mut UiChildBuilder)
) -> Entity {
    let mut bundle = ImageBundle::default();
    class.apply(parent.assets, &mut bundle);
    parent
        .spawn((bundle, extras))
        .with_children(children).id()
}

/// Spawns a [`NodeBundle`] composed of [`NodeBundle`] cells in the form of a grid.
/// The callback function argument spawns the contents of those cells.
pub fn grid(
    rows: usize,
    columns: usize,
    class: impl Class<NodeBundle>,
    parent: &mut UiChildBuilder,
    children: impl FnMut(&mut UiChildBuilder, usize, usize)
) -> Entity {
    gridi(rows, columns, class, (), parent, children)
}

/// Spawns a [`NodeBundle`] composed of [`NodeBundle`] cells in the form of a grid.
/// The callback function argument spawns the contents of those cells.
pub fn gridi(
    rows: usize,
    columns: usize,
    class: impl Class<NodeBundle>,
    extras: impl Bundle,
    parent: &mut UiChildBuilder,
    mut children: impl FnMut(&mut UiChildBuilder, usize, usize)
) -> Entity {
    // Spawns container
    let mut container_bundle = NodeBundle::default();
    class.apply(&mut container_bundle);
    container_bundle.style.flex_wrap = FlexWrap::Wrap;
    let mut container = parent.spawn((container_bundle, extras));

    // Spawns cells as children of the container
    let cell_bundle = NodeBundle {
        style: Style {
            width: Val::Percent(100.0 / columns as f32),
            height: Val::Percent(100.0 / rows as f32),
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
