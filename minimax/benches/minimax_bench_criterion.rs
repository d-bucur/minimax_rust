use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minimax::{connect4::Connect4Game, game::Player, minimax::minimax, tictactoe::TicTacToeGame}; // TODO minimax::minimax::minimax is funny, need better names

fn tictactoe_benchmark(c: &mut Criterion) {
    c.bench_function("tictactoe_full_game", |b| {
        b.iter(|| {
            let board_str = "
        ...
        ...
        ...";
            let game = TicTacToeGame::from_state(board_str, Player::X);
            minimax(black_box(&game), None);
        })
    });
}

fn connect_benchmark(c: &mut Criterion) {
    c.bench_function("tictactoe_full_game", |b| {
        b.iter(|| {
            let board_str = "
                .......
                .......
                .......
                .......
                .......
                .......";
            let game = Connect4Game::from_state(board_str, None, Player::X);
            minimax(black_box(&game), Some(6));
        })
    });
}

criterion_group!(benches, tictactoe_benchmark, connect_benchmark);
criterion_main!(benches);
