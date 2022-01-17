use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

mod components;
mod constants;
mod resources;
mod systems;

// use components::*;
use constants::*;
use resources::*;
use systems::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: WINDOW_SIZE,
            height: WINDOW_SIZE,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_stage("stage_setup", SystemStage::single(create_stage.system()))
        .add_system_set(
            SystemSet::new()
                .label("Input")
                .with_system(cursor_system.system())
                .with_system(drug_system.system())
                .with_system(game_over_system.system()),
        )
        .add_system(
            selection_validation_system
                .system()
                .label("Validation")
                .after("Input"),
        )
        .add_system_set(
            SystemSet::new()
                .label("Action")
                .with_system(change_color_system.system())
                .with_system(delete_tiles_system.system())
                .after("Validation"),
        )
        .add_system_set(
            SystemSet::new()
                .after("Action")
                .with_run_criteria(run_if_deleted_tile.system())
                .with_system(switch_player_system.system())
                .with_system(check_game_over_system.system()),
        )
        .add_plugins(DefaultPlugins)
        .add_event::<SelectedEvent>()
        .add_event::<DeletedTileEvent>()
        .add_event::<GameOverEvent>()
        .insert_resource(ClearColor(COLOR_BACKGROUND.into()))
        .insert_resource(Cursor::default())
        .insert_resource(Selection::default())
        .insert_resource(PlayerSetting::default())
        //debug
        .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
