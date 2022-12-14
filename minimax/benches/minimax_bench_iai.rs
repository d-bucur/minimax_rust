use iai::black_box;
use minimax::{connect4::Connect4Game, game::Player, minimax::minimax, tictactoe::TicTacToeGame};

fn tictactoe_benchmark() {
    let board_str = "
        ...
        ...
        ...";
    let game = TicTacToeGame::from_state(board_str, Player::X);
    minimax(black_box(&game), None);
}

fn connect4_benchmark() {
    let board_str = "
        .......
        .......
        .......
        .......
        .......
        .......";
    let game = Connect4Game::from_state(board_str, None, Player::X);
    minimax(black_box(&game), Some(7));
}

iai::main!(tictactoe_benchmark, connect4_benchmark);
