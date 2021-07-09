use bevy::prelude::*;

pub const ARENA_WIDTH: u32 = 13;
pub const ARENA_HEIGHT: u32 = 13;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn square(side: f32) -> Self {
        Size {
            width: side,
            height: side,
        }
    }
}

pub fn size_scaleing(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();

    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            window.width() / ARENA_WIDTH as f32 * sprite_size.width,
            window.height() / ARENA_HEIGHT as f32 * sprite_size.height,
        );
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
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
