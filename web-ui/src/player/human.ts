import { Game } from "../model/model";
import { DasTimer, generateSeed } from "../model/util";
import { GameRenderer } from "../render/game-renderer";

const capturedKeys = [
  "KeyR",
  "KeyP",
  "ArrowLeft",
  "ArrowRight",
  "ArrowDown",
  "Space",
  "KeyZ",
  "KeyX",
  "KeyA",
  "KeyC",
];

export class HumanPlayer {
  game: Game;
  renderer: GameRenderer;
  started = false;
  paused = false;
  leftPressed = false;
  leftTimer = new DasTimer(12, 1);
  rightPressed = false;
  rightTimer = new DasTimer(12, 1);
  downPressed = false;
  downTimer = new DasTimer(0, 1);
  gravityTimer = new DasTimer(0, 60);
  interval: number | undefined;
  statusTextCountdown = 0;
  statusText = "";

  constructor(renderer: GameRenderer) {
    this.game = new Game();
    this.renderer = renderer;
  }

  start() {
    this.game.start(generateSeed());
    this.started = true;
    this.paused = false;
    this.interval = requestAnimationFrame(this.tick);
    window.addEventListener("keydown", this.handleKeyDown);
    window.addEventListener("keyup", this.handleKeyUp);
  }

  stop() {
    this.started = false;
    if (this.interval !== undefined) {
      cancelAnimationFrame(this.interval);
      this.interval = undefined;
    }
    window.removeEventListener("keydown", this.handleKeyDown);
    window.removeEventListener("keyup", this.handleKeyUp);
  }

  handleKeyDown = (e: KeyboardEvent) => {
    if (e.ctrlKey || e.shiftKey || e.altKey || e.metaKey) {
      return;
    }
    if (capturedKeys.includes(e.code)) {
      e.preventDefault();
    }
    if (e.repeat) {
      return;
    }
    if (!this.started) {
      return;
    }
    if (e.code === "KeyR") {
      this.game.start(generateSeed());
    }
    if (this.game.finished) {
      return;
    }
    if (e.code === "KeyP") {
      this.paused = !this.paused;
    }
    if (this.paused) {
      return;
    }
    if (e.code === "ArrowLeft") {
      this.game.apply("shift-left");
      this.gravityTimer.reset();
      this.rightPressed = false;
      this.leftPressed = true;
      this.leftTimer.reset();
    } else if (e.code === "ArrowRight") {
      this.game.apply("shift-right");
      this.gravityTimer.reset();
      this.leftPressed = false;
      this.rightPressed = true;
      this.rightTimer.reset();
    } else if (e.code === "ArrowDown") {
      this.game.apply("shift-down");
      this.gravityTimer.reset();
      this.downPressed = true;
      this.downTimer.reset();
    } else if (e.code === "Space") {
      const info = this.game.apply("hard-drop");
      if (info) {
        if (info.tspin) {
          this.statusTextCountdown = 120;
          if (info.linesCleared === 1) {
            this.statusText = "T-Spin Single!";
          } else if (info.linesCleared === 2) {
            this.statusText = "T-Spin Double!";
          } else if (info.linesCleared === 3) {
            this.statusText = "T-Spin Triple!";
          } else {
            this.statusText = "T-Spin!";
          }
        } else if (info.linesCleared >= 1) {
          this.statusTextCountdown = 120;
          if (info.linesCleared === 1) {
            this.statusText = "Single!";
          } else if (info.linesCleared === 2) {
            this.statusText = "Double!";
          } else if (info.linesCleared === 3) {
            this.statusText = "Triple!";
          } else {
            this.statusText = "Quad!";
          }
        }
      }
      this.gravityTimer.reset();
      this.leftTimer.reset();
      this.rightTimer.reset();
    } else if (e.code === "KeyZ") {
      this.game.apply("rotate-ccw");
      this.gravityTimer.reset();
    } else if (e.code === "KeyX") {
      this.game.apply("rotate-cw");
      this.gravityTimer.reset();
    } else if (e.code === "KeyA") {
      this.game.apply("rotate-180");
      this.gravityTimer.reset();
    } else if (e.code === "KeyC") {
      this.game.apply("hold");
      this.gravityTimer.reset();
    }
  };

  handleKeyUp = (e: KeyboardEvent) => {
    if (e.ctrlKey || e.shiftKey || e.altKey || e.metaKey) {
      return;
    }
    if (capturedKeys.includes(e.code)) {
      e.preventDefault();
    }
    if (e.repeat || !this.started || this.game.finished || this.paused) {
      return;
    }
    if (e.code === "ArrowLeft") {
      this.leftPressed = false;
    } else if (e.code === "ArrowRight") {
      this.rightPressed = false;
    } else if (e.code === "ArrowDown") {
      this.downPressed = false;
    }
  };

  tick = () => {
    if (this.started && !this.paused && !this.game.finished) {
      if (this.leftPressed) {
        if (this.leftTimer.tick()) {
          this.game.apply("shift-left");
        }
      } else if (this.rightPressed) {
        if (this.rightTimer.tick()) {
          this.game.apply("shift-right");
        }
      }
      if (this.downPressed) {
        if (this.downTimer.tick()) {
          this.game.apply("shift-down");
        }
      }
      if (this.gravityTimer.tick()) {
        this.game.gravityShift();
      }
    }
    if (this.statusTextCountdown > 0) {
      this.statusTextCountdown--;
      if (this.statusTextCountdown === 0) {
        this.statusText = "";
      }
    }

    this.renderer.render({
      game: this.game,
      paused: this.paused,
      statusText: this.statusText,
    });
    this.interval = requestAnimationFrame(this.tick);
  };
}
