use itertools::iproduct;
use std::fmt::Debug;

use crate::{game::*, minimax::*};

pub struct TicTacToeGame {
    pub current_player: Player,
    pub board: [[Player; 3]; 3],
}

impl TicTacToeGame {
    fn _score(&self, i: usize, j: usize) -> i32 {
        match self.board[i][j] {
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
            let score: i32 = pos.map(|(i, j)| self._score(i, j)).sum();
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
            .filter(|(i, j)| self.board[*i][*j] == Player::None)
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

fn win_positions_to_check() -> impl Iterator<Item = impl Iterator<Item = (usize, usize)>> {
     // not sure if actually allocates arrays here, should profile or preallocate arrays
     // some weird iterators here needed to have the same type and be able to chain
    let horizontal = (0..3).map(|j| [0, 1, 2].into_iter().zip([j, j, j]));
    let vertical = (0..3).map(|i| [i, i, i].into_iter().zip([0, 1, 2]));
    let diagonal1 = (0..1).map(|_| [0, 1, 2].into_iter().zip([0, 1, 2]));
    let diagonal2 = (0..1).map(|_| [0, 1, 2].into_iter().zip([2, 1, 0]));
    horizontal.chain(vertical).chain(diagonal1).chain(diagonal2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_winner() {
        !todo!()
    }
}