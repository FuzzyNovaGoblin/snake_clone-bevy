use bevy::prelude::*;

use crate::{
    coord_system::{Position, Size as MySize, ARENA_HEIGHT, ARENA_WIDTH},
    food::Food,
    GameOverEvent, Materials,
};

/// # Data
pub struct SnakeHead {
    direction: Direction,
    next_direction: Direction,
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

/// # Functions
pub fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &mut SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_pos: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    if let Some((head_entity, mut head)) = heads.iter_mut().next() {
        head.direction = head.next_direction;
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
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

        if head_pos.x < 0
            || head_pos.x >= ARENA_WIDTH as i32
            || head_pos.y < 0
            || head_pos.y >= ARENA_HEIGHT as i32
            || segment_positions.contains(&*head_pos)
        {
            game_over_writer.send(GameOverEvent)
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
                next_direction: Direction::Up,
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
                head.next_direction
            };

        if head.next_direction.opposite() != dir {
            head.next_direction = dir;
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
    mut growth_reader: EventReader<GrowthEvent>,
    tail_pos: Res<LastTailPosition>,
    materials: Res<Materials>,
    mut segments: ResMut<SnakeSegments>,
) {
    if growth_reader.iter().next().is_some() {
        segments.0.push(spawn_segment(
            commands,
            &materials.segment_material,
            tail_pos.0.unwrap(),
        ));
    }
}
