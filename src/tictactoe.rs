use std::fmt::Debug;

use itertools::iproduct;

use crate::{game::*, minimax::*};

#[derive(Debug)]
pub struct TicTacToeGame {
    pub current_player: Player,
    pub board: [[Player; 3]; 3],
}

impl TicTacToeGame {
    fn new() -> Self {
        Self {
            current_player: Player::X,
            ..Default::default()
        }
    }
    fn _score(&self, x: usize, y: usize) -> i32 {
        match self.board[x][y] {
            Player::X => 1,
            Player::O => -1,
            Player::None => 0,
        }
    }
}

impl Default for TicTacToeGame {
    fn default() -> Self {
        Self {
            current_player: Player::X,
            board: [[Player::None; 3]; 3],
        }
    }
}

impl MinimaxDriver for TicTacToeGame {
    fn get_winner(&self) -> Player {
        let win_positions = [[(0usize, 0usize), (0usize, 1usize), (0usize, 2usize)]]; // TODO add all positions
        for pos in win_positions {
            let score: i32 = pos.iter().map(|(x, y)| self._score(*x, *y)).sum();
            if score == 3 {
                return Player::X;
            } else if score == -3 {
                return Player::O;
            }
        }
        return Player::None;
    }

    fn get_possible_moves(&self) -> Vec<Move> {
        iproduct!(0..3, 0..3)
            .filter(|(x, y)| self.board[*x][*y] == Player::None)
            .collect()
    }

    fn apply_move(&mut self, next_move: Move) {
        self.board[next_move.0][next_move.1] = self.current_player;
        self.current_player = self.current_player.next();
        // TODO should return new board instead?
    }

    fn get_hash(&self) {
        todo!()
    }
}
