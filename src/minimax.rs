use std::collections::HashMap;

use rand::seq::SliceRandom;
use tracing::*;

use crate::game::*;

pub trait MinimaxDriver {
    fn get_winner(&self) -> Player;

    fn get_possible_moves(&self) -> Vec<Move>; // TODO iterator instead?

    fn apply_move(&mut self, next_move: Move); // TODO move types should be specific for each game

    fn get_hash(&self); // TODO can't implement Hash because it is not object safe
}

#[derive(Default)]
pub struct DecisionTreeNode {
    pub score: i32,
    // board needed?
    pub moves: HashMap<Move, DecisionTreeNode>,
    pub best_move: Option<Move>,
}

impl DecisionTreeNode {
    pub fn get_best_move(&self) -> Move {
        return (0, 0)
    }
}

pub fn minimax(game: &dyn MinimaxDriver) -> DecisionTreeNode {
    // TODO actual implementation
    let possible_moves = game.get_possible_moves();
    warn!("Possible moves {:?}", possible_moves.len());
    DecisionTreeNode {
        best_move: possible_moves.choose(&mut rand::thread_rng()).cloned(),
        ..Default::default()
    }
}
