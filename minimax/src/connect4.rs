use std::{fmt::Debug, iter::repeat};

use crate::{game::*, minimax::*};

const WIDTH: usize = 7;
const HEIGHT: usize = 6;

#[derive(Clone)]
pub struct Connect4Game {
    pub current_player: Player,
    pub board: BoardType,
    last_move: Option<Move>,
}

impl Connect4Game {
    pub fn from_state(board_str: &str, last_move: Option<Move>, current_player: Player) -> Self {
        let mut game = Connect4Game::default();
        game.current_player = current_player;
        game.last_move = last_move;
        let board_chars = board_str.chars().filter(|c| !c.is_whitespace());
        itertools::iproduct!(0..HEIGHT, 0..WIDTH)
            .zip(board_chars)
            .for_each(|((i, j), c)| game.board.set(i, j, Player::from(c)));
        return game;
    }

    fn _score(&self, i: usize, j: usize) -> i32 {
        match self.board.get(i, j) {
            Player::X => 1,
            Player::O => -1,
            Player::None => 0,
        }
    }

    fn _search_for_winner(&self, dir_less: (i32, i32), dir_more: (i32, i32)) -> Option<Player> {
        let last_move = self.last_move.unwrap();
        let last_player = self.current_player.next();
        // expand in two directions to find connected pieces
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
            if self.board.get(a.0 as usize, a.1 as usize) != last_player {
                break;
            }
            width += 1;
            a.0 += dir_less.0;
            a.1 += dir_less.1;
        }
        while width < 5 && b.1 < WIDTH as i32 && b.0 > 0 && b.0 < HEIGHT as i32 {
            if self.board.get(b.0 as usize, b.1 as usize) != last_player {
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

type BoardType = [Player; WIDTH * HEIGHT];

trait BoardTypeAccess {
    fn get(&self, i: usize, j: usize) -> Player;
    fn set(&mut self, i: usize, j: usize, val: Player);
}

impl BoardTypeAccess for BoardType {
    fn get(&self, i: usize, j: usize) -> Player {
        self[i * WIDTH + j]
    }

    fn set(&mut self, i: usize, j: usize, val: Player) {
        self[i * WIDTH + j] = val
    }
}

impl MinimaxDriver for Connect4Game {
    fn get_winner(&self) -> Player {
        if let Some(_) = self.last_move {
            const DIRECTIONS: [((i32, i32), (i32, i32)); 4] = [
                ((0, -1), (0, 1)),
                ((-1, -1), (1, 1)),
                ((-1, 0), (1, 0)),
                ((1, -1), (-1, 1)),
            ];
            for dir in DIRECTIONS {
                if let Some(winner) = self._search_for_winner(dir.0, dir.1) {
                    return winner;
                }
            }
        }
        return Player::None;
    }

    fn get_possible_moves(&self) -> Box<dyn Iterator<Item = Move> + '_> {
        Box::new(
            (0..WIDTH)
                .map(|j| {
                    (0..HEIGHT)
                        .rev()
                        .zip(repeat(j))
                        .find(|(i, j)| self.board.get(*i, *j) == Player::None)
                })
                .filter(|&p| p.is_some())
                .map(|p| p.unwrap()),
        )
    }

    /// No checks are applied. Assumes that the move has been taken from [`get_possible_moves()`]
    fn apply_move(&self, next_move: Move) -> Box<dyn MinimaxDriver> {
        let mut new_game = Box::new(self.clone());
        new_game
            .board
            .set(next_move.0, next_move.1, self.current_player);
        new_game.current_player = self.current_player.next();
        new_game.last_move = Some(next_move);
        return new_game;
    }

    fn get_hash(&self) -> GameHash {
        // let grid_values = self.board.iter().flat_map(|r| r.iter().map(|p| p));
        let mut hash: u128 = self
            .board
            .iter()
            .zip(1..43)
            .map(|(val, pos)| (*val as u128) * 4u128.pow(pos))
            .sum();
        hash += self.current_player as u128;
        hash
    }

    fn get_current_player(&self) -> Player {
        self.current_player
    }

    fn has_ended(&self) -> bool {
        self.get_winner() != Player::None || self.get_possible_moves().count() == 0
    }
}

impl Debug for Connect4Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                write!(f, "{} ", String::from(self.board.get(i, j)))?;
            }
            writeln!(f)?;
        }
        write!(f, "next: {:?}", &self.current_player)
    }
}

impl Default for Connect4Game {
    fn default() -> Self {
        Self {
            current_player: Player::X,
            board: [Player::None; WIDTH * HEIGHT],
            last_move: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::minimax::Minimax;

    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        "
        .......
        .......
        .......
        .......
        .......
        .XXXX..",
        Some((5, 3))
    )]
    #[case(
        "
        .......
        .......
        .X.....
        .X.....
        .X.....
        .XOOO..",
        Some((2, 1))
    )]
    #[case(
        "
        .......
        .......
        ....X..
        .X.XO..
        .XXOO..
        .XOOO..",
        Some((3, 3))
    )]
    #[case(
        "
        .......
        .......
        .X.....
        .OX....
        .OXX...
        .OOXX..",
        Some((4, 3))
    )]
    fn test_winner_is_detected(#[case] board_str: &str, #[case] last_move: Option<Move>) {
        let game = Connect4Game::from_state(board_str, last_move, crate::game::Player::O);
        println!("{:?}", game);
        assert_eq!(game.get_winner(), Player::X);
    }

    #[test]
    fn test_possible_moves() {
        let state = "
        .......
        .O.X...
        .XOO...
        .OXOX..
        .OXOXX.
        .OOXXO.";
        let game = Connect4Game::from_state(state, Some((4, 5)), crate::game::Player::O);
        let mut actual: Vec<Move> = game.get_possible_moves().collect();
        let mut expected = vec![(5, 0), (0, 1), (1, 2), (0, 3), (2, 4), (3, 5), (5, 6)];
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_winning_moves_one_turn() {
        let mut minimax = Minimax::new(MinimaxParams {
            max_depth: 1,
            ..Default::default()
        });
        let state = "
        .......
        .O.X...
        .XOO...
        .OXOX..
        .OXOXX.
        .OOXXO.";
        let game = Connect4Game::from_state(state, Some((4, 5)), crate::game::Player::X);
        let node = minimax.minimax(&game);
        assert_eq!(node.get_best_move(), Some((2, 4)));
    }

    // TODO test with longer winning moves
}
