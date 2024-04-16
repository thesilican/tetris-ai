import { PieceInfo } from "./computed";
import { Xorshift } from "./rng";

export const BOARD_WIDTH = 10;
export const BOARD_HEIGHT = 40;
export const BOARD_VISIBLE_HEIGHT = 20;

export type PieceType = "O" | "I" | "T" | "L" | "J" | "S" | "Z";
export type Tile = " " | PieceType | "G";
export type GameMove =
  | "shift-left"
  | "shift-right"
  | "rotate-cw"
  | "rotate-ccw"
  | "rotate-180"
  | "hold"
  | "soft-drop"
  | "hard-drop";

export type LockInfo = {
  linesCleared: number;
};

export class Piece {
  pieceType: PieceType;
  rotation: number;
  location: [number, number];

  constructor(
    pieceType: PieceType,
    rotation: number,
    location: [number, number]
  ) {
    this.pieceType = pieceType;
    this.rotation = rotation;
    this.location = location;
  }

  reset() {
    this.rotation = 0;
    this.location = PieceInfo.SpawnLocation[this.pieceType];
  }

  clone(): Piece {
    return new Piece(this.pieceType, this.rotation, [
      this.location[0],
      this.location[1],
    ]);
  }

  rotate(amount: number, board: Board): boolean {
    const [oldX, oldY] = this.location;
    const oldRot = this.rotation;
    const newRot = (this.rotation + amount) % 4;
    this.rotation = newRot;

    const kickTable = PieceInfo.KickTable[this.pieceType][oldRot][newRot];
    const [left, right, bottom, top] =
      PieceInfo.LocationBounds[this.pieceType][this.rotation];

    for (const [dx, dy] of kickTable) {
      const newX = oldX + dx;
      const newY = oldY + dy;
      this.location = [newX, newY];
      if (
        newX >= left &&
        newX <= right &&
        newY >= bottom &&
        newY <= top &&
        !board.intersectsWith(this)
      ) {
        return true;
      }
    }

    this.rotation = oldRot;
    this.location = [oldX, oldY];
    return false;
  }

  rotateCw(board: Board): boolean {
    return this.rotate(1, board);
  }

  rotate180(board: Board): boolean {
    return this.rotate(2, board);
  }

  rotateCcw(board: Board): boolean {
    return this.rotate(3, board);
  }

  shift([dx, dy]: [number, number], board: Board): boolean {
    const [oldX, oldY] = this.location;
    const newX = oldX + dx;
    const newY = oldY + dy;
    this.location = [newX, newY];

    const [left, right, bottom, top] =
      PieceInfo.LocationBounds[this.pieceType][this.rotation];
    if (
      newX >= left &&
      newX <= right &&
      newY >= bottom &&
      newY <= top &&
      !board.intersectsWith(this)
    ) {
      return true;
    }
    this.location = [oldX, oldY];
    return false;
  }

  shiftLeft(board: Board): boolean {
    return this.shift([-1, 0], board);
  }

  shiftRight(board: Board): boolean {
    return this.shift([1, 0], board);
  }

  dasLeft(board: Board): boolean {
    if (!this.shiftLeft(board)) {
      return false;
    }
    while (this.shiftLeft(board)) {}
    return true;
  }

  dasRight(board: Board): boolean {
    if (!this.shiftRight(board)) {
      return false;
    }
    while (this.shiftRight(board)) {}
    return true;
  }

  shiftDown(board: Board): boolean {
    return this.shift([0, -1], board);
  }

  softDrop(board: Board): boolean {
    if (!this.shiftDown(board)) {
      return false;
    }
    while (this.shiftDown(board)) {}
    return true;
  }
}

export class Board {
  tiles: Tile[];

  constructor() {
    this.tiles = Array.from(Array(BOARD_WIDTH * BOARD_HEIGHT)).map((_) => " ");
  }

  get(x: number, y: number): Tile {
    const idx = y * BOARD_WIDTH + x;
    if (idx < 0 || idx >= BOARD_WIDTH * BOARD_HEIGHT) {
      throw new Error("indexed board out of bounds");
    }
    return this.tiles[idx];
  }

  set(x: number, y: number, tile: Tile) {
    const idx = y * BOARD_WIDTH + x;
    if (idx < 0 || idx >= BOARD_WIDTH * BOARD_HEIGHT) {
      throw new Error("indexed board out of bounds");
    }
    this.tiles[idx] = tile;
  }

  addGarbage(col: number, height: number) {
    // Copy rows up
    for (let j = BOARD_HEIGHT - 1; j >= height; j--) {
      for (let i = 0; i < BOARD_WIDTH; i++) {
        this.tiles[j * BOARD_WIDTH + i] =
          this.tiles[(j - height) * BOARD_WIDTH + i];
      }
    }

    // Set garbage rows
    for (let j = 0; j < height; j++) {
      for (let i = 0; i < BOARD_WIDTH; i++) {
        const tile = i === col ? " " : "G";
        this.tiles[j * BOARD_WIDTH + i] = tile;
      }
    }
  }

  intersectsWith(piece: Piece): boolean {
    let shape = PieceInfo.Shapes[piece.pieceType][piece.rotation];
    for (let i = 0; i < 4; i++) {
      for (let j = 0; j < 4; j++) {
        const x = piece.location[0] + i;
        const y = piece.location[1] + j;
        if (x < 0 || x >= BOARD_WIDTH || y < 0 || y >= BOARD_HEIGHT) {
          continue;
        }
        if (shape[i][j] && this.get(x, y) !== " ") {
          return true;
        }
      }
    }
    return false;
  }

  lock(piece: Piece): LockInfo {
    const shape = PieceInfo.Shapes[piece.pieceType][piece.rotation];

    for (let i = 0; i < 4; i++) {
      for (let j = 0; j < 4; j++) {
        if (shape[i][j]) {
          this.set(
            piece.location[0] + i,
            piece.location[1] + j,
            piece.pieceType
          );
        }
      }
    }

    let linesCleared = 0;
    for (let j = 0; j < BOARD_HEIGHT; j++) {
      let full = true;
      for (let i = 0; i < 10; i++) {
        if (this.get(i, j) === " ") {
          full = false;
          break;
        }
      }

      if (full) {
        linesCleared++;
      } else {
        for (let i = 0; i < 10; i++) {
          this.set(i, j - linesCleared, this.get(i, j));
        }
      }
    }

    for (let j = 0; j < linesCleared; j++) {
      for (let i = 0; i < 10; i++) {
        this.set(i, BOARD_HEIGHT - linesCleared + j, " ");
      }
    }

    return {
      linesCleared,
    };
  }

  pretty(): string {
    const chars = [];
    for (let j = BOARD_VISIBLE_HEIGHT - 1; j >= 0; j--) {
      chars.push("|");
      for (let i = 0; i < BOARD_WIDTH; i++) {
        chars.push(this.get(i, j));
      }
      chars.push("|");
      chars.push("\n");
    }
    chars.push("+" + "-".repeat(BOARD_WIDTH) + "+");
    return chars.join("");
  }
}

export class Bag {
  queue: PieceType[];
  rng: Xorshift;

  constructor(seed = 0) {
    this.queue = [];
    this.rng = new Xorshift(seed);
  }

  private refill() {
    const pieces: PieceType[] = ["O", "I", "T", "L", "J", "S", "Z"];
    for (let i = 6; i >= 1; i--) {
      const j = this.rng.next() % (i + 1);
      [pieces[i], pieces[j]] = [pieces[j], pieces[i]];
    }
    this.queue.push(...pieces);
  }

  next(): PieceType {
    if (this.queue.length === 0) {
      this.refill();
    }
    return this.queue.shift()!;
  }
}

export class Game {
  board: Board;
  active: Piece;
  hold: PieceType | null;
  queue: PieceType[];
  canHold: boolean;
  bag: Bag;

  constructor(seed = 0) {
    this.board = new Board();
    this.active = new Piece("O", 0, [0, 0]);
    this.hold = null;
    this.queue = [];
    this.canHold = true;
    this.bag = new Bag(seed);
  }

  start(seed = 0) {
    this.bag = new Bag(seed);
    this.board = new Board();
    const active = this.bag.next();
    this.active = new Piece(active, 0, PieceInfo.SpawnLocation[active]);
    this.hold = null;
    this.queue = [];
    for (let i = 0; i < 6; i++) {
      this.queue.push(this.bag.next());
    }
    this.canHold = true;
  }

  swapHold(): boolean {
    if (!this.canHold) {
      return false;
    }
    this.canHold = false;

    let hold: PieceType;
    if (this.hold) {
      hold = this.hold;
    } else {
      const piece = this.queue.shift();
      if (piece === undefined) {
        return false;
      }
      hold = piece;
    }
    this.hold = this.active.pieceType;
    this.active.pieceType = hold;
    this.active.reset();
    return true;
  }

  hardDrop() {
    if (this.queue.length === 0) {
      throw new Error("cannot hard drop while queue is empty");
    }

    this.active.softDrop(this.board);
    this.board.lock(this.active);
    this.active.pieceType = this.queue.shift()!;
    this.active.reset();
    this.canHold = true;

    if (this.queue.length < 6) {
      this.queue.push(this.bag.next());
    }
  }
}
