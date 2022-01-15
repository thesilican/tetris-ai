# tetris-ai

Exploring various ideas related to tetris ai, written in Rust (and a little Python)

<p align="center">
<img src="./demo-1.webp" alt="demo 1" height="200" />
<img src="./demo-2.webp" alt="demo 1" height="200" />
</p>

List of crates (as of 2021-12-26)

- `c4w-bot` - Attempt to create a bot that does center 4 wide stacking
- `common` - Highly optimized tetris pieces, board, game model, and AI interface. Used by pretty much every other crate
- `deep-bot` - Recursive tetris ai based on simple heuristics
- `ml-bot` - Attempt to create a bot using a neural network for heruistics
- `ml` - Experimenting with Python ML (tensorflow)
- `pc-finder` - Attempt to create a bot that finds perfect clears
- `processor` - Process TETR.IO replays, for use as training data for ml-bot
- `wasm-test` - Attempting to compile a tetris AI to webassembly for use in the browser
- `ws-server` - HTTP and websocket server, to communicate wtih a tetris frontend
