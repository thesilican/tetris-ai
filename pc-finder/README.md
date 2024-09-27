# pc-finder

This perfect clear (PC) bot is a bot that searches for [perfect clear][pc]
setups from the current board state. From most board states the PC bot
is able to find a setup that leads to a perfect clear.

The bot code has two sections: tree generation and tree traversal

## Tree generation

During tree search we build a directed graph of all board states
4 or less blocks tall that could lead to a perfect clear. This
involves:
- Generating a list of "tessellations", each of which is a configuration of
10 tetris pieces that tiles the bottom 10x4 of the board
- Generating a tree of board states starting from the empty board,
ensuring that the pieces fit a tessellation at each step
- Pruning the tree by visiting nodes backwards from the empty board,
ensuring that only edges that terminate at the empty state

The resulting tree is then compressed and output into a binary format.

## Tree traversal

Tree traversal is the part that occurs during the actual AI runtime.
The tree from the previous step is loaded into memory.

During runtime, the AI takes the board state and attempts to find a path
through the tree to the empty board.

[pc]: https://harddrop.com/wiki/Perfect_clear
