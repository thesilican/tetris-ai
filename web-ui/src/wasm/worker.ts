import { evaluate, init_pc_finder } from "web-wasm";
import { Action } from "../model/model";
import { RequestMessage, ResponseMessage } from "./types";

// Load PC Table
(async () => {
  const url = new URL(
    import.meta.env.BASE_URL + "./pc-table.bin",
    self.location.origin
  );
  const res = await fetch(url);
  if (res.status === 200) {
    const array = new Uint8Array(await res.arrayBuffer());
    init_pc_finder(array);
    console.log("Loaded PC Table");
  } else {
    throw new Error("Failed to fetch pc-table.bin");
  }
})();

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
