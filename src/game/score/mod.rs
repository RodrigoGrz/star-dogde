use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::{AppState, game::SimulationState};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()
            .init_resource::<ScoreRemoveEnemy>()
            .init_resource::<HighScores>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(Update, 
                (
                    update_score,
                    update_score_remove_enemy,
                )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)
            )
        )
            .add_systems(Update, update_high_scores)
            .add_systems(Update, high_scores_updated)
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}