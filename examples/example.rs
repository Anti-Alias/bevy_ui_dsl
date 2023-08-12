mod classes;

use bevy::prelude::*;
use bevy_ui_dsl::*;
use classes::*;

/// Marker component for specific entities in the UI
#[derive(Component, Debug)]
enum UiId {
    HiyaButton,
    HowdyButton
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))    // Nearest neighbor scaling for pixel graphics!
        .add_systems(Startup, startup)
        .add_systems(Update, handle_interactions)
        .run();
}

fn startup(mut commands: Commands, assets: Res<AssetServer>, mut scale: ResMut<UiScale>) {

    // Obligatory camera
    commands.spawn(Camera2dBundle::default());
    scale.scale = 2.0;

    // Allocates IDs that will be set later.
    // IDs can also be Option<Entity> if that is what you prefer.
    let mut root_id = Entity::PLACEHOLDER;
    let mut hiya_id = Entity::PLACEHOLDER;
    let mut howdy_id = Entity::PLACEHOLDER;
    
    // "root" usually kicks things off. It behaves the same as "node", but takes different arguments.
    root(c_root, &assets, &mut commands, |p| {                                  // Spawns the root NodeBundle. AssetServer and Commands get propagated.
        node((c_column, c_green), p, |p| {                                      // Spawns the left pane as a NodeBundle.
            text("This is the left pane!", c_text, c_pixel, p);                 // Spawns a TextBundle.
            text("Do you like it?", c_text, c_pixel, p);
            text_button("Hiya", c_button_left, c_pixel, p).set(&mut hiya_id);   // Spawns a ButtonBundle with a TextBundle child in the middle. Convenience widget.
            grid(6, 6, c_grid, p, |p, _row, _col| {                             // Spawns a NodeBundle container with a NodeBundle for each cell (6x6).
                image(c_inv_slot, p);
            });
            text("Le grid", c_text, c_pixel, p);
        });
        node((c_column, c_blue), p, |p| {
            text("This is the middle pane!", c_text, c_pixel, p);
            text("Indeed, I do!", c_text, c_pixel, p);
            text_button("Howdy", c_button_middle, c_pixel, p).set(&mut howdy_id);
        });
    }).set(&mut root_id);

    // You can insert widgets to an existing entity after the fact.
    // Here, we're adding a third column to the UI.
    commands.entity(root_id).with_ui_children(&assets, |p| {
        node((c_column, c_yellow), p, |p| {
            text("This is the right pane!", c_text, c_pixel, p);
            text("I say, don't quit your day job...", c_text, c_pixel, p);
        });
    });

    // Inserts marker components into the gathered entities.
    // Useful when you need to interact with specific entities in the UI.
    commands.entity(hiya_id).insert(UiId::HiyaButton);
    commands.entity(howdy_id).insert(UiId::HowdyButton);
}

// ---------- System that handles interactions with ui ----------
fn handle_interactions(ui_entities: Query<(&UiId, &Interaction), Changed<Interaction>>) {
    for (id, inter) in &ui_entities {
        match (id, inter) {
            (UiId::HiyaButton, Interaction::Pressed) => println!("Hiya button pressed!!!"),
            (UiId::HiyaButton, Interaction::Hovered) => println!("Hiya button hovered!!!"),
            (UiId::HowdyButton, Interaction::Pressed) => println!("Howdy button pressed!!!"),
            (UiId::HowdyButton, Interaction::Hovered) => println!("Howdy button hovered!!!"),
            _ => {}
        }
    }
}
