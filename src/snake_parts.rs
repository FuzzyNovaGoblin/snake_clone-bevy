use bevy::{math, prelude::*};

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
    mut head_positions: Query<(&mut Position, &SnakeHead)>,
) {
    for  (mut pos, head) in head_positions.iter_mut() {
        match head.direction {
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1,
            Direction::Up => pos.y += 1,
            Direction::Down => pos.y -= 1,
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

pub fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    for mut head in heads.iter_mut() {
        let dir: Direction =
            if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                Direction::Left
            } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                Direction::Right
            } else if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                Direction::Up
            } else if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                Direction::Down
            } else {
                head.direction
            };

        if head.direction.opposite() != dir {
            head.direction = dir;
        }
    }
}
