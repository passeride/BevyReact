use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;
use std::{borrow::BorrowMut, cell::RefCell};
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
struct CostDistance {
    source: Vec2,
    dest: Vec2,
    distance: f32,
}

#[derive(Component, Debug)]
struct TravelPath {
    pub path: Vec<Uuid>,
    pub current_pos: Vec2,
    pub path_count: usize,
}

fn get_random_f32() -> f32 {
    return rand::thread_rng().gen_range(-400.0..400.0);
}

fn get_random_uuid() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1200.,
            height: 800.,
            title: "I am a window!".to_string(),
            vsync: true,
            #[cfg(target_arch = "wasm32")]
            canvas: Some(String::from(".game")),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(TravelPath {
            path: vec![],
            current_pos: Vec2::new(0.0, 0.0),
            path_count: 2,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(update_path_closest_neighboor)
        .run();
}

fn update_path_closest_neighboor(
    mut commands: Commands,
    query: Query<(&Destination, &Transform)>,
    mut path: Option<ResMut<TravelPath>>,
) {
    GLOBAL_MOVE_UP.with(|text| {
        if *text.borrow() {
            *text.borrow_mut() = false;
            let mut path_02 = path.unwrap();
            let mut path_unwrap = path_02.as_mut();
            let mut closest_uuid = get_random_uuid();
            let mut closest_distance = 1000.0;
            let mut closest_position = Vec2::default();
            for (dest, trans) in query.iter() {
                if !path_unwrap.path.contains(&dest.id) {
                    let new_post_3d: Vec3 = trans.translation;
                    let new_pos = Vec2::new(new_post_3d.x, new_post_3d.y);
                    let distance_to_current_pos = path_unwrap.current_pos.distance(new_pos);
                    if distance_to_current_pos < closest_distance {
                        closest_distance = distance_to_current_pos;
                        closest_uuid = dest.id;
                        closest_position = new_pos;
                    }
                }
            }

            let line = shapes::Line(path_unwrap.current_pos, closest_position);

            commands.spawn_bundle(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::BLACK, 5.0)),
                Transform::default(),
            ));

            path_unwrap.current_pos = closest_position;
            path_unwrap.path.push(closest_uuid);
            return;
        }
    });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(10.0),
        ..shapes::RegularPolygon::default()
    };

    // let mut entity_vec = vec![];

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ))
        .with_children(|parent| {
            for x in 0..30 {
                let rnd_x = get_random_f32();
                let rnd_y = get_random_f32();
                let mut entity = parent.spawn_bundle(GeometryBuilder::build_as(
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
                // entity_vec.push(entity);
            }
        });

    // for source in entity_vec.iter() {
    //     for dest in entity_vec.iter() {
    //         if (source.id() != dest.id()) {
    //             let mut path = commands.spawn_bundle(GeometryBuilder::build_as(
    //                 &shape,
    //                 DrawMode::Outlined {
    //                     fill_mode: FillMode::color(Color::CYAN),
    //                     outline_mode: StrokeMode::new(Color::BLACK, 10.0),
    //                 },
    //                 Transform::from_translation(Vec3::new(rnd_x, rnd_y, 0.0)),
    //             ));
    //             let distance = 10.0;
    //             path.insert(CostDistance {
    //                 dest: dest..id(),
    //                 source: source.id(),
    //                 distance: distance,
    //             });
    //             source.add_child(path)
    //         }
    //     }
    // }

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        ..Default::default()
    });
}
