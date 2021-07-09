use bevy::core::FixedTimestep;
use bevy::prelude::*;

use bevy::{sprite::ColorMaterial, DefaultPlugins};
use food::{food_spawner, Food};
use score::ScorePlugin;
use snake_parts::*;

mod coord_system;
mod food;
mod score;
mod snake_parts;

pub struct GameOverEvent;

pub struct Materials {
    head_material: Handle<ColorMaterial>,
    segment_material: Handle<ColorMaterial>,
    food_material: Handle<ColorMaterial>,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        segment_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    });
    commands.insert_resource(SnakeSegments::default());
    commands.insert_resource(LastTailPosition::default());
}

fn game_over(
    mut cmds: Commands,
    mut game_over_event_reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    segments: Query<Entity, With<SnakeSegment>>,
    head_q: Query<Entity, With<SnakeHead>>,
    materials: Res<Materials>,
    food: Query<Entity, With<Food>>,
) {
    if game_over_event_reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()).chain(head_q.iter()) {
            cmds.entity(ent).despawn();
        }
        spawn_snake(cmds, materials, segments_res);
    }
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Snake".to_string(),
            width: 700.0,
            height: 700.0,
            ..Default::default()
        })
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
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
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(coord_system::size_scaleing.system())
                .with_system(coord_system::position_translation.system()),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.150))
                .with_system(snake_movement.system().label(SnakeMovement::Movement))
                .with_system(
                    snake_eating
                        .system()
                        .label(SnakeMovement::Eating)
                        .after(SnakeMovement::Movement),
                )
                .with_system(
                    snake_growth
                        .system()
                        .label(SnakeMovement::Growth)
                        .after(SnakeMovement::Eating),
                )
                .with_system(game_over.system().after(SnakeMovement::Growth)),
        )
        .add_system(food_spawner.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ScorePlugin)
        .run();
}
