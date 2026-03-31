use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::{window::PrimaryWindow};

use crate::{AppState, events::*};

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single().expect("No primary window found");

    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn transition_to_game_state(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if app_state.get().clone() != AppState::Game {
            next_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if app_state.get().clone() != AppState::MainMenu {
            next_state.set(AppState::MainMenu);
            println!("Entered AppState::MainMenu");
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.write(AppExit::Success);
    }
}

pub fn handle_game_over(
    mut next_state: ResMut<NextState<AppState>>,
    mut game_over_event_reader: MessageReader<GameOver>
) {
    for event in game_over_event_reader.read() {
        println!("Your final score is: {}", event.score.to_string());
        next_state.set(AppState::GameOver);
    }
}