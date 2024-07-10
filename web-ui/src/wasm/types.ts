import type { Action, Game } from "../model/model";

export type RequestMessage = {
  type: "evaluate";
  id: number;
  ai: string;
  game: Game;
};

export type ResponseMessage =
  | {
      type: "ready";
    }
  | {
      type: "evaluate";
      id: number;
      success: boolean;
      actions: Action[];
      message: string;
    };
