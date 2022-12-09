use std::{cmp::min, fmt::Debug, iter::repeat};

use bevy::prelude::UVec2;

use crate::{game::*, minimax::MinimaxDriver};

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

pub struct Connect4Game {
    pub current_player: Player,
    pub board: [[Player; WIDTH]; HEIGHT],
    last_move: Option<Move>,
}

impl Connect4Game {
    fn _score(&self, i: usize, j: usize) -> i32 {
        match self.board[i][j] {
            Player::X => 1,
            Player::O => -1,
            Player::None => 0,
        }
    }

    fn _search_for_winner(&self, dir_less: (i32, i32), dir_more: (i32, i32)) -> Option<Player> {
        let last_move = self.last_move.unwrap();
        let last_player = self.current_player.next();
        // don't really need both a and b, can use only one
        let mut a = (
            last_move.0 as i32 + dir_less.0,
            last_move.1 as i32 + dir_less.1,
        );
        let mut b = (
            last_move.0 as i32 + dir_more.0,
            last_move.1 as i32 + dir_more.1,
        );
        let mut width = 1;
        while width < 5 && a.1 > 0 && a.0 > 0 && a.0 < HEIGHT as i32 {
            if self.board[a.0 as usize][a.1 as usize] != last_player {
                break;
            }
            width += 1;
            a.0 += dir_less.0;
            a.1 += dir_less.1;
        }
        while width < 5 && b.1 < WIDTH as i32 && b.0 > 0 && b.0 < HEIGHT as i32 {
            if self.board[b.0 as usize][b.1 as usize] != last_player {
                break;
            }
            width += 1;
            b.0 += dir_more.0;
            b.1 += dir_more.1;
        }
        if width >= 4 {
            return Some(last_player);
        }
        return None;
    }
}

impl MinimaxDriver for Connect4Game {
    fn get_winner(&self) -> Player {
        if let Some(_) = self.last_move {
            const DIRECTIONS: [((i32, i32), (i32, i32)); 4] = [
                ((0, -1), (0, 1)),
                ((-1, -1), (1, 1)),
                ((-1, 0), (1, 0)),
                ((1, -1), (1, 1)),
            ]; // hope this gets optimized by compiler. Should profile
            for dir in DIRECTIONS {
                if let Some(winner) = self._search_for_winner(dir.0, dir.1) {
                    return winner;
                }
            }
        }
        return Player::None;
    }

    fn get_possible_moves(&self) -> Vec<Move> {
        (0..WIDTH)
            .map(|j| {
                (0..HEIGHT)
                    .rev()
                    .zip(repeat(j))
                    .find(|(i, j)| self.board[*i][*j] == Player::None)
            })
            .filter(|&p| p.is_some())
            .map(|p| p.unwrap())
            .collect()
    }

    fn apply_move(&mut self, next_move: Move) {
        self.board[next_move.0][next_move.1] = self.current_player;
        self.current_player = if self.get_winner() == Player::None {
            // TODO should cache winner to avoid computing 2 times
            self.current_player.next()
        } else {
            Player::None
        };
        self.last_move = Some(next_move)
    }

    fn get_hash(&self) {
        todo!()
    }
}

impl Debug for Connect4Game {
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

impl Default for Connect4Game {
    fn default() -> Self {
        Self {
            current_player: Player::X,
            board: [[Player::None; WIDTH]; HEIGHT],
            last_move: None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_winner() {
        !todo!()
    }
}