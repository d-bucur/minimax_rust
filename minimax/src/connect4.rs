use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    iter::repeat,
};

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
    /// Does not validate if the state is correct or reachable (ie might have board filled with X)
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
        while width < 5 && a.1 >= 0 && a.0 >= 0 && a.0 < HEIGHT as i32 {
            if self.board.get(a.0 as usize, a.1 as usize) != last_player {
                break;
            }
            width += 1;
            a.0 += dir_less.0;
            a.1 += dir_less.1;
        }
        while width < 5 && b.1 < WIDTH as i32 && b.0 >= 0 && b.0 < HEIGHT as i32 {
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
    fn get_safe(&self, i: isize, j: isize) -> Option<Player>;
    fn set(&mut self, i: usize, j: usize, val: Player);
}

impl BoardTypeAccess for BoardType {
    fn get(&self, i: usize, j: usize) -> Player {
        self[i * WIDTH + j]
    }

    fn set(&mut self, i: usize, j: usize, val: Player) {
        self[i * WIDTH + j] = val
    }

    fn get_safe(&self, i: isize, j: isize) -> Option<Player> {
        if i < 0 || i >= WIDTH as isize || j < 0 || j >= HEIGHT as isize {
            return None;
        }
        Some(self.get(i as usize, j as usize))
    }
}

impl MinimaxDriver for Connect4Game {
    fn get_winner(&self) -> Player {
        if let Some(_) = self.last_move {
            const DIRECTIONS: [((i32, i32), (i32, i32)); 4] = [
                ((0, -1), (0, 1)),  // horizontal search
                ((-1, -1), (1, 1)), // diagonal \
                ((-1, 0), (1, 0)),  // vertical search
                ((1, -1), (-1, 1)), // diagonal /
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

    fn evaluate_score(&self) -> EvaluationScore {
        const MAX_SCORE: i32 = 1000;
        // TODO this part is horrible, should refactor after it's working
        // this should replace winner function when done
        let mut score = 0;
        let mut threats: [HashSet<Move>; 3] = Default::default();

        let direction_iterators: Vec<Box<dyn Iterator<Item = ((isize, isize), (isize, isize))>>> = vec![
            // sweep right
            Box::new(
                (0..HEIGHT as isize)
                    .rev()
                    .zip(repeat(0))
                    .zip(repeat((0, 1))),
            ),
            // sweep up
            Box::new(
                repeat(HEIGHT as isize - 1)
                    .zip(0..WIDTH as isize)
                    .zip(repeat((-1, 0))),
            ),
            // sweep diagonal \
            Box::new(
                repeat(HEIGHT as isize - 1)
                    .zip(1..WIDTH as isize)
                    // add some higher positions manually
                    .chain(vec![(4,6),(3,6),(2,6),(1,6)])
                    .zip(repeat((-1, -1))),
            ),
            // sweep diagonal /
            Box::new(
                repeat(HEIGHT as isize - 1)
                    .zip(1..WIDTH as isize)
                    // add some higher positions manually
                    .chain(vec![(4,0),(3,0),(2,0),(1,0)])
                    .zip(repeat((-1, 1))),
            ),
        ];

        for it in direction_iterators {
            for ((i, j), dir) in it {
                // search for adjacent pieces in a sliding window of 4
                let mut window = WindowCount::default();
                let mut k = 0;
                let mut empties: HashSet<Move> = Default::default();
                while let Some(p) = self.board.get_safe(i + k * dir.0, j + k * dir.1) {
                    window.count[p as usize] += 1;
                    if p == Player::None {
                        empties.insert(((i + k * dir.0) as usize, (j + k * dir.1) as usize));
                    }

                    k += 1;
                    if k > 4 {
                        let exit_pos = (
                            (i + (k - 5) * dir.0) as usize,
                            (j + (k - 5) * dir.1) as usize,
                        );
                        let exit_val = self.board.get(exit_pos.0, exit_pos.1);
                        if exit_val == Player::None {
                            empties.remove(&exit_pos);
                        }
                        window.count[exit_val as usize] -= 1;
                    }

                    if window.count[Player::X as usize] == 3
                        && window.count[Player::None as usize] == 1
                    {
                        let p = empties.iter().next().unwrap();
                        threats[Player::X as usize].insert(p.clone());
                    } else if window.count[Player::X as usize] == 4 {
                        return EvaluationScore {
                            score: MAX_SCORE,
                            is_terminal: true,
                        };
                    }
                    if window.count[Player::O as usize] == 3
                        && window.count[Player::None as usize] == 1
                    {
                        let p = empties.iter().next().unwrap();
                        threats[Player::O as usize].insert(p.clone());
                    } else if window.count[Player::O as usize] == 4 {
                        return EvaluationScore {
                            score: -MAX_SCORE,
                            is_terminal: true,
                        };
                    }
                }
                
            }
            score += threats[Player::X as usize].iter().count() as i32 * 10;
            score -= threats[Player::O as usize].iter().count() as i32 * 10;

            // let next_move_threats_x = threats[Player::X as usize]
            //     .iter()
            //     .filter(
            //         |(i, j)| match self.board.get_safe(*i as isize, *j as isize) {
            //             Some(Player::X) | Some(Player::O) | None => true,
            //             _ => false,
            //         },
            //     )
            //     .count();
            // let next_move_threats_y = threats[Player::X as usize]
            //     .iter()
            //     .filter(
            //         |(i, j)| match self.board.get_safe(*i as isize, *j as isize) {
            //             Some(Player::X) | Some(Player::O) | None => true,
            //             _ => false,
            //         },
            //     )
            //     .count();
            // if next_move_threats_x > 1 {
            //     score = MAX_SCORE;
            // } else {
            //     score += threats[Player::X as usize].iter().count() as i32 * 10;
            // }
            // if next_move_threats_y > 1 {
            //     score = -MAX_SCORE;
            // } else {
            //     score -= threats[Player::O as usize].iter().count() as i32 * 10;
            // }
        }
        EvaluationScore {
            score: score,
            is_terminal: false,
        }
    }

    // fn evaluate_score_no_heuristic(&self) -> EvaluationScore {
    //     match self.get_winner() {
    //         Player::None => EvaluationScore {
    //             score: 0,
    //             // Not sure that it's not terminal, could be a draw, but since the minimax function
    //             // will iterate over possible moves this shouldn't be an issue
    //             is_terminal: false,
    //         },
    //         Player::X => EvaluationScore {
    //             score: 1000,
    //             is_terminal: true,
    //         },
    //         Player::O => EvaluationScore {
    //             score: -1000,
    //             is_terminal: true,
    //         },
    //     }
    // }
}

#[derive(Default)]
struct WindowCount {
    count: [usize; 3],
}

impl Debug for Connect4Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                write!(f, "{} ", String::from(self.board.get(i, j)))?;
            }
            writeln!(f)?;
        }
        write!(
            f,
            "nx: {:?}\nls: {:?}",
            &self.current_player, &self.last_move
        )
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
    #[case(
        "
        . . . . . . .
        O . . . . . .
        X . . . . . .
        X X . . . . .
        X X X O X . O
        O O O X X O O",
        Some((4, 2))
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

    // see https://sites.math.rutgers.edu/~zeilberg/C4/Introduction.html
    // for more win in x puzzles
    #[test]
    fn test_win_in_one() {
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

    // TODO repeat of ttt test. refactor
    fn play(game: Connect4Game, depth: u32) -> (Box<dyn MinimaxDriver>, i32) {
        let mut moves = 0;
        let mut minimax = Minimax::new(MinimaxParams {
            max_depth: depth,
            ..Default::default()
        });
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

    #[test]
    fn test_win_in_two() {
        let state = "
        .......
        .......
        .......
        XX.....
        XX.OX.O
        OOOXXOO";
        let game = Connect4Game::from_state(state, None, crate::game::Player::X);
        let (final_game, moves) = play(game, 7);
        assert_eq!(final_game.get_winner(), Player::X);
        assert_eq!(moves, 3);
    }

    #[test]
    fn test_win_in_three() {
        let state = "
        .......
        .......
        ..X....
        X.O....
        O.X....
        XXOOOXO";
        let game = Connect4Game::from_state(state, None, crate::game::Player::X);
        let (final_game, moves) = play(game, 7);
        assert_eq!(final_game.get_winner(), Player::X);
        // assert_eq!(moves, 5); // TODO pruning does not always select the shortest path
    }

    #[test]
    fn test_win_in_four() {
        let state = "
        .......
        .......
        .......
        .O..OXO
        OX.XXXO
        XO.OXOX";
        let game = Connect4Game::from_state(state, None, crate::game::Player::X);
        let (final_game, moves) = play(game, 9);
        assert_eq!(final_game.get_winner(), Player::X);
        // assert_eq!(moves, 7); // TODO pruning does not always select the shortest path
    }

    #[test]
    fn test_win_in_five() {
        let state = "
        .......
        .......
        .......
        .....X.
        .XOOXO.
        .XXOOXO";
        let game = Connect4Game::from_state(state, None, crate::game::Player::X);
        let (final_game, moves) = play(game, 9);
        assert_eq!(final_game.get_winner(), Player::X);
        assert_eq!(moves, 9);
    }

    #[test]
    fn test_score() {
        let state = "
        . . . . . . .
        O . . . . . .
        X . . . . . .
        X X . . . . .
        X X X O X . O
        O O O X X O O";
        let game = Connect4Game::from_state(state, Some((4, 5)), crate::game::Player::X);
        let score = game.evaluate_score();
        assert_eq!(score.score, 1000);
    }
}
