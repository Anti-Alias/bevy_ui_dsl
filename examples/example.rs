use bevy_ui_dsl::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(startup)
        .run();
}


fn startup(mut c: Commands, assets: Res<AssetServer>, mut scale: ResMut<UiScale>) {

    // Obligatory camera
    c.spawn(Camera2dBundle::default());
    scale.scale = 2.0;

    // Load asset and make 
    let font: Handle<Font> = assets.load("fonts/prstartk.ttf");
    let image: Handle<Image> = assets.load("buttons/tab.png");
    let f_text = || s_font(font.clone());
    let s_button = || s_button(image.clone());

    // Builds DOM
    node_root(&mut c, s_root, |p| {
        node_with(p, s_left, |p| {
            text(p, "This is the left pane!", s_text, f_text);
            text(p, "Do you like it?", s_text, f_text);
            text_button(p, "Button 1", s_button, f_text);
        });
        node_with(p, s_right, |p| {
            text(p, "This is the right pane!", s_text, f_text);
            text(p, "Indeed, I do!", s_text, f_text);
            text_button(p, "Button 2", s_button, f_text);
        });
    });
}


// --------------- Styles ---------------
fn s_root() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            ..default()
        },
        ..default()
    }
}

fn s_left() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.), Val::Percent(100.)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.)),
            ..default()
        },
        background_color: Color::RED.into(),
        ..default()
    }
}

fn s_right() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(50.), Val::Percent(100.)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.)),
            ..default()
        },
        background_color: Color::BLUE.into(),
        ..default()
    }
}

fn s_text() -> TextBundle {
    TextBundle {
        style: Style {
            margin: UiRect::all(Val::Px(10.)),
            ..default()
        },
        ..default()
    }
}

fn s_font(font: Handle<Font>) -> TextStyle {
    TextStyle {
        font,
        font_size: 8.,
        color: Color::WHITE.into(),
    }
}

fn s_button(image: Handle<Image>) -> ButtonBundle {
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