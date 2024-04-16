import { Rect, Vec2 } from "./math";

export class Canvas {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  public dim = new Vec2(0, 0);

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    const ctx = canvas.getContext("2d");
    if (ctx === null) {
      throw new Error("Cannot obtain 2d context");
    }
    this.ctx = ctx;
    this.updateDim();
    window.addEventListener("resize", this.handleResize.bind(this));
  }

  private updateDim() {
    this.dim.x = this.canvas.clientWidth * window.devicePixelRatio;
    this.dim.y = this.canvas.clientHeight * window.devicePixelRatio;
    this.canvas.width = this.dim.x;
    this.canvas.height = this.dim.y;
  }

  private handleResize() {
    this.updateDim();
  }

  drawRect(rect: Rect, color = "black") {
    this.ctx.fillStyle = color;
    this.ctx.fillRect(rect.min.x, rect.min.y, rect.width, rect.height);
  }

  drawText(text: string, position: Vec2, fontSize = 24, color = "black") {
    this.ctx.font = `${fontSize}px Arial`;
    const metrics = this.ctx.measureText(text);
    const height =
      metrics.actualBoundingBoxAscent + metrics.actualBoundingBoxDescent;
    // this.ctx.fillStyle = "red";
    // this.ctx.fillRect(position.x, position.y, metrics.width, height);
    this.ctx.fillStyle = color;
    this.ctx.fillText(text, position.x, position.y + height);
  }

  clear() {
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
  }
}
