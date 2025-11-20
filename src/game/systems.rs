use bevy::prelude::*;

use crate::{game::SimulationState};

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