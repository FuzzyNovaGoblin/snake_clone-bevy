use bevy::core::FixedTimestep;
use bevy::prelude::*;

use bevy::{sprite::ColorMaterial, DefaultPlugins};
use snake_parts::{snake_movement, snake_movement_input, SnakeMovement};

mod coord_system;
mod snake_parts;

pub struct Materials {
    head_material: Handle<ColorMaterial>,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
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
        .add_startup_stage(
            "game_setup",
            SystemStage::single(snake_parts::spawn_snake.system()),
        )
        .add_system(
            snake_movement_input
                .system()
                .label(SnakeMovement::Input)
                .before(SnakeMovement::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.20))
                .with_system(snake_movement.system()),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(coord_system::size_scaleing.system())
                .with_system(coord_system::position_translation.system()),
        )
        .add_plugins(DefaultPlugins)
        .run();
}
