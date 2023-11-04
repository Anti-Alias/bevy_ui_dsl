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
    scale.0 = 2.0;

    // Spawns ui and gathers entity ids
    let mut hiya = None;
    let mut howdy = None;
    root(c_root, &assets, &mut commands, |p| {                                  // Spawns the root NodeBundle. AssetServer gets propagated.
        node((c_half, c_green), p, |p| {                                        // Spawns the left pane as a NodeBundle.
            text("This is the left pane!", c_text, c_pixel, p);                 // Spawns a TextBundle.
            text("Do you like it?", c_text, c_pixel, p);
            text_button("Hiya", c_button_left, c_pixel, p).set(&mut hiya);      // Spawns a ButtonBundle with a TextBundle child in the middle. Convenience widget.
            grid(6, 6, c_grid, p, |p, _row, _col| {                             // Spawns a NodeBundle container with a NodeBundle for each cell (6x6).
                image(c_inv_slot, p);
            });
            text("Le grid", c_text, c_pixel, p);
        });
        node((c_half, c_blue), p, |p| {
            text("This is the right pane!", c_text, c_pixel, p);
            text("Indeed, I do!", c_text, c_pixel, p);
            text_button("Howdy", c_button_right, c_pixel, p).set(&mut howdy);
        });
    });

    // Inserts marker components into the gathered entities.
    // Useful when you need to interact with specific entities in the UI.
    commands
        .entity(hiya.unwrap())
        .insert(UiId::HiyaButton);
    commands
        .entity(howdy.unwrap())
        .insert(UiId::HowdyButton);
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
