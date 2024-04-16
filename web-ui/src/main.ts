import "sanitize.css";
import "sanitize.css/typography.css";
import { Game } from "./model/model";
import { Canvas } from "./render/canvas";
import { GameRenderer } from "./render/game-renderer";
import "./styles.css";

const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;
const canvas = new Canvas(canvasElement);
const gameRenderer = new GameRenderer(canvas);
const game = new Game();
game.start(123);

const DAS = 10;
const ARR = 0;
let leftPressed = false;
let leftDas = 0;
let leftArr = 0;
let rightPressed = false;
let rightDas = 0;
let rightArr = 0;

window.addEventListener("keydown", (event) => {
  if (event.repeat) {
    return;
  }
  if (event.ctrlKey || event.shiftKey || event.altKey || event.metaKey) {
    return;
  }
  event.preventDefault();
  if (event.code === "ArrowLeft") {
    game.active.shiftLeft(game.board);
    rightPressed = false;
    leftPressed = true;
    leftDas = DAS;
    leftArr = 0;
  } else if (event.code === "ArrowRight") {
    game.active.shiftRight(game.board);
    leftPressed = false;
    rightPressed = true;
    rightDas = DAS;
    rightArr = 0;
  } else if (event.code === "ArrowDown") {
    game.active.softDrop(game.board);
  } else if (event.code === "Space") {
    game.hardDrop();
    leftDas = DAS;
    rightDas = DAS;
    leftArr = 0;
    rightDas = 0;
  } else if (event.code === "KeyZ") {
    game.active.rotateCcw(game.board);
  } else if (event.code === "KeyX") {
    game.active.rotateCw(game.board);
  } else if (event.code === "KeyC") {
    game.swapHold();
  }
  console.log(event);
});
window.addEventListener("keyup", (event) => {
  if (event.repeat) {
    return;
  }
  if (event.ctrlKey || event.shiftKey || event.altKey || event.metaKey) {
    return;
  }
  event.preventDefault();
  if (event.code === "ArrowLeft") {
    leftPressed = false;
  } else if (event.code === "ArrowRight") {
    rightPressed = false;
  }
});

function tick() {
  if (leftPressed) {
    if (leftDas > 1) {
      leftDas--;
    } else if (leftArr > 1) {
      leftArr--;
    } else {
      if (ARR === 0) {
        game.active.dasLeft(game.board);
      } else {
        leftArr = ARR;
        game.active.shiftLeft(game.board);
      }
    }
  } else if (rightPressed) {
    if (rightDas > 1) {
      rightDas--;
    } else if (rightArr > 1) {
      rightArr--;
    } else {
      if (ARR === 0) {
        game.active.dasRight(game.board);
      } else {
        rightArr = ARR;
        game.active.shiftRight(game.board);
      }
    }
  }

  gameRenderer.draw(game);
  requestAnimationFrame(tick);
}
requestAnimationFrame(tick);
