use bevy::prelude::*;

mod game;
mod minimax;
mod tictactoe;
mod tictactoe_plugin;

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
        .add_startup_system(setup)
        .add_plugin(tictactoe_plugin::TicTacToeGamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}