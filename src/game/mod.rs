use bevy::prelude::*;

pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;

use crate::{
    AppState, events::GameOver, game::
    {
        enemy::EnemyPlugin,
        player::PlayerPlugin,
        score::ScorePlugin,
        star::StarPlugin, systems::{pause_simulation, resume_simulation, toggle_simulation}
    }
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<SimulationState>()
            .add_message::<GameOver>()
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StarPlugin)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}