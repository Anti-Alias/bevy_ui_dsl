# Bevy UI DSL

A tiny, plugin-less, macro-less "domain specific language" designed to make building UIs in Bevy more pleasant. This DSL uses the same ingredients that **bevy_ui** uses, so those already familiar with **bevy_ui** should have an easy time learning it.

## UI Example

![Example Image](example.png)

```rust
use bevy_ui_dsl::*;

fn startup(mut commands: Commands, assets: Res<AssetServer>, mut scale: ResMut<UiScale>) {

    // Obligatory camera
    commands.spawn(Camera2dBundle::default());
    scale.scale = 2.0;

    // Allocates IDs that will be set later.
    // IDs can also be Option<Entity> if that is what you prefer.
    let mut root = Entity::PLACEHOLDER;
    let mut hiya = Entity::PLACEHOLDER;
    let mut howdy = Entity::PLACEHOLDER;
    
    // "root" usually kicks things off. It behaves the same as "node", but takes different arguments.
    root(c_root, &assets, &mut commands, |p| {                                  // Spawns the root NodeBundle. AssetServer and Commands get propagated.
        node((c_column, c_green), p, |p| {                                      // Spawns the left pane as a NodeBundle.
            text("This is the left pane!", c_text, c_pixel, p);                 // Spawns a TextBundle.
            text("Do you like it?", c_text, c_pixel, p);
            text_button("Hiya", c_button_left, c_pixel, p).set(&mut hiya);   // Spawns a ButtonBundle with a TextBundle child in the middle. Convenience widget.
            grid(6, 6, c_grid, p, |p, _row, _col| {                             // Spawns a NodeBundle container with a NodeBundle for each cell (6x6).
                image(c_inv_slot, p);
            });
            text("Le grid", c_text, c_pixel, p);
        });
        node((c_column, c_blue), p, |p| {
            text("This is the middle pane!", c_text, c_pixel, p);
            text("Indeed, I do!", c_text, c_pixel, p);
            text_button("Howdy", c_button_middle, c_pixel, p).set(&mut howdy);
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

```

This system spawns a UI using widgets like **root**, **node**, **text**, **text_button**, etc.
You can even create your own widgets! They're just functions! The callback approach is heavily inspired by [egui](https://github.com/emilk/egui).

In this example, **root** is a function that takes a class called **c_root**. The **c_root** function just manipulates a NodeBundle, which is NodeBundle::default() by default. Ultimately, the NodeBundle in question gets spawned.

Like **root**, **node** also takes in a class (or a tuple of classes) and spawns a NodeBundle. When a tuple of classes is supplied, the callback functions are applied in order of left to middle.

Widget functions return the entity spawned. With extension methods, these entities can be "escaped" so that components and bundles can be inserted later. This is great for separating the UI creation code from the bundle insertion code. There are two traits responsible for this:
```rust
pub trait EntityWriter<E> {
    fn set(self, entity: &mut E);   // E is either an &mut Entity, or an &mut Option<Entity>.
}
pub trait EntityPusher {
    fn push(self, destination: &mut Vec<Entity>);
}
```

Oftentimes, though, it's easier to insert a marker component on the fly instead of inserting it later. For this reason, every built-in widget has an inline variant.
These variants accept an extra bundle as an argument.

| base          | inline        |
|---------------|---------------|
| root          | rooti         |
| blank         | blanki        |
| node          | nodei         |
| text          | texti         |
| button        | buttoni       |
| simple_button | simple_buttoni|
| text_button   | text_buttoni  |
| image         | imagei        |
| image_pane    | image_panei   |
| grid          | gridi         |

So, instead of:
```rust
text_button("Hiya", c_button_left, c_pixel, p).set(&mut hiya)
/// ... then later ...
commands.entity(hiya.unwrap()).insert(UiId::HiyaButton);
```
One can write:
```rust
text_buttoni("Hiya", c_button_left, c_pixel, UiId::HiyaButton);
```
Feel free to compare and constrast
[example.rs](examples/example.rs) with [example_inline.rs](examples/example_inline.rs). Which style you pick comes down to taste and situation. The "set" and "push" style is better when the insertion logic is complex and easily separatable. The "inline" style is better for marker components, as well as situations where separating the UI logic from the insertion logic is inconvenient.


## Class Examples
```rust
pub fn c_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.)
}

pub fn c_column(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.flex_direction = FlexDirection::Column;
    s.flex_grow = 1.0;
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    s.padding = UiRect::all(Val::Px(10.));
}

pub fn c_green(b: &mut NodeBundle) {
    b.background_color = Color::rgb_u8(125, 212, 148).into();
}

pub fn c_blue(b: &mut NodeBundle) {
    b.background_color = Color::rgb_u8(125, 164, 212).into();
}

pub fn c_yellow(b: &mut NodeBundle) {
    b.background_color = Color::rgb_u8(100, 100, 50).into();
}

pub fn c_text(_a: &AssetServer, b: &mut TextBundle) {
    b.style.margin = UiRect::all(Val::Px(10.));
}

pub fn c_button_left(assets: &AssetServer, b: &mut ButtonBundle) {
    let s = &mut b.style;
    s.width = Val::Px(64.);
    s.height = Val::Px(24.);
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    b.background_color = Color::rgb_u8(66, 135, 245).into();
    b.image = assets.load("button.png").into();
}

pub fn c_button_middle(assets: &AssetServer, b: &mut ButtonBundle) {
    let s = &mut b.style;
    s.width = Val::Px(64.);
    s.height = Val::Px(24.);
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    b.background_color = Color::rgb_u8(57, 179, 118).into();
    b.image = assets.load("button.png").into();
}

pub fn c_grid(b: &mut NodeBundle) {
    b.style.width = Val::Px(200.);
    b.style.height = Val::Px(200.);
    b.style.margin = UiRect::all(Val::Px(10.));
}

pub fn c_inv_slot(assets: &AssetServer, b: &mut ImageBundle) {
    b.style.width = Val::Px(32.);
    b.style.height = Val::Px(32.);
    b.image = assets.load("item_slot.png").into();
}

pub fn c_pixel(assets: &AssetServer, s: &mut TextStyle) {
    s.font = assets.load("prstartk.ttf").into();
    s.font_size = 8.;
    s.color = Color::WHITE.into();
}
```


## Widget Example
Creating a widget is just a matter of creating a function that follows a certain convention. No more, no less. It will usually require a **Class<T>** or **AssetClass<T>** (can be a callback function, or tuple of callback functions), a parent (needed to spawn the widget itself), and, for container widgets, a callback function used to spawn children of the widget. See [widgets.rs](src/widgets.rs) for examples.
```rust
/// Spawns a [`NodeBundle`] with children.
pub fn node(
    class: impl Class<NodeBundle>,              // Class (or classes) that manipulate the bundle.
    parent: &mut UiChildBuilder,                // Parent entity to add NodeBundle to.
    children: impl FnOnce(&mut UiChildBuilder)  // Callback function that spawns children of the newly spawned NodeBundle.
) -> Entity {
    let mut bundle = NodeBundle::default();             // Initializes the NodeBundle.
    class.apply(&mut bundle);                           // Applies class (or classes) to that bundle.
    parent.spawn(bundle).with_children(children).id()   // Spawns updated bundle with children!
}

```

## Compatibility

| bevy | bevy-ui-dsl |
| ---- | ----------- |
| 0.11 | 0.5 - 0.6   |
| 0.10 | 0.3 - 0.4   |
| 0.9  | 0.1 - 0.2   |
