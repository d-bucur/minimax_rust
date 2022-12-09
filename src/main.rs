use bevy::prelude::*;

mod connect4;
mod game;
mod minimax;
mod tictactoe;
mod tictactoe_plugin;

// TODO connect other states and plugins
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum AppState {
    Menu,
    TicTacToe,
    Connect4,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy test".to_string(),
                width: 1024.,
                height: 600.,
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_state(AppState::TicTacToe)
        .add_startup_system(setup)
        .add_plugin(tictactoe_plugin::TicTacToeGamePlugin)
        .run();
}

// SYSTEMS
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
