use bevy::prelude::*;

use crate::{game::SimulationState};

#[derive(Component)]
pub struct PauseOverlay;

pub fn pause_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    mut next_state: ResMut<NextState<SimulationState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if simulation_state.get().clone() == SimulationState::Running {
            next_state.set(SimulationState::Paused);
            println!("Simulation Paused.");
        }
        if simulation_state.get().clone() == SimulationState::Paused {
            next_state.set(SimulationState::Running);
            println!("Simulation Running.");
        }
    }
}

pub fn spawn_pause_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        PauseOverlay,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("PAUSED"),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 80.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    });
}

pub fn despawn_pause_overlay(
    mut commands: Commands,
    query: Query<Entity, With<PauseOverlay>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}