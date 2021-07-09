use bevy::prelude::*;

use crate::{
    coord_system::{Position, Size as MySize},
    Materials,
};

/// # Data
pub struct SnakeHead {
    direction: Direction,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

/// # Implements
impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

impl SnakeHead {
    pub fn get_dir(&self) -> Direction {
        self.direction
    }
    pub fn set_dir(&mut self, dir: Direction) {
        self.direction = dir;
    }
}

/// # Functions

pub fn snake_movement(
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

pub fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),

            ..Default::default()
        })
        .insert(SnakeHead {
            direction: Direction::Up,
        })
        .insert(Position { x: 3, y: 3 })
        .insert(MySize::square(0.8));
}
