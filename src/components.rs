use crate::constants::*;
use bevy::prelude::*;

pub struct Tile;
pub struct MainCamera;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new<T: Into<i32>>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl From<Vec2> for Position {
    fn from(vec: Vec2) -> Self {
        let convert = |coord: f32, tile_size: f32| {
            ((coord + ARENA_SIZE * 0.5) / tile_size - 0.5).round() as i32
        };
        let tile_size = ARENA_SIZE / GRID_NUM as f32;
        Self {
            x: convert(vec.x, tile_size),
            y: convert(vec.y, tile_size),
        }
    }
}

impl From<&Position> for Transform {
    fn from(pos: &Position) -> Self {
        let convert = |pos: f32, tile_size: f32| (pos + 0.5) * tile_size - ARENA_SIZE * 0.5;
        let tile_size = ARENA_SIZE / GRID_NUM as f32;
        Transform::from_xyz(
            convert(pos.x as f32, tile_size),
            convert(pos.y as f32, tile_size),
            0.0,
        )
    }
}
