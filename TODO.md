# High level
- Implement abstract minimax AI for turn based board game
- Example implementations for tictactoe, connect4 and later chess
- Frontend using Bevy
- Unit testing for game rules and minimax solutions
- Performance focused: implement benchmarks to measure performance gains (criterion + iai + CI)
- secondary frontend: export to wasm and hook up to javascript game

# Steps
- ~~cache~~. need to understand how to save and reuse cache for next algorithm calls
- compare with python project. do they analyze same amount of nodes? does it still lose in same situations?
- games in bevy
- chess
- symmetry optimization on cache?
- add level adjustment: a driver that will watch player moves and choose an answer in the same range (ie, average to 50% best move out of the ordered scores). This won't work at all with pruning so maybe drop it
- monte carlo tree search?
- compile to wasm, use a js frontend for the game (without bevy)

## chess specific
- moveset implementation. lichess for [reference](https://github.com/lichess-org/scalachess/blob/master/src/main/scala/Actor.scala)
- heuristics/refactor how score works
- progressive deepening / reusing the cache 
- [uneven tree distribution](https://youtu.be/STjW3eH0Cik?t=2644)
- parallel computing
- some [details](https://github.com/official-stockfish/Stockfish#a-note-on-classical-evaluation-versus-nnue-evaluation) on the ai of stockfish
- implement [uci](https://en.wikipedia.org/wiki/Universal_Chess_Interface)
- publish as [lichess bot](https://lichess.org/player/bots): https://lichess.org/api#tag/Bot  https://lichess.org/@/thibault/blog/how-to-create-a-lichess-bot/FuKyvDuB
- opening books?

## possible games
- make it work on the graph codinggame game
- checkers
- go
- an RTS (discrete sampling of the game state)