use crate::{
    coord_system::{ARENA_HEIGHT, ARENA_WIDTH},
    snake_parts::{SnakeHead, SnakeSegment, SnakeSegments},
};
use bevy::prelude::*;
use rand::random;

use crate::{
    coord_system::{Position, Size as MySize},
    Materials,
};

pub struct Food;

pub fn food_spawner(
    mut commands: Commands,
    materials: Res<Materials>,
    foods: Query<&Food>,
    seg_positions: Query<&Position, With<SnakeSegment>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    if foods.iter().len() < 2 {
        let bad_positions: Vec<Position> = seg_positions
            .iter()
            .chain(head_positions.iter())
            .map(|p| *p)
            .collect();

        let mut points = Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        };
        let mut counter = 0;
        while bad_positions.contains(&points) {
            counter += 1;

            points.x = (random::<f32>() * ARENA_WIDTH as f32) as i32;
            points.y = (random::<f32>() * ARENA_HEIGHT as f32) as i32;
            if counter > 50 {
                break;
            }
        }

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.food_material.clone(),
                ..Default::default()
            })
            .insert(points)
            .insert(Food)
            .insert(MySize::square(0.8));
    }
}
