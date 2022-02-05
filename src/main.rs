use bevy::{ecs::world, prelude::*};
use bevy_prototype_lyon::prelude::*;
use rand::Rng;
use std::cell::RefCell;
use uuid::Uuid;
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

fn get_random_buf() -> Result<[u8; 32], getrandom::Error> {
    let mut buf = [0u8; 32];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}

#[derive(Component)]
struct Destination {
    id: Uuid,
}

#[derive(Component)]
struct TravelPath {
    pub path: Vec<Uuid>,
    pub path_count: usize,
}

fn get_random_f32() -> f32 {
    return rand::thread_rng().gen_range(-200.0..200.0);
}

fn get_random_uuid() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 500.,
            height: 300.,
            title: "I am a window!".to_string(),
            vsync: true,
            #[cfg(target_arch = "wasm32")]
            canvas: Some(String::from(".game")),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(TravelPath {
            path: vec![],
            path_count: 2,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(update_path)
        .run();
}

fn update_path(query: Query<(&Destination, &Transform)>, mut path: Option<ResMut<TravelPath>>) {
    let path_unwrap = path.unwrap();
    // info!("Adding another dest to path");
    if (path_unwrap.path.len() < path_unwrap.path_count) {
        info!("Adding another dest to path");
        for (dest, trans) in query.iter() {
            info!("Here is {}", dest.id);
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(20.0),
        ..shapes::RegularPolygon::default()
    };

    for x in 0..10 {
        let rnd_x = get_random_f32();
        let rnd_y = get_random_f32();
        let mut entity = commands.spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            Transform::from_translation(Vec3::new(rnd_x, rnd_y, 0.0)),
        ));
        entity.insert(Destination {
            id: get_random_uuid(),
        });
    }

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        ..Default::default()
    });
}
