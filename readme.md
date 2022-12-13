A project that implements an abstract minimax algorithm that can drive a Tic Tac Toe and a Connect 4 game. Made with Rust and Bevy as a frontend.

This is a remake of my [old project](https://gitlab.com/d-bucur/tictactree) in Python

## Tasks
```
cargo test
```
Run tests for minimax algorithm

```
cargo run -p game_frontend
```
Run the Bevy game

```
cargo run -p tree_visualizer
```
Run the tree visualizer. This will generate an svg of the decision tree given a certain starting board position. Graphviz needs to be installed

```
cargo bench --bench minimax_bench_iai
```
Run benchmarks using iai. Valgrind need to be installed on the system

```
cargo bench --bench minimax_bench_criterion
```
Run benchmarks using criterion. Note that the result is much less stable than iai

