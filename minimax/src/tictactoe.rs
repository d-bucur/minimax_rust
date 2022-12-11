use itertools::iproduct;
use std::fmt::Debug;

use crate::{game::*, minimax::*};

#[derive(Clone)]
pub struct TicTacToeGame {
    pub current_player: Player,
    pub board: [[Player; 3]; 3], // TODO optimize into array
}

impl TicTacToeGame {
    /// Does not validate if the state is correct or reachable (ie might have board filled with X)
    pub fn from_state(board_str: &str, current_player: Player) -> Self {
        let mut game = TicTacToeGame::default();
        game.current_player = current_player;
        let board_chars = board_str.chars().filter(|c| !c.is_whitespace());
        iproduct!(0..3, 0..3)
            .zip(board_chars)
            .for_each(|((i, j), c)| game.board[i][j] = Player::from(c));
        return game;
    }

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

    fn apply_move(&self, next_move: Move) -> Box<dyn MinimaxDriver> {
        let mut new_game = Box::new(self.clone());
        new_game.board[next_move.0][next_move.1] = self.current_player;

        new_game.current_player = if new_game.get_winner() == Player::None {
            // TODO should cache winner to avoid computing 2 times
            self.current_player.next()
        } else {
            Player::None
        };
        return new_game;
    }

    fn get_hash(&self) {
        todo!()
    }

    fn get_current_player(&self) -> Player {
        self.current_player
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
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        "
        XXX
        O.O
        ..."
    )]
    #[case(
        "
        X.X
        OXO
        X.."
    )]
    #[case(
        "
        X.X
        OXO
        ..X"
    )]
    #[case(
        "
        OXO
        OXO
        .XX"
    )]
    fn test_winner_is_detected(#[case] board_str: &str) {
        let game = TicTacToeGame::from_state(board_str, crate::game::Player::O);
        assert_eq!(game.get_winner(), Player::X);
    }

    #[test]
    fn test_get_possible_moves() {
        let state = "
        X.X
        O.O
        X.O";
        let game = TicTacToeGame::from_state(state, Player::X);
        let mut actual = game.get_possible_moves();
        let mut expected = vec![(0, 1), (1, 1), (2, 1)];
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_winning_moves_one_turn(log_collector: ()) {
        let state = "
        X.X
        X.O
        O.O";
        let game = TicTacToeGame::from_state(state, Player::X);
        let node = minimax(&game);
        assert!([Some((0, 1)), Some((1, 1))].contains(&node.get_best_move()));

        let game = TicTacToeGame::from_state(state, Player::O);
        let node = minimax(&game);
        assert_eq!(Some((2, 1)), node.get_best_move());
    }

    #[test]
    fn test_game_is_not_winnable() {
        let state = "
        ...
        ...
        ...";
        let game = TicTacToeGame::from_state(state, Player::X);
        todo!();
    }

    #[fixture]
    fn log_collector() {
        // kind of hacky way to enable logs in tests
        let collector = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(collector);
    }
}
