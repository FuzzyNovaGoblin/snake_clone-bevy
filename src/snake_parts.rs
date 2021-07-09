use bevy::prelude::*;

use crate::{
    coord_system::{Position, Size as MySize},
    food::Food,
    Materials,
};

/// # Data
pub struct SnakeHead {
    direction: Direction,
}
pub struct SnakeSegment;
pub struct GrowthEvent;

#[derive(Default)]
pub struct SnakeSegments(Vec<Entity>);

#[derive(Default)]
pub struct LastTailPosition(Option<Position>);

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
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_pos: ResMut<LastTailPosition>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        }

        for (pos, segment) in segment_positions.iter().zip(segments.0.iter().skip(1)) {
            *positions.get_mut(*segment).unwrap() = *pos;
        }
        last_tail_pos.0 = Some(*segment_positions.last().unwrap());
    }
}

pub fn spawn_snake(
    mut commands: Commands,
    materials: Res<Materials>,
    mut segments: ResMut<SnakeSegments>,
) {
    segments.0 = vec![
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
            .insert(MySize::square(0.8))
            .id(),
        spawn_segment(
            commands,
            &materials.segment_material,
            Position { x: 3, y: 2 },
        ),
    ];
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

pub fn spawn_segment(
    mut commands: Commands,
    material: &Handle<ColorMaterial>,
    position: Position,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            material: material.clone(),
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(MySize::square(0.65))
        .id()
}

pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    foods: Query<(Entity, &Position), With<Food>>,
    head_q: Query<&Position, With<SnakeHead>>,
) {
    if let Some(head_pos) = head_q.iter().next() {
        for (e, pos) in foods.iter() {
            if pos == head_pos {
                commands.entity(e).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_growth(
    commands: Commands,
    mut growthReader: EventReader<GrowthEvent>,
    tail_pos: Res<LastTailPosition>,
    materials: Res<Materials>,
    mut segments: ResMut<SnakeSegments>,
) {
    if growthReader.iter().next().is_some() {
        println!("in here",);
        segments.0.push(spawn_segment(
            commands,
            &materials.segment_material,
            tail_pos.0.unwrap(),
        ));
    }
}
