//! This crate simplifies the process of creating widgets in bevy using a simple extensible DSL.

mod widgets;
#[cfg(feature = "class_helpers")]
pub mod class_helpers;

use bevy_text::TextStyle;
pub use widgets::*;
use bevy_ui::node_bundles::{NodeBundle, ImageBundle, AtlasImageBundle, TextBundle, ButtonBundle};
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

impl<B> Class<B> for () {
    fn apply(self, _b: &mut B) {}
}

impl<F, B> Class<B> for F
where
    F: FnOnce(&mut B),
{
    fn apply(self, b: &mut B) {
        self(b);
    }
}

impl<C1, C2, B> Class<B> for (C1, C2)
where
    C1: Class<B>,
    C2: Class<B>,
{
    fn apply(self, b: &mut B) {
        self.0.apply(b);
        self.1.apply(b);
    }
}

impl<C1, C2, C3, B> Class<B> for (C1, C2, C3)
where
    C1: Class<B>,
    C2: Class<B>,
    C3: Class<B>,
{
    fn apply(self, b: &mut B) {
        self.0.apply(b);
        self.1.apply(b);
        self.2.apply(b);
    }
}

impl<C1, C2, C3, C4, B> Class<B> for (C1, C2, C3, C4)
where
    C1: Class<B>,
    C2: Class<B>,
    C3: Class<B>,
    C4: Class<B>,
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

impl Class<AtlasImageBundle> for AtlasImageBundle {
    fn apply(self, b: &mut AtlasImageBundle) {
        *b = self;
    }
}

impl Class<TextBundle> for TextBundle {
    fn apply(self, b: &mut TextBundle) {
        *b = self;
    }
}

impl Class<ButtonBundle> for ButtonBundle {
    fn apply(self, b: &mut ButtonBundle) {
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
    F: FnOnce(&AssetServer, &mut B)
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



/// Adds a helper method to [`Entity`] that allows it to be sent to some container of an [`Entity`] ergonomically.
pub trait EntityWriter<E> {
    fn set(self, entity: &mut E);
}

/// Adds a helper method to [`Entity`] that allows it to be pushed to a Vec of entities ergonomically.
pub trait EntityPusher {
    fn push(self, destination: &mut Vec<Entity>);
}

impl EntityWriter<Option<Entity>> for Entity {
    fn set(self, entity: &mut Option<Entity>) {
        *entity = Some(self);
    }
}

impl EntityWriter<Entity> for Entity {
    fn set(self, entity: &mut Entity) {
        *entity = self;
    }
}

impl EntityPusher for Entity {
    fn push(self, entities: &mut Vec<Entity>) {
        entities.push(self);
    }
}


/// Allows for easy injection of a UI to an existing [`Entity`].
pub trait BuildUiChildren {
    fn with_ui_children(&mut self, asset: &AssetServer, f: impl FnOnce(&mut UiChildBuilder)) -> &mut Self;
}

impl<'w, 's, 'a> BuildUiChildren for EntityCommands<'w, 's, 'a> {
    fn with_ui_children(
        &mut self,
        assets: &AssetServer,
        f: impl FnOnce(&mut UiChildBuilder)
    ) -> &mut Self {
        self.with_children(|builder| {
            let mut p = UiChildBuilder { builder, assets: &assets };
            f(&mut p);
        })
    }
}
