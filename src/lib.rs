//! This crate simplifies the process of creating widgets in bevy using a simple extensible DSL.

mod widgets;
#[cfg(feature = "class_helpers")]
pub mod class_helpers;

use bevy_text::TextStyle;
pub use widgets::*;
use bevy_ui::node_bundles::{NodeBundle, ImageBundle, TextBundle, ButtonBundle};
use bevy_asset::AssetServer;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::EntityCommands;
use bevy_ecs::bundle::Bundle;
use bevy_hierarchy::{ChildBuilder, BuildChildren};


/// Wrapper for [`ChildBuilder`] that also propogates an [`AssetServer`] for the children that need it.
// It has enough ' for a lifetime ;)
pub struct UiChildBuilder<'a, 'b, 'c, 'd> {
    builder: &'a mut ChildBuilder<'b, 'c, 'd>,
    assets: &'a AssetServer
}

impl<'a, 'b, 'c, 'd> UiChildBuilder<'a, 'b, 'c, 'd> {
    pub fn spawn(&mut self, bundle: impl Bundle) -> UiEntityCommands<'a, 'b, 'c, '_> {
        let commands: EntityCommands<'b, 'c, '_> = self.builder.spawn(bundle);
        UiEntityCommands {
            assets: self.assets,
            commands
        }
    }
    pub fn assets(&self) -> &AssetServer { self.assets }
}

/// Wrapper for [`EntityCommands`] that also propagates an [`AssetServer`] for the children that need it.
pub struct UiEntityCommands<'a, 'b, 'c, 'd> {
    commands: EntityCommands<'b, 'c, 'd>,
    assets: &'a AssetServer
}

impl<'a, 'b, 'c, 'd> UiEntityCommands<'a, 'b, 'c, 'd> {
    pub fn id(&self) -> Entity {
        self.commands.id()
    }
    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.commands.insert(bundle);
        self
    }
    pub fn with_children(mut self, spawn_children: impl FnOnce(&mut UiChildBuilder)) -> Self {
        self.commands.with_children(|builder| {
            let mut ui_builder = UiChildBuilder {
                assets: self.assets,
                builder
            };
            spawn_children(&mut ui_builder);
        });
        self
    }
}

/// Something that can overwrite a value, typically a node bundle.
pub trait Class<B> {
    fn apply(self, b: &mut B);
}

impl<F, B> Class<B> for F
where
    F: FnOnce(&mut B),
{
    fn apply(self, b: &mut B) {
        self(b);
    }
}

impl<F1, F2, B> Class<B> for (F1, F2)
where
    F1: FnOnce(&mut B),
    F2: FnOnce(&mut B)
{
    fn apply(self, b: &mut B) {
        self.0(b);
        self.1(b);
    }
}

impl<F1, F2, F3, B> Class<B> for (F1, F2, F3)
where
    F1: FnOnce(&mut B),
    F2: FnOnce(&mut B),
    F3: FnOnce(&mut B)
{
    fn apply(self, b: &mut B) {
        self.0(b);
        self.1(b);
        self.2(b);
    }
}

impl<F1, F2, F3, F4, B> Class<B> for (F1, F2, F3, F4)
where
    F1: FnOnce(&mut B),
    F2: FnOnce(&mut B),
    F3: FnOnce(&mut B),
    F4: FnOnce(&mut B)
{
    fn apply(self, b: &mut B) {
        self.0(b);
        self.1(b);
        self.2(b);
        self.3(b);
    }
}

impl Class<NodeBundle> for NodeBundle {
    fn apply(self, b: &mut NodeBundle) {
        *b = self;
    }
}

impl Class<ImageBundle> for ImageBundle {
    fn apply(self, b: &mut ImageBundle) {
        *b = self;
    }
}



/// Something that can overwrite a value, typically a node bundle.
/// Depends on an [`AssetServer`], unlike [`Class`].
pub trait AssetClass<B> {
    fn apply(self, assets: &AssetServer, b: &mut B);
}

impl<F, B> AssetClass<B> for F
where
    F: FnOnce(&AssetServer, &mut B)
{
    fn apply(self, a: &AssetServer, b: &mut B) {
        self(a, b);
    }
}

impl<F1, F2, B> AssetClass<B> for (F1, F2)
where
    F1: FnOnce(&AssetServer, &mut B),
    F2: FnOnce(&AssetServer, &mut B)
{
    fn apply(self, a: &AssetServer, b: &mut B) {
        self.0(a, b);
        self.1(a, b);
    }
}

impl<F1, F2, F3, B> AssetClass<B> for (F1, F2, F3)
where
    F1: FnOnce(&AssetServer, &mut B),
    F2: FnOnce(&AssetServer, &mut B),
    F3: FnOnce(&AssetServer, &mut B)
{
    fn apply(self, a: &AssetServer, b: &mut B) {
        self.0(a, b);
        self.1(a, b);
        self.2(a, b);
    }
}

impl<F1, F2, F3, F4, B> AssetClass<B> for (F1, F2, F3, F4)
where
    F1: FnOnce(&AssetServer, &mut B),
    F2: FnOnce(&AssetServer, &mut B),
    F3: FnOnce(&AssetServer, &mut B),
    F4: FnOnce(&AssetServer, &mut B)
{
    fn apply(self, a: &AssetServer, b: &mut B) {
        self.0(a, b);
        self.1(a, b);
        self.2(a, b);
        self.3(a, b);
    }
}

impl AssetClass<ButtonBundle> for ButtonBundle {
    fn apply(self, _a: &AssetServer, b: &mut ButtonBundle) {
        *b = self;
    }
}

impl AssetClass<TextBundle> for TextBundle {
    fn apply(self, _a: &AssetServer, b: &mut TextBundle) {
        *b = self;
    }
}

impl AssetClass<TextStyle> for TextStyle {
    fn apply(self, _a: &AssetServer, b: &mut TextStyle) {
        *b = self;
    }
}

/// Adds a helper method to [`Entity`] that allows it to be sent to an [`Option`][`Entity`]
/// ergonomically.
pub trait EntityWriter {
    fn set(self, entity: &mut Option<Entity>);
    fn push(self, destination: &mut Vec<Entity>);
}

impl EntityWriter for Entity {
    fn set(self, entity: &mut Option<Entity>) {
        *entity = Some(self);
    }
    fn push(self, destination: &mut Vec<Entity>) {
        destination.push(self);
    }
}