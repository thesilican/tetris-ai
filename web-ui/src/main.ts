import "sanitize.css";
import "sanitize.css/typography.css";
import { Game } from "./model/model";
import { Canvas } from "./render/canvas";
import { GameRenderer } from "./render/game-renderer";
import "./styles.css";
import { DasTimer, generateSeed } from "./model/util";

const startGameButton = document.getElementById(
  "start-game"
) as HTMLButtonElement;
const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;
const canvas = new Canvas(canvasElement);
const gameRenderer = new GameRenderer(canvas);
const game = new Game();

startGameButton.addEventListener("click", () => {
  game.start(generateSeed());
});

let leftPressed = false;
let leftTimer = new DasTimer(8, 1);
let rightPressed = false;
let rightTimer = new DasTimer(8, 1);
let downPressed = false;
let downTimer = new DasTimer(0, 1);
let gravityTimer = new DasTimer(0, 60);

window.addEventListener("keydown", (event) => {
  if (event.repeat) {
    return;
  }
  if (event.ctrlKey || event.shiftKey || event.altKey || event.metaKey) {
    return;
  }
  if (!game.started) {
    return;
  }
  if (event.code === "KeyR") {
    event.preventDefault();
    game.start(generateSeed());
  }
  if (game.finished) {
    return;
  }
  if (event.code === "KeyP") {
    event.preventDefault();
    game.paused = !game.paused;
  }
  if (game.paused) {
    return;
  }
  if (event.code === "ArrowLeft") {
    event.preventDefault();
    game.active.shiftLeft(game.board);
    gravityTimer.reset();
    rightPressed = false;
    leftPressed = true;
    leftTimer.reset();
  } else if (event.code === "ArrowRight") {
    event.preventDefault();
    game.active.shiftRight(game.board);
    gravityTimer.reset();
    leftPressed = false;
    rightPressed = true;
    rightTimer.reset();
  } else if (event.code === "ArrowDown") {
    event.preventDefault();
    game.active.shiftDown(game.board);
    gravityTimer.reset();
    downPressed = true;
    downTimer.reset();
  } else if (event.code === "Space") {
    event.preventDefault();
    game.hardDrop();
    gravityTimer.reset();
    leftTimer.reset();
    rightTimer.reset();
  } else if (event.code === "KeyZ") {
    event.preventDefault();
    game.active.rotateCcw(game.board);
    gravityTimer.reset();
  } else if (event.code === "KeyX") {
    event.preventDefault();
    game.active.rotateCw(game.board);
    gravityTimer.reset();
  } else if (event.code === "KeyA") {
    event.preventDefault();
    game.active.rotate180(game.board);
    gravityTimer.reset();
  } else if (event.code === "KeyC") {
    event.preventDefault();
    game.swapHold();
    gravityTimer.reset();
  }
});

window.addEventListener("keyup", (event) => {
  if (event.repeat) {
    return;
  }
  if (event.ctrlKey || event.shiftKey || event.altKey || event.metaKey) {
    return;
  }
  if (!game.started || game.finished || game.paused) {
    return;
  }
  if (event.code === "ArrowLeft") {
    event.preventDefault();
    leftPressed = false;
  } else if (event.code === "ArrowRight") {
    event.preventDefault();
    rightPressed = false;
  } else if (event.code === "ArrowDown") {
    event.preventDefault();
    downPressed = false;
  }
});

function tick() {
  if (game.started && !game.paused && !game.finished) {
    if (leftPressed) {
      if (leftTimer.tick()) {
        game.active.shiftLeft(game.board);
      }
    } else if (rightPressed) {
      if (rightTimer.tick()) {
        game.active.shiftRight(game.board);
      }
    }
    if (downPressed) {
      if (downTimer.tick()) {
        game.active.shiftDown(game.board);
      }
    }
    if (gravityTimer.tick()) {
      game.gravityShift();
    }
  }

  gameRenderer.draw(game);
  requestAnimationFrame(tick);
}
requestAnimationFrame(tick);
