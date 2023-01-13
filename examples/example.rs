use bevy::prelude::*;
use bevy_ui_dsl::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))    // Nearest neighbor scaling for pixel graphics!
        .add_startup_system(startup)
        .run();
}


fn startup(mut commands: Commands, assets: Res<AssetServer>, mut scale: ResMut<UiScale>) {

    // Obligatory camera
    commands.spawn(Camera2dBundle::default());
    scale.scale = 2.0;

    // Builds DOM
    let c = &mut commands;
    root(c_root, assets, c, |p| {                                   // Spawns a root NodeBundle. AssetServer gets propagated.
        node(c_left, p, |p| {                                       // Spawns the left pane as a NodeBundle
            text("This is the left pane!", c_text, t_pixel, p);     // Spawns a TextBundle
            text("Do you like it?", c_text, t_pixel, p);            // Spawns a TextBundle
            text_button("Hiya", c_button_left, t_pixel, p);         // Spawns a ButtonBundle with a TextBundle child in the middle. Convenience widget.
            grid(6, 6, c_grid, p, |p, _row, _col| {                 // Spawns a NodeBundle container with a NodeBundle for each cell (6x6). Each cell contains a single ImageBundle.
                simple_image(c_inv_slot, p)
            });
            text("Le grid", c_text, t_pixel, p);            // Spawns a TextBundle
        });
        node(c_right, p, |p| {
            text("This is the right pane!", c_text, t_pixel, p);
            text("Indeed, I do!", c_text, t_pixel, p);
            text_button("Howdy", c_button_right, t_pixel, p);
        });
    });
}


// --------------- Classes (they're really just producers of bundles, but it's useful to think of them as .css classes) ---------------
fn c_root() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            ..default()
        },
        ..default()
    }
}

fn c_left() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.), Val::Percent(100.)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.)),
            ..default()
        },
        background_color: Color::rgb_u8(125, 212, 148).into(),
        ..default()
    }
}

fn c_right() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.), Val::Percent(100.)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.)),
            ..default()
        },
        background_color: Color::rgb_u8(125, 164, 212).into(),
        ..default()
    }
}

fn c_text() -> TextBundle {
    TextBundle {
        style: Style {
            margin: UiRect::all(Val::Px(10.)),
            ..default()
        },
        ..default()
    }
}

fn c_button_left(assets: &AssetServer) -> ButtonBundle {
    let image: Handle<Image> = assets.load("button.png");
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(64.), Val::Px(24.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb_u8(66, 135, 245).into(),
        image: UiImage(image),
        ..default()
    }
}

fn c_button_right(assets: &AssetServer) -> ButtonBundle {
    let image: Handle<Image> = assets.load("button.png");
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(64.), Val::Px(24.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgb_u8(57, 179, 118).into(),
        image: UiImage(image),
        ..default()
    }
}

fn c_grid() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(200.), Val::Px(200.)),
            margin: UiRect::all(Val::Px(10.)),
            ..default()
        },
        ..default()
    }
}

fn c_inv_slot(assets: &AssetServer) -> ImageBundle {
    ImageBundle {
        style: Style {
            size: Size::new(Val::Px(32.), Val::Px(32.)),
            ..default()
        },
        image: assets.load("item_slot.png").into(),
        ..default()
    }
}


// --------------- Text styles ---------------
fn t_pixel(assets: &AssetServer) -> TextStyle {
    TextStyle {
        font: assets.load("prstartk.ttf"),
        font_size: 8.,
        color: Color::WHITE.into(),
    }
}