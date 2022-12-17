use std::{collections::HashMap, rc::Rc};

use tracing::*;

use crate::game::*;

pub type GameHash = u128; // this won't be enough for chess for example
pub type Score = i32;
pub type NodeType = Rc<DecisionTreeNode>;

pub struct EvaluationScore {
    pub score: Score,
    pub is_terminal: bool,
}

pub trait MinimaxDriver: core::fmt::Debug {
    fn get_winner(&self) -> Player; // TODO remove from trait, not needed anymore

    fn evaluate_score(&self) -> EvaluationScore;

    fn get_possible_moves(&self) -> Box<dyn Iterator<Item = Move> + '_>; // TODO should move into evaluation to avoid doing it twice

    fn apply_move(&self, next_move: Move) -> Box<dyn MinimaxDriver>; // TODO move types should be specific for each game. Can probably use generics here

    fn get_hash(&self) -> GameHash; // TODO can't implement Hash because it is not object safe

    fn get_current_player(&self) -> Player; // TODO only needed to know if maximizing player or minimizing player. maybe better to abstract this somehow?

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

pub struct MinimaxParams {
    pub max_depth: u32,
    pub depth_factor: f32,
    pub weight_suboptimal: f32,
}

impl Default for MinimaxParams {
    fn default() -> Self {
        Self {
            max_depth: 12,
            depth_factor: 0.99,
            weight_suboptimal: 0.,
        }
    }
}

pub struct Minimax {
    params: MinimaxParams,
    cache: HashMap<GameHash, NodeType>,
    nodes_examined_total: u128, // very optimistic size, would probably run out of memory before that
    nodes_examined_last_run: u128,
}

impl Minimax {
    pub fn new(params: MinimaxParams) -> Self {
        Self {
            params: params,
            cache: Default::default(),
            nodes_examined_last_run: 0,
            nodes_examined_total: 0,
        }
    }

    pub fn get_internal_stats(&self) -> (u128, u128, usize) {
        (
            self.nodes_examined_total,
            self.nodes_examined_last_run,
            self.cache.len(),
        )
    }
}

impl Minimax {
    // TOOD maybe use generic instead of dynamic dispatch
    pub fn minimax(&mut self, game: &dyn MinimaxDriver) -> NodeType {
        // TODO suboptimal breaks the pruning if too high, and way slower
        // disabling depth factor is also slightly faster
        let previous_total = self.nodes_examined_total;
        let res = self._minimax(game, 0, Score::MIN, Score::MAX);
        self.nodes_examined_last_run = self.nodes_examined_total - previous_total;
        res
    }

    fn _minimax(
        &mut self,
        game: &dyn MinimaxDriver,
        current_depth: u32,
        mut alfa: Score, // best for maximizing player
        mut beta: Score, // best for minimizing player
    ) -> NodeType {
        let cache_key = game.get_hash();
        // TODO caching breaks with pruning, not sure how to solve this. see test_doesnt_make_noob_mistake
        // either make caching optional, as it seems to work better than pruning for tictactoe
        // or remove completely
        // if self.cache.contains_key(&cache_key) {
        //     return self.cache.get(&cache_key).unwrap().clone();
        // }

        let score_eval = game.evaluate_score();
        if score_eval.is_terminal || current_depth >= self.params.max_depth {
            let node = Rc::new(DecisionTreeNode {
                score: score_eval.score,
                ..Default::default()
            });
            self.cache.insert(cache_key, node.clone());
            return node;
        }

        let new_states = game.get_possible_moves().map(|m| (m, game.apply_move(m)));
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

            analized_moves += 1;
            // break early to prune solutions that will never be taken
            if beta <= alfa {
                // trace!("Pruning {}, {}", alfa, beta);
                break;
            }
        }
        if analized_moves > 0 {
            suboptimal_value /= analized_moves as f32;
        }

        self.nodes_examined_total += 1;

        let score_final = if analized_moves == 0 {
            // could this break when using heuristics?
            score_eval.score
        } else {
            (((best_value * score_multiplier) as f32 * (1. - self.params.weight_suboptimal)
                + suboptimal_value * self.params.weight_suboptimal)
                * self.params.depth_factor) as Score
        };

        // return the tree node
        let node = Rc::new(DecisionTreeNode {
            best_move: best_move,
            score: score_final,
            moves: child_results_map,
            alfa: alfa,
            beta: beta,
        });
        debug!("Minimax in node: \n{:?}", game);
        debug!("Node: {:?}", node);
        // TODO big bug for progressive deepening: cache size is bigger than examined nodes.
        // the cache will keep nodes that have been pruned away, but the result might change based on what the root node is, and thus the starting move
        // cache could be invalidated on each run as a simple solution, but maybe some cache values on the selected branches can be kept
        // need to dive deeper into this
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
