use itertools::iproduct;
use std::fmt::Debug;

use crate::{game::*, minimax::*};

#[derive(Clone)]
pub struct TicTacToeGame {
    pub current_player: Player,
    pub board: BoardType,
}

impl TicTacToeGame {
    /// Does not validate if the state is correct or reachable (ie might have board filled with X)
    pub fn from_state(board_str: &str, current_player: Player) -> Self {
        let mut game = TicTacToeGame::default();
        game.current_player = current_player;
        let board_chars = board_str.chars().filter(|c| !c.is_whitespace());
        iproduct!(0..3, 0..3)
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
}

type BoardType = [Player; 9];

trait BoardTypeAccess {
    fn get(&self, i: usize, j: usize) -> Player;
    fn set(&mut self, i: usize, j: usize, val: Player);
}

impl BoardTypeAccess for BoardType {
    fn get(&self, i: usize, j: usize) -> Player {
        self[i * 3 + j]
    }

    fn set(&mut self, i: usize, j: usize, val: Player) {
        self[i * 3 + j] = val
    }
}

impl Default for TicTacToeGame {
    fn default() -> Self {
        Self {
            current_player: Player::X,
            board: [Player::None; 9],
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

    fn get_possible_moves(&self) -> Box<dyn Iterator<Item = Move> + '_> {
        Box::new(iproduct!(0..3, 0..3).filter(|(i, j)| self.board.get(*i, *j) == Player::None))
    }

    fn apply_move(&self, next_move: Move) -> Box<dyn MinimaxDriver> {
        let mut new_game = Box::new(self.clone());
        new_game
            .board
            .set(next_move.0, next_move.1, self.current_player);

        new_game.current_player = self.current_player.next();
        return new_game;
    }

    fn get_hash(&self) -> GameHash {
        let mut hash: u128 = self
            .board
            .iter()
            .zip(1..10)
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

    fn evaluate_score(&self) -> EvaluationScore {
        match self.get_winner() {
            Player::None => EvaluationScore {
                score: 0,
                // Not sure that it's not terminal, could be a draw, but since the minimax function
                // will iterate over possible moves this shouldn't be an issue 
                is_terminal: false,
            },
            Player::X => EvaluationScore {
                score: 1000,
                is_terminal: true,
            },
            Player::O => EvaluationScore {
                score: -1000,
                is_terminal: true,
            },
        }
    }
}

impl Debug for TicTacToeGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..3 {
            for j in 0..3 {
                write!(f, "{} ", String::from(self.board.get(i, j)))?;
            }
            writeln!(f)?;
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
        let mut actual: Vec<Move> = game.get_possible_moves().collect();
        let mut expected = vec![(0, 1), (1, 1), (2, 1)];
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn test_winning_moves_one_turn() {
        let state = "
        X.X
        X.O
        O.O";
        let game = TicTacToeGame::from_state(state, Player::X);
        let mut minimax = Minimax::new(MinimaxParams::default());
        let node = minimax.minimax(&game);
        assert!([Some((0, 1)), Some((1, 1))].contains(&node.get_best_move()));

        let game = TicTacToeGame::from_state(state, Player::O);
        let mut minimax = Minimax::new(MinimaxParams::default());
        let node = minimax.minimax(&game);
        assert_eq!(Some((2, 1)), node.get_best_move());
    }

    fn play(game: TicTacToeGame) -> (Box<dyn MinimaxDriver>, i32) {
        let mut moves = 0;
        let mut minimax = Minimax::new(MinimaxParams::default());
        let mut decision_node = minimax.minimax(&game);
        let mut new_game = Box::new(game) as Box<dyn MinimaxDriver>;
        while decision_node.best_move.is_some() {
            let next_move = decision_node.best_move.unwrap();
            new_game = new_game.apply_move(next_move);
            decision_node = decision_node.moves.get(&next_move).unwrap().clone();
            moves += 1;
        }
        return (new_game, moves);
    }

    #[rstest]
    fn test_winning_moves_two_turns() {
        let state = "
        ...
        OXX
        ..O";
        let game = TicTacToeGame::from_state(state, Player::O);
        let (final_game, moves) = play(game);
        assert_eq!(moves, 3);
        assert_eq!(final_game.get_winner(), Player::O);
    }

    #[rstest]
    fn test_winning_moves_three_turns() {
        let state = "
        X..
        .O.
        O.X";
        let game = TicTacToeGame::from_state(state, Player::X);
        let (final_game, moves) = play(game);
        assert_eq!(moves, 3);
        assert_eq!(final_game.get_winner(), Player::X);

        let state = "
        X.O
        .O.
        ..X";
        let game = TicTacToeGame::from_state(state, Player::X);
        let (final_game, moves) = play(game);
        assert_eq!(moves, 3);
        assert_eq!(final_game.get_winner(), Player::X);
    }
    
    #[rstest]
    fn test_doesnt_make_noob_mistake() {
        let state = "
        X..
        .O.
        ..X";
        let game = TicTacToeGame::from_state(state, Player::O);
        let (final_game, _moves) = play(game);
        assert_eq!(final_game.get_winner(), Player::None);
    }
    
    #[rstest]
    fn test_punishes_noob_openings() {
        let state = "
        XO.
        ...
        ...";
        let game = TicTacToeGame::from_state(state, Player::X);
        let (final_game, moves) = play(game);
        assert_eq!(moves, 5);
        assert_eq!(final_game.get_winner(), Player::X);

        let state = "
        X..
        ...
        O..";
        let game = TicTacToeGame::from_state(state, Player::X);
        let (final_game, moves) = play(game);
        assert_eq!(moves, 5);
        assert_eq!(final_game.get_winner(), Player::X);
    }

    #[rstest]
    fn test_fastest_win() {
        let state = "
        OOX
        O.X
        X..";
        let game = TicTacToeGame::from_state(state, Player::X);
        let (final_game, moves) = play(game);
        assert_eq!(moves, 1);
        assert_eq!(final_game.get_winner(), Player::X);
    }

    #[test]
    fn test_best_moves_always_end_in_draw() {
        let state = "
        ...
        ...
        ...";
        let game = TicTacToeGame::from_state(state, Player::X);
        let (final_game, _moves) = play(game);
        assert_eq!(final_game.get_winner(), Player::None);
    }

    #[test]
    fn test_hash() {
        let state = "
        ...
        ...
        ...";
        let mut game = TicTacToeGame::from_state(state, Player::X);
        assert_eq!(game.get_hash(), 1);
        game.current_player = Player::O;
        assert_eq!(game.get_hash(), 2);

        let state = "
        X..
        ...
        ...";
        let mut game = TicTacToeGame::from_state(state, Player::X);
        assert_eq!(game.get_hash(), 5);
        game.current_player = Player::O;
        assert_eq!(game.get_hash(), 6);
    }

    #[fixture]
    fn log_collector() {
        // kind of hacky way to enable logs in tests
        let collector = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(collector).unwrap();
    }
}
