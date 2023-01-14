mod widgets;
pub use widgets::*;

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
    fn with_children(mut self, spawn_children: impl FnOnce(&mut UiChildBuilder)) -> Self {
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