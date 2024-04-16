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
