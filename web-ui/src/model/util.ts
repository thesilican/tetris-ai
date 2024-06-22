export class Xorshift {
  state: number;
  constructor(seed: number) {
    this.state = seed | 0;
    if (this.state === 0) {
      this.state = 1;
    }
    // Run a few times to warm up the generator
    for (let i = 0; i < 10; i++) {
      this.next();
    }
  }

  next() {
    this.state ^= this.state << 13;
    this.state ^= this.state << 17;
    this.state ^= this.state << 5;
    return this.state >>> 0;
  }
}

export class DasTimer {
  maxDas: number;
  maxArr: number;
  das: number;
  arr: number;

  constructor(das: number, arr: number) {
    if (das < 0) {
      throw new Error("expected das to be >= 0");
    }
    if (arr < 1) {
      throw new Error("expected arr to be >= 1");
    }
    this.maxDas = das;
    this.maxArr = arr;
    this.das = das;
    this.arr = arr;
  }

  reset() {
    this.das = this.maxDas;
    this.arr = this.maxArr;
  }

  tick() {
    if (this.das > 0) {
      this.das--;
      return this.das === 0;
    }
    this.arr--;
    if (this.arr === 0) {
      this.arr = this.maxArr;
      return true;
    } else {
      return false;
    }
  }
}

export function generateSeed() {
  return Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
}
