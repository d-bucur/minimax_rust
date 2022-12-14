# High level
- Implement abstract minimax AI for turn based board game
- Example implementations for tictactoe, connect4 and later chess
- Frontend using Bevy
- Unit testing for game rules and minimax solutions
- Performance focused: implement benchmarks to measure performance gains (criterion + iai + CI)
- secondary frontend: export to wasm and hook up to javascript game

# Steps
- ~~minimax base on tictactoe~~
- ~~parameters for age and best chance of winning~~
- ~~setup iai~~
- cache
- ~~connect4 working~~
- games in bevy
- alfa beta pruning
- symmetry optimization on cache?
- chess
- make it work on the graph codinggame game
- add level adjustment: a driver that will watch player moves and choose an answer in the same range (ie, average to 50% best move out of the ordered scores)
- monte carlo tree search?
- compile to wasm, use a js frontend for the game (without bevy)
- make it work on an RTS (discrete sampling of the game state)