use std::{collections::HashMap, rc::Rc};

use tracing::*;

use crate::game::*;

pub type GameHash = u128; // this won't be enough for chess for example

pub trait MinimaxDriver: core::fmt::Debug {
    fn get_winner(&self) -> Player;

    fn get_possible_moves(&self) -> Vec<Move>; // TODO iterator instead?

    fn apply_move(&self, next_move: Move) -> Box<dyn MinimaxDriver>; // TODO move types should be specific for each game. Can probably use generics here

    fn get_hash(&self) -> GameHash; // TODO can't implement Hash because it is not object safe

    fn get_current_player(&self) -> Player;

    fn has_ended(&self) -> bool; // TODO not used at all?
}

#[derive(Default)]
pub struct DecisionTreeNode {
    // TODO need to divide into min and max scores?
    pub score: i32,
    // TODO is a reference to the board needed for debug?
    pub moves: HashMap<Move, Rc<DecisionTreeNode>>, // TODO can moves be a vector instead?
    pub best_move: Option<Move>,
}

impl DecisionTreeNode {
    pub fn get_best_move(&self) -> Option<Move> {
        return self.best_move;
    }
}

// TODO make struct and keep around stuff like the cache and visit count

// TODO extract params into struct with defaults
pub fn minimax(game: &dyn MinimaxDriver, max_depth: Option<u32>) -> Rc<DecisionTreeNode> {
    let mut cache: HashMap<GameHash, Rc<DecisionTreeNode>> = HashMap::new();
    _minimax(game, 0, 0.95, max_depth, 0.25, &mut cache)
}

// TODO profile with struct values instead of passing constant params
fn _minimax(
    game: &dyn MinimaxDriver,
    current_depth: u32,
    depth_factor: f32,
    max_depth: Option<u32>,
    weight_suboptimal: f32,
    cache: &mut HashMap<GameHash, Rc<DecisionTreeNode>>,
) -> Rc<DecisionTreeNode> {
    let cache_key = game.get_hash();
    if cache.contains_key(&cache_key) {
        return cache.get(&cache_key).unwrap().clone();
    }
    let winner = game.get_winner();
    if winner != Player::None {
        let score_multiplier = winner.score_multiplier();
        const MAX_SCORE: i32 = 100;
        let node = Rc::new(DecisionTreeNode {
            score: score_multiplier * MAX_SCORE,
            ..Default::default()
        });
        cache.insert(cache_key, node.clone());
        return node;
    }

    if current_depth >= max_depth.unwrap_or(1000) {
        let node = Rc::new(DecisionTreeNode {
            ..Default::default()
        });
        cache.insert(cache_key, node.clone());
        return node;
    }

    let possible_moves = game.get_possible_moves();
    let new_states = possible_moves.iter().map(|&m| (m, game.apply_move(m)));
    let child_results = new_states.map(|(pos, game)| {
        (
            pos,
            _minimax(
                game.as_ref(),
                current_depth + 1,
                depth_factor,
                max_depth,
                weight_suboptimal,
                cache,
            ),
        )
    });
    let child_results_map: HashMap<(usize, usize), Rc<DecisionTreeNode>> = child_results.collect();

    // this is where the actual minmax happens!
    let mut best_move = None;
    let mut best_value = 0;
    let mut suboptimal_value = 0.;
    let score_multiplier = game.get_current_player().score_multiplier();

    for (pos, node) in child_results_map.iter() {
        if node.score * score_multiplier >= best_value || best_move.is_none() {
            best_move = Some(pos);
            best_value = node.score * score_multiplier;
        }
        suboptimal_value += node.score as f32;
    }
    suboptimal_value /= possible_moves.len() as f32;

    let node = Rc::new(DecisionTreeNode {
        best_move: best_move.cloned(),
        score: (((best_value * score_multiplier) as f32 * (1. - weight_suboptimal)
            + suboptimal_value * weight_suboptimal)
            * depth_factor) as i32,
        moves: child_results_map,
    });
    debug!("Minimax in node: \n{:?}", game);
    debug!("Node: {:?}", node);
    debug!("Possible moves {:?}", possible_moves.len());
    debug!("-------------");
    cache.insert(cache_key, node.clone());
    node
    // TODO node cache
}

impl Player {
    pub fn score_multiplier(&self) -> i32 {
        match &self {
            Player::X => 1,
            Player::O => -1,
            Player::None => 0,
        }
    }
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
