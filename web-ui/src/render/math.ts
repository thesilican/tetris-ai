/**
 * Represents a 2-dimensional vector.
 */
export class Vec2 {
  x: number;
  y: number;

  constructor(vec: Vec2);
  constructor(x: number, y: number);
  constructor(x: number | Vec2, y?: number) {
    if (x instanceof Vec2) {
      this.x = x.x;
      this.y = x.y;
    } else if (typeof x === "number" && typeof y === "number") {
      this.x = x;
      this.y = y;
    } else {
      throw new Error("Invalid arguments");
    }
  }

  add(other: Vec2): Vec2 {
    return new Vec2(this.x + other.x, this.y + other.y);
  }

  sub(other: Vec2): Vec2 {
    return new Vec2(this.x - other.x, this.y - other.y);
  }

  scale(other: number): Vec2 {
    return new Vec2(this.x * other, this.y * other);
  }

  toString(precision = 1) {
    const x = this.x.toFixed(precision).padStart(6);
    const y = this.y.toFixed(precision).padStart(6);
    return `(${x}, ${y})`;
  }
}

/**
 * Represents a rectangle.
 */
export class Rect {
  min: Vec2;
  max: Vec2;

  get width() {
    return this.max.x - this.min.x;
  }

  get height() {
    return this.max.y - this.min.y;
  }

  get dim() {
    return new Vec2(this.width, this.height);
  }

  constructor(min: Vec2, max: Vec2);
  constructor(x: number, y: number, w: number, h: number);
  constructor(x: Vec2 | number, y: Vec2 | number, w?: number, h?: number) {
    if (x instanceof Vec2 && y instanceof Vec2) {
      this.min = x;
      this.max = y;
    } else if (
      typeof x === "number" &&
      typeof y === "number" &&
      typeof w === "number" &&
      typeof h === "number"
    ) {
      this.min = new Vec2(x, y);
      this.max = new Vec2(x + w, y + h);
    } else {
      throw new Error("Invalid arguments");
    }
  }

  inBounds(vec: Vec2) {
    return (
      vec.x >= this.min.x &&
      vec.x < this.max.x &&
      vec.y >= this.min.y &&
      vec.y < this.max.y
    );
  }
}

/**
 * Clamp a number between a minimum and maximum value.
 */
export function clamp(x: number, min: number, max: number) {
  return Math.max(min, Math.min(x, max));
}
