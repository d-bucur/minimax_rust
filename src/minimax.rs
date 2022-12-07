use std::collections::HashMap;

use crate::game::*;

pub trait MinimaxDriver {
    fn get_winner(&self) -> Player;

    fn get_possible_moves(&self) -> Vec<Move>;

    fn apply_move(&mut self, next_move: Move);

    fn get_hash(&self); // TODO force implementation of Hash trait instead??
}

#[derive(Default)]
pub struct DecisionTreeNode {
    pub score: i32,
    // board needed?
    pub moves: HashMap<Move, DecisionTreeNode>,
    pub best_move: Move,
}


pub fn minimax(game: Box<&dyn MinimaxDriver>) -> DecisionTreeNode {
    DecisionTreeNode { ..Default::default() }
}