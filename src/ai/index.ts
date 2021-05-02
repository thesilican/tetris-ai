export type AIRequest = {
  matrix: boolean[][];
  queue: number[];
  current: number;
  hold: number | null;
};
export type AIMove =
  | "shiftLeft"
  | "shiftRight"
  | "rotateLeft"
  | "rotateRight"
  | "rotate180"
  | "hold"
  | "softDrop"
  | "hardDrop";
export type AIResponse = {
  moves: AIMove[];
  score: number | null;
};
export interface ITetrisAI {
  evaluate(req: AIRequest): Promise<AIResponse | null>;
}

export function objIsAIRequest(obj: any): obj is AIRequest {
  if (
    typeof obj !== "object" ||
    obj === null ||
    typeof obj.hasOwnProperty !== "function"
  ) {
    // console.log("1");
    return false;
  }
  if (
    !obj.hasOwnProperty("current") ||
    !obj.hasOwnProperty("hold") ||
    !obj.hasOwnProperty("queue") ||
    !obj.hasOwnProperty("matrix")
  ) {
    // console.log("2");
    return false;
  }

  function isPiece(obj: any): boolean {
    return (
      typeof obj === "number" && Number.isInteger(obj) && obj >= 0 && obj < 7
    );
  }

  if (!isPiece(obj.current)) {
    // console.log("3");
    return false;
  }
  if (!isPiece(obj.hold) && obj.hold !== null) {
    // console.log("4");
    return false;
  }
  const queue = obj.queue;
  if (!Array.isArray(queue)) {
    return false;
  }
  for (const queueObj of queue) {
    if (!isPiece(queueObj)) {
      // console.log("6");
      return false;
    }
  }
  const matrix = obj.matrix;
  if (!Array.isArray(matrix) || matrix.length !== 10) {
    // console.log("7");
    return false;
  }
  for (const row of matrix) {
    if (!Array.isArray(row) || row.length !== 20) {
      // console.log("8");
      return false;
    }
    for (const el of row) {
      if (typeof el !== "boolean") {
        // console.log("9");
        return false;
      }
    }
  }
  return true;
}
export function objIsAIResponse(obj: any): obj is AIResponse {
  if (
    typeof obj !== "object" ||
    obj === null ||
    typeof obj.hasOwnProperty !== "function"
  ) {
    return false;
  }
  if (typeof obj.score !== "number" && obj.score !== null) {
    return false;
  }

  const moves = obj.moves;
  if (!Array.isArray(moves)) {
    return false;
  }
  for (const move of moves) {
    if (
      move !== "shiftLeft" &&
      move !== "shiftRight" &&
      move !== "rotateLeft" &&
      move !== "rotateRight" &&
      move !== "rotate180" &&
      move !== "hold" &&
      move !== "softDrop" &&
      move !== "hardDrop"
    ) {
      return false;
    }
  }
  return true;
}

export { RustyAI } from "./rusty";
export { DumbAI } from "./dumb";
