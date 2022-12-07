use bevy::prelude::*;

use crate::{tictactoe::TicTacToeGame, minimax::*};

pub struct TicTacToeGamePlugin;

impl Plugin for TicTacToeGamePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(GameResource::default())
        .add_startup_system(setup)
        .add_system(render_game)
        .add_system(make_move)
        .add_event::<GameStateChangedEvent>();
    }
}

// RESOURCES
#[derive(Resource, Default)]
struct GameResource {
    // game: Box<dyn MinimaxDriver + Sync + Send + core::fmt::Debug>,
    game: TicTacToeGame
}

// impl GameResource {
//     fn new() -> Self {
//         GameResource { game: Box::new(TicTacToeGame::default())}
//     }

// }

// EVENTS
struct GameStateChangedEvent;

// SYSTEMS
fn render_game(mut state_changed_event: EventReader<GameStateChangedEvent>, game: Res<GameResource>) {
    for event in state_changed_event.iter() {
        println!("{:?}", game.game)
    }
}

fn make_move(mut state_changed_event: EventReader<GameStateChangedEvent>, mut game: ResMut<GameResource>) {
    for event in state_changed_event.iter() {
        let boxed = Box::from(&game.game as &dyn MinimaxDriver);
        println!("Best move: {:?}", minimax(boxed).best_move);
    }
}

fn setup(mut state_changed_event: EventWriter<GameStateChangedEvent>) {
    state_changed_event.send(GameStateChangedEvent);
}