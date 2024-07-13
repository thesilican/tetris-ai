import { Vec2 } from "./math";
import tileOSvg from "../assets/tile-o.svg";
import tileISvg from "../assets/tile-i.svg";
import tileTSvg from "../assets/tile-t.svg";
import tileLSvg from "../assets/tile-l.svg";
import tileJSvg from "../assets/tile-j.svg";
import tileSSvg from "../assets/tile-s.svg";
import tileZSvg from "../assets/tile-z.svg";
import tileGraySvg from "../assets/tile-gray.svg";

export class Sprite {
  dim: Vec2;
  ready: Promise<void>;
  image: HTMLImageElement;

  constructor(url: string, dim: Vec2) {
    this.dim = dim;
    this.image = new Image(dim.x, dim.y);
    this.ready = new Promise((res) => {
      this.image.src = url;
      this.image.addEventListener("load", () => res());
    });
  }
}

export const SPRITE_O = new Sprite(tileOSvg, new Vec2(100, 100));
export const SPRITE_I = new Sprite(tileISvg, new Vec2(100, 100));
export const SPRITE_T = new Sprite(tileTSvg, new Vec2(100, 100));
export const SPRITE_L = new Sprite(tileLSvg, new Vec2(100, 100));
export const SPRITE_J = new Sprite(tileJSvg, new Vec2(100, 100));
export const SPRITE_S = new Sprite(tileSSvg, new Vec2(100, 100));
export const SPRITE_Z = new Sprite(tileZSvg, new Vec2(100, 100));
export const SPRITE_GRAY = new Sprite(tileGraySvg, new Vec2(100, 100));
