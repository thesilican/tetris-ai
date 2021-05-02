import { CLIAI } from "./cli";

export class RustyAI extends CLIAI {
  constructor() {
    const command = "rust/rusty-ai/target/release/rusty-ai";
    const args = [
      "api",
      "false",
      // Downstacker:
      "PuDw3r2oNtK9TeZhPpwa3Lvq4G++x7VAvs9SFb8YPAI+P3qy",
      // Points:
      // "Pl3vz78Jv2G+FHNmvU3rWD6tNxu9ws5aO6jcXr8v4am+V8l9",
    ];
    super(command, args);
  }
}
