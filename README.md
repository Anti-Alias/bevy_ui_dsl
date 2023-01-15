# Bevy UI DSL

A "domain specific language" designed to make building UIs in Bevy more pleasant. This DSL uses the same ingredients that **bevy_ui** uses, so those already familiar with **bevy_ui** should have an easy time learning it.

## Example

```rust
fn startup(mut commands: Commands, assets: Res<AssetServer>, mut scale: ResMut<UiScale>) {

    // Obligatory camera
    commands.spawn(Camera2dBundle::default());
    scale.scale = 2.0;

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
    // Useful when you need to interact with specific entities in the UI
    commands
        .entity(hiya.unwrap())
        .insert(UiId::HiyaButton);
    commands
        .entity(howdy.unwrap())
        .insert(UiId::HowdyButton);
}

```

This system spawns a UI using widgets like **root**, **node**, **text**, **text_button**, etc.
You can even create your own widgets! They're just functions!

In this example, **root** is a function that takes a class called **c_root**. The **c_root** function just manipulates a NodeBundle, which is NodeBundle::default() by default. Ultimately, the NodeBundle in question gets spawned.

Like **root**, **node** also takes in a class (or a tuple of classes) and spawns a NodeBundle. When a tuple of classes is supplied, the callback functions are applied in order of left to right.

## Class Examples

```rust
fn c_root(bundle: &mut NodeBundle) {
    bundle.style.size = Size::new(Val::Percent(100.), Val::Percent(100.)),
        ..default()
    };
}

fn c_half(b: &mut NodeBundle) {
    b.style = Style {
        size: Size::new(Val::Percent(50.), Val::Percent(100.)),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(10.)),
        ..default()
    };
}

fn c_green(b: &mut NodeBundle) {
    b.background_color = Color::rgb_u8(125, 212, 148).into();
}

fn c_blue(b: &mut NodeBundle) {
    b.background_color = Color::rgb_u8(125, 164, 212).into();
}

fn c_text(_a: &AssetServer, b: &mut TextBundle) {
    b.style = Style {
        margin: UiRect::all(Val::Px(10.)),
        ..default()
    };
}

fn c_button_left(assets: &AssetServer, b: &mut ButtonBundle) {
    b.style = Style {
        size: Size::new(Val::Px(64.), Val::Px(24.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    b.background_color = Color::rgb_u8(66, 135, 245).into();
    b.image = assets.load("button.png").into();
}

fn c_button_right(assets: &AssetServer, b: &mut ButtonBundle) {
    b.style = Style {
        size: Size::new(Val::Px(64.), Val::Px(24.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    b.background_color = Color::rgb_u8(57, 179, 118).into();
    b.image = assets.load("button.png").into();
}

fn c_grid(b: &mut NodeBundle) {
    b.style = Style {
        size: Size::new(Val::Px(200.), Val::Px(200.)),
        margin: UiRect::all(Val::Px(10.)),
        ..default()
    };
}

fn c_inv_slot(assets: &AssetServer, b: &mut ImageBundle) {
    b.style = Style {
        size: Size::new(Val::Px(32.), Val::Px(32.)),
        ..default()
    };
    b.image = assets.load("item_slot.png").into();
}

fn c_pixel(assets: &AssetServer, s: &mut TextStyle) {
    s.font = assets.load("prstartk.ttf").into();
    s.font_size = 8.;
    s.color = Color::WHITE.into();
}
```

Some classes only depend a single bundle. Others depend on an AssetServer to manipulate their respective types.

# TODO
* Allow users to insert extra components to widgets by returning an EntityCommands.
* Create an example with interactive components.