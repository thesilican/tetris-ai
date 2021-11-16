# Tetris AI JSON Format

Model

```
Game:
- board: Board
- current: Piece
- hold: Piece | null
- queue: PieceType[] // max length 8
- canHold: bool

Board: ((0 | 1)[24])[10]

Piece:
- type: PieceType
- rot: number // 0-4
- loc: [number, number]

PieceType: "O" | "I" | "T" | "L" | "J" | "S" | "Z"
```

```
GameMove:
 | "shiftLeft"
 | "shiftRight"
 | "rotateCW"
 | "rotate180"
 | "rotateCCW"
 | "hold"
 | "softDrop"
 | "hardDrop"

GameAction:
 | "shiftLeft"
 | "shiftRight"
 | "shiftDown"
 | "rotateCW"
 | "rotate180"
 | "rotateCCW"
 | "softDrop"
 | "hold"
 | "lock"
 | AddGarbageAction

AddGarbageAction:
- addGarbage:
  - col: number
  - height: number

Stream:
- queue: PieceType[]
```

Ai

```
AiRes: AiResSuccess | AiResFail

AiResSuccess:
- success: true
- moves: GameMove[]
- score: number | null

AiResFail:
- success: false
- reason: string
```

Processing

```
FrameCollection:
- name: string
- frames: Game[]

Replay:
- name: string
- stream: Stream
- actions: GameAction

KeyFrame:
- start: Game
- end: Game
- actions: GameAction[]

TestCase:
- board: (0 | 1)[240]
- label: 0 | 1
```
