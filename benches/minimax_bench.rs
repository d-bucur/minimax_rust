use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minimax::{tictactoe::TicTacToeGame, game::Player, minimax::minimax}; // TODO minimax::minimax::minimax is funny, need better names

fn tictactoe_benchmark(c: &mut Criterion) {
    c.bench_function("tictactoe", |b| b.iter(|| {
        let board_str = "
        OXO
        OXO
        .XX";
        let game = TicTacToeGame::from_state(board_str, Player::O);
        minimax(black_box(&game));
    }));
}

criterion_group!(benches, tictactoe_benchmark);
criterion_main!(benches);