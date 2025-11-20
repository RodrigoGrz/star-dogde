pub mod events;
mod systems;
mod game;
mod main_menu;

use systems::*;

use bevy::prelude::*;

use crate::{game::GamePlugin, main_menu::MainMenuPlugin};

fn main() {
    App::new()
        // Bevy Plugin
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        // My Plugins
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        // Startup Systems
        .add_systems(Startup, spawn_camera)
        // Systems
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu_state)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run();
}

#[derive(States, Debug, Hash, Clone, Copy, Eq, PartialEq, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}