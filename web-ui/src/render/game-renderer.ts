import { PieceInfo } from "../model/computed";
import { Game, PieceType, Tile } from "../model/model";
import { Canvas } from "./canvas";
import { Rect, Vec2 } from "./math";

const TILE_COLORS: { [key in Tile]: string } = {
  " ": "transparent",
  G: "gray",
  O: "#f0f000",
  I: "#00f0f0",
  T: "#a000f0",
  L: "#f0a000",
  J: "#0000f0",
  S: "#00f000",
  Z: "#f00000",
};

export class GameRenderer {
  canvas: Canvas;
  scale: number;
  translate: Vec2;

  constructor(canvas: Canvas) {
    this.canvas = canvas;
    this.scale = 1;
    this.translate = new Vec2(0, 0);
  }

  drawDebug() {
    this.canvas.drawText(`${this.canvas.dim.x}`, new Vec2(0, 0), 100, "black");
  }

  drawRect(rect: Rect, color: string) {
    const newRect = new Rect(
      rect.min.scale(this.scale).add(this.translate),
      rect.max.scale(this.scale).add(this.translate)
    );
    this.canvas.drawRect(newRect, color);
  }

  drawText(text: string, position: Vec2, size = 1) {
    const newPosition = position.scale(this.scale).add(this.translate);
    this.canvas.drawText(text, newPosition, this.scale * size, "black");
  }

  drawPiece(
    pieceType: PieceType,
    rotation: number,
    position: Vec2,
    overrideColor?: string
  ) {
    const shape = PieceInfo.Shapes[pieceType][rotation];
    for (let i = 0; i < 4; i++) {
      for (let j = 0; j < 4; j++) {
        if (shape[i][j]) {
          const color = overrideColor ?? TILE_COLORS[pieceType];
          this.drawRect(
            new Rect(position.x + i, position.y + 3 - j, 1, 1),
            color
          );
        }
      }
    }
  }

  draw(game: Game) {
    this.canvas.clear();

    const RENDER_AREA_WIDTH = 22;
    const RENDER_AREA_HEIGHT = 25;

    // Determing scaling
    this.scale = Math.floor(
      Math.min(
        (this.canvas.dim.y / RENDER_AREA_HEIGHT) * 0.9,
        (this.canvas.dim.x / RENDER_AREA_WIDTH) * 0.9
      )
    );
    const translateX = Math.floor(
      (this.canvas.dim.x - RENDER_AREA_WIDTH * this.scale) / 2
    );
    const translateY = Math.floor(
      (this.canvas.dim.y - RENDER_AREA_HEIGHT * this.scale) / 2
    );
    this.translate = new Vec2(translateX, translateY);

    // Draw game boundary
    // this.drawRect(
    //   new Rect(0, 0, RENDER_AREA_WIDTH, RENDER_AREA_HEIGHT),
    //   "lightgray"
    // );

    // Draw game well
    this.drawRect(new Rect(5, 4, 12, 21), "black");
    this.drawRect(new Rect(6, 4, 10, 20), "white");

    // Draw board
    for (let j = 0; j < 40; j++) {
      for (let i = 0; i < 10; i++) {
        const tile = game.board.get(i, j);
        const color = TILE_COLORS[tile];
        this.drawRect(new Rect(i + 6, 23 - j, 1, 1), color);
      }
    }

    // Draw ghost piece
    const ghost = game.active.clone();
    ghost.softDrop(game.board);
    const ghostShape = PieceInfo.Shapes[ghost.pieceType][ghost.rotation];
    for (let i = 0; i < 4; i++) {
      for (let j = 0; j < 4; j++) {
        if (ghostShape[i][j]) {
          const x = ghost.location[0] + i;
          const y = ghost.location[1] + j;
          this.drawRect(new Rect(6 + x, 23 - y, 1, 1), "#aaa");
        }
      }
    }

    // Draw active piece
    const shape = PieceInfo.Shapes[game.active.pieceType][game.active.rotation];
    for (let i = 0; i < 4; i++) {
      for (let j = 0; j < 4; j++) {
        if (shape[i][j]) {
          const x = game.active.location[0] + i;
          const y = game.active.location[1] + j;
          const color = TILE_COLORS[game.active.pieceType];
          this.drawRect(new Rect(6 + x, 23 - y, 1, 1), color);
        }
      }
    }

    // Draw hold
    this.drawText("Hold", new Vec2(1, 4));
    if (game.hold) {
      const overrideColor = game.canHold ? undefined : TILE_COLORS["G"];
      this.drawPiece(game.hold, 0, new Vec2(0, 5), overrideColor);
    }

    // Draw queue
    this.drawText("Queue", new Vec2(18, 4));
    const len = Math.min(game.queue.length, 6);
    for (let i = 0; i < len; i++) {
      this.drawPiece(game.queue[i], 0, new Vec2(18, 5 + 3 * i));
    }
  }
}
