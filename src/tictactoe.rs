use std::{fmt::Debug, iter::repeat};

use bevy::reflect::Tuple;
use itertools::iproduct;

use crate::{game::*, minimax::*};

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
        for pos in win_positions_to_check() {
            let score: i32 = pos.map(|(x, y)| self._score(x, y)).sum();
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
        self.current_player = if self.get_winner() == Player::None {
            // TODO should cache winner to avoid computing 2 times
            self.current_player.next()
        } else {
            Player::None
        }
        // TODO should return new board instead?
    }

    fn get_hash(&self) {
        todo!()
    }
}

impl Debug for TicTacToeGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            for &cell in row {
                write!(f, "{} ", String::from(cell));
            }
            writeln!(f);
        }
        write!(f, "next: {:?}", &self.current_player)
    }
}

impl From<Player> for String {
    fn from(player: Player) -> Self {
        match player {
            Player::X => "X",
            Player::O => "O",
            Player::None => ".",
        }
        .into()
    }
}

fn win_positions_to_check() -> impl Iterator<Item = impl Iterator<Item = (usize, usize)>> {
    let horizontal = (0..3).map(|y| [0, 1, 2].into_iter().zip([y, y, y])); // not sure if actually allocates arrays here, need to profile or preallocate arrays
    let vertical = (0..3).map(|x| [x, x, x].into_iter().zip([0, 1, 2]));
    // some weird iterators here needed to have the same type and be able to chain
    let diagonal1 = (0..1).map(|_| [0, 1, 2].into_iter().zip([0, 1, 2]));
    let diagonal2 = (0..1).map(|_| [0, 1, 2].into_iter().zip([2, 1, 0]));
    horizontal.chain(vertical).chain(diagonal1).chain(diagonal2)
}
