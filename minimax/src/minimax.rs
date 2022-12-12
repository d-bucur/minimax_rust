use std::collections::HashMap;

use tracing::*;

use crate::game::*;

pub trait MinimaxDriver: core::fmt::Debug {
    fn get_winner(&self) -> Player;

    fn get_possible_moves(&self) -> Vec<Move>; // TODO iterator instead?

    fn apply_move(&self, next_move: Move) -> Box<dyn MinimaxDriver>; // TODO move types should be specific for each game

    fn get_hash(&self); // TODO can't implement Hash because it is not object safe

    fn get_current_player(&self) -> Player;

    fn has_ended(&self) -> bool;
}

#[derive(Default)]
pub struct DecisionTreeNode {
    // TODO need to divide into min and max scores?
    pub score: i32,
    // TODO is a reference to the board needed for debug?
    pub moves: HashMap<Move, DecisionTreeNode>,
    pub best_move: Option<Move>,
}

impl DecisionTreeNode {
    pub fn get_best_move(&self) -> Option<Move> {
        return self.best_move;
    }
}

// TODO extract params into struct with defaults
pub fn minimax(game: &dyn MinimaxDriver, max_depth: Option<u32>) -> DecisionTreeNode {
    _minimax(game, 0, 0.95, max_depth)
}

// TODO profile with struct values instead of passing constant params
pub fn _minimax(
    game: &dyn MinimaxDriver,
    current_depth: u32,
    depth_factor: f32,
    max_depth: Option<u32>,
) -> DecisionTreeNode {
    let winner = game.get_winner();

    if winner != Player::None {
        let score_multiplier = match winner {
            Player::X => 1,
            Player::O => -1,
            Player::None => 0,
        };
        const MAX_SCORE: i32 = 100;
        return DecisionTreeNode {
            score: score_multiplier * MAX_SCORE,
            ..Default::default()
        };
    }

    if current_depth >= max_depth.unwrap_or(1000) {
        return DecisionTreeNode {
            ..Default::default()
        };
    }

    let possible_moves = game.get_possible_moves();
    let new_states = possible_moves.iter().map(|&m| (m, game.apply_move(m)));
    let child_results = new_states.map(|(m, g)| {
        (
            m,
            _minimax(g.as_ref(), current_depth + 1, depth_factor, max_depth),
        )
    });
    let child_results_map: HashMap<(usize, usize), DecisionTreeNode> = child_results.collect();

    // this is where the actual minmax happens!
    let best_move_in_child = if game.get_current_player() == Player::O {
        child_results_map
            .iter()
            .min_by_key(|(_pos, node)| node.score)
    } else {
        child_results_map
            .iter()
            .max_by_key(|(_pos, node)| node.score)
    };

    // TODO refactor to combine with struct below
    let (max_pos, max_val) = if best_move_in_child.is_some() {
        (
            Some(best_move_in_child.unwrap().0.clone()),
            best_move_in_child.unwrap().1.score,
        )
    } else {
        (None, 0)
    };

    let node = DecisionTreeNode {
        best_move: max_pos,
        score: (max_val as f32 * depth_factor) as i32,
        moves: child_results_map,
    };
    debug!("Minimax in node: \n{:?}", game);
    debug!("Node: {:?}", node);
    debug!("Possible moves {:?}", possible_moves.len());
    debug!("-------------");
    node
    // TODO node cache
}

impl core::fmt::Debug for DecisionTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecisionTreeNode")
            .field("score", &self.score)
            .field("best_move", &self.best_move)
            .field("moves", &format_args!("{:?}", self.moves.keys())) // stop on keys to avoid recursion
            .finish()
    }
}
