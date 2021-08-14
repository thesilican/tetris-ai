import { deepEqual } from "./util";

// O I T L J S Z GARBAGE EMPTY
export type Tile = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | null;
export type CanvasFrame = {
  active: HTMLCanvasElement;
  board: HTMLCanvasElement;
};
export type Frame = {
  board: Tile[][];
  active: Tile[][];
  hold: [piece: Tile, isGray: boolean];
  queue: Tile[];
};

type Vec2<T = number> = [T, T];
type Bounds = {
  board: Vec2<Vec2>;
  hold: Vec2;
  queue: Vec2[];
};
const BOUNDS_P1: Bounds = {
  board: [
    [397, 228], // Low (x,y)
    [606, 670], // High (x,y)
  ],
  hold: [312, 272],
  queue: [
    [679, 272],
    [679, 342],
    [679, 412],
    [679, 482],
    [679, 552],
  ],
};

function recognizeColor(img: HTMLCanvasElement, x: number, y: number): Tile {
  const data = img.getContext("2d")!.getImageData(x, y, 1, 1).data;
  const [r, g, b, a] = data;
  type ColorsType = [Tile, [number, number, number]][];
  const COLORS: ColorsType = [
    [0, [0xff, 0xff, 0x00]],
    [1, [0x00, 0x77, 0xff]],
    [2, [0xff, 0x00, 0xff]],
    [3, [0xff, 0x77, 0x00]],
    [4, [0x00, 0x00, 0xff]],
    [5, [0x00, 0xff, 0x00]],
    [6, [0xff, 0x00, 0x00]],
    [7, [0xff, 0xff, 0xff]],
    [null, [0x00, 0x00, 0x00]],
  ];
  let best: Tile = null;
  let bestScore = Infinity;
  for (const [i, [sr, sg, sb]] of COLORS) {
    let dr = Math.abs(r - sr);
    let dg = Math.abs(g - sg);
    let db = Math.abs(b - sb);
    if (dr + dg + db < bestScore) {
      best = i;
      bestScore = dr + dg + db;
    }
  }
  return best;
}

function recognizeBoard(img: HTMLCanvasElement, bounds: Vec2<Vec2>): Tile[][] {
  const lowX = bounds[0][0];
  const lowY = bounds[0][1];
  const stepX = (bounds[1][0] - lowX) / 9;
  const stepY = (bounds[1][1] - lowY) / 19;
  const board: Tile[][] = [];
  for (let i = 0; i < 10; i++) {
    const col: Tile[] = [];
    for (let j = 0; j < 24; j++) {
      const x = Math.round(lowX + stepX * i);
      const y = Math.round(lowY + stepY * (20 - j - 1));
      const val = recognizeColor(img, x, y);
      col.push(val);
    }
    board.push(col);
  }
  return board;
}

function recognizePiece(img: HTMLCanvasElement, center: Vec2): Tile {
  const STEP = 12;
  const startX = center[0] - 3.5 * STEP;
  const startY = center[1] - 1.5 * STEP;

  const fingerprint: (0 | 1)[][] = [];
  for (let j = 0; j < 4; j++) {
    const row: (1 | 0)[] = [];
    for (let i = 0; i < 8; i++) {
      const x = Math.round(startX + i * STEP);
      const y = Math.round(startY + j * STEP);
      const color = recognizeColor(img, x, y);
      row.push(color === null ? 0 : 1);
    }
    fingerprint.push(row);
  }
  const O = [
    [0, 0, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 0, 0],
  ];
  const I = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [0, 0, 0, 0, 0, 0, 0, 0],
  ];
  const T = [
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
  ];
  const L = [
    [0, 0, 0, 0, 0, 1, 1, 0],
    [0, 0, 0, 0, 0, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
  ];
  const J = [
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 0],
  ];
  const S = [
    [0, 0, 0, 1, 1, 1, 1, 0],
    [0, 0, 0, 1, 1, 1, 1, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
  ];
  const Z = [
    [0, 1, 1, 1, 1, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 0],
    [0, 0, 0, 1, 1, 1, 1, 0],
  ];
  const EMPTY = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
  ];
  if (deepEqual(O, fingerprint)) {
    return 0;
  }
  if (deepEqual(I, fingerprint)) {
    return 1;
  }
  if (deepEqual(T, fingerprint)) {
    return 2;
  }
  if (deepEqual(L, fingerprint)) {
    return 3;
  }
  if (deepEqual(J, fingerprint)) {
    return 4;
  }
  if (deepEqual(S, fingerprint)) {
    return 5;
  }
  if (deepEqual(Z, fingerprint)) {
    return 6;
  }
  if (deepEqual(EMPTY, fingerprint)) {
    return null;
  }
  console.log(fingerprint);
  throw "up";
}

function recognizeHold(img: HTMLCanvasElement, center: Vec2): [Tile, boolean] {
  const piece = recognizePiece(img, center);
  const x = center[0];
  const y = center[1] + 6;
  const color = recognizeColor(img, x, y);
  const isGray = color === 7;
  return [piece, isGray];
}

function recognizeQueue(img: HTMLCanvasElement, centers: Vec2[]): Tile[] {
  return centers.map((center) => recognizePiece(img, center));
}

export function recognizeFrame(canvasFrames: CanvasFrame): Frame {
  const bounds = BOUNDS_P1;
  const { active: imgActive, board: imgBoard } = canvasFrames;
  const board = recognizeBoard(imgBoard, bounds.board);
  const active = recognizeBoard(imgActive, bounds.board);
  const hold = recognizeHold(imgActive, bounds.hold);
  const queue = recognizeQueue(imgActive, bounds.queue);

  return { board, active, hold, queue };
}

export function printFrame(frame: Frame) {
  const CELLS: Map<Tile, string> = new Map([
    [null, "░"],
    [0, "O"],
    [1, "I"],
    [2, "T"],
    [3, "L"],
    [4, "J"],
    [5, "S"],
    [6, "Z"],
    [7, "X"],
  ]);
  const { board, active, hold, queue } = frame;
  const rows = [];
  for (let j = 24; j >= 0; j--) {
    const row = [];
    for (let i = 0; i < 10; i++) {
      const cell = board[i][j];
      row.push(CELLS.get(cell));
    }
    row.push(" ");
    for (let i = 0; i < 10; i++) {
      const cell = active[i][j];
      row.push(CELLS.get(cell));
    }
    rows.push(row.join(""));
  }
  rows.push(`${CELLS.get(hold[0])} ${hold[1]}`);
  rows.push(queue.map((x) => CELLS.get(x)).join(""));
  console.log(rows.join("\n"));
}
