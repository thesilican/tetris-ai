import { Action, Game } from "../model/model";
import { DasTimer, generateSeed } from "../model/util";
import { GameRenderer } from "../render/game-renderer";
import type { RequestMessage, ResponseMessage } from "../wasm/types";

let idCounter = 0;

export class AiPlayer {
  aiType: string;
  game: Game;
  renderer: GameRenderer;
  worker: Worker;
  pre: HTMLPreElement;
  paused = false;
  started = false;
  timer = new DasTimer(0, 1);
  requestId: number | undefined;
  actionsQueue: Action[] = [];
  interval: number | undefined;

  constructor(
    aiType: string,
    renderer: GameRenderer,
    worker: Worker,
    pre: HTMLPreElement
  ) {
    this.aiType = aiType;
    this.game = new Game();
    this.renderer = renderer;
    this.worker = worker;
    this.pre = pre;
  }

  start() {
    this.started = true;
    this.actionsQueue = [];
    this.requestId = undefined;
    this.game.start(generateSeed());
    this.interval = requestAnimationFrame(this.tick);
    this.worker.addEventListener("message", this.handleMessage);
    window.addEventListener("keydown", this.handleKeyDown);
  }

  stop() {
    this.started = false;
    if (this.interval !== undefined) {
      cancelAnimationFrame(this.interval);
      this.interval = undefined;
    }
    this.worker.removeEventListener("message", this.handleMessage);
    window.removeEventListener("keydown", this.handleKeyDown);
  }

  setSpeed(speed: number) {
    if (speed < 1 || speed > 10) {
      throw new Error("speed out of range");
    }
    // Speed from 1-10
    this.timer = new DasTimer(0, 11 - speed);
  }

  handleKeyDown = (e: KeyboardEvent) => {
    if (e.repeat || e.ctrlKey || e.altKey || e.shiftKey || e.metaKey) {
      return;
    }
    if (e.code === "KeyP") {
      this.paused = !this.paused;
    }
    if (e.code === "KeyR") {
      this.actionsQueue = [];
      this.game.start(generateSeed());
      this.requestId = undefined;
    }
  };

  handleMessage = (e: MessageEvent<ResponseMessage>) => {
    if (e.data.type === "evaluate") {
      if (e.data.id !== this.requestId) {
        return;
      }
      this.actionsQueue.push(...e.data.actions);
      this.requestId = undefined;
      this.pre.innerText = e.data.message;
    } else if (e.data.type === "ready") {
      // Reset evaluation in case request was sent before worker was ready
      this.requestId = undefined;
    }
  };

  tick = () => {
    if (!this.game.finished && !this.paused) {
      if (this.timer.tick()) {
        const action = this.actionsQueue.shift();
        if (action) {
          this.game.apply(action);
        } else {
          if (!this.requestId) {
            this.requestId = idCounter++;
            const message: RequestMessage = {
              type: "evaluate",
              ai: this.aiType,
              game: this.game,
              id: this.requestId,
            };
            this.worker.postMessage(message);
          }
        }
      }
    }
    this.renderer.render({ game: this.game, paused: this.paused });
    this.interval = requestAnimationFrame(this.tick);
  };
}
