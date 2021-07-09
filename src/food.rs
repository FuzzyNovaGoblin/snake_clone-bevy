use crate::coord_system::{ARENA_HEIGHT, ARENA_WIDTH};
use bevy::prelude::*;
use rand::random;

use crate::{coord_system::{Position, Size as MySize}, Materials};

pub struct Food;

pub fn food_spawner(mut commands: Commands, materials: Res<Materials>, foods: Query<&Food>) {
    if foods.iter().len() < 2 {
        let points = (
            (random::<f32>() * ARENA_WIDTH as f32) as i32,
            (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        );
        println!("points: {}, {}", points.0, points.1);
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.food_material.clone(),
                ..Default::default()
            })
            .insert(Position {
                x: points.0,
                y: points.1,
            })
            .insert(Food)
            .insert(MySize::square(0.8));
    }
}
