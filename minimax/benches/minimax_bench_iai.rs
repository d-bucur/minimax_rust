use iai::black_box;
use minimax::{tictactoe::TicTacToeGame, game::Player, minimax::minimax};

fn tictactoe_benchmark() {
    let board_str = "
        ...
        ...
        ...";
    let game = TicTacToeGame::from_state(board_str, Player::X);
    minimax(black_box(&game), None);
}

iai::main!(tictactoe_benchmark);