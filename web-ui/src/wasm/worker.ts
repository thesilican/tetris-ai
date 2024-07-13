import { evaluate } from "web-wasm";
import { RequestMessage, ResponseMessage } from "./types";
import { Action } from "../model/model";

self.addEventListener("message", (e: MessageEvent<RequestMessage>) => {
  if (e.data.type === "evaluate") {
    const start = performance.now();
    const ai = e.data.ai;
    const game = JSON.stringify(e.data.game);
    const response = evaluate(ai, game);
    const end = performance.now();
    const elapsed = Math.max(Math.ceil(end - start), 1);

    const message: ResponseMessage = {
      id: e.data.id,
      type: "evaluate",
      success: response.success(),
      actions: response.actions() as Action[],
      message: `Time: ${elapsed} ms\n${response.message()}`,
    };
    self.postMessage(message);
    response.free();
  }
});

self.postMessage({ type: "ready" } as ResponseMessage);
