use bevy::{prelude::*, sprite::Anchor};

const WINDOW_WIDTH: f32 = 500.0;
const WINDOW_HEIGHT: f32 = 300.0;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "stage_test".to_string(),
            width: 500.,
            height: 300.,
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0 - WINDOW_WIDTH / 2.0,0.0 - WINDOW_HEIGHT / 2.0,0.),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT / 10.0)),
            anchor: Anchor::BottomLeft,
            ..default()
        },
        ..default()
    });
}