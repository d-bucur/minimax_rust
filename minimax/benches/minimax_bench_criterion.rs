use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minimax::{connect4::Connect4Game, game::Player, minimax::*, tictactoe::TicTacToeGame}; // TODO minimax::minimax::minimax is funny, need better names

fn tictactoe_benchmark(c: &mut Criterion) {
    let board_str = "
...
...
...";
    let game = TicTacToeGame::from_state(board_str, Player::X);
    let mut minimax = Minimax::new(MinimaxParams::default());
    c.bench_function("tictactoe_full_game", |b| {
        b.iter(|| {
            minimax.minimax(black_box(&game));
        })
    });
}

fn connect_benchmark(c: &mut Criterion) {
    let board_str = "
        .......
        .......
        .......
        .......
        .......
        .......";
    let game = Connect4Game::from_state(board_str, None, Player::X);
    let mut minimax = Minimax::new(MinimaxParams {
        max_depth: 12,
        ..Default::default()
    });
    c.bench_function("connect4_full_game", |b| {
        b.iter(|| {
            minimax.minimax(black_box(&game));
        })
    });
}

criterion_group!(benches, tictactoe_benchmark, connect_benchmark);
criterion_main!(benches);
