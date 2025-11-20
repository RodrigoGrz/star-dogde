pub mod events;
mod systems;

pub mod enemy;
mod player;
pub mod score;
pub mod star;

use events::*;
use systems::*;

use bevy::prelude::*;

use crate::{enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin, star::StarPlugin};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_event::<GameOver>()
    .add_plugins(EnemyPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(ScorePlugin)
    .add_plugins(StarPlugin)
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, exit_game)
    .add_systems(Update, handle_game_over)
    .run();
}

