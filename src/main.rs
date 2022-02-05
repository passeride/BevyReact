use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local!(static GLOBAL_MOVE_UP: RefCell<bool> = RefCell::new(false));
thread_local!(static GLOBAL_MOVE_DOWN: RefCell<bool> = RefCell::new(false));
thread_local!(static GLOBAL_MOVE_RIGHT: RefCell<bool> = RefCell::new(false));
thread_local!(static GLOBAL_MOVE_LEFT: RefCell<bool> = RefCell::new(false));

#[wasm_bindgen]
pub fn move_up() {
    GLOBAL_MOVE_UP.with(|text| *text.borrow_mut() = true);
}

#[wasm_bindgen]
pub fn move_down() {
    GLOBAL_MOVE_DOWN.with(|text| *text.borrow_mut() = true);
}

#[wasm_bindgen]
pub fn move_right() {
    GLOBAL_MOVE_RIGHT.with(|text| *text.borrow_mut() = true);
}

#[wasm_bindgen]
pub fn move_left() {
    GLOBAL_MOVE_LEFT.with(|text| *text.borrow_mut() = true);
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            #[cfg(target_arch = "wasm32")]
            canvas: Some(String::from(".game")),
            width: 500.,
            height: 300.,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(20.0),
        ..shapes::RegularPolygon::default()
    };

    for x in 0..10 {
        commands.spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            Transform::from_translation(Vec3::new(x as f32, x as f32, 0.0)),
        ));
    }
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        ..Default::default()
    });
}
