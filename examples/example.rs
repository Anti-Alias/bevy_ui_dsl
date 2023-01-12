use bevy_ui_dsl::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .run();
}


fn startup(mut c: Commands, assets: Res<AssetServer>) {

    // Obligatory camera
    c.spawn(Camera2dBundle::default());

    // Load asset and make 
    let font: Handle<Font> = assets.load("fonts/prstartk.ttf");
    let s_font = || s_font(font.clone());

    // Builds DOM
    node_root(&mut c, s_root, |p| {
        node_with(p, s_left, |p| {
            text(p, "This is the left pane!", s_text, s_font);
            text(p, "Do you like it?", s_text, s_font);
        });
        node_with(p, s_right, |p| {
            text(p, "This is the right pane!", s_text, s_font);
            text(p, "Indeed, I do!", s_text, s_font);
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
        font_size: 12.,
        color: Color::WHITE.into(),
    }
}