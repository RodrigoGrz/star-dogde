mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::main_menu::systems::interactions::*;
use crate::main_menu::systems::layout::{despawn_main_menu, spawn_game_over_menu, spawn_main_menu};

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Main Menu
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(Update, (
                interact_with_play_button,
                interact_with_quit_button,
            ).run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)

            .add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(Update, (
                interact_with_restart_button,
                interact_with_back_to_menu_button,
            ).run_if(in_state(AppState::GameOver)))
            .add_systems(OnExit(AppState::GameOver), despawn_main_menu);
    }
}