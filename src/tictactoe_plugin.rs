use bevy::{ecs::schedule::ShouldRun, prelude::*};

use crate::{game::*, minimax::*, tictactoe::TicTacToeGame};

pub struct TicTacToeGamePlugin;

impl Plugin for TicTacToeGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameResource::default())
            .add_startup_system(setup)
            .add_system(render_game)
            .add_system(make_move.with_run_criteria(should_move))
            .add_event::<GameStateChangedEvent>();
    }
}

// RESOURCES
#[derive(Resource, Default)]
struct GameResource(TicTacToeGame);

// EVENTS
struct GameStateChangedEvent;

// SYSTEMS
fn render_game(
    // TODO can probably listen for bevy changed events instead of generating own
    mut state_changed_event: EventReader<GameStateChangedEvent>,
    game: Res<GameResource>,
) {
    for event in state_changed_event.iter() {
        println!("{:?}", game.0)
    }
}

fn make_move(
    mut state_changed_event: EventWriter<GameStateChangedEvent>,
    mut game: ResMut<GameResource>,
) {
    let best_move = minimax(&game.0).best_move;
    println!("Best move: {:?}", best_move);
    if let Some(best_move) = best_move {
        game.0.apply_move(best_move);
        let winner = game.0.get_winner();
        if winner == Player::None {
            state_changed_event.send(GameStateChangedEvent);
        } else {
            println!("{:?}", game.0);
            println!("{:?} won", winner);
        }
    }
}

fn should_move(mut state_changed_event: EventReader<GameStateChangedEvent>) -> ShouldRun {
    for _ in state_changed_event.iter() {
        return ShouldRun::Yes;
    }
    return ShouldRun::No;
}

fn setup(mut state_changed_event: EventWriter<GameStateChangedEvent>) {
    state_changed_event.send(GameStateChangedEvent);
}
