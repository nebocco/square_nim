use crate::{components::*, constants::*, resources::*};
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands.insert_resource(Colors {
        tile: materials.add(COLOR_TILE.into()),
        selected: materials.add(COLOR_SELECTED.into()),
        invalid: materials.add(COLOR_INVALID.into()),
    });
}

pub fn create_stage(commands: Commands, colors: Res<Colors>) {
    let stage = generate_random_map(0.8);
    display_map(commands, stage, colors);
}

fn display_map(mut commands: Commands, stage: Vec<Vec<bool>>, colors: Res<Colors>) {
    assert!(stage.len() == GRID_NUM && stage[0].len() == GRID_NUM);
    for (y, row) in stage.into_iter().enumerate() {
        for (x, b) in row.into_iter().enumerate() {
            if b {
                let pos = Position::new(x as i32, y as i32);
                spawn_tile(&mut commands, pos, &colors);
            }
        }
    }
}

fn spawn_tile(commands: &mut Commands, pos: Position, colors: &Res<Colors>) {
    let tile_size = ARENA_SIZE / GRID_NUM as f32 * 0.9;

    commands
        .spawn_bundle(SpriteBundle {
            material: colors.tile.clone(),
            sprite: Sprite::new(Vec2::new(tile_size, tile_size)),
            transform: (&pos).into(),
            ..Default::default()
        })
        .insert(Tile)
        .insert(pos);
}

fn generate_random_map(p: f64) -> Vec<Vec<bool>> {
    assert!(0. <= p && p <= 1.);
    let mut rng = thread_rng();
    (0..GRID_NUM)
        .map(|_| (0..GRID_NUM).map(|_| rng.gen_bool(p)).collect())
        .collect()
}

pub fn cursor_system(
    wnds: Res<Windows>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut cursor: ResMut<Cursor>,
) {
    let wnd = wnds.get_primary().unwrap();
    if let Some(pos) = wnd.cursor_position() {
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let p = pos - size / 2.0;
        let camera_transform = q_camera.single().unwrap();
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        cursor.current_position = pos_wld.truncate().truncate().into();
    }
}

pub fn change_color_system(
    cursor: Res<Cursor>,
    selection: Res<Selection>,
    tiles: Query<(&Position, &mut Handle<ColorMaterial>), With<Tile>>,
    colors: Res<Colors>,
) {
    if let Some(Position {
        x: mut lx,
        y: mut ly,
    }) = cursor.last_clicked_position
    {
        let Position {
            x: mut rx,
            y: mut ry,
        } = cursor.current_position;
        if lx > rx {
            std::mem::swap(&mut lx, &mut rx);
        }
        if ly > ry {
            std::mem::swap(&mut ly, &mut ry)
        }
        let is_valid = selection.is_valid.unwrap();
        tiles.for_each_mut(|(&Position { x, y }, mut handle)| {
            *handle = if lx <= x && x <= rx && ly <= y && y <= ry {
                if is_valid {
                    colors.selected.clone()
                } else {
                    colors.invalid.clone()
                }
            } else {
                colors.tile.clone()
            }
        });
    } else {
        let cursor_position = cursor.current_position;
        tiles.for_each_mut(|(&pos, mut handle)| {
            *handle = if pos == cursor_position {
                colors.selected.clone()
            } else {
                colors.tile.clone()
            };
        });
    }
}

pub fn drug_system(
    mut cursor: ResMut<Cursor>,
    mouse: Res<Input<MouseButton>>,
    mut selected_event: EventWriter<SelectedEvent>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        cursor.last_clicked_position = Some(cursor.current_position);
    } else if mouse.just_released(MouseButton::Left) {
        selected_event.send(SelectedEvent)
    }
}

pub fn selection_validation_system(
    cursor: Res<Cursor>,
    mut selection: ResMut<Selection>,
    tiles: Query<&Position, With<Tile>>,
) {
    selection.is_valid = if let Some(Position {
        x: mut lx,
        y: mut ly,
    }) = cursor.last_clicked_position
    {
        let Position {
            x: mut rx,
            y: mut ry,
        } = cursor.current_position;
        if lx > rx {
            std::mem::swap(&mut lx, &mut rx);
        }
        if ly > ry {
            std::mem::swap(&mut ly, &mut ry)
        }
        let target_count = tiles
            .iter()
            .filter(|&&Position { x, y }| lx <= x && x <= rx && ly <= y && y <= ry)
            .count();
        Some(target_count as i32 == (rx - lx + 1) * (ry - ly + 1))
    } else {
        None
    };
}

pub fn delete_tiles_system(
    mut commands: Commands,
    mut cursor: ResMut<Cursor>,
    selection: Res<Selection>,
    mut event_reader: EventReader<SelectedEvent>,
    mut event_writer: EventWriter<DeletedTileEvent>,
    tiles: Query<(Entity, &Position), With<Tile>>,
) {
    if event_reader.iter().next().is_some() {
        if let Some(Position {
            x: mut lx,
            y: mut ly,
        }) = cursor.last_clicked_position.take()
        {
            if let Some(true) = selection.is_valid {
                let Position {
                    x: mut rx,
                    y: mut ry,
                } = cursor.current_position;
                if lx > rx {
                    std::mem::swap(&mut lx, &mut rx);
                }
                if ly > ry {
                    std::mem::swap(&mut ly, &mut ry)
                }
                if lx == rx || ly == ry {
                    return;
                }
                tiles.for_each_mut(|(ent, &Position { x, y })| {
                    if lx <= x && x <= rx && ly <= y && y <= ry {
                        commands.entity(ent).despawn();
                    }
                });
                event_writer.send(DeletedTileEvent);
            }
        }
    }
}

pub fn switch_player_system(mut player_setting: ResMut<PlayerSetting>) {
    player_setting.current_player ^= true;
    eprintln!(
        "current turn: {}",
        if player_setting.current_player == false {
            "Player 1"
        } else {
            "Player 2"
        }
    );
}

pub fn check_game_over_system(
    mut event_writer: EventWriter<GameOverEvent>,
    tile_positions: Query<&Position, With<Tile>>,
) {
    let mut grid = vec![vec![false; GRID_NUM]; GRID_NUM];
    tile_positions.for_each(|pos| {
        grid[pos.y as usize][pos.x as usize] = true;
    });
    for i in 0..GRID_NUM - 1 {
        for j in 0..GRID_NUM - 1 {
            if grid[i][j] && grid[i + 1][j] && grid[i][j + 1] && grid[i + 1][j + 1] {
                return;
            }
        }
    }
    event_writer.send(GameOverEvent);
}

pub fn run_if_deleted_tile(mut event_reader: EventReader<DeletedTileEvent>) -> ShouldRun {
    if event_reader.iter().next().is_some() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn game_over_system(
    mut event_reader: EventReader<GameOverEvent>,
    player_setting: Res<PlayerSetting>,
    //     mut windows: ResMut<Windows>,
) {
    if event_reader.iter().next().is_some() {
        eprintln!(
            "{} WIN!",
            if player_setting.current_player == false {
                "Player 2"
            } else {
                "Player 1"
            }
        );
        // let window = windows.get_primary_mut().unwrap();
        // window.set_cursor_lock_mode(true);
    }
}
