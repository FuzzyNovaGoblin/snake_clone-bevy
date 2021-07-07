use bevy::prelude::*;

use bevy::{sprite::ColorMaterial, DefaultPlugins};

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

struct Position {
    x: i32,
    y: i32,
}

struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(side: f32) -> Self {
        Size {
            width: side,
            height: side,
        }
    }
}

struct SnakeHead;
struct Materials {
    head_material: Handle<ColorMaterial>,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),

            ..Default::default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn size_scaleing(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();

    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            window.width() / ARENA_WIDTH as f32 * sprite_size.width,
            window.height() / ARENA_HEIGHT as f32 * sprite_size.height,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    let window = windows.get_primary().unwrap();

    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    for (sprite_pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(sprite_pos.x as f32, window.width(), ARENA_WIDTH as f32),
            convert(sprite_pos.y as f32, window.height(), ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Position, With<SnakeHead>>,
) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            pos.y += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            pos.y -= 1;
        }
    }
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Snake".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()))
        .add_system(snake_movement.system())
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(size_scaleing.system())
                .with_system(position_translation.system()),
        )
        .add_plugins(DefaultPlugins)
        .run();
}