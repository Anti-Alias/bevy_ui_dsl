//! This crate simplifies the process of creating widgets in bevy using a simple extensible DSL.

#[cfg(feature = "class_helpers")]
pub mod class_helpers;
mod widgets;

pub use widgets::*;
use bevy_asset::AssetServer;
use bevy_ecs::bundle::Bundle;
use bevy_ecs::entity::Entity;
use bevy_ecs::system::EntityCommands;
use bevy_hierarchy::{BuildChildren, ChildBuilder};
use bevy_text::TextStyle;
use bevy_ui::node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle};


/// Wrapper for [`ChildBuilder`] that also propogates an [`AssetServer`] for the children that need it.
pub struct UiChildBuilder<'a, 'b> {
    builder: &'a mut ChildBuilder<'b>,
    assets: &'a AssetServer,
}

impl<'a, 'b> UiChildBuilder<'a, 'b> {

    /// Create a new [`UiChildBuilder`] for adding to children of a node.
    pub fn new(builder: &'a mut ChildBuilder<'b>, assets: &'a AssetServer) -> Self {
        Self { builder, assets }
    }

    pub fn spawn<'c>(&'c mut self, bundle: impl Bundle) -> UiEntityCommands<'a, 'c> {
        let commands: EntityCommands<'c> = self.builder.spawn(bundle);
        UiEntityCommands {
            assets: self.assets,
            commands,
        }
    }
    pub fn assets(&self) -> &AssetServer {
        self.assets
    }
}

/// Wrapper for [`EntityCommands`] that also propagates an [`AssetServer`] for the children that need it.
pub struct UiEntityCommands<'a, 'b> {
    commands: EntityCommands<'b>,
    assets: &'a AssetServer
}

impl<'a, 'b> UiEntityCommands<'a, 'b> {
    pub fn id(&self) -> Entity {
        self.commands.id()
    }
    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.commands.insert(bundle);
        self
    }
    pub fn with_children(mut self, spawn_children: impl FnOnce(&mut UiChildBuilder)) -> Self {
        self.commands.with_children(move |builder| {
            let mut ui_builder = UiChildBuilder {
                assets: self.assets,
                builder,
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

impl<T> Class<T> for () {
    fn apply(self, _b: &mut T) {}
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
    F1: Class<B>,
    F2: Class<B>,
{
    fn apply(self, b: &mut B) {
        self.0.apply(b);
        self.1.apply(b);
    }
}

impl<F1, F2, F3, B> Class<B> for (F1, F2, F3)
where
    F1: Class<B>,
    F2: Class<B>,
    F3: Class<B>,
{
    fn apply(self, b: &mut B) {
        self.0.apply(b);
        self.1.apply(b);
        self.2.apply(b);
    }
}

impl<F1, F2, F3, F4, B> Class<B> for (F1, F2, F3, F4)
where
    F1: Class<B>,
    F2: Class<B>,
    F3: Class<B>,
    F4: Class<B>,
{
    fn apply(self, b: &mut B) {
        self.0.apply(b);
        self.1.apply(b);
        self.2.apply(b);
        self.3.apply(b);
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

impl<T> AssetClass<T> for () {
    fn apply(self, _a: &AssetServer, _b: &mut T) {}
}

impl<F, B> AssetClass<B> for F
where
    F: FnOnce(&AssetServer, &mut B),
{
    fn apply(self, a: &AssetServer, b: &mut B) {
        self(a, b);
    }
}

impl<F1, F2, B> AssetClass<B> for (F1, F2)
where
    F1: AssetClass<B>,
    F2: AssetClass<B>,
{
    fn apply(self, a: &AssetServer, b: &mut B) {
        self.0.apply(a, b);
        self.1.apply(a, b);
    }
}

impl<F1, F2, F3, B> AssetClass<B> for (F1, F2, F3)
where
    F1: AssetClass<B>,
    F2: AssetClass<B>,
    F3: AssetClass<B>,
{
    fn apply(self, a: &AssetServer, b: &mut B) {
        self.0.apply(a, b);
        self.1.apply(a, b);
        self.2.apply(a, b);
    }
}

impl<F1, F2, F3, F4, B> AssetClass<B> for (F1, F2, F3, F4)
where
    F1: AssetClass<B>,
    F2: AssetClass<B>,
    F3: AssetClass<B>,
    F4: AssetClass<B>,
{
    fn apply(self, a: &AssetServer, b: &mut B) {
        self.0.apply(a, b);
        self.1.apply(a, b);
        self.2.apply(a, b);
        self.3.apply(a, b);
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
    /// Copies this entity into an Option.
    fn set(self, entity: &mut Option<Entity>) {
        *entity = Some(self);
    }
    /// Pushes a copy of this Entity into a Vec.
    fn push(self, entities: &mut Vec<Entity>) {
        entities.push(self);
    }
}
