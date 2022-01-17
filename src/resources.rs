use crate::components::*;
use bevy::prelude::*;

pub struct Colors {
    pub tile: Handle<ColorMaterial>,
    pub selected: Handle<ColorMaterial>,
    pub invalid: Handle<ColorMaterial>,
}

pub struct Cursor {
    pub current_position: Position,
    pub last_clicked_position: Option<Position>,
}

#[derive(Clone, PartialEq)]
pub enum Agent {
    Player,
    Computer,
}

pub struct PlayerSetting {
    pub player1: Agent,
    pub player2: Agent,
    pub current_player: bool, // false: player1, true: player2
}

impl Default for PlayerSetting {
    fn default() -> Self {
        Self {
            player1: Agent::Player,
            player2: Agent::Computer,
            current_player: false,
        }
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            current_position: Position::new(-1, -1),
            last_clicked_position: None,
        }
    }
}

pub struct SelectedEvent;
pub struct DeletedTileEvent;
pub struct GameOverEvent;

#[derive(Default)]
pub struct Selection {
    pub is_valid: Option<bool>,
}
