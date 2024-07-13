import { GameRenderer } from "../render/game-renderer";

export class WelcomePlayer {
  interval: number | undefined;
  renderer: GameRenderer;

  constructor(renderer: GameRenderer) {
    this.renderer = renderer;
  }

  start() {
    this.interval = requestAnimationFrame(this.tick);
  }

  stop() {
    if (this.interval !== undefined) {
      cancelAnimationFrame(this.interval);
      this.interval = undefined;
    }
  }

  tick = () => {
    this.renderer.renderWelcome();
    this.interval = requestAnimationFrame(this.tick);
  };
}
