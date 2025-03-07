import { PieceInfo } from "../model/computed";
import { Game, PieceType, Tile } from "../model/model";
import { Rect, Vec2 } from "./math";
import {
  Sprite,
  SPRITE_GRAY,
  SPRITE_I,
  SPRITE_J,
  SPRITE_L,
  SPRITE_O,
  SPRITE_S,
  SPRITE_T,
  SPRITE_Z,
} from "./sprite";

type DrawTextOptions = {
  text: string;
  position: Vec2;
  fontSize?: number;
  color?: string;
  align?: "baseline" | "top" | "center" | "bottom";
  jusitfy?: "left" | "center" | "right";
  debug?: boolean;
};

type DrawCircleOptions = {
  position: Vec2;
  radius: number;
  color?: string;
};

type DrawRectOptions = {
  rect: Rect;
  color?: string;
};

type DrawSpriteOptions = {
  sprite: Sprite;
  position: Vec2;
  dim?: Vec2;
};

type RenderGameOptions = {
  game: Game;
  paused?: boolean;
  statusText?: string;
};

const COLOR_BACKGROUND = "#1e1e1e";
const COLOR_FOREGROUND = "#ffffff";
const COLOR_SURFACE = "#333333";
const COLOR_GHOST = "#a9b0c8";
const RENDER_AREA_WIDTH = 22;
const RENDER_AREA_HEIGHT = 25;

export class GameRenderer {
  canvas: HTMLCanvasElement;
  ctx: CanvasRenderingContext2D;
  dim: Vec2;
  scale: number;
  translate: Vec2;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    const ctx = canvas.getContext("2d");
    if (ctx === null) {
      throw new Error("Cannot obtain 2d context");
    }
    this.ctx = ctx;
    this.scale = 1;
    this.translate = new Vec2(0, 0);
    this.dim = new Vec2(0, 0);
    this.updateDim();
    window.addEventListener("resize", this.handleResize);
  }

  updateDim() {
    this.dim.x = this.canvas.clientWidth * window.devicePixelRatio;
    this.dim.y = this.canvas.clientHeight * window.devicePixelRatio;
    this.canvas.width = this.dim.x;
    this.canvas.height = this.dim.y;
    this.scale = Math.floor(
      Math.min(this.dim.y / RENDER_AREA_HEIGHT, this.dim.x / RENDER_AREA_WIDTH)
    );
    const translateX = Math.floor(
      (this.dim.x - RENDER_AREA_WIDTH * this.scale) / 2
    );
    const translateY = Math.floor(
      (this.dim.y - RENDER_AREA_HEIGHT * this.scale) / 2
    );
    this.translate = new Vec2(translateX, translateY);
  }

  handleResize = () => {
    this.updateDim();
  };

  clear() {
    this.ctx.fillStyle = COLOR_BACKGROUND;
    this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
  }

  drawCircle(options: DrawCircleOptions) {
    const position = options.position;
    const radius = options.radius;
    const color = options.color ?? COLOR_FOREGROUND;
    this.ctx.fillStyle = color;
    this.ctx.beginPath();
    this.ctx.arc(position.x, position.y, radius, 0, Math.PI * 2);
    this.ctx.fill();
  }

  drawRect(options: DrawRectOptions) {
    const rect = options.rect;
    this.ctx.fillStyle = options.color ?? COLOR_FOREGROUND;
    this.ctx.fillRect(rect.min.x, rect.min.y, rect.width, rect.height);
  }

  drawText(options: DrawTextOptions) {
    const text = options.text;
    const position = options.position;
    const fontSize = options.fontSize ?? 24;
    const color = options.color ?? COLOR_FOREGROUND;
    const align = options.align ?? "baseline";
    const justify = options.jusitfy ?? "left";
    const debug = options.debug ?? false;

    this.ctx.font = `${fontSize}px Arial`;
    const metrics = this.ctx.measureText(text);
    const width = metrics.width;
    const height =
      metrics.actualBoundingBoxAscent + metrics.actualBoundingBoxDescent;
    let textX = position.x;
    let textY = position.y;
    if (align === "baseline") {
    } else if (align === "top") {
      textY += metrics.actualBoundingBoxAscent;
    } else if (align === "center") {
      textY += metrics.actualBoundingBoxAscent - Math.round(height / 2);
    } else if (align === "bottom") {
      textY -= metrics.actualBoundingBoxDescent;
    }
    if (justify === "left") {
    } else if (justify === "center") {
      textX -= Math.round(width / 2);
    } else if (justify === "right") {
      textX -= width;
    }
    if (debug) {
      this.drawRect({
        rect: new Rect(
          textX,
          textY - metrics.actualBoundingBoxAscent,
          width,
          height
        ),
        color: "red",
      });
    }
    this.ctx.fillStyle = color;
    this.ctx.fillText(text, textX, textY);
    if (debug) {
      const size = Math.round(fontSize / 20);
      this.drawRect({
        rect: new Rect(textX - size, textY - size, size * 2, size * 2),
        color: "green",
      });
      this.drawCircle({
        position,
        radius: size,
        color: "cyan",
      });
    }
  }

  drawSprite(options: DrawSpriteOptions) {
    const sprite = options.sprite;
    const pos = options.position;
    const dim = options.dim;
    const spriteDim = dim ?? sprite.dim;
    this.ctx.drawImage(sprite.image, pos.x, pos.y, spriteDim.x, spriteDim.y);
  }

  drawScaledCircle(options: DrawCircleOptions) {
    const newOptions = {
      ...options,
      position: options.position.scale(this.scale).add(this.translate),
      radius: options.radius * this.scale,
    };
    this.drawCircle(newOptions);
  }

  drawScaledRect(options: DrawRectOptions) {
    const newRect = new Rect(
      options.rect.min.scale(this.scale).add(this.translate),
      options.rect.max.scale(this.scale).add(this.translate)
    );
    const newOptions = {
      ...options,
      rect: newRect,
    };
    this.drawRect(newOptions);
  }

  drawScaledText(options: DrawTextOptions) {
    const newOptions = {
      ...options,
      position: options.position.scale(this.scale).add(this.translate),
      fontSize:
        options.fontSize !== undefined
          ? options.fontSize * this.scale
          : 1 * this.scale,
    };
    this.drawText(newOptions);
  }

  drawScaledSprite(options: DrawSpriteOptions) {
    const newOptions = {
      ...options,
      position: options.position.scale(this.scale).add(this.translate),
      dim: options.dim
        ? options.dim.scale(this.scale)
        : options.sprite.dim.scale(this.scale),
    };
    this.drawSprite(newOptions);
  }

  drawTile(tile: Tile, position: Vec2) {
    if (tile === " ") {
      return;
    }
    const lookup = {
      O: SPRITE_O,
      I: SPRITE_I,
      T: SPRITE_T,
      L: SPRITE_L,
      J: SPRITE_J,
      S: SPRITE_S,
      Z: SPRITE_Z,
      G: SPRITE_GRAY,
    };
    const sprite = lookup[tile];
    this.drawScaledSprite({
      sprite,
      position,
      dim: new Vec2(1, 1),
    });
  }

  drawPiece(
    pieceType: PieceType,
    rotation: number,
    position: Vec2,
    overrideTile?: Tile
  ) {
    const shape = PieceInfo.Shapes[pieceType][rotation];
    for (let i = 0; i < 4; i++) {
      for (let j = 0; j < 4; j++) {
        if (shape[i][j]) {
          const tile = overrideTile ?? pieceType;
          this.drawTile(tile, new Vec2(position.x + i, position.y + 3 - j));
        }
      }
    }
  }

  drawUi() {
    // Draw game well
    this.drawScaledRect({
      rect: new Rect(5.75, 4, 10.5, 20.25),
      color: COLOR_FOREGROUND,
    });
    this.drawScaledRect({
      rect: new Rect(6, 4, 10, 20),
      color: COLOR_BACKGROUND,
    });
    this.drawScaledText({
      text: "Queue",
      position: new Vec2(18, 4),
    });
    this.drawScaledText({
      text: "Hold",
      position: new Vec2(1, 4),
    });
  }

  renderWelcome() {
    this.clear();

    this.drawUi();
    this.drawScaledRect({
      rect: new Rect(6, 10, 10, 1),
      color: COLOR_SURFACE,
    });
    this.drawScaledText({
      text: "Press start to play",
      fontSize: 0.75,
      position: new Vec2(11, 10.5),
      align: "center",
      jusitfy: "center",
    });
  }

  render(options: RenderGameOptions) {
    const game = options.game;
    this.clear();

    // Draw UI
    this.drawUi();

    // Draw board
    for (let j = 0; j < 40; j++) {
      for (let i = 0; i < 10; i++) {
        const tile = game.board.get(i, j);
        this.drawTile(tile, new Vec2(i + 6, 23 - j));
      }
    }

    if (!game.finished) {
      // Draw ghost piece
      const ghost = game.active.clone();
      ghost.softDrop(game.board);
      const ghostShape = PieceInfo.Shapes[ghost.pieceType][ghost.rotation];
      for (let i = 0; i < 4; i++) {
        for (let j = 0; j < 4; j++) {
          if (ghostShape[i][j]) {
            const x = ghost.positionX + i;
            const y = ghost.positionY + j;
            this.drawScaledRect({
              rect: new Rect(6 + x, 23 - y, 1, 1),
              color: COLOR_GHOST,
            });
          }
        }
      }

      // Draw active piece
      const shape =
        PieceInfo.Shapes[game.active.pieceType][game.active.rotation];
      for (let i = 0; i < 4; i++) {
        for (let j = 0; j < 4; j++) {
          if (shape[i][j]) {
            const x = game.active.positionX + i;
            const y = game.active.positionY + j;
            this.drawTile(game.active.pieceType, new Vec2(6 + x, 23 - y));
          }
        }
      }
    }

    // Draw hold
    if (game.hold) {
      const overrideTile = game.canHold ? undefined : "G";
      this.drawPiece(game.hold, 0, new Vec2(0, 5), overrideTile);
    }

    // Draw queue
    const len = Math.min(game.queue.length, 6);
    for (let i = 0; i < len; i++) {
      this.drawPiece(game.queue[i], 0, new Vec2(18, 5 + 3 * i));
    }

    // Draw text
    this.drawScaledText({
      position: new Vec2(0, 24),
      text: `Lines: ${game.score}`,
      align: "bottom",
    });
    if (options.statusText) {
      this.drawScaledText({
        position: new Vec2(2.5, 22),
        text: options.statusText,
        jusitfy: "center",
        fontSize: 0.75,
      });
    }
    if (options.paused ?? false) {
      this.drawScaledRect({
        rect: new Rect(6, 10, 10, 1),
        color: COLOR_SURFACE,
      });
      this.drawScaledText({
        text: "Paused",
        fontSize: 0.75,
        position: new Vec2(11, 10.5),
        align: "center",
        jusitfy: "center",
      });
    }
    if (game.finished) {
      this.drawScaledRect({
        rect: new Rect(6, 10, 10, 1),
        color: COLOR_SURFACE,
      });
      this.drawScaledText({
        text: "Game Over",
        fontSize: 0.75,
        position: new Vec2(11, 10.5),
        align: "center",
        jusitfy: "center",
      });
    }
  }
}
