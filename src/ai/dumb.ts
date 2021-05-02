import type { AIMove, AIRequest, AIResponse, ITetrisAI } from "./index";

export class DumbAI implements ITetrisAI {
  async evaluate(_: AIRequest): Promise<AIResponse> {
    let moves: AIMove[] = [];
    if (Math.random() < 0.5) {
      moves.push("hold");
    }
    switch (Math.floor(Math.random() * 4)) {
      case 0:
        break;
      case 1:
        moves.push("rotateLeft");
        break;
      case 2:
        moves.push("rotate180");
        break;
      case 3:
        moves.push("rotateRight");
        break;
      default:
        throw new Error("This should never happen");
    }
    let shift: AIMove = Math.random() < 0.5 ? "shiftLeft" : "shiftRight";
    let shiftAmount = Math.floor(Math.random() * 4);
    for (let i = 0; i < shiftAmount; i++) {
      moves.push(shift);
    }
    moves.push("hardDrop");
    return { moves, score: 0 };
  }
}
