use iai::black_box;
use minimax::{connect4::Connect4Game, game::Player, minimax::*, tictactoe::TicTacToeGame};

fn tictactoe_benchmark() {
    let mut minimax = Minimax::new(MinimaxParams::default());
    let board_str = "
        ...
        ...
        ...";
    let game = TicTacToeGame::from_state(board_str, Player::X);
    minimax.minimax(black_box(&game));
}

fn connect4_benchmark() {
    let mut minimax = Minimax::new(MinimaxParams {
        max_depth: 10,
        ..Default::default()
    });
    let board_str = "
        .......
        .......
        .......
        .......
        .......
        .......";
    let game = Connect4Game::from_state(board_str, None, Player::X);
    minimax.minimax(black_box(&game));
}

iai::main!(tictactoe_benchmark, connect4_benchmark);
