use std::{collections::HashMap, rc::Rc};

use tracing::*;

use crate::game::*;

pub type GameHash = u128; // this won't be enough for chess for example
pub type Score = i32;
pub type NodeType = Rc<DecisionTreeNode>;

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
    pub score: Score,
    pub moves: HashMap<Move, NodeType>, // TODO can moves be a vector instead?
    pub best_move: Option<Move>,
    // TODO only for debug
    pub alfa: Score,
    pub beta: Score,
}

impl DecisionTreeNode {
    pub fn get_best_move(&self) -> Option<Move> {
        return self.best_move;
    }
}

pub struct Minimax {
    pub max_depth: u32,
    pub max_score: Score,
    pub depth_factor: f32,
    pub weight_suboptimal: f32,
    pub cache: HashMap<GameHash, NodeType>,
}

impl Default for Minimax {
    fn default() -> Self {
        Self {
            max_score: 1000,
            max_depth: 100,
            depth_factor: 0.99,
            weight_suboptimal: 0.,
            cache: Default::default(),
        }
    }
}

impl Minimax {
    // TOOD maybe use generic instead of dynamic dispatch
    pub fn minimax(&mut self, game: &dyn MinimaxDriver) -> NodeType {
        // TODO suboptimal breaks the pruning if too high, and way slower
        // disabling depth factor is also slightly faster
        self._minimax(game, 0, Score::MIN, Score::MAX)
    }

    fn _minimax(
        &mut self,
        game: &dyn MinimaxDriver,
        current_depth: u32,
        mut alfa: Score, // best for maximizing player
        mut beta: Score, // best for minimizing player
    ) -> NodeType {
        let cache_key = game.get_hash();
        if self.cache.contains_key(&cache_key) {
            return self.cache.get(&cache_key).unwrap().clone();
        }
        let winner = game.get_winner();
        if winner != Player::None {
            let score_multiplier = winner.score_multiplier();
            let node = Rc::new(DecisionTreeNode {
                score: score_multiplier * self.max_score,
                ..Default::default()
            });
            self.cache.insert(cache_key, node.clone());
            return node;
        }

        if current_depth >= self.max_depth {
            let node = Rc::new(DecisionTreeNode {
                ..Default::default()
            });
            self.cache.insert(cache_key, node.clone());
            return node;
        }

        let possible_moves = game.get_possible_moves();
        let new_states = possible_moves.iter().map(|&m| (m, game.apply_move(m)));

        // this is where the actual minmax happens!
        let mut best_move = None;
        let mut best_value = 0;
        let mut suboptimal_value = 0.;
        let mut analized_moves = 0;
        let score_multiplier = game.get_current_player().score_multiplier();
        let mut child_results_map: HashMap<(usize, usize), NodeType> = Default::default();

        for (pos, game) in new_states {
            let node_eval = self._minimax(game.as_ref(), current_depth + 1, alfa, beta);
            if node_eval.score * score_multiplier >= best_value || best_move.is_none() {
                best_move = Some(pos);
                best_value = node_eval.score * score_multiplier;
            }
            suboptimal_value += node_eval.score as f32;
            if score_multiplier > 0 {
                alfa = std::cmp::max(alfa, node_eval.score)
            } else {
                beta = std::cmp::min(beta, node_eval.score)
            }
            child_results_map.insert(pos, node_eval);

            // break early to prune solutions that will never be taken
            if beta <= alfa {
                // trace!("Pruning {}, {}", alfa, beta);
                break;
            }
            analized_moves += 1;
        }
        if analized_moves > 0 {
            suboptimal_value /= analized_moves as f32;
        }

        let node = Rc::new(DecisionTreeNode {
            best_move: best_move,
            score: (((best_value * score_multiplier) as f32 * (1. - self.weight_suboptimal)
                + suboptimal_value * self.weight_suboptimal)
                * self.depth_factor) as Score,
            moves: child_results_map,
            alfa: alfa,
            beta: beta,
        });
        debug!("Minimax in node: \n{:?}", game);
        debug!("Node: {:?}", node);
        debug!("Possible moves {:?}", possible_moves.len());
        self.cache.insert(cache_key, node.clone());
        node
    }
}

impl Player {
    pub fn score_multiplier(&self) -> Score {
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
