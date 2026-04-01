use bevy::prelude::*;

use super::resources::*;
use crate::{events::GameOver, game::enemy::components::Enemy};

pub fn insert_score(
    mut commands: Commands,
) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(
    mut commands: Commands
) {
    commands.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>) {
    if score .is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: MessageReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}

pub fn update_score_remove_enemy(
    mut commands: Commands,
    mut score_remove_enemy: ResMut<ScoreRemoveEnemy>,
    enemies: Query<Entity, With<Enemy>>,
) {
    if score_remove_enemy.is_changed() {
        if score_remove_enemy.value >= 10 {
            if let Some(enemy) = enemies.iter().next() {
                commands.entity(enemy).despawn();
            }
            score_remove_enemy.value = 0;
        }
    }
}