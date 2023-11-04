mod classes;
use bevy::prelude::*;
use bevy_ui_dsl::*;
use classes::*;

#[derive(Component, Debug)]
enum UiId {
    HiyaButton,
    HowdyButton
}

/// Inline version of example.rs.
/// Functionally equivalent.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, startup)
        .add_systems(Update, handle_interactions)
        .run();
}

fn startup(mut commands: Commands, assets: Res<AssetServer>, mut scale: ResMut<UiScale>) {

    commands.spawn(Camera2dBundle::default());
    scale.0 = 2.0;

    root(c_root, &assets, &mut commands, |p| {
        node((c_half, c_green), p, |p| {
            text("This is the left pane!", c_text, c_pixel, p);
            text("Do you like it?", c_text, c_pixel, p);
            text_buttoni("Hiya", c_button_left, c_pixel, UiId::HiyaButton, p);      // Inline variant of text_button
            grid(6, 6, c_grid, p, |p, _row, _col| {
                image(c_inv_slot, p);
            });
            text("Le grid", c_text, c_pixel, p);
        });
        node((c_half, c_blue), p, |p| {
            text("This is the right pane!", c_text, c_pixel, p);
            text("Indeed, I do!", c_text, c_pixel, p);
            text_buttoni("Howdy", c_button_right, c_pixel, UiId::HowdyButton, p);   // Inline variant of text_button
        });
    });
}

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
